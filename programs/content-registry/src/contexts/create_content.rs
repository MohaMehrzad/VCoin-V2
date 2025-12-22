use anchor_lang::prelude::*;
use crate::constants::*;
use crate::state::{RegistryConfig, ContentRecord, UserEnergy, UserRateLimit};

#[derive(Accounts)]
#[instruction(tracking_id: [u8; 32])]
pub struct CreateContent<'info> {
    #[account(
        mut,
        seeds = [REGISTRY_CONFIG_SEED],
        bump = registry_config.bump
    )]
    pub registry_config: Account<'info, RegistryConfig>,
    
    #[account(
        init,
        payer = author,
        space = ContentRecord::LEN,
        seeds = [CONTENT_RECORD_SEED, tracking_id.as_ref()],
        bump
    )]
    pub content_record: Account<'info, ContentRecord>,
    
    #[account(
        mut,
        seeds = [USER_ENERGY_SEED, author.key().as_ref()],
        bump = user_energy.bump
    )]
    pub user_energy: Account<'info, UserEnergy>,
    
    #[account(
        init_if_needed,
        payer = author,
        space = UserRateLimit::LEN,
        seeds = [RATE_LIMIT_SEED, author.key().as_ref()],
        bump
    )]
    pub rate_limit: Account<'info, UserRateLimit>,
    
    #[account(mut)]
    pub author: Signer<'info>,
    
    pub system_program: Program<'info, System>,
}

