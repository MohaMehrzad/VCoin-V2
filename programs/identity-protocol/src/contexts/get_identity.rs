use anchor_lang::prelude::*;

use crate::constants::IDENTITY_SEED;
use crate::state::Identity;

#[derive(Accounts)]
pub struct GetIdentity<'info> {
    #[account(
        seeds = [IDENTITY_SEED, identity.owner.as_ref()],
        bump = identity.bump
    )]
    pub identity: Account<'info, Identity>,
}

