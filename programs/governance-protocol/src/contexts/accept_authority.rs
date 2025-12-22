use anchor_lang::prelude::*;
use crate::constants::GOV_CONFIG_SEED;
use crate::errors::GovernanceError;
use crate::state::GovernanceConfig;

/// Context for accepting a pending authority transfer (H-02 security fix)
#[derive(Accounts)]
pub struct AcceptAuthority<'info> {
    #[account(
        mut,
        seeds = [GOV_CONFIG_SEED],
        bump = governance_config.bump,
        constraint = governance_config.pending_authority == new_authority.key() @ GovernanceError::NotPendingAuthority,
        constraint = governance_config.pending_authority != Pubkey::default() @ GovernanceError::NoPendingTransfer
    )]
    pub governance_config: Account<'info, GovernanceConfig>,
    
    /// The new authority accepting the transfer - must sign
    pub new_authority: Signer<'info>,
}

