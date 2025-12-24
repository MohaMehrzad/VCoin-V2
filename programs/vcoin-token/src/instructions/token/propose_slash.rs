use anchor_lang::prelude::*;

use crate::constants::SLASH_STATUS_PENDING;
use crate::contexts::ProposeSlash;
use crate::errors::VCoinError;
use crate::events::SlashProposed;

/// Propose a slash request (H-01 Security Fix)
/// Only the permanent delegate can propose slashes
/// Slashes require governance approval + 48h timelock before execution
pub fn handler(
    ctx: Context<ProposeSlash>,
    target: Pubkey,
    request_id: u64,
    amount: u64,
    reason_hash: [u8; 32],
) -> Result<()> {
    let clock = Clock::get()?;
    
    // C-01 Security Fix: Validate request_id matches current timestamp
    // This ensures PDA seeds are consistent between propose/approve/execute
    // since approve_slash and execute_slash use created_at (which equals clock.unix_timestamp)
    require!(
        request_id == clock.unix_timestamp as u64,
        VCoinError::InvalidRequestId
    );
    
    require!(amount > 0, VCoinError::ZeroSlashAmount);
    
    // Verify target has sufficient balance
    require!(
        ctx.accounts.target_account.amount >= amount,
        VCoinError::SlashingExceedsBalance
    );
    
    let slash_request = &mut ctx.accounts.slash_request;
    
    slash_request.target = target;
    slash_request.amount = amount;
    slash_request.reason_hash = reason_hash;
    slash_request.proposer = ctx.accounts.authority.key();
    slash_request.proposal_id = 0; // Set when approved by governance
    slash_request.status = SLASH_STATUS_PENDING;
    slash_request.created_at = clock.unix_timestamp;
    slash_request.timelock_end = 0; // Set when approved
    slash_request.executed_at = 0;
    slash_request.bump = ctx.bumps.slash_request;
    
    // L-01: Emit slash proposed event
    emit!(SlashProposed {
        target,
        amount,
        reason_hash,
        proposer: ctx.accounts.authority.key(),
        timestamp: clock.unix_timestamp,
    });
    
    msg!("Slash proposed: {} VCoin from {}", amount, target);
    msg!("Awaiting governance approval...");
    
    Ok(())
}

