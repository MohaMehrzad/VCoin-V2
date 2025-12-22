use anchor_lang::prelude::*;
use anchor_spl::token_2022;

use crate::constants::STAKING_POOL_SEED;
use crate::contexts::Unstake;
use crate::errors::StakingError;
use crate::state::StakingTier;

/// Unstake VCoin after lock expires
/// Burns all veVCoin
pub fn handler(ctx: Context<Unstake>, amount: u64) -> Result<()> {
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
    
    // Transfer VCoin back to user
    let seeds = &[
        STAKING_POOL_SEED,
        &[pool_bump],
    ];
    let signer_seeds = &[&seeds[..]];
    
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
    
    msg!("Unstaked {} VCoin", amount);
    msg!("veVCoin burned: {}", vevcoin_to_burn);
    msg!("Remaining stake: {}", new_staked_amount);
    
    Ok(())
}

