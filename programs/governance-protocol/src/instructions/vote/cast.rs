use anchor_lang::prelude::*;
use crate::contexts::CastVote;
use crate::errors::GovernanceError;
use crate::events::VoteCast;
use crate::state::{VoteChoice, calculate_voting_power};

pub fn handler(
    ctx: Context<CastVote>,
    choice: u8,
    vevcoin_balance: u64,
    five_a_score: u16,
    tier: u8,
) -> Result<()> {
    let proposal = &mut ctx.accounts.proposal;
    
    // Verify voting period
    let clock = Clock::get()?;
    require!(
        clock.unix_timestamp >= proposal.start_time,
        GovernanceError::VotingNotStarted
    );
    require!(
        clock.unix_timestamp <= proposal.end_time,
        GovernanceError::VotingEnded
    );
    require!(
        !proposal.is_private_voting,
        GovernanceError::ZKVotingNotEnabled
    );
    
    // M-07 Security Fix: Validate delegation expiry if voting with delegated power
    if let Some(delegation) = &ctx.accounts.delegation {
        // Check that delegation hasn't expired (0 = never expires)
        if delegation.expires_at > 0 {
            require!(
                clock.unix_timestamp < delegation.expires_at,
                GovernanceError::DelegationExpired
            );
        }
        // Ensure the delegation is for this voter and still valid
        require!(
            delegation.delegate == ctx.accounts.voter.key(),
            GovernanceError::Unauthorized
        );
    }
    
    let vote_choice = VoteChoice::from_u8(choice)
        .ok_or(GovernanceError::InvalidVoteChoice)?;
    
    // Calculate voting power
    let vote_weight = calculate_voting_power(vevcoin_balance, five_a_score, tier);
    
    // Record vote
    let vote_record = &mut ctx.accounts.vote_record;
    vote_record.voter = ctx.accounts.voter.key();
    vote_record.proposal = proposal.key();
    vote_record.vote_weight = vote_weight;
    vote_record.vote_choice = choice;
    vote_record.voted_at = clock.unix_timestamp;
    vote_record.is_private = false;
    vote_record.revealed = true;
    vote_record.bump = ctx.bumps.vote_record;
    
    // Update proposal vote counts
    match vote_choice {
        VoteChoice::For => {
            proposal.votes_for = proposal.votes_for.saturating_add(vote_weight as u128);
        }
        VoteChoice::Against => {
            proposal.votes_against = proposal.votes_against.saturating_add(vote_weight as u128);
        }
        VoteChoice::Abstain => {
            proposal.votes_abstain = proposal.votes_abstain.saturating_add(vote_weight as u128);
        }
    }
    
    emit!(VoteCast {
        proposal_id: proposal.id,
        voter: vote_record.voter,
        choice,
        weight: vote_weight,
        is_private: false,
    });
    
    msg!("Vote cast: {} with weight {}", choice, vote_weight);
    Ok(())
}

