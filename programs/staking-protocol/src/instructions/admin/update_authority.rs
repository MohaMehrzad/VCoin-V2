use anchor_lang::prelude::*;

use crate::contexts::AdminAction;
use crate::errors::StakingError;
use crate::events::AuthorityTransferProposed;

/// Propose a new authority (step 1 of two-step transfer - H-02 security fix)
/// The new authority must call accept_authority to complete the transfer
/// H-NEW-01: Sets the activation timestamp for 24h timelock enforcement
pub fn handler(ctx: Context<AdminAction>, new_authority: Pubkey) -> Result<()> {
    let pool = &mut ctx.accounts.pool;
    
    require!(
        ctx.accounts.authority.key() == pool.authority,
        StakingError::Unauthorized
    );
    
    // Cannot propose self
    require!(
        new_authority != pool.authority,
        StakingError::CannotProposeSelf
    );
    
    // Cannot propose zero address
    require!(
        new_authority != Pubkey::default(),
        StakingError::InvalidAuthority
    );
    
    let clock = Clock::get()?;
    
    pool.pending_authority = new_authority;
    // H-NEW-01: Record timestamp for timelock enforcement
    pool.pending_authority_activated_at = clock.unix_timestamp;
    
    // L-01: Emit authority transfer proposed event
    emit!(AuthorityTransferProposed {
        current_authority: pool.authority,
        proposed_authority: new_authority,
        timestamp: clock.unix_timestamp,
    });
    
    msg!("Authority transfer proposed to: {} (active after 24h timelock)", new_authority);
    
    Ok(())
}

