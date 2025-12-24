use anchor_lang::prelude::*;
use crate::constants::{VALID_URI_PREFIX_IPFS, VALID_URI_PREFIX_HTTPS, VALID_URI_PREFIX_AR, MAX_URI_LENGTH, USER_STAKE_SEED};
use crate::contexts::CreateProposal;
use crate::errors::GovernanceError;
use crate::events::ProposalCreated;
use crate::state::ProposalStatus;

/// Check if URI has a valid prefix (L-04 Security Fix)
fn is_valid_uri(uri: &str) -> bool {
    let bytes = uri.as_bytes();
    bytes.starts_with(VALID_URI_PREFIX_IPFS) ||
    bytes.starts_with(VALID_URI_PREFIX_HTTPS) ||
    bytes.starts_with(VALID_URI_PREFIX_AR)
}

pub fn handler(
    ctx: Context<CreateProposal>,
    title_hash: [u8; 32],
    description_uri: String,
    proposal_type: u8,
    enable_private_voting: bool,
) -> Result<()> {
    let config = &mut ctx.accounts.governance_config;
    let proposer_key = ctx.accounts.proposer.key();
    
    require!(!config.paused, GovernanceError::GovernancePaused);
    require!(description_uri.len() <= MAX_URI_LENGTH, GovernanceError::Overflow);
    
    // L-04: Validate URI format
    require!(is_valid_uri(&description_uri), GovernanceError::InvalidDescriptionUri);
    
    // =========================================================================
    // H-NEW-05: Enforce proposal threshold - verify proposer has enough veVCoin
    // =========================================================================
    
    // Verify UserStake PDA derivation from staking program
    let (expected_proposer_stake_pda, _) = Pubkey::find_program_address(
        &[USER_STAKE_SEED, proposer_key.as_ref()],
        &config.staking_program
    );
    require!(
        ctx.accounts.proposer_stake.key() == expected_proposer_stake_pda,
        GovernanceError::InvalidUserStakePDA
    );
    
    // Read veVCoin balance from UserStake account
    // UserStake layout: discriminator(8) + owner(32) + staked_amount(8) + lock_duration(8) 
    //                   + lock_end(8) + stake_start(8) + tier(1) + ve_vcoin_amount(8) + bump(1)
    let proposer_vevcoin = if ctx.accounts.proposer_stake.data_is_empty() {
        0u64
    } else {
        let stake_data = ctx.accounts.proposer_stake.try_borrow_data()?;
        require!(stake_data.len() >= 82, GovernanceError::InvalidUserStakeData);
        
        // ve_vcoin_amount is at offset 73-80 (u64 little-endian)
        u64::from_le_bytes(
            stake_data[73..81].try_into().map_err(|_| GovernanceError::InvalidUserStakeData)?
        )
    };
    
    // Verify proposer meets the threshold
    require!(
        proposer_vevcoin >= config.proposal_threshold,
        GovernanceError::InsufficientVeVCoin
    );
    
    let clock = Clock::get()?;
    
    // Increment proposal count
    config.proposal_count = config.proposal_count.saturating_add(1);
    
    let proposal = &mut ctx.accounts.proposal;
    proposal.id = config.proposal_count;
    proposal.proposer = ctx.accounts.proposer.key();
    proposal.title_hash = title_hash;
    
    let uri_bytes = description_uri.as_bytes();
    proposal.description_uri[..uri_bytes.len()].copy_from_slice(uri_bytes);
    proposal.uri_len = uri_bytes.len() as u8;
    
    proposal.proposal_type = proposal_type;
    proposal.start_time = clock.unix_timestamp;
    proposal.end_time = clock.unix_timestamp + config.voting_period;
    proposal.votes_for = 0;
    proposal.votes_against = 0;
    proposal.votes_abstain = 0;
    proposal.status = ProposalStatus::Active as u8;
    proposal.execution_time = 0;
    proposal.executed = false;
    proposal.is_private_voting = enable_private_voting;
    proposal.bump = ctx.bumps.proposal;
    
    emit!(ProposalCreated {
        id: proposal.id,
        proposer: proposal.proposer,
        proposal_type,
        start_time: proposal.start_time,
        end_time: proposal.end_time,
    });
    
    msg!("Proposal created: {}", proposal.id);
    Ok(())
}

