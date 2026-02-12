use anchor_lang::prelude::*;

use crate::contexts::UpdateConfig;
use crate::errors::VeVCoinError;
use crate::events::AuthorityTransferProposed;

/// Propose a new authority (step 1 of two-step transfer - H-02 security fix)
pub fn handler(ctx: Context<UpdateConfig>, new_authority: Pubkey) -> Result<()> {
    let config = &mut ctx.accounts.config;
    
    require!(
        ctx.accounts.authority.key() == config.authority,
        VeVCoinError::Unauthorized
    );
    
    require!(
        new_authority != config.authority,
        VeVCoinError::CannotProposeSelf
    );
    
    require!(
        new_authority != Pubkey::default(),
        VeVCoinError::InvalidAuthority
    );
    
    config.pending_authority = new_authority;

    let clock = Clock::get()?;

    // C-01: Record activation timestamp for timelock enforcement
    config.pending_authority_activated_at = clock.unix_timestamp;
    
    // L-01: Emit authority transfer proposed event
    emit!(AuthorityTransferProposed {
        current_authority: config.authority,
        proposed_authority: new_authority,
        timestamp: clock.unix_timestamp,
    });
    
    msg!("Authority transfer proposed to: {}", new_authority);
    
    Ok(())
}

