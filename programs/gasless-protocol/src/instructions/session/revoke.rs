use anchor_lang::prelude::*;
use crate::contexts::RevokeSessionKey;
use crate::errors::GaslessError;
use crate::events::SessionKeyRevoked;

pub fn handler(ctx: Context<RevokeSessionKey>) -> Result<()> {
    let session = &mut ctx.accounts.session_key;
    let user_stats = &mut ctx.accounts.user_stats;
    
    require!(!session.is_revoked, GaslessError::SessionRevoked);
    
    session.is_revoked = true;
    
    // Clear active session if this was it
    if user_stats.active_session == session.session_pubkey {
        user_stats.active_session = Pubkey::default();
    }
    
    emit!(SessionKeyRevoked {
        user: session.user,
        session_pubkey: session.session_pubkey,
        actions_used: session.actions_used,
    });
    
    msg!("Session key revoked after {} actions", session.actions_used);
    Ok(())
}

