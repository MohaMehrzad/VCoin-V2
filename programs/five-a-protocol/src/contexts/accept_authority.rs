use anchor_lang::prelude::*;

use crate::constants::FIVE_A_CONFIG_SEED;
use crate::errors::FiveAError;
use crate::state::FiveAConfig;

/// Context for accepting a pending authority transfer (H-02 security fix)
#[derive(Accounts)]
pub struct AcceptAuthority<'info> {
    #[account(
        mut,
        seeds = [FIVE_A_CONFIG_SEED],
        bump = five_a_config.bump,
        constraint = five_a_config.pending_authority == new_authority.key() @ FiveAError::NotPendingAuthority,
        constraint = five_a_config.pending_authority != Pubkey::default() @ FiveAError::NoPendingTransfer
    )]
    pub five_a_config: Account<'info, FiveAConfig>,
    
    /// The new authority accepting the transfer - must sign
    pub new_authority: Signer<'info>,
}

