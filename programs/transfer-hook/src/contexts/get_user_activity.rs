use anchor_lang::prelude::*;

use crate::constants::USER_ACTIVITY_SEED;
use crate::state::UserActivity;

#[derive(Accounts)]
#[instruction()]
pub struct GetUserActivity<'info> {
    #[account(
        seeds = [USER_ACTIVITY_SEED, user.key().as_ref()],
        bump = user_activity.bump
    )]
    pub user_activity: Account<'info, UserActivity>,
    
    /// CHECK: Just used for PDA derivation
    pub user: UncheckedAccount<'info>,
}

