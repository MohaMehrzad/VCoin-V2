use anchor_lang::prelude::*;
use crate::constants::REGISTRY_CONFIG_SEED;
use crate::errors::ContentError;
use crate::state::RegistryConfig;

/// Context for accepting a pending authority transfer (H-02 security fix)
#[derive(Accounts)]
pub struct AcceptAuthority<'info> {
    #[account(
        mut,
        seeds = [REGISTRY_CONFIG_SEED],
        bump = registry_config.bump,
        constraint = registry_config.pending_authority == new_authority.key() @ ContentError::NotPendingAuthority,
        constraint = registry_config.pending_authority != Pubkey::default() @ ContentError::NoPendingTransfer
    )]
    pub registry_config: Account<'info, RegistryConfig>,
    
    /// The new authority accepting the transfer - must sign
    pub new_authority: Signer<'info>,
}

