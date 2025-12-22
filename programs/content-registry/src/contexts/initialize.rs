use anchor_lang::prelude::*;
use crate::constants::REGISTRY_CONFIG_SEED;
use crate::state::RegistryConfig;

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(
        init,
        payer = authority,
        space = RegistryConfig::LEN,
        seeds = [REGISTRY_CONFIG_SEED],
        bump
    )]
    pub registry_config: Account<'info, RegistryConfig>,
    
    #[account(mut)]
    pub authority: Signer<'info>,
    
    pub system_program: Program<'info, System>,
}

