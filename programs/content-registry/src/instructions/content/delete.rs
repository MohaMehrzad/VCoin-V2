use anchor_lang::prelude::*;
use crate::contexts::DeleteContent;
use crate::errors::ContentError;
use crate::events::ContentDeleted;
use crate::state::ContentState;

pub fn handler(ctx: Context<DeleteContent>) -> Result<()> {
    let content = &mut ctx.accounts.content_record;
    
    require!(
        content.state != ContentState::Deleted as u8,
        ContentError::ContentAlreadyDeleted
    );
    
    let clock = Clock::get()?;
    
    content.state = ContentState::Deleted as u8;
    content.updated_at = clock.unix_timestamp;
    
    // Update registry stats
    let config = &mut ctx.accounts.registry_config;
    config.active_content_count = config.active_content_count.saturating_sub(1);
    
    emit!(ContentDeleted {
        tracking_id: content.tracking_id,
        author: content.author,
        timestamp: clock.unix_timestamp,
    });
    
    msg!("Content deleted");
    Ok(())
}

