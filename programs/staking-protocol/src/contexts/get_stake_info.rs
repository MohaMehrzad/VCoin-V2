use anchor_lang::prelude::*;

use crate::constants::USER_STAKE_SEED;
use crate::state::UserStake;

#[derive(Accounts)]
pub struct GetStakeInfo<'info> {
    /// CHECK: Just for PDA derivation
    pub user: UncheckedAccount<'info>,
    
    #[account(
        seeds = [USER_STAKE_SEED, user.key().as_ref()],
        bump
    )]
    pub user_stake: Account<'info, UserStake>,
}

