use anchor_lang::prelude::*;
use crate::contexts::GetSessionInfo;

pub fn handler(ctx: Context<GetSessionInfo>) -> Result<()> {
    let session = &ctx.accounts.session_key;
    msg!("User: {}", session.user);
    msg!("Session Key: {}", session.session_pubkey);
    msg!("Scope: {:#06x}", session.scope);
    msg!("Actions: {}/{}", session.actions_used, session.max_actions);
    msg!("Spent: {}/{} VCoin", session.vcoin_spent, session.max_spend);
    msg!("Revoked: {}", session.is_revoked);
    Ok(())
}

