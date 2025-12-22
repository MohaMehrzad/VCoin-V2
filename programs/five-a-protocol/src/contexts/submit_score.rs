use anchor_lang::prelude::*;

use crate::constants::{FIVE_A_CONFIG_SEED, USER_SCORE_SEED, ORACLE_SEED, PENDING_SCORE_SEED};
use crate::state::{FiveAConfig, UserScore, Oracle, PendingScoreUpdate};

/// H-05 Security Fix: Submit score now uses consensus mechanism
/// First oracle creates PendingScoreUpdate, subsequent oracles confirm
/// Once required_consensus is reached, score is applied
#[derive(Accounts)]
pub struct SubmitScore<'info> {
    #[account(
        mut,
        seeds = [FIVE_A_CONFIG_SEED],
        bump = five_a_config.bump
    )]
    pub five_a_config: Account<'info, FiveAConfig>,
    
    #[account(
        init_if_needed,
        payer = oracle,
        space = UserScore::LEN,
        seeds = [USER_SCORE_SEED, user.key().as_ref()],
        bump
    )]
    pub user_score: Account<'info, UserScore>,
    
    /// H-05: Pending score update for consensus
    #[account(
        init_if_needed,
        payer = oracle,
        space = PendingScoreUpdate::LEN,
        seeds = [PENDING_SCORE_SEED, user.key().as_ref()],
        bump
    )]
    pub pending_score: Account<'info, PendingScoreUpdate>,
    
    #[account(
        mut,
        seeds = [ORACLE_SEED, oracle.key().as_ref()],
        bump = oracle_account.bump
    )]
    pub oracle_account: Account<'info, Oracle>,
    
    /// CHECK: User whose score is being updated
    pub user: UncheckedAccount<'info>,
    
    #[account(mut)]
    pub oracle: Signer<'info>,
    
    pub system_program: Program<'info, System>,
}

