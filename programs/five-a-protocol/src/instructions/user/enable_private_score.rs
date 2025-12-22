use anchor_lang::prelude::*;

use crate::contexts::UpdateUserScore;

/// Enable private score mode
pub fn handler(ctx: Context<UpdateUserScore>) -> Result<()> {
    ctx.accounts.user_score.is_private = true;
    msg!("Private score mode enabled");
    Ok(())
}

