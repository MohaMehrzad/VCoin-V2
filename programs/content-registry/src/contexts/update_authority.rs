use anchor_lang::prelude::*;
use crate::constants::REGISTRY_CONFIG_SEED;
use crate::errors::ContentError;
use crate::state::RegistryConfig;

#[derive(Accounts)]
pub struct UpdateAuthority<'info> {
    #[account(
        mut,
        seeds = [REGISTRY_CONFIG_SEED],
        bump = registry_config.bump,
        has_one = authority @ ContentError::Unauthorized
    )]
    pub registry_config: Account<'info, RegistryConfig>,
    
    pub authority: Signer<'info>,
}

