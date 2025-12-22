use anchor_lang::prelude::*;
use crate::contexts::DeleteContent;
use crate::errors::ContentError;
use crate::events::ContentDeleted;
use crate::state::ContentState;

/// Soft-delete content by changing state to Deleted (L-06 Documentation).
/// 
/// # Important Note
/// This is a **soft delete** operation. Content data remains on-chain for audit 
/// purposes. The account is NOT closed and data is still retrievable via 
/// on-chain reads.
/// 
/// This behavior is by design for:
/// - **Regulatory compliance**: Content history may be required for legal purposes
/// - **Dispute resolution**: Deleted content can be referenced in disputes
/// - **Audit trails**: Complete history of content lifecycle is preserved
/// 
/// For true data removal, consider using the account rent reclamation feature
/// after governance approval, or wait for the platform's data retention policy
/// to expire the data through a separate cleanup mechanism.
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

