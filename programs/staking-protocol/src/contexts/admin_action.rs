use anchor_lang::prelude::*;

use crate::constants::STAKING_POOL_SEED;
use crate::state::StakingPool;

#[derive(Accounts)]
pub struct AdminAction<'info> {
    pub authority: Signer<'info>,
    
    #[account(
        mut,
        seeds = [STAKING_POOL_SEED],
        bump
    )]
    pub pool: Account<'info, StakingPool>,
}

