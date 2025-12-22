use anchor_lang::prelude::*;
use crate::contexts::GetContent;

pub fn handler(ctx: Context<GetContent>) -> Result<()> {
    let content = &ctx.accounts.content_record;
    msg!("Tracking ID: {:?}", content.tracking_id);
    msg!("Author: {}", content.author);
    msg!("Type: {}", content.content_type);
    msg!("State: {}", content.state);
    msg!("Version: {}", content.version);
    Ok(())
}

