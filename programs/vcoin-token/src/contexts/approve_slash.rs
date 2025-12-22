use anchor_lang::prelude::*;

use crate::constants::{VCOIN_CONFIG_SEED, SLASH_REQUEST_SEED};
use crate::errors::VCoinError;
use crate::state::{VCoinConfig, SlashRequest};

/// Context for approving a slash request (H-01 Security Fix)
/// Only the governance authority can approve slashes
#[derive(Accounts)]
pub struct ApproveSlash<'info> {
    #[account(
        seeds = [VCOIN_CONFIG_SEED],
        bump = config.bump,
        // Governance approval requires authority or governance program
        constraint = authority.key() == config.authority @ VCoinError::GovernanceApprovalRequired
    )]
    pub config: Account<'info, VCoinConfig>,
    
    #[account(
        mut,
        seeds = [SLASH_REQUEST_SEED, slash_request.target.as_ref(), &slash_request.created_at.to_le_bytes()],
        bump = slash_request.bump,
        constraint = slash_request.is_pending() @ VCoinError::InvalidSlashStatus
    )]
    pub slash_request: Account<'info, SlashRequest>,
    
    /// The governance authority approving the slash
    pub authority: Signer<'info>,
}

