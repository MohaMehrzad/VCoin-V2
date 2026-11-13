use crate::contexts::UpdateConfig;
use anchor_lang::prelude::*;

pub fn handler(ctx: Context<UpdateConfig>, paused: bool) -> Result<()> {
    ctx.accounts.governance_config.paused = paused;
    msg!("Governance paused: {}", paused);
    Ok(())
}
