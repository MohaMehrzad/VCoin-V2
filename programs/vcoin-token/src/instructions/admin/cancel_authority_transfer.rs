use anchor_lang::prelude::*;

use crate::contexts::UpdateConfig;
use crate::errors::VCoinError;
use crate::events::AuthorityTransferCancelled;

/// Cancel a pending authority transfer (H-02 security fix)
pub fn handler(ctx: Context<UpdateConfig>) -> Result<()> {
    let config = &mut ctx.accounts.config;
    
    require!(
        ctx.accounts.authority.key() == config.authority,
        VCoinError::Unauthorized
    );
    
    require!(
        config.pending_authority != Pubkey::default(),
        VCoinError::NoPendingTransfer
    );
    
    let cancelled = config.pending_authority;
    config.pending_authority = Pubkey::default();
    
    let clock = Clock::get()?;
    
    // L-01: Emit authority transfer cancelled event
    emit!(AuthorityTransferCancelled {
        authority: config.authority,
        cancelled_pending: cancelled,
        timestamp: clock.unix_timestamp,
    });
    
    msg!("Authority transfer to {} cancelled", cancelled);
    
    Ok(())
}

