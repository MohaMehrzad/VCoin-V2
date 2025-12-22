use anchor_lang::prelude::*;

use crate::constants::STAKING_POOL_SEED;
use crate::errors::StakingError;
use crate::state::StakingPool;

/// Context for accepting a pending authority transfer (H-02 security fix)
/// The new authority must sign to accept the transfer
#[derive(Accounts)]
pub struct AcceptAuthority<'info> {
    #[account(
        mut,
        seeds = [STAKING_POOL_SEED],
        bump = pool.bump,
        constraint = pool.pending_authority == new_authority.key() @ StakingError::NotPendingAuthority,
        constraint = pool.pending_authority != Pubkey::default() @ StakingError::NoPendingTransfer
    )]
    pub pool: Account<'info, StakingPool>,
    
    /// The new authority accepting the transfer - must sign
    pub new_authority: Signer<'info>,
}

