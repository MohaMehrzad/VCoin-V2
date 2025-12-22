use anchor_lang::prelude::*;
use anchor_spl::token_2022;

use crate::constants::STAKING_POOL_SEED;
use crate::contexts::Unstake;
use crate::errors::StakingError;
use crate::events::Unstaked;
use crate::state::StakingTier;

/// Unstake VCoin after lock expires
/// Burns veVCoin proportionally
/// M-01 Security Fix: Added reentrancy guard for CPI protection
pub fn handler(ctx: Context<Unstake>, amount: u64) -> Result<()> {
    // M-01: Check reentrancy guard before proceeding
    require!(!ctx.accounts.pool.reentrancy_guard, StakingError::ReentrancyDetected);
    
    // Get values for validation first
    let staked_amount = ctx.accounts.user_stake.staked_amount;
    let lock_end = ctx.accounts.user_stake.lock_end;
    let ve_vcoin_amount = ctx.accounts.user_stake.ve_vcoin_amount;
    
    require!(staked_amount > 0, StakingError::NoActiveStake);
    require!(amount > 0, StakingError::ZeroStakeAmount);
    require!(amount <= staked_amount, StakingError::InsufficientStake);
    
    let clock = Clock::get()?;
    let now = clock.unix_timestamp;
    
    // Check lock has expired
    require!(now >= lock_end, StakingError::TokensStillLocked);
    
    // Calculate new stake and tier
    let new_staked_amount = staked_amount.checked_sub(amount).ok_or(StakingError::Overflow)?;
    let new_tier = StakingTier::from_amount(new_staked_amount);
    
    // Calculate new veVCoin (0 if fully unstaking)
    let new_vevcoin = if new_staked_amount > 0 {
        // Maintain proportional veVCoin for remaining stake
        let remaining_ratio = (new_staked_amount as u128) * 1000 / (staked_amount as u128);
        (ve_vcoin_amount as u128 * remaining_ratio / 1000) as u64
    } else {
        0
    };
    let vevcoin_to_burn = ve_vcoin_amount.checked_sub(new_vevcoin).unwrap_or(0);
    
    // Get bump from context
    let pool_bump = ctx.bumps.pool;
    let current_total_stakers = ctx.accounts.pool.total_stakers;
    let current_total_staked = ctx.accounts.pool.total_staked;
    
    let seeds = &[
        STAKING_POOL_SEED,
        &[pool_bump],
    ];
    let signer_seeds = &[&seeds[..]];
    
    // M-01: Set reentrancy guard before CPI operations
    {
        let pool_mut = &mut ctx.accounts.pool;
        pool_mut.reentrancy_guard = true;
    }
    
    // === CRITICAL FIX C-04: Burn veVCoin via CPI FIRST ===
    if vevcoin_to_burn > 0 {
        vevcoin_token::cpi::burn_vevcoin(
            CpiContext::new_with_signer(
                ctx.accounts.vevcoin_program.to_account_info(),
                vevcoin_token::cpi::accounts::BurnVeVCoin {
                    staking_protocol: ctx.accounts.pool.to_account_info(),
                    user: ctx.accounts.user.to_account_info(),
                    config: ctx.accounts.vevcoin_config.to_account_info(),
                    user_account: ctx.accounts.user_vevcoin.to_account_info(),
                    mint: ctx.accounts.vevcoin_mint.to_account_info(),
                    user_token_account: ctx.accounts.user_vevcoin_account.to_account_info(),
                    token_program: ctx.accounts.token_program.to_account_info(),
                },
                signer_seeds,
            ),
            vevcoin_to_burn,
        )?;
    }
    // === END CRITICAL FIX ===
    
    // Transfer VCoin back to user
    token_2022::transfer_checked(
        CpiContext::new_with_signer(
            ctx.accounts.token_program.to_account_info(),
            token_2022::TransferChecked {
                from: ctx.accounts.pool_vault.to_account_info(),
                to: ctx.accounts.user_vcoin_account.to_account_info(),
                authority: ctx.accounts.pool.to_account_info(),
                mint: ctx.accounts.vcoin_mint.to_account_info(),
            },
            signer_seeds,
        ),
        amount,
        ctx.accounts.vcoin_mint.decimals,
    )?;
    
    // Update state after CPI
    let user_stake = &mut ctx.accounts.user_stake;
    let pool = &mut ctx.accounts.pool;
    
    // Update user stake
    let is_full_unstake = new_staked_amount == 0;
    user_stake.staked_amount = new_staked_amount;
    user_stake.tier = new_tier.as_u8();
    user_stake.ve_vcoin_amount = new_vevcoin;
    
    if is_full_unstake {
        user_stake.lock_duration = 0;
        user_stake.lock_end = 0;
        pool.total_stakers = current_total_stakers.checked_sub(1).ok_or(StakingError::Overflow)?;
    }
    
    // Update pool
    pool.total_staked = current_total_staked.checked_sub(amount).ok_or(StakingError::Overflow)?;
    
    // M-01: Clear reentrancy guard after CPI operations complete
    pool.reentrancy_guard = false;
    
    // L-01: Emit unstaking event
    emit!(Unstaked {
        user: ctx.accounts.user.key(),
        amount,
        vevcoin_burned: vevcoin_to_burn,
        remaining_stake: new_staked_amount,
        timestamp: now,
    });
    
    msg!("Unstaked {} VCoin", amount);
    msg!("veVCoin burned: {}", vevcoin_to_burn);
    msg!("Remaining stake: {}", new_staked_amount);
    
    Ok(())
}

