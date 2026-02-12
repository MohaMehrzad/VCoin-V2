use anchor_lang::prelude::*;
use crate::contexts::UpdateAuthority;
use crate::errors::GovernanceError;

/// Propose a new authority (step 1 of two-step transfer - H-02 security fix)
pub fn handler(ctx: Context<UpdateAuthority>, new_authority: Pubkey) -> Result<()> {
    let config = &mut ctx.accounts.governance_config;

    require!(
        new_authority != config.authority,
        GovernanceError::CannotProposeSelf
    );

    require!(
        new_authority != Pubkey::default(),
        GovernanceError::InvalidAuthority
    );

    config.pending_authority = new_authority;

    // H-02: Record activation timestamp for timelock enforcement
    let clock = Clock::get()?;
    config.pending_authority_activated_at = clock.unix_timestamp;

    msg!("Authority transfer proposed to: {}", new_authority);
    Ok(())
}

