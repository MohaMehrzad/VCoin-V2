use anchor_lang::prelude::*;
use crate::constants::USER_GASLESS_SEED;
use crate::state::UserGaslessStats;

#[derive(Accounts)]
pub struct GetUserStats<'info> {
    #[account(
        seeds = [USER_GASLESS_SEED, user_stats.user.as_ref()],
        bump = user_stats.bump
    )]
    pub user_stats: Account<'info, UserGaslessStats>,
}

