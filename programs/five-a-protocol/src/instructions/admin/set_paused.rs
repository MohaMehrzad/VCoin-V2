use anchor_lang::prelude::*;

use crate::contexts::UpdateConfig;

/// Pause/unpause protocol
pub fn handler(ctx: Context<UpdateConfig>, paused: bool) -> Result<()> {
    ctx.accounts.five_a_config.paused = paused;
    msg!("5A Protocol paused: {}", paused);
    Ok(())
}

