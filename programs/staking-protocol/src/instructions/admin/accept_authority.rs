use anchor_lang::prelude::*;

use crate::constants::AUTHORITY_TRANSFER_TIMELOCK;
use crate::contexts::AcceptAuthority;
use crate::errors::StakingError;
use crate::events::AuthorityTransferAccepted;

/// Accept authority transfer (step 2 of two-step transfer - H-02 security fix)
/// The pending authority must sign to accept the transfer
/// H-NEW-01: Enforces 24-hour timelock before acceptance
pub fn handler(ctx: Context<AcceptAuthority>) -> Result<()> {
    let pool = &mut ctx.accounts.pool;
    let clock = Clock::get()?;
    
    // H-NEW-01: Verify 24-hour timelock has elapsed since proposal
    require!(
        clock.unix_timestamp >= pool.pending_authority_activated_at + AUTHORITY_TRANSFER_TIMELOCK,
        StakingError::AuthorityTransferTimelock
    );
    
    let old_authority = pool.authority;
    let new_authority = ctx.accounts.new_authority.key();
    
    // Transfer authority
    pool.authority = new_authority;
    pool.pending_authority = Pubkey::default();
    pool.pending_authority_activated_at = 0;
    
    // L-01: Emit authority transfer accepted event
    emit!(AuthorityTransferAccepted {
        old_authority,
        new_authority,
        timestamp: clock.unix_timestamp,
    });
    
    msg!("Authority transferred from {} to {}", old_authority, new_authority);
    
    Ok(())
}

