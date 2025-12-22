use anchor_lang::prelude::*;
use crate::contexts::UpdateConfig;

pub fn handler(ctx: Context<UpdateConfig>, paused: bool) -> Result<()> {
    ctx.accounts.governance_config.paused = paused;
    msg!("Governance paused: {}", paused);
    Ok(())
}

