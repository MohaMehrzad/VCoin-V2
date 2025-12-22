use anchor_lang::prelude::*;

use crate::constants::{IDENTITY_CONFIG_SEED, IDENTITY_SEED};
use crate::errors::IdentityError;
use crate::state::{IdentityConfig, Identity};

#[derive(Accounts)]
pub struct AdminUpdateIdentity<'info> {
    #[account(
        mut,
        seeds = [IDENTITY_CONFIG_SEED],
        bump = identity_config.bump,
        has_one = authority @ IdentityError::Unauthorized
    )]
    pub identity_config: Account<'info, IdentityConfig>,
    
    #[account(
        mut,
        seeds = [IDENTITY_SEED, identity.owner.as_ref()],
        bump = identity.bump
    )]
    pub identity: Account<'info, Identity>,
    
    pub authority: Signer<'info>,
}

