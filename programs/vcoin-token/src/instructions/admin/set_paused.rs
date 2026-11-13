use anchor_lang::prelude::*;

use crate::contexts::UpdateConfig;
use crate::events::PauseChanged;

pub fn handler(ctx: Context<UpdateConfig>, paused: bool) -> Result<()> {
    let config = &mut ctx.accounts.config;
    config
        .admin
        .require_authority(&ctx.accounts.authority.key())?;
    config.paused = paused;
    emit!(PauseChanged { paused });
    Ok(())
}
