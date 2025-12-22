use anchor_lang::prelude::*;
use crate::constants::GOV_CONFIG_SEED;
use crate::state::GovernanceConfig;

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(
        init,
        payer = authority,
        space = GovernanceConfig::LEN,
        seeds = [GOV_CONFIG_SEED],
        bump
    )]
    pub governance_config: Account<'info, GovernanceConfig>,
    
    #[account(mut)]
    pub authority: Signer<'info>,
    
    pub system_program: Program<'info, System>,
}

