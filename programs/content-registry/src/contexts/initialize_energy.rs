use anchor_lang::prelude::*;
use crate::constants::ENERGY_CONFIG_SEED;
use crate::state::EnergyConfig;

#[derive(Accounts)]
pub struct InitializeEnergy<'info> {
    #[account(
        init,
        payer = authority,
        space = EnergyConfig::LEN,
        seeds = [ENERGY_CONFIG_SEED],
        bump
    )]
    pub energy_config: Account<'info, EnergyConfig>,
    
    #[account(mut)]
    pub authority: Signer<'info>,
    
    pub system_program: Program<'info, System>,
}

