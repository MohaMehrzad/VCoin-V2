use anchor_lang::prelude::*;

use crate::constants::USER_STAKE_SEED;
use crate::state::UserStake;

#[derive(Accounts)]
pub struct UpdateTier<'info> {
    pub user: Signer<'info>,
    
    #[account(
        mut,
        seeds = [USER_STAKE_SEED, user.key().as_ref()],
        bump,
        constraint = user_stake.owner == user.key()
    )]
    pub user_stake: Account<'info, UserStake>,
}

