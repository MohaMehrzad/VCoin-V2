use anchor_lang::prelude::*;
use crate::constants::*;
use crate::contexts::EditContent;
use crate::errors::ContentError;
use crate::events::ContentEdited;
use crate::state::ContentState;
use crate::utils::regenerate_energy;

pub fn handler(
    ctx: Context<EditContent>,
    new_content_hash: [u8; 32],
    new_content_uri: String,
) -> Result<()> {
    let content = &mut ctx.accounts.content_record;
    
    require!(
        content.state != ContentState::Deleted as u8,
        ContentError::CannotEditDeleted
    );
    require!(new_content_uri.len() <= 128, ContentError::ContentURITooLong);
    
    let clock = Clock::get()?;
    let time_since_creation = clock.unix_timestamp - content.created_at;
    
    // Free edits within 1 hour of creation
    if time_since_creation > FREE_EDIT_WINDOW {
        let user_energy = &mut ctx.accounts.user_energy;
        regenerate_energy(user_energy, clock.unix_timestamp)?;
        
        require!(
            user_energy.current_energy >= ENERGY_COST_EDIT_AFTER_1H,
            ContentError::InsufficientEnergy
        );
        
        user_energy.current_energy = user_energy.current_energy.saturating_sub(ENERGY_COST_EDIT_AFTER_1H);
    }
    
    // Store previous hash for history
    content.previous_hash = content.content_hash;
    content.content_hash = new_content_hash;
    
    let uri_bytes = new_content_uri.as_bytes();
    content.content_uri = [0u8; 128];
    content.content_uri[..uri_bytes.len()].copy_from_slice(uri_bytes);
    content.uri_len = uri_bytes.len() as u8;
    
    content.state = ContentState::Edited as u8;
    content.version = content.version.saturating_add(1);
    content.updated_at = clock.unix_timestamp;
    
    emit!(ContentEdited {
        tracking_id: content.tracking_id,
        author: content.author,
        version: content.version,
        new_hash: new_content_hash,
        timestamp: clock.unix_timestamp,
    });
    
    msg!("Content edited: version {}", content.version);
    Ok(())
}

