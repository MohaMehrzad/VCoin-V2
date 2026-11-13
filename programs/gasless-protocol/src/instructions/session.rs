use anchor_lang::prelude::*;

use crate::constants::MAX_SESSION_DURATION;
use crate::contexts::{CreateSession, RevokeSession};
use crate::errors::GaslessError;
use crate::events::{SessionCreated, SessionRevoked};
use crate::state::validate_scope;

pub fn create_session(
    ctx: Context<CreateSession>,
    target_program: Pubkey,
    scope: u16,
    valid_until: i64,
) -> Result<()> {
    require!(!ctx.accounts.config.paused, GaslessError::Paused);
    validate_scope(scope)?;
    let now = Clock::get()?.unix_timestamp;
    let max = now
        .checked_add(MAX_SESSION_DURATION)
        .ok_or(GaslessError::Overflow)?;
    // Out-of-range values are rejected, never silently replaced (audit GAS-06).
    require!(
        valid_until > now && valid_until <= max,
        GaslessError::InvalidValidity
    );

    let session = &mut ctx.accounts.session;
    session.authority = ctx.accounts.authority.key();
    session.target_program = target_program;
    session.session_signer = ctx.accounts.session_signer.key();
    session.scope = scope;
    session.valid_until = valid_until;
    session.payer = ctx.accounts.payer.key();
    session.bump = ctx.bumps.session;

    emit!(SessionCreated {
        session: session.key(),
        authority: session.authority,
        session_signer: session.session_signer,
        target_program,
        scope,
        valid_until,
    });
    Ok(())
}

/// The authority may revoke at any time; anyone may close an expired session (its rent
/// returns to the original payer), so expired sessions never accumulate.
pub fn revoke_session(ctx: Context<RevokeSession>) -> Result<()> {
    let session = &ctx.accounts.session;
    let revoker = ctx.accounts.revoker.key();
    if revoker != session.authority {
        let now = Clock::get()?.unix_timestamp;
        require!(session.is_expired(now), GaslessError::RevokeNotAllowed);
    }
    emit!(SessionRevoked {
        session: session.key(),
        authority: session.authority,
        revoked_by: revoker
    });
    Ok(())
}
