use anchor_lang::prelude::*;
use crate::constants::*;
use crate::state::{Proposal, PrivateVotingConfig};

#[derive(Accounts)]
pub struct EnablePrivateVoting<'info> {
    #[account(
        seeds = [PROPOSAL_SEED, proposal.id.to_le_bytes().as_ref()],
        bump = proposal.bump,
        has_one = proposer
    )]
    pub proposal: Account<'info, Proposal>,
    
    #[account(
        init,
        payer = proposer,
        space = PrivateVotingConfig::LEN,
        seeds = [PRIVATE_VOTING_SEED, proposal.key().as_ref()],
        bump
    )]
    pub private_voting_config: Account<'info, PrivateVotingConfig>,
    
    #[account(mut)]
    pub proposer: Signer<'info>,
    
    pub system_program: Program<'info, System>,
}

