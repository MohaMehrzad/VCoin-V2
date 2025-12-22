use anchor_lang::prelude::*;
use crate::constants::*;
use crate::state::{GovernanceConfig, Proposal};

#[derive(Accounts)]
pub struct FinalizeProposal<'info> {
    #[account(
        seeds = [GOV_CONFIG_SEED],
        bump = governance_config.bump
    )]
    pub governance_config: Account<'info, GovernanceConfig>,
    
    #[account(
        mut,
        seeds = [PROPOSAL_SEED, proposal.id.to_le_bytes().as_ref()],
        bump = proposal.bump
    )]
    pub proposal: Account<'info, Proposal>,
    
    pub finalizer: Signer<'info>,
}

