use anchor_lang::prelude::*;

use crate::contexts::GetIdentity;

/// Get identity info
pub fn handler(ctx: Context<GetIdentity>) -> Result<()> {
    let identity = &ctx.accounts.identity;
    msg!("Owner: {}", identity.owner);
    msg!("Verification level: {}", identity.verification_level);
    msg!("Active: {}", identity.is_active);
    Ok(())
}

