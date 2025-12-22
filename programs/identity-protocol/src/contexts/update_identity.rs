use anchor_lang::prelude::*;

use crate::constants::IDENTITY_SEED;
use crate::state::Identity;

#[derive(Accounts)]
pub struct UpdateIdentity<'info> {
    #[account(
        mut,
        seeds = [IDENTITY_SEED, owner.key().as_ref()],
        bump = identity.bump,
        has_one = owner
    )]
    pub identity: Account<'info, Identity>,
    
    pub owner: Signer<'info>,
}

