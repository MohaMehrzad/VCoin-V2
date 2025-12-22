use anchor_lang::prelude::*;
use crate::contexts::FinalizeProposal;
use crate::errors::GovernanceError;
use crate::state::ProposalStatus;

pub fn handler(ctx: Context<FinalizeProposal>) -> Result<()> {
    let config = &ctx.accounts.governance_config;
    let proposal = &mut ctx.accounts.proposal;
    
    let clock = Clock::get()?;
    require!(
        clock.unix_timestamp > proposal.end_time,
        GovernanceError::VotingNotEnded
    );
    
    let total_votes = proposal.votes_for + proposal.votes_against + proposal.votes_abstain;
    
    // Check quorum
    if total_votes < config.quorum as u128 {
        proposal.status = ProposalStatus::Rejected as u8;
        msg!("Proposal rejected: quorum not reached");
        return Ok(());
    }
    
    // Determine outcome
    if proposal.votes_for > proposal.votes_against {
        proposal.status = ProposalStatus::Passed as u8;
        proposal.execution_time = clock.unix_timestamp + config.timelock_delay;
        msg!("Proposal passed, execution time: {}", proposal.execution_time);
    } else {
        proposal.status = ProposalStatus::Rejected as u8;
        msg!("Proposal rejected");
    }
    
    Ok(())
}

