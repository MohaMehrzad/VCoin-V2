use anchor_lang::prelude::*;
use crate::contexts::UpdateEngagement;
use crate::errors::ContentError;

pub fn handler(
    ctx: Context<UpdateEngagement>,
    engagement_count: u32,
) -> Result<()> {
    let content = &mut ctx.accounts.content_record;

    // C-AUDIT-10: Enforce monotonic engagement increase — engagement can never decrease
    require!(
        engagement_count >= content.engagement_count,
        ContentError::EngagementCannotDecrease
    );

    content.engagement_count = engagement_count;

    msg!("Engagement updated: {}", engagement_count);
    Ok(())
}

