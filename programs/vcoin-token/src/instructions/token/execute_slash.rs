use anchor_lang::prelude::*;
use anchor_spl::token_2022;

use crate::constants::{VCOIN_CONFIG_SEED, SLASH_STATUS_EXECUTED};
use crate::contexts::ExecuteSlash;
use crate::errors::VCoinError;

/// Execute an approved slash (H-01 Security Fix)
/// Requires:
/// 1. Slash request to be approved by governance
/// 2. 48 hour timelock to have expired
/// 3. Permanent delegate to sign
pub fn handler(ctx: Context<ExecuteSlash>) -> Result<()> {
    let clock = Clock::get()?;
    let slash_request = &mut ctx.accounts.slash_request;
    let config = &ctx.accounts.config;
    
    // Verify timelock has expired
    require!(
        slash_request.is_timelock_expired(clock.unix_timestamp),
        VCoinError::TimelockNotExpired
    );
    
    // Verify target still has sufficient balance
    require!(
        ctx.accounts.target_account.amount >= slash_request.amount,
        VCoinError::SlashingExceedsBalance
    );
    
    // Burn the slashed tokens using permanent delegate authority
    let seeds = &[
        VCOIN_CONFIG_SEED,
        &[config.bump],
    ];
    let signer_seeds = &[&seeds[..]];
    
    token_2022::burn(
        CpiContext::new_with_signer(
            ctx.accounts.token_program.to_account_info(),
            token_2022::Burn {
                mint: ctx.accounts.mint.to_account_info(),
                from: ctx.accounts.target_account.to_account_info(),
                authority: config.to_account_info(),
            },
            signer_seeds,
        ),
        slash_request.amount,
    )?;
    
    // Mark as executed
    slash_request.status = SLASH_STATUS_EXECUTED;
    slash_request.executed_at = clock.unix_timestamp;
    
    // L-01: Emit slash executed event
    emit!(crate::events::SlashExecuted {
        target: slash_request.target,
        amount: slash_request.amount,
        executor: ctx.accounts.executor.key(),
        timestamp: clock.unix_timestamp,
    });
    
    msg!("Slash executed: {} VCoin burned from {}", 
        slash_request.amount, slash_request.target);
    
    Ok(())
}

