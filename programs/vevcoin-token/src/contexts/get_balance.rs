use anchor_lang::prelude::*;

use crate::state::UserVeVCoin;

#[derive(Accounts)]
pub struct GetBalance<'info> {
    /// CHECK: Just a pubkey for PDA derivation
    pub user: UncheckedAccount<'info>,
    
    #[account(
        seeds = [UserVeVCoin::SEED, user.key().as_ref()],
        bump
    )]
    pub user_account: Account<'info, UserVeVCoin>,
}

