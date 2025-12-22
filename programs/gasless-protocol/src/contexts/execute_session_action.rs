use anchor_lang::prelude::*;
use crate::constants::*;
use crate::errors::GaslessError;
use crate::state::{GaslessConfig, SessionKey, UserGaslessStats};

/// H-03 Security Fix: Require session key signature for session actions
/// The session_signer must match the session_key.session_pubkey and sign the transaction
#[derive(Accounts)]
pub struct ExecuteSessionAction<'info> {
    #[account(
        mut,
        seeds = [GASLESS_CONFIG_SEED],
        bump = config.bump
    )]
    pub config: Account<'info, GaslessConfig>,
    
    #[account(
        mut,
        seeds = [SESSION_KEY_SEED, session_key.user.as_ref(), session_key.session_pubkey.as_ref()],
        bump = session_key.bump
    )]
    pub session_key: Account<'info, SessionKey>,
    
    #[account(
        mut,
        seeds = [USER_GASLESS_SEED, user.key().as_ref()],
        bump = user_stats.bump
    )]
    pub user_stats: Account<'info, UserGaslessStats>,
    
    /// H-03 Fix: Session key must sign to prove ownership
    /// This prevents attacks where someone else tries to execute session actions
    #[account(
        constraint = session_signer.key() == session_key.session_pubkey @ GaslessError::InvalidSessionSigner
    )]
    pub session_signer: Signer<'info>,
    
    /// CHECK: User account validated against session_key.user
    #[account(
        constraint = user.key() == session_key.user @ GaslessError::Unauthorized
    )]
    pub user: AccountInfo<'info>,
}

