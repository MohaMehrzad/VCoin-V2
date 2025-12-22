use anchor_lang::prelude::*;

use crate::constants::VCOIN_CONFIG_SEED;
use crate::state::VCoinConfig;

#[derive(Accounts)]
pub struct UpdateConfig<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,
    
    #[account(
        mut,
        seeds = [VCOIN_CONFIG_SEED],
        bump = config.bump
    )]
    pub config: Account<'info, VCoinConfig>,
}

