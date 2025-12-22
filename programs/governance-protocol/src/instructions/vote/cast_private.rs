use anchor_lang::prelude::*;
use crate::contexts::CastPrivateVote;
use crate::errors::GovernanceError;
use crate::events::VoteCast;

pub fn handler(
    ctx: Context<CastPrivateVote>,
    encrypted_choice: [u8; 32],
    encrypted_weight: [u8; 32],
    zk_proof: [u8; 128],
) -> Result<()> {
    let proposal = &ctx.accounts.proposal;
    let private_config = &ctx.accounts.private_voting_config;
    
    require!(proposal.is_private_voting, GovernanceError::ZKVotingNotEnabled);
    require!(private_config.is_enabled, GovernanceError::ZKVotingNotEnabled);
    
    let clock = Clock::get()?;
    require!(
        clock.unix_timestamp >= proposal.start_time,
        GovernanceError::VotingNotStarted
    );
    require!(
        clock.unix_timestamp <= proposal.end_time,
        GovernanceError::VotingEnded
    );
    
    // Record private vote
    let vote_record = &mut ctx.accounts.vote_record;
    vote_record.voter = ctx.accounts.voter.key();
    vote_record.proposal = proposal.key();
    vote_record.vote_weight = 0; // Hidden until reveal
    vote_record.vote_choice = 0; // Hidden until reveal
    vote_record.voted_at = clock.unix_timestamp;
    vote_record.is_private = true;
    vote_record.encrypted_choice = encrypted_choice;
    vote_record.encrypted_weight = encrypted_weight;
    vote_record.zk_proof = zk_proof;
    vote_record.revealed = false;
    vote_record.bump = ctx.bumps.vote_record;
    
    emit!(VoteCast {
        proposal_id: proposal.id,
        voter: vote_record.voter,
        choice: 0, // Hidden
        weight: 0, // Hidden
        is_private: true,
    });
    
    msg!("Private vote cast");
    Ok(())
}

