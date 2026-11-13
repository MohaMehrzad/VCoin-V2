use crate::contexts::UpdateConfig;
use anchor_lang::prelude::*;

pub fn handler(ctx: Context<UpdateConfig>, paused: bool) -> Result<()> {
    ctx.accounts.config.paused = paused;
    msg!("Gasless Protocol paused: {}", paused);
    Ok(())
}
