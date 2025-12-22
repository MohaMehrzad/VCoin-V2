use anchor_lang::prelude::*;

use crate::contexts::UpdateUserScore;

/// Disable private score mode
pub fn handler(ctx: Context<UpdateUserScore>) -> Result<()> {
    ctx.accounts.user_score.is_private = false;
    msg!("Private score mode disabled");
    Ok(())
}

