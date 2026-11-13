use crate::constants::USER_GASLESS_SEED;
use crate::state::UserGaslessStats;
use anchor_lang::prelude::*;

#[derive(Accounts)]
pub struct GetUserStats<'info> {
    #[account(
        seeds = [USER_GASLESS_SEED, user_stats.user.as_ref()],
        bump = user_stats.bump
    )]
    pub user_stats: Account<'info, UserGaslessStats>,
}
