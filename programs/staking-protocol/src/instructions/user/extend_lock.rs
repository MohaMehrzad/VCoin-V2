use anchor_lang::prelude::*;

use crate::constants::{MIN_LOCK_DURATION, MAX_LOCK_DURATION, STAKING_POOL_SEED};
use crate::contexts::ExtendLock;
use crate::errors::StakingError;
use crate::events::LockExtended;
use crate::state::StakingTier;
use crate::utils::calculate_vevcoin;

/// Extend lock duration to increase veVCoin
/// C-04 Security Fix: Added veVCoin CPI minting
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
    
    // Capture pool bump before CPI
    let pool_bump = ctx.bumps.pool;
    
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
    
    // Update stake
    let user_stake = &mut ctx.accounts.user_stake;
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
