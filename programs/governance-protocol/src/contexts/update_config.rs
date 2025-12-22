use anchor_lang::prelude::*;
use crate::constants::GOV_CONFIG_SEED;
use crate::errors::GovernanceError;
use crate::state::GovernanceConfig;

#[derive(Accounts)]
pub struct UpdateConfig<'info> {
    #[account(
        mut,
        seeds = [GOV_CONFIG_SEED],
        bump = governance_config.bump,
        has_one = authority @ GovernanceError::Unauthorized
    )]
    pub governance_config: Account<'info, GovernanceConfig>,
    
    pub authority: Signer<'info>,
}

