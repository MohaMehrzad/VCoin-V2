use anchor_lang::prelude::*;
use crate::constants::*;
use crate::contexts::CreateSessionKey;
use crate::errors::GaslessError;
use crate::events::SessionKeyCreated;
use crate::state::FeeMethod;

pub fn handler(
    ctx: Context<CreateSessionKey>,
    session_pubkey: Pubkey,
    scope: u16,
    duration_seconds: i64,
    max_actions: u32,
    max_spend: u64,
    fee_method: u8,
) -> Result<()> {
    let config = &ctx.accounts.config;
    let session = &mut ctx.accounts.session_key;
    let user_stats = &mut ctx.accounts.user_stats;
    
    require!(!config.paused, GaslessError::ProtocolPaused);
    
    let clock = Clock::get()?;
    
    // Limit session duration
    let duration = if duration_seconds > 0 && duration_seconds <= SESSION_DURATION {
        duration_seconds
    } else {
        SESSION_DURATION
    };
    
    // Limit max actions
    let actions = if max_actions > 0 && max_actions <= MAX_SESSION_ACTIONS {
        max_actions
    } else {
        MAX_SESSION_ACTIONS
    };
    
    // Limit max spend
    let spend = if max_spend > 0 && max_spend <= MAX_SESSION_SPEND {
        max_spend
    } else {
        MAX_SESSION_SPEND
    };
    
    // Parse fee method
    let method = match fee_method {
        0 => FeeMethod::PlatformSubsidized,
        1 => FeeMethod::VCoinDeduction,
        2 => FeeMethod::SSCREDeduction,
        _ => FeeMethod::VCoinDeduction,
    };
    
    session.user = ctx.accounts.user.key();
    session.session_pubkey = session_pubkey;
    session.scope = scope;
    session.created_at = clock.unix_timestamp;
    session.expires_at = clock.unix_timestamp + duration;
    session.actions_used = 0;
    session.max_actions = actions;
    session.vcoin_spent = 0;
    session.max_spend = spend;
    session.is_revoked = false;
    session.last_action_at = 0;
    session.fee_method = method;
    session.bump = ctx.bumps.session_key;
    
    // Update user stats
    user_stats.user = ctx.accounts.user.key();
    user_stats.sessions_created = user_stats.sessions_created.saturating_add(1);
    user_stats.active_session = session_pubkey;
    user_stats.bump = ctx.bumps.user_stats;
    
    emit!(SessionKeyCreated {
        user: session.user,
        session_pubkey,
        scope,
        expires_at: session.expires_at,
        fee_method,
    });
    
    msg!("Session key created: scope={:#06x}, expires in {}s", scope, duration);
    Ok(())
}

