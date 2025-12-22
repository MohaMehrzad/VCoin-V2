use anchor_lang::prelude::*;

use crate::constants::VCOIN_CONFIG_SEED;
use crate::errors::VCoinError;
use crate::state::VCoinConfig;

/// Context for accepting a pending authority transfer (H-02 security fix)
#[derive(Accounts)]
pub struct AcceptAuthority<'info> {
    #[account(
        mut,
        seeds = [VCOIN_CONFIG_SEED],
        bump = config.bump,
        constraint = config.pending_authority == new_authority.key() @ VCoinError::NotPendingAuthority,
        constraint = config.pending_authority != Pubkey::default() @ VCoinError::NoPendingTransfer
    )]
    pub config: Account<'info, VCoinConfig>,
    
    /// The new authority accepting the transfer - must sign
    pub new_authority: Signer<'info>,
}

