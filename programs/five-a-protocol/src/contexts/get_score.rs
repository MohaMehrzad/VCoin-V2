use anchor_lang::prelude::*;

use crate::constants::USER_SCORE_SEED;
use crate::state::UserScore;

#[derive(Accounts)]
pub struct GetScore<'info> {
    #[account(
        seeds = [USER_SCORE_SEED, user_score.user.as_ref()],
        bump = user_score.bump
    )]
    pub user_score: Account<'info, UserScore>,
}

