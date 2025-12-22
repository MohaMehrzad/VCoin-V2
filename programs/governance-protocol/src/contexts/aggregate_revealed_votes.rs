use anchor_lang::prelude::*;
use crate::constants::*;
use crate::errors::GovernanceError;
use crate::state::{Proposal, PrivateVotingConfig, GovernanceConfig};

#[derive(Accounts)]
pub struct AggregateRevealedVotes<'info> {
    #[account(
        mut,
        seeds = [PROPOSAL_SEED, proposal.id.to_le_bytes().as_ref()],
        bump = proposal.bump
    )]
    pub proposal: Account<'info, Proposal>,
    
    #[account(
        mut,
        seeds = [PRIVATE_VOTING_SEED, proposal.key().as_ref()],
        bump = private_voting_config.bump
    )]
    pub private_voting_config: Account<'info, PrivateVotingConfig>,
    
    #[account(
        seeds = [GOV_CONFIG_SEED],
        bump = governance_config.bump,
        has_one = authority @ GovernanceError::Unauthorized
    )]
    pub governance_config: Account<'info, GovernanceConfig>,
    
    pub authority: Signer<'info>,
}

