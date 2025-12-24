use anchor_lang::prelude::*;

use crate::contexts::UpdateConfig;
use crate::errors::VCoinError;
use crate::events::AuthorityTransferProposed;

/// Propose a new authority (step 1 of two-step transfer - H-02 security fix)
/// H-NEW-01: Sets the activation timestamp for 24h timelock enforcement
pub fn handler(ctx: Context<UpdateConfig>, new_authority: Pubkey) -> Result<()> {
    let config = &mut ctx.accounts.config;
    
    require!(
        ctx.accounts.authority.key() == config.authority,
        VCoinError::Unauthorized
    );
    
    require!(
        new_authority != config.authority,
        VCoinError::CannotProposeSelf
    );
    
    require!(
        new_authority != Pubkey::default(),
        VCoinError::InvalidAuthority
    );
    
    let clock = Clock::get()?;
    
    config.pending_authority = new_authority;
    // H-NEW-01: Record timestamp for timelock enforcement
    config.pending_authority_activated_at = clock.unix_timestamp;
    
    // L-01: Emit authority transfer proposed event
    emit!(AuthorityTransferProposed {
        current_authority: config.authority,
        proposed_authority: new_authority,
        timestamp: clock.unix_timestamp,
    });
    
    msg!("Authority transfer proposed to: {} (active after 24h timelock)", new_authority);
    
    Ok(())
}

