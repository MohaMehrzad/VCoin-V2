use anchor_lang::prelude::*;
use crate::constants::USER_ENERGY_SEED;
use crate::state::UserEnergy;

#[derive(Accounts)]
pub struct GetEnergy<'info> {
    #[account(
        seeds = [USER_ENERGY_SEED, user_energy.user.as_ref()],
        bump = user_energy.bump
    )]
    pub user_energy: Account<'info, UserEnergy>,
}

