use anchor_lang::prelude::*;

use crate::constants::{IDENTITY_CONFIG_SEED, IDENTITY_SEED, SAS_ATTESTATION_SEED};
use crate::state::{IdentityConfig, Identity, UserSASAttestation};

#[derive(Accounts)]
pub struct LinkSASAttestation<'info> {
    #[account(
        seeds = [IDENTITY_CONFIG_SEED],
        bump = identity_config.bump
    )]
    pub identity_config: Account<'info, IdentityConfig>,
    
    #[account(
        mut,
        seeds = [IDENTITY_SEED, user.key().as_ref()],
        bump = identity.bump
    )]
    pub identity: Account<'info, Identity>,
    
    #[account(
        init,
        payer = user,
        space = UserSASAttestation::LEN,
        seeds = [SAS_ATTESTATION_SEED, user.key().as_ref()],
        bump
    )]
    pub sas_attestation: Account<'info, UserSASAttestation>,
    
    #[account(mut)]
    pub user: Signer<'info>,
    
    /// The attester signing this attestation
    pub attester: Signer<'info>,
    
    pub system_program: Program<'info, System>,
}

