use anchor_lang::prelude::*;

use crate::constants::USER_SCORE_SEED;
use crate::state::UserScore;

#[derive(Accounts)]
pub struct UpdateUserScore<'info> {
    #[account(
        mut,
        seeds = [USER_SCORE_SEED, user.key().as_ref()],
        bump = user_score.bump,
        constraint = user_score.user == user.key()
    )]
    pub user_score: Account<'info, UserScore>,
    
    pub user: Signer<'info>,
}

