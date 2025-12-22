use anchor_lang::prelude::*;
use crate::contexts::UpdateEngagement;

pub fn handler(
    ctx: Context<UpdateEngagement>,
    engagement_count: u32,
) -> Result<()> {
    let content = &mut ctx.accounts.content_record;
    content.engagement_count = engagement_count;
    
    msg!("Engagement updated: {}", engagement_count);
    Ok(())
}

