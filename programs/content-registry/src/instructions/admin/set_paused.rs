use anchor_lang::prelude::*;
use crate::contexts::UpdateConfig;

pub fn handler(ctx: Context<UpdateConfig>, paused: bool) -> Result<()> {
    ctx.accounts.registry_config.paused = paused;
    msg!("Content registry paused: {}", paused);
    Ok(())
}

