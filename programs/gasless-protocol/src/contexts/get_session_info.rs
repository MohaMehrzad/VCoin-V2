use anchor_lang::prelude::*;
use crate::constants::SESSION_KEY_SEED;
use crate::state::SessionKey;

#[derive(Accounts)]
pub struct GetSessionInfo<'info> {
    #[account(
        seeds = [SESSION_KEY_SEED, session_key.user.as_ref(), session_key.session_pubkey.as_ref()],
        bump = session_key.bump
    )]
    pub session_key: Account<'info, SessionKey>,
}

