use anchor_lang::prelude::*;
use crate::contexts::AggregateRevealedVotes;
use crate::errors::GovernanceError;
use crate::events::ZKRevealComplete;

pub fn handler(
    ctx: Context<AggregateRevealedVotes>,
    aggregated_for: u128,
    aggregated_against: u128,
    aggregated_abstain: u128,
) -> Result<()> {
    let proposal = &mut ctx.accounts.proposal;
    let private_config = &mut ctx.accounts.private_voting_config;
    
    require!(private_config.reveal_started, GovernanceError::RevealNotStarted);
    require!(
        private_config.shares_received >= private_config.decryption_threshold,
        GovernanceError::InvalidDecryptionShare
    );
    
    // Update aggregated totals
    private_config.aggregated_for = aggregated_for;
    private_config.aggregated_against = aggregated_against;
    private_config.aggregated_abstain = aggregated_abstain;
    private_config.reveal_completed = true;
    
    // Update proposal with revealed totals
    proposal.votes_for = aggregated_for;
    proposal.votes_against = aggregated_against;
    proposal.votes_abstain = aggregated_abstain;
    
    emit!(ZKRevealComplete {
        proposal_id: proposal.id,
        votes_for: aggregated_for,
        votes_against: aggregated_against,
        votes_abstain: aggregated_abstain,
    });
    
    msg!("ZK reveal complete: For={}, Against={}, Abstain={}", 
        aggregated_for, aggregated_against, aggregated_abstain);
    Ok(())
}

