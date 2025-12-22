use anchor_lang::prelude::*;
use crate::constants::*;
use crate::state::{RegistryConfig, ContentRecord};

#[derive(Accounts)]
pub struct DeleteContent<'info> {
    #[account(
        mut,
        seeds = [REGISTRY_CONFIG_SEED],
        bump = registry_config.bump
    )]
    pub registry_config: Account<'info, RegistryConfig>,
    
    #[account(
        mut,
        seeds = [CONTENT_RECORD_SEED, content_record.tracking_id.as_ref()],
        bump = content_record.bump,
        has_one = author
    )]
    pub content_record: Account<'info, ContentRecord>,
    
    pub author: Signer<'info>,
}

