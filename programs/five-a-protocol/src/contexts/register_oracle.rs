use anchor_lang::prelude::*;

use crate::constants::{FIVE_A_CONFIG_SEED, ORACLE_SEED};
use crate::errors::FiveAError;
use crate::state::{FiveAConfig, Oracle};

#[derive(Accounts)]
pub struct RegisterOracle<'info> {
    #[account(
        mut,
        seeds = [FIVE_A_CONFIG_SEED],
        bump = five_a_config.bump,
        has_one = authority @ FiveAError::Unauthorized
    )]
    pub five_a_config: Account<'info, FiveAConfig>,
    
    #[account(
        init,
        payer = authority,
        space = Oracle::LEN,
        seeds = [ORACLE_SEED, oracle_wallet.key().as_ref()],
        bump
    )]
    pub oracle: Account<'info, Oracle>,
    
    /// CHECK: Oracle wallet to register
    pub oracle_wallet: UncheckedAccount<'info>,
    
    #[account(mut)]
    pub authority: Signer<'info>,
    
    pub system_program: Program<'info, System>,
}

