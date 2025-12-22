use anchor_lang::prelude::*;
use crate::constants::*;
use crate::errors::GaslessError;
use crate::state::{GaslessConfig, SessionKey, UserGaslessStats};

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
        bump = session_key.bump,
        constraint = session_key.user == user.key() @ GaslessError::Unauthorized
    )]
    pub session_key: Account<'info, SessionKey>,
    
    #[account(
        mut,
        seeds = [USER_GASLESS_SEED, user.key().as_ref()],
        bump = user_stats.bump
    )]
    pub user_stats: Account<'info, UserGaslessStats>,
    
    pub user: Signer<'info>,
}

