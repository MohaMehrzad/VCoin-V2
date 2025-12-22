use anchor_lang::prelude::*;

use crate::constants::{IDENTITY_CONFIG_SEED, IDENTITY_SEED};
use crate::state::{IdentityConfig, Identity};

#[derive(Accounts)]
pub struct CreateIdentity<'info> {
    #[account(
        mut,
        seeds = [IDENTITY_CONFIG_SEED],
        bump = identity_config.bump
    )]
    pub identity_config: Account<'info, IdentityConfig>,
    
    #[account(
        init,
        payer = owner,
        space = Identity::LEN,
        seeds = [IDENTITY_SEED, owner.key().as_ref()],
        bump
    )]
    pub identity: Account<'info, Identity>,
    
    #[account(mut)]
    pub owner: Signer<'info>,
    
    pub system_program: Program<'info, System>,
}

