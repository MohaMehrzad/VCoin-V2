use anchor_lang::prelude::*;
use crate::constants::{USER_STAKE_SEED, USER_SCORE_SEED};
use crate::contexts::CastVote;
use crate::errors::GovernanceError;
use crate::events::VoteCast;
use crate::state::{VoteChoice, calculate_voting_power};

/// C-NEW-01 Security Fix: On-chain verification of voting power
/// 
/// All voting power components are now read directly from on-chain accounts:
/// - veVCoin balance: from UserStake.ve_vcoin_amount (staking-protocol)
/// - Tier: from UserStake.tier (staking-protocol)
/// - 5A Score: from UserScore.composite_score (five-a-protocol)
/// 
/// This prevents vote manipulation where attackers could claim arbitrary values.
pub fn handler(
    ctx: Context<CastVote>,
    choice: u8,
) -> Result<()> {
    let config = &ctx.accounts.config;
    let proposal = &mut ctx.accounts.proposal;
    let voter_key = ctx.accounts.voter.key();
    
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
    
    // =========================================================================
    // C-NEW-01: Verify and read voting power from on-chain accounts
    // =========================================================================
    
    // Verify UserStake PDA derivation from staking program
    let (expected_user_stake_pda, _) = Pubkey::find_program_address(
        &[USER_STAKE_SEED, voter_key.as_ref()],
        &config.staking_program
    );
    require!(
        ctx.accounts.user_stake.key() == expected_user_stake_pda,
        GovernanceError::InvalidUserStakePDA
    );
    
    // Verify UserScore PDA derivation from five-a program  
    let (expected_user_score_pda, _) = Pubkey::find_program_address(
        &[USER_SCORE_SEED, voter_key.as_ref()],
        &config.five_a_program
    );
    require!(
        ctx.accounts.user_score.key() == expected_user_score_pda,
        GovernanceError::InvalidUserScorePDA
    );
    
    // Read veVCoin balance and tier from UserStake account data
    // UserStake layout: discriminator(8) + owner(32) + staked_amount(8) + lock_duration(8) 
    //                   + lock_end(8) + stake_start(8) + tier(1) + ve_vcoin_amount(8) + bump(1)
    let (vevcoin_balance, tier) = if ctx.accounts.user_stake.data_is_empty() {
        // No stake account = 0 veVCoin, tier 0
        (0u64, 0u8)
    } else {
        let stake_data = ctx.accounts.user_stake.try_borrow_data()?;
        require!(stake_data.len() >= 82, GovernanceError::InvalidUserStakeData);
        
        // tier is at offset 72 (8+32+8+8+8+8)
        let tier = stake_data[72];
        // ve_vcoin_amount is at offset 73-80 (u64 little-endian)
        let ve_vcoin_amount = u64::from_le_bytes(
            stake_data[73..81].try_into().map_err(|_| GovernanceError::InvalidUserStakeData)?
        );
        (ve_vcoin_amount, tier)
    };
    
    // Read 5A composite score from UserScore account data
    // UserScore layout: discriminator(8) + user(32) + authenticity(2) + accuracy(2) + agility(2)
    //                   + activity(2) + approved(2) + composite_score(2) + ...
    let five_a_score = if ctx.accounts.user_score.data_is_empty() {
        // No score account = 0 score
        0u16
    } else {
        let score_data = ctx.accounts.user_score.try_borrow_data()?;
        require!(score_data.len() >= 52, GovernanceError::InvalidUserScoreData);
        
        // composite_score is at offset 50 (8+32+2+2+2+2+2)
        u16::from_le_bytes(
            score_data[50..52].try_into().map_err(|_| GovernanceError::InvalidUserScoreData)?
        )
    };
    
    // M-07 Security Fix: Validate delegation expiry if voting with delegated power
    // H-NEW-03: Also validate that veVCoin balance doesn't exceed delegated amount
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
            delegation.delegate == voter_key,
            GovernanceError::Unauthorized
        );
        // H-NEW-03: Verify claimed veVCoin doesn't exceed delegated amount
        require!(
            vevcoin_balance <= delegation.delegated_amount,
            GovernanceError::ExceedsDelegatedAmount
        );
    }
    
    let vote_choice = VoteChoice::from_u8(choice)
        .ok_or(GovernanceError::InvalidVoteChoice)?;
    
    // Calculate voting power using verified on-chain values
    let vote_weight = calculate_voting_power(vevcoin_balance, five_a_score, tier);
    
    // Record vote
    let vote_record = &mut ctx.accounts.vote_record;
    vote_record.voter = voter_key;
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
    
    msg!("Vote cast: {} with verified weight {} (veVCoin: {}, tier: {}, 5A: {})", 
         choice, vote_weight, vevcoin_balance, tier, five_a_score);
    Ok(())
}

