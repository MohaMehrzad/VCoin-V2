use anchor_lang::prelude::*;
use crate::constants::USER_ENERGY_SEED;
use crate::state::UserEnergy;

#[derive(Accounts)]
pub struct InitializeUserEnergy<'info> {
    #[account(
        init,
        payer = user,
        space = UserEnergy::LEN,
        seeds = [USER_ENERGY_SEED, user.key().as_ref()],
        bump
    )]
    pub user_energy: Account<'info, UserEnergy>,
    
    #[account(mut)]
    pub user: Signer<'info>,
    
    pub system_program: Program<'info, System>,
}

