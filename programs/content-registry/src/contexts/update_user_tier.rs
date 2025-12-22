use anchor_lang::prelude::*;
use crate::constants::*;
use crate::errors::ContentError;
use crate::state::{UserEnergy, RegistryConfig};

#[derive(Accounts)]
pub struct UpdateUserTier<'info> {
    #[account(
        mut,
        seeds = [USER_ENERGY_SEED, user_energy.user.as_ref()],
        bump = user_energy.bump
    )]
    pub user_energy: Account<'info, UserEnergy>,
    
    #[account(
        seeds = [REGISTRY_CONFIG_SEED],
        bump = registry_config.bump,
        has_one = authority @ ContentError::Unauthorized
    )]
    pub registry_config: Account<'info, RegistryConfig>,
    
    pub authority: Signer<'info>,
}

