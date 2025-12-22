use anchor_lang::prelude::*;

use crate::constants::{MIN_LOCK_DURATION, MAX_LOCK_DURATION};
use crate::contexts::ExtendLock;
use crate::errors::StakingError;
use crate::events::LockExtended;
use crate::state::StakingTier;
use crate::utils::calculate_vevcoin;

/// Extend lock duration to increase veVCoin
pub fn handler(ctx: Context<ExtendLock>, new_lock_duration: i64) -> Result<()> {
    let user_stake = &mut ctx.accounts.user_stake;
    
    require!(user_stake.staked_amount > 0, StakingError::NoActiveStake);
    require!(new_lock_duration >= MIN_LOCK_DURATION, StakingError::LockDurationTooShort);
    require!(new_lock_duration <= MAX_LOCK_DURATION, StakingError::LockDurationTooLong);
    
    let clock = Clock::get()?;
    let now = clock.unix_timestamp;
    
    // Calculate new lock end
    let new_lock_end = now.checked_add(new_lock_duration).ok_or(StakingError::Overflow)?;
    
    // New lock end must be after current lock end
    require!(new_lock_end > user_stake.lock_end, StakingError::CannotShortenLock);
    
    // Calculate new veVCoin
    let tier = StakingTier::from_amount(user_stake.staked_amount);
    let old_vevcoin = user_stake.ve_vcoin_amount;
    let new_vevcoin = calculate_vevcoin(user_stake.staked_amount, new_lock_duration, tier)?;
    let vevcoin_to_mint = new_vevcoin.checked_sub(old_vevcoin).unwrap_or(0);
    
    // Capture old_lock_end before updating
    let old_lock_end = user_stake.lock_end;
    
    // Update stake
    user_stake.lock_duration = new_lock_duration;
    user_stake.lock_end = new_lock_end;
    user_stake.ve_vcoin_amount = new_vevcoin;
    
    // L-01: Emit lock extension event
    emit!(LockExtended {
        user: ctx.accounts.user.key(),
        old_lock_end,
        new_lock_end,
        new_vevcoin,
        timestamp: now,
    });
    
    msg!("Extended lock to {} seconds", new_lock_duration);
    msg!("New lock end: {}", new_lock_end);
    msg!("Additional veVCoin: {}", vevcoin_to_mint);
    msg!("Total veVCoin: {}", new_vevcoin);
    
    Ok(())
}

