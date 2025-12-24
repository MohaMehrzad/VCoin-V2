use anchor_lang::prelude::*;

use crate::constants::AUTHORITY_TRANSFER_TIMELOCK;
use crate::contexts::AcceptAuthority;
use crate::errors::VCoinError;
use crate::events::AuthorityTransferAccepted;

/// Accept authority transfer (step 2 of two-step transfer - H-02 security fix)
/// H-NEW-01: Enforces 24-hour timelock before acceptance
pub fn handler(ctx: Context<AcceptAuthority>) -> Result<()> {
    let config = &mut ctx.accounts.config;
    let clock = Clock::get()?;
    
    // H-NEW-01: Verify 24-hour timelock has elapsed since proposal
    require!(
        clock.unix_timestamp >= config.pending_authority_activated_at + AUTHORITY_TRANSFER_TIMELOCK,
        VCoinError::AuthorityTransferTimelock
    );
    
    let old_authority = config.authority;
    let new_authority = ctx.accounts.new_authority.key();
    
    config.authority = new_authority;
    config.pending_authority = Pubkey::default();
    config.pending_authority_activated_at = 0;
    
    // L-01: Emit authority transfer accepted event
    emit!(AuthorityTransferAccepted {
        old_authority,
        new_authority,
        timestamp: clock.unix_timestamp,
    });
    
    msg!("Authority transferred from {} to {}", old_authority, new_authority);
    
    Ok(())
}

