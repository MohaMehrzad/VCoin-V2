use anchor_lang::prelude::*;

use crate::constants::{SLASH_STATUS_APPROVED, SLASH_TIMELOCK_SECONDS};
use crate::contexts::ApproveSlash;
use crate::events::SlashApproved;

/// Approve a slash request (H-01 Security Fix)
/// Only governance authority can approve
/// Starts the 48 hour timelock before execution is allowed
pub fn handler(ctx: Context<ApproveSlash>, proposal_id: u64) -> Result<()> {
    let clock = Clock::get()?;
    let slash_request = &mut ctx.accounts.slash_request;
    
    slash_request.proposal_id = proposal_id;
    slash_request.status = SLASH_STATUS_APPROVED;
    slash_request.timelock_end = clock.unix_timestamp + SLASH_TIMELOCK_SECONDS;
    
    // L-01: Emit slash approved event
    emit!(SlashApproved {
        target: slash_request.target,
        amount: slash_request.amount,
        approver: ctx.accounts.authority.key(),
        timelock_end: slash_request.timelock_end,
        timestamp: clock.unix_timestamp,
    });
    
    msg!("Slash request approved by governance (proposal {})", proposal_id);
    msg!("Timelock ends at: {}", slash_request.timelock_end);
    msg!("48 hour waiting period started...");
    
    Ok(())
}

