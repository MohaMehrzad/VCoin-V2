use anchor_lang::prelude::*;
use crate::constants::GASLESS_CONFIG_SEED;
use crate::state::GaslessConfig;

#[derive(Accounts)]
pub struct GetConfigStats<'info> {
    #[account(
        seeds = [GASLESS_CONFIG_SEED],
        bump = config.bump
    )]
    pub config: Account<'info, GaslessConfig>,
}

