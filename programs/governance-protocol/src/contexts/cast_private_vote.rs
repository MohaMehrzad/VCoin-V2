use anchor_lang::prelude::*;
use crate::constants::*;
use crate::state::{Proposal, PrivateVotingConfig, VoteRecord};

#[derive(Accounts)]
pub struct CastPrivateVote<'info> {
    #[account(
        seeds = [PROPOSAL_SEED, proposal.id.to_le_bytes().as_ref()],
        bump = proposal.bump
    )]
    pub proposal: Account<'info, Proposal>,
    
    #[account(
        seeds = [PRIVATE_VOTING_SEED, proposal.key().as_ref()],
        bump = private_voting_config.bump
    )]
    pub private_voting_config: Account<'info, PrivateVotingConfig>,
    
    #[account(
        init,
        payer = voter,
        space = VoteRecord::LEN,
        seeds = [VOTE_RECORD_SEED, proposal.key().as_ref(), voter.key().as_ref()],
        bump
    )]
    pub vote_record: Account<'info, VoteRecord>,
    
    #[account(mut)]
    pub voter: Signer<'info>,
    
    pub system_program: Program<'info, System>,
}

