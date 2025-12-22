use anchor_lang::prelude::*;
use crate::constants::GASLESS_CONFIG_SEED;
use crate::errors::GaslessError;
use crate::state::GaslessConfig;

/// Context for accepting a pending authority transfer (H-02 security fix)
#[derive(Accounts)]
pub struct AcceptAuthority<'info> {
    #[account(
        mut,
        seeds = [GASLESS_CONFIG_SEED],
        bump = config.bump,
        constraint = config.pending_authority == new_authority.key() @ GaslessError::NotPendingAuthority,
        constraint = config.pending_authority != Pubkey::default() @ GaslessError::NoPendingTransfer
    )]
    pub config: Account<'info, GaslessConfig>,
    
    /// The new authority accepting the transfer - must sign
    pub new_authority: Signer<'info>,
}

