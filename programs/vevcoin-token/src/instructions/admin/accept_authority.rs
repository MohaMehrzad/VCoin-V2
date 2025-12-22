use anchor_lang::prelude::*;

use crate::contexts::AcceptAuthority;
use crate::events::AuthorityTransferAccepted;

/// Accept authority transfer (step 2 of two-step transfer - H-02 security fix)
pub fn handler(ctx: Context<AcceptAuthority>) -> Result<()> {
    let config = &mut ctx.accounts.config;
    
    let old_authority = config.authority;
    let new_authority = ctx.accounts.new_authority.key();
    
    config.authority = new_authority;
    config.pending_authority = Pubkey::default();
    
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

