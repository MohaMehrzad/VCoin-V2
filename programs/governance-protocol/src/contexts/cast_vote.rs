use anchor_lang::prelude::*;
use crate::constants::*;
use crate::state::{Proposal, VoteRecord, Delegation};

#[derive(Accounts)]
pub struct CastVote<'info> {
    #[account(
        mut,
        seeds = [PROPOSAL_SEED, proposal.id.to_le_bytes().as_ref()],
        bump = proposal.bump
    )]
    pub proposal: Account<'info, Proposal>,
    
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
    
    /// M-07 Security Fix: Optional delegation account for voting on behalf of delegator
    /// If provided, expiry is validated in the handler
    pub delegation: Option<Account<'info, Delegation>>,
    
    pub system_program: Program<'info, System>,
}

