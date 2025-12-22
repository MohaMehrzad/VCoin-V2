use anchor_lang::prelude::*;
use crate::constants::*;
use crate::errors::ContentError;
use crate::state::{ContentRecord, RegistryConfig};

#[derive(Accounts)]
pub struct UpdateEngagement<'info> {
    #[account(
        mut,
        seeds = [CONTENT_RECORD_SEED, content_record.tracking_id.as_ref()],
        bump = content_record.bump
    )]
    pub content_record: Account<'info, ContentRecord>,
    
    #[account(
        seeds = [REGISTRY_CONFIG_SEED],
        bump = registry_config.bump,
        has_one = authority @ ContentError::Unauthorized
    )]
    pub registry_config: Account<'info, RegistryConfig>,
    
    pub authority: Signer<'info>,
}

