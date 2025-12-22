use anchor_lang::prelude::*;

use crate::contexts::UpdateConfig;

/// Pause/unpause protocol
pub fn handler(ctx: Context<UpdateConfig>, paused: bool) -> Result<()> {
    ctx.accounts.identity_config.paused = paused;
    msg!("Identity protocol paused: {}", paused);
    Ok(())
}

