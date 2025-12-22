use anchor_lang::prelude::*;

use crate::constants::IDENTITY_CONFIG_SEED;
use crate::state::IdentityConfig;

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(
        init,
        payer = authority,
        space = IdentityConfig::LEN,
        seeds = [IDENTITY_CONFIG_SEED],
        bump
    )]
    pub identity_config: Account<'info, IdentityConfig>,
    
    /// CHECK: Treasury account for subscription payments
    pub treasury: UncheckedAccount<'info>,
    
    #[account(mut)]
    pub authority: Signer<'info>,
    
    pub system_program: Program<'info, System>,
}

