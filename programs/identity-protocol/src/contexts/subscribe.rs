use anchor_lang::prelude::*;

use crate::constants::{IDENTITY_CONFIG_SEED, SUBSCRIPTION_SEED};
use crate::state::{IdentityConfig, Subscription};

#[derive(Accounts)]
pub struct Subscribe<'info> {
    #[account(
        seeds = [IDENTITY_CONFIG_SEED],
        bump = identity_config.bump
    )]
    pub identity_config: Account<'info, IdentityConfig>,
    
    #[account(
        init_if_needed,
        payer = user,
        space = Subscription::LEN,
        seeds = [SUBSCRIPTION_SEED, user.key().as_ref()],
        bump
    )]
    pub subscription: Account<'info, Subscription>,
    
    #[account(mut)]
    pub user: Signer<'info>,
    
    pub system_program: Program<'info, System>,
}

