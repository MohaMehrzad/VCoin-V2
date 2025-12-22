use anchor_lang::prelude::*;
use crate::constants::*;
use crate::state::{GaslessConfig, SessionKey, UserGaslessStats};

#[derive(Accounts)]
#[instruction(session_pubkey: Pubkey)]
pub struct CreateSessionKey<'info> {
    #[account(
        seeds = [GASLESS_CONFIG_SEED],
        bump = config.bump
    )]
    pub config: Account<'info, GaslessConfig>,
    
    #[account(
        init,
        payer = user,
        space = SessionKey::LEN,
        seeds = [SESSION_KEY_SEED, user.key().as_ref(), session_pubkey.as_ref()],
        bump
    )]
    pub session_key: Account<'info, SessionKey>,
    
    #[account(
        init_if_needed,
        payer = user,
        space = UserGaslessStats::LEN,
        seeds = [USER_GASLESS_SEED, user.key().as_ref()],
        bump
    )]
    pub user_stats: Account<'info, UserGaslessStats>,
    
    #[account(mut)]
    pub user: Signer<'info>,
    
    pub system_program: Program<'info, System>,
}

