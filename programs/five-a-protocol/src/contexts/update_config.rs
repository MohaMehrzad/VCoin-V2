use anchor_lang::prelude::*;

use crate::constants::FIVE_A_CONFIG_SEED;
use crate::errors::FiveAError;
use crate::state::FiveAConfig;

#[derive(Accounts)]
pub struct UpdateConfig<'info> {
    #[account(
        mut,
        seeds = [FIVE_A_CONFIG_SEED],
        bump = five_a_config.bump,
        has_one = authority @ FiveAError::Unauthorized
    )]
    pub five_a_config: Account<'info, FiveAConfig>,
    
    pub authority: Signer<'info>,
}

