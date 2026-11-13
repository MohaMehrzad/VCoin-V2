use anchor_lang::prelude::*;

use crate::contexts::UpdateConfig;
use crate::events::{GuardianSet, PauseChanged};

pub fn handler(ctx: Context<UpdateConfig>, paused: bool) -> Result<()> {
    let config = &mut ctx.accounts.config;
    config
        .admin
        .require_pause_rights(&ctx.accounts.authority.key(), paused)?;
    config.paused = paused;
    emit!(PauseChanged { paused });
    Ok(())
}

/// Sets or clears the emergency guardian, which may pause but never unpause.
pub fn set_guardian(ctx: Context<UpdateConfig>, guardian: Pubkey) -> Result<()> {
    let admin = &mut ctx.accounts.config.admin;
    admin.set_guardian(&ctx.accounts.authority.key(), guardian)?;
    emit!(GuardianSet { guardian });
    Ok(())
}
