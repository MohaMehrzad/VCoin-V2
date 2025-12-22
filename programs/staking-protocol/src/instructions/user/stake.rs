use anchor_lang::prelude::*;
use anchor_spl::token_2022;

use crate::constants::{MIN_LOCK_DURATION, MAX_LOCK_DURATION};
use crate::contexts::Stake;
use crate::errors::StakingError;
use crate::state::StakingTier;
use crate::utils::calculate_vevcoin;

/// Stake VCoin with a lock duration
/// Mints veVCoin proportional to stake amount, lock duration, and tier
pub fn handler(ctx: Context<Stake>, amount: u64, lock_duration: i64) -> Result<()> {
    let pool = &mut ctx.accounts.pool;
    let user_stake = &mut ctx.accounts.user_stake;
    
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
    
    // Update user stake
    let is_new_staker = user_stake.staked_amount == 0;
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
    
    msg!("Staked {} VCoin", amount);
    msg!("Lock duration: {} seconds", lock_duration);
    msg!("Tier: {:?}", new_tier.as_u8());
    msg!("veVCoin minted: {}", vevcoin_to_mint);
    msg!("Total veVCoin: {}", new_vevcoin);
    
    Ok(())
}

