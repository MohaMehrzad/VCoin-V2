use anchor_lang::prelude::*;
use solana_curve25519::ristretto::{
    add_ristretto, validate_ristretto, PodRistrettoPoint,
};

use crate::constants::{USER_STAKE_SEED, USER_SCORE_SEED};
use crate::contexts::CastPrivateVote;
use crate::crypto::types::{ElGamalCiphertext, VoteValidityProof, CompressedOrProof, SumProof};
use crate::crypto::verify_vote::verify_vote_validity_proof;
use crate::errors::GovernanceError;
use crate::events::PrivateVoteCast;
use crate::state::{PrivateVotingConfig, calculate_voting_power};

/// Vote validity proof size: 3 * 96 (OR proofs) + 64 (sum proof) = 352 bytes
const VOTE_PROOF_SIZE: usize = 352;

pub fn handler(
    ctx: Context<CastPrivateVote>,
    ct_for: [u8; 64],
    ct_against: [u8; 64],
    ct_abstain: [u8; 64],
    proof_data: Vec<u8>,
) -> Result<()> {
    // Parse proof from heap-allocated Vec to avoid stack overflow
    require!(proof_data.len() == VOTE_PROOF_SIZE, GovernanceError::InvalidZKProof);
    let proof = parse_proof(&proof_data)?;

    let proposal = &ctx.accounts.proposal;
    let private_config = &mut ctx.accounts.private_voting_config;
    let voter_key = ctx.accounts.voter.key();

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

    // Compute weight from on-chain accounts (separate stack frame)
    let weight = compute_weight(
        &ctx.accounts.config,
        &ctx.accounts.user_stake,
        &ctx.accounts.user_score,
        &voter_key,
    )?;

    // Parse and validate ciphertexts (separate stack frame)
    let ct_for_parsed = parse_ciphertext(&ct_for)?;
    let ct_against_parsed = parse_ciphertext(&ct_against)?;
    let ct_abstain_parsed = parse_ciphertext(&ct_abstain)?;

    // Verify vote validity proof (separate stack frame via #[inline(never)])
    verify_vote_validity_proof(
        &private_config.encryption_pubkey,
        &ct_for_parsed,
        &ct_against_parsed,
        &ct_abstain_parsed,
        weight,
        &proof,
    )?;

    // Store ciphertexts in VoteRecord
    let vote_record = &mut ctx.accounts.vote_record;
    vote_record.voter = voter_key;
    vote_record.proposal = proposal.key();
    vote_record.vote_weight = weight;
    vote_record.vote_choice = 0;
    vote_record.voted_at = clock.unix_timestamp;
    vote_record.is_private = true;
    vote_record.ct_for = ct_for;
    vote_record.ct_against = ct_against;
    vote_record.ct_abstain = ct_abstain;
    vote_record.revealed = false;
    vote_record.bump = ctx.bumps.vote_record;

    // Homomorphic accumulation (separate stack frame)
    accumulate_ciphertexts(private_config, &ct_for, &ct_against, &ct_abstain)?;

    private_config.total_private_votes = private_config
        .total_private_votes
        .checked_add(1)
        .ok_or(GovernanceError::Overflow)?;

    emit!(PrivateVoteCast {
        proposal_id: proposal.id,
        voter: voter_key,
        total_private_votes: private_config.total_private_votes,
    });

    msg!("Private vote cast with verified weight {}", weight);
    Ok(())
}

/// Compute voting power from on-chain accounts
#[inline(never)]
fn compute_weight(
    config: &crate::state::GovernanceConfig,
    user_stake: &AccountInfo,
    user_score: &AccountInfo,
    voter_key: &Pubkey,
) -> Result<u64> {
    let (expected_user_stake_pda, _) = Pubkey::find_program_address(
        &[USER_STAKE_SEED, voter_key.as_ref()],
        &config.staking_program,
    );
    require!(
        user_stake.key() == expected_user_stake_pda,
        GovernanceError::InvalidUserStakePDA
    );

    let (expected_user_score_pda, _) = Pubkey::find_program_address(
        &[USER_SCORE_SEED, voter_key.as_ref()],
        &config.five_a_program,
    );
    require!(
        user_score.key() == expected_user_score_pda,
        GovernanceError::InvalidUserScorePDA
    );

    let (vevcoin_balance, tier) = if user_stake.data_is_empty() {
        (0u64, 0u8)
    } else {
        let stake_data = user_stake.try_borrow_data()?;
        require!(stake_data.len() >= 82, GovernanceError::InvalidUserStakeData);
        let tier = stake_data[72];
        let ve_vcoin_amount = u64::from_le_bytes(
            stake_data[73..81].try_into().map_err(|_| GovernanceError::InvalidUserStakeData)?
        );
        (ve_vcoin_amount, tier)
    };

    let five_a_score = if user_score.data_is_empty() {
        0u16
    } else {
        let score_data = user_score.try_borrow_data()?;
        require!(score_data.len() >= 52, GovernanceError::InvalidUserScoreData);
        u16::from_le_bytes(
            score_data[50..52].try_into().map_err(|_| GovernanceError::InvalidUserScoreData)?
        )
    };

    Ok(calculate_voting_power(vevcoin_balance, five_a_score, tier))
}

/// Homomorphically accumulate ciphertexts into PrivateVotingConfig
#[inline(never)]
fn accumulate_ciphertexts(
    config: &mut PrivateVotingConfig,
    ct_for: &[u8; 64],
    ct_against: &[u8; 64],
    ct_abstain: &[u8; 64],
) -> Result<()> {
    config.accumulated_ct_for_r = point_add(&config.accumulated_ct_for_r, &ct_for[..32])?;
    config.accumulated_ct_for_c = point_add(&config.accumulated_ct_for_c, &ct_for[32..64])?;
    config.accumulated_ct_against_r = point_add(&config.accumulated_ct_against_r, &ct_against[..32])?;
    config.accumulated_ct_against_c = point_add(&config.accumulated_ct_against_c, &ct_against[32..64])?;
    config.accumulated_ct_abstain_r = point_add(&config.accumulated_ct_abstain_r, &ct_abstain[..32])?;
    config.accumulated_ct_abstain_c = point_add(&config.accumulated_ct_abstain_c, &ct_abstain[32..64])?;
    Ok(())
}

/// Parse a 64-byte ciphertext into an ElGamalCiphertext, validating both points
#[inline(never)]
fn parse_ciphertext(data: &[u8; 64]) -> Result<ElGamalCiphertext> {
    let mut r = [0u8; 32];
    let mut c = [0u8; 32];
    r.copy_from_slice(&data[..32]);
    c.copy_from_slice(&data[32..64]);

    require!(
        validate_ristretto(&PodRistrettoPoint(r)),
        GovernanceError::InvalidRistrettoPoint
    );
    require!(
        validate_ristretto(&PodRistrettoPoint(c)),
        GovernanceError::InvalidRistrettoPoint
    );

    Ok(ElGamalCiphertext { r, c })
}

/// Parse a VoteValidityProof from raw bytes (352 bytes)
#[inline(never)]
fn parse_proof(data: &[u8]) -> Result<VoteValidityProof> {
    fn read32(data: &[u8], offset: usize) -> [u8; 32] {
        let mut buf = [0u8; 32];
        buf.copy_from_slice(&data[offset..offset + 32]);
        buf
    }

    fn read_or_proof(data: &[u8], offset: usize) -> CompressedOrProof {
        CompressedOrProof {
            c0: read32(data, offset),
            s0: read32(data, offset + 32),
            s1: read32(data, offset + 64),
        }
    }

    Ok(VoteValidityProof {
        or_proof_for: read_or_proof(data, 0),
        or_proof_against: read_or_proof(data, 96),
        or_proof_abstain: read_or_proof(data, 192),
        sum_proof: SumProof {
            c: read32(data, 288),
            s: read32(data, 320),
        },
    })
}

/// Add two Ristretto points, returning the result as [u8; 32]
#[inline(never)]
fn point_add(acc: &[u8; 32], new: &[u8]) -> Result<[u8; 32]> {
    let mut new_bytes = [0u8; 32];
    new_bytes.copy_from_slice(new);

    let acc_point = PodRistrettoPoint(*acc);
    let new_point = PodRistrettoPoint(new_bytes);

    // Handle identity (all zeros)
    if acc_point.0 == [0u8; 32] {
        return Ok(new_point.0);
    }

    let result = add_ristretto(&acc_point, &new_point)
        .ok_or(GovernanceError::InvalidRistrettoPoint)?;

    Ok(result.0)
}
