use anchor_lang::prelude::*;

use crate::constants::IDENTITY_CONFIG_SEED;
use crate::errors::IdentityError;
use crate::state::IdentityConfig;

/// Context for accepting a pending authority transfer (H-02 security fix)
#[derive(Accounts)]
pub struct AcceptAuthority<'info> {
    #[account(
        mut,
        seeds = [IDENTITY_CONFIG_SEED],
        bump = identity_config.bump,
        constraint = identity_config.pending_authority == new_authority.key() @ IdentityError::NotPendingAuthority,
        constraint = identity_config.pending_authority != Pubkey::default() @ IdentityError::NoPendingTransfer
    )]
    pub identity_config: Account<'info, IdentityConfig>,
    
    /// The new authority accepting the transfer - must sign
    pub new_authority: Signer<'info>,
}

