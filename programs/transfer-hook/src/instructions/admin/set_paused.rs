use anchor_lang::prelude::*;

use crate::contexts::UpdateConfig;

/// Pause/unpause the hook
pub fn handler(ctx: Context<UpdateConfig>, paused: bool) -> Result<()> {
    ctx.accounts.hook_config.paused = paused;
    msg!("Hook paused status: {}", paused);
    Ok(())
}

