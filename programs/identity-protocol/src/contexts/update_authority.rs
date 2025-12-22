use anchor_lang::prelude::*;

use crate::constants::IDENTITY_CONFIG_SEED;
use crate::errors::IdentityError;
use crate::state::IdentityConfig;

#[derive(Accounts)]
pub struct UpdateAuthority<'info> {
    #[account(
        mut,
        seeds = [IDENTITY_CONFIG_SEED],
        bump = identity_config.bump,
        has_one = authority @ IdentityError::Unauthorized
    )]
    pub identity_config: Account<'info, IdentityConfig>,
    
    pub authority: Signer<'info>,
}

