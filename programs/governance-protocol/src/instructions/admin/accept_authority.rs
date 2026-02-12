use anchor_lang::prelude::*;
use crate::constants::AUTHORITY_TRANSFER_TIMELOCK;
use crate::contexts::AcceptAuthority;
use crate::errors::GovernanceError;

/// Accept authority transfer (step 2 of two-step transfer - H-02 security fix)
pub fn handler(ctx: Context<AcceptAuthority>) -> Result<()> {
    let config = &mut ctx.accounts.governance_config;

    // H-02: Enforce 24-hour timelock before authority transfer can be accepted
    let clock = Clock::get()?;
    require!(
        clock.unix_timestamp >= config.pending_authority_activated_at + AUTHORITY_TRANSFER_TIMELOCK,
        GovernanceError::AuthorityTransferTimelock
    );

    let old_authority = config.authority;
    let new_authority = ctx.accounts.new_authority.key();

    config.authority = new_authority;
    config.pending_authority = Pubkey::default();
    config.pending_authority_activated_at = 0;

    msg!("Authority transferred from {} to {}", old_authority, new_authority);

    Ok(())
}

