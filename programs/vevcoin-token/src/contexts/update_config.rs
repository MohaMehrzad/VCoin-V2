use anchor_lang::prelude::*;

use crate::constants::VEVCOIN_CONFIG_SEED;
use crate::state::VeVCoinConfig;

#[derive(Accounts)]
pub struct UpdateConfig<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,
    
    #[account(
        mut,
        seeds = [VEVCOIN_CONFIG_SEED],
        bump
    )]
    pub config: Account<'info, VeVCoinConfig>,
}

