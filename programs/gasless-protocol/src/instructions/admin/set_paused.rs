use anchor_lang::prelude::*;
use crate::contexts::UpdateConfig;

pub fn handler(ctx: Context<UpdateConfig>, paused: bool) -> Result<()> {
    ctx.accounts.config.paused = paused;
    msg!("Gasless Protocol paused: {}", paused);
    Ok(())
}

