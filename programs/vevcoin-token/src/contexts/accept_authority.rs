use anchor_lang::prelude::*;

use crate::constants::VEVCOIN_CONFIG_SEED;
use crate::errors::VeVCoinError;
use crate::state::VeVCoinConfig;

/// Context for accepting a pending authority transfer (H-02 security fix)
#[derive(Accounts)]
pub struct AcceptAuthority<'info> {
    #[account(
        mut,
        seeds = [VEVCOIN_CONFIG_SEED],
        bump = config.bump,
        constraint = config.pending_authority == new_authority.key() @ VeVCoinError::NotPendingAuthority,
        constraint = config.pending_authority != Pubkey::default() @ VeVCoinError::NoPendingTransfer
    )]
    pub config: Account<'info, VeVCoinConfig>,
    
    /// The new authority accepting the transfer - must sign
    pub new_authority: Signer<'info>,
}

