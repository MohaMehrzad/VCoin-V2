use anchor_lang::prelude::*;

use crate::contexts::AcceptAuthority;
use crate::events::AuthorityTransferAccepted;

/// Accept authority transfer (step 2 of two-step transfer - H-02 security fix)
/// The pending authority must sign to accept the transfer
pub fn handler(ctx: Context<AcceptAuthority>) -> Result<()> {
    let pool = &mut ctx.accounts.pool;
    
    let old_authority = pool.authority;
    let new_authority = ctx.accounts.new_authority.key();
    
    // Transfer authority
    pool.authority = new_authority;
    pool.pending_authority = Pubkey::default();
    
    let clock = Clock::get()?;
    
    // L-01: Emit authority transfer accepted event
    emit!(AuthorityTransferAccepted {
        old_authority,
        new_authority,
        timestamp: clock.unix_timestamp,
    });
    
    msg!("Authority transferred from {} to {}", old_authority, new_authority);
    
    Ok(())
}

