use crate::constants::GASLESS_CONFIG_SEED;
use crate::errors::GaslessError;
use crate::state::GaslessConfig;
use anchor_lang::prelude::*;

#[derive(Accounts)]
pub struct UpdateConfig<'info> {
    #[account(
        mut,
        seeds = [GASLESS_CONFIG_SEED],
        bump = config.bump,
        has_one = authority @ GaslessError::Unauthorized
    )]
    pub config: Account<'info, GaslessConfig>,

    pub authority: Signer<'info>,
}
