use crate::constants::GOV_CONFIG_SEED;
use crate::errors::GovernanceError;
use crate::state::GovernanceConfig;
use anchor_lang::prelude::*;

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
