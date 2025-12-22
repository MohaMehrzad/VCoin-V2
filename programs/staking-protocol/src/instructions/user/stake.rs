use anchor_lang::prelude::*;
use anchor_spl::token_2022;

use crate::constants::{MIN_LOCK_DURATION, MAX_LOCK_DURATION, STAKING_POOL_SEED};
use crate::contexts::Stake;
use crate::errors::StakingError;
use crate::events::Staked;
use crate::state::StakingTier;
use crate::utils::calculate_vevcoin;

/// Stake VCoin with a lock duration
/// Mints veVCoin proportional to stake amount, lock duration, and tier
/// M-01 Security Fix: Added reentrancy guard for CPI protection
pub fn handler(ctx: Context<Stake>, amount: u64, lock_duration: i64) -> Result<()> {
    let pool = &ctx.accounts.pool;
    let user_stake = &ctx.accounts.user_stake;
    
    // M-01: Check reentrancy guard before proceeding
    require!(!pool.reentrancy_guard, StakingError::ReentrancyDetected);
    
    // Validations
    require!(!pool.paused, StakingError::PoolPaused);
    require!(amount > 0, StakingError::ZeroStakeAmount);
    require!(lock_duration >= MIN_LOCK_DURATION, StakingError::LockDurationTooShort);
    require!(lock_duration <= MAX_LOCK_DURATION, StakingError::LockDurationTooLong);
    
    let clock = Clock::get()?;
    let now = clock.unix_timestamp;
    
    // Calculate new total stake and tier
    let new_staked_amount = user_stake.staked_amount
        .checked_add(amount)
        .ok_or(StakingError::Overflow)?;
    let new_tier = StakingTier::from_amount(new_staked_amount);
    
    // Calculate lock end
    let lock_end = now.checked_add(lock_duration).ok_or(StakingError::Overflow)?;
    
    // For existing stakes, new lock must not be shorter
    if user_stake.staked_amount > 0 && lock_end < user_stake.lock_end {
        return Err(StakingError::InvalidLockExtension.into());
    }
    
    // Calculate veVCoin to mint
    // If adding to existing stake, calculate delta
    let old_vevcoin = user_stake.ve_vcoin_amount;
    let new_vevcoin = calculate_vevcoin(new_staked_amount, lock_duration, new_tier)?;
    let vevcoin_to_mint = new_vevcoin.checked_sub(old_vevcoin).unwrap_or(0);
    
    // Capture pool bump before mutable borrow
    let pool_bump = ctx.bumps.pool;
    let is_new_staker = user_stake.staked_amount == 0;
    
    // M-01: Set reentrancy guard before CPI operations
    // Note: We set this via mutable borrow scope to ensure it's set
    {
        let pool_mut = &mut ctx.accounts.pool;
        pool_mut.reentrancy_guard = true;
    }
    
    // Transfer VCoin to pool vault
    token_2022::transfer_checked(
        CpiContext::new(
            ctx.accounts.token_program.to_account_info(),
            token_2022::TransferChecked {
                from: ctx.accounts.user_vcoin_account.to_account_info(),
                to: ctx.accounts.pool_vault.to_account_info(),
                authority: ctx.accounts.user.to_account_info(),
                mint: ctx.accounts.vcoin_mint.to_account_info(),
            },
        ),
        amount,
        ctx.accounts.vcoin_mint.decimals,
    )?;
    
    // === CRITICAL FIX C-04: Mint veVCoin via CPI ===
    if vevcoin_to_mint > 0 {
        let seeds = &[STAKING_POOL_SEED, &[pool_bump]];
        let signer_seeds = &[&seeds[..]];
        
        vevcoin_token::cpi::mint_vevcoin(
            CpiContext::new_with_signer(
                ctx.accounts.vevcoin_program.to_account_info(),
                vevcoin_token::cpi::accounts::MintVeVCoin {
                    staking_protocol: ctx.accounts.pool.to_account_info(),
                    user: ctx.accounts.user.to_account_info(),
                    config: ctx.accounts.vevcoin_config.to_account_info(),
                    user_account: ctx.accounts.user_vevcoin.to_account_info(),
                    mint: ctx.accounts.vevcoin_mint.to_account_info(),
                    user_token_account: ctx.accounts.user_vevcoin_account.to_account_info(),
                    payer: ctx.accounts.user.to_account_info(),
                    token_program: ctx.accounts.token_program.to_account_info(),
                    system_program: ctx.accounts.system_program.to_account_info(),
                },
                signer_seeds,
            ),
            vevcoin_to_mint,
        )?;
    }
    // === END CRITICAL FIX ===
    
    // Update user stake
    let user_stake = &mut ctx.accounts.user_stake;
    let pool = &mut ctx.accounts.pool;
    
    user_stake.owner = ctx.accounts.user.key();
    user_stake.staked_amount = new_staked_amount;
    user_stake.lock_duration = lock_duration;
    user_stake.lock_end = lock_end;
    user_stake.tier = new_tier.as_u8();
    user_stake.ve_vcoin_amount = new_vevcoin;
    
    if is_new_staker {
        user_stake.stake_start = now;
        user_stake.bump = ctx.bumps.user_stake;
        pool.total_stakers = pool.total_stakers.checked_add(1).ok_or(StakingError::Overflow)?;
    }
    
    // Update pool
    pool.total_staked = pool.total_staked.checked_add(amount).ok_or(StakingError::Overflow)?;
    
    // M-01: Clear reentrancy guard after CPI operations complete
    pool.reentrancy_guard = false;
    
    // L-01: Emit staking event
    emit!(Staked {
        user: ctx.accounts.user.key(),
        amount,
        lock_duration,
        vevcoin_minted: vevcoin_to_mint,
        tier: new_tier.as_u8(),
        timestamp: now,
    });
    
    msg!("Staked {} VCoin", amount);
    msg!("Lock duration: {} seconds", lock_duration);
    msg!("Tier: {:?}", new_tier.as_u8());
    msg!("veVCoin minted: {}", vevcoin_to_mint);
    msg!("Total veVCoin: {}", new_vevcoin);
    
    Ok(())
}

