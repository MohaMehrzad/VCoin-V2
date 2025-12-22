use anchor_lang::prelude::*;
use crate::contexts::CreateContent;
use crate::errors::ContentError;
use crate::events::{ContentCreated, EnergySpent};
use crate::state::{ContentType, ContentState};
use crate::utils::{regenerate_energy, check_and_update_rate_limit};

pub fn handler(
    ctx: Context<CreateContent>,
    tracking_id: [u8; 32],
    content_hash: [u8; 32],
    content_uri: String,
    content_type: u8,
) -> Result<()> {
    require!(!ctx.accounts.registry_config.paused, ContentError::RegistryPaused);
    require!(content_uri.len() <= 128, ContentError::ContentURITooLong);
    
    let content_type_enum = ContentType::from_u8(content_type)
        .ok_or(ContentError::InvalidContentType)?;
    
    let clock = Clock::get()?;
    
    // Check and spend energy
    let user_energy = &mut ctx.accounts.user_energy;
    let energy_cost = content_type_enum.energy_cost();
    
    // Regenerate energy first
    regenerate_energy(user_energy, clock.unix_timestamp)?;
    
    require!(
        user_energy.current_energy >= energy_cost,
        ContentError::InsufficientEnergy
    );
    
    // Check rate limit
    let rate_limit = &mut ctx.accounts.rate_limit;
    check_and_update_rate_limit(rate_limit, user_energy.tier, clock.unix_timestamp)?;
    
    // Spend energy
    user_energy.current_energy = user_energy.current_energy.saturating_sub(energy_cost);
    user_energy.energy_spent_today = user_energy.energy_spent_today.saturating_add(energy_cost as u32);
    
    // Create content record
    let content = &mut ctx.accounts.content_record;
    content.tracking_id = tracking_id;
    content.author = ctx.accounts.author.key();
    content.content_hash = content_hash;
    
    let uri_bytes = content_uri.as_bytes();
    content.content_uri[..uri_bytes.len()].copy_from_slice(uri_bytes);
    content.uri_len = uri_bytes.len() as u8;
    
    content.content_type = content_type;
    content.state = ContentState::Active as u8;
    content.version = 1;
    content.created_at = clock.unix_timestamp;
    content.updated_at = clock.unix_timestamp;
    content.previous_hash = [0u8; 32];
    content.energy_spent = energy_cost;
    content.refund_claimed = false;
    content.engagement_count = 0;
    content.bump = ctx.bumps.content_record;
    
    // Update registry stats
    let config = &mut ctx.accounts.registry_config;
    config.total_content_count = config.total_content_count.saturating_add(1);
    config.active_content_count = config.active_content_count.saturating_add(1);
    
    emit!(ContentCreated {
        tracking_id,
        author: content.author,
        content_type,
        content_hash,
        timestamp: clock.unix_timestamp,
    });
    
    emit!(EnergySpent {
        user: content.author,
        amount: energy_cost,
        action: "create_content".to_string(),
        remaining: user_energy.current_energy,
    });
    
    msg!("Content created: {:?}", tracking_id);
    Ok(())
}

