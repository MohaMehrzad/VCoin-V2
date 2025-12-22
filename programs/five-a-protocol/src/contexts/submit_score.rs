use anchor_lang::prelude::*;

use crate::constants::{FIVE_A_CONFIG_SEED, USER_SCORE_SEED, ORACLE_SEED};
use crate::state::{FiveAConfig, UserScore, Oracle};

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

