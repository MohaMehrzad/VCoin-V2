use anchor_lang::prelude::*;
use solana_curve25519::ristretto::{
    subtract_ristretto, multiply_ristretto, PodRistrettoPoint,
};
use solana_curve25519::scalar::PodScalar;
use solana_sha256_hasher::hashv;

use crate::constants::DECRYPTION_SHARE_SEED;
use crate::contexts::AggregateRevealedVotes;
use crate::crypto::constants::GENERATOR_H;
use crate::errors::GovernanceError;
use crate::events::ZKRevealComplete;

/// Aggregate revealed votes using trustless on-chain tally verification.
///
/// Anyone can submit the tally. The on-chain verification ensures:
///   tally_cat * H == C_sum_cat - D_cat
/// for each category (for/against/abstain), where D is the combined decryption.
///
/// This makes tally fabrication cryptographically impossible.
pub fn handler(
    ctx: Context<AggregateRevealedVotes>,
    tally_for: u64,
    tally_against: u64,
    tally_abstain: u64,
    lagrange_coefficients: Vec<[u8; 32]>,
) -> Result<()> {
    let proposal = &mut ctx.accounts.proposal;
    let private_config = &mut ctx.accounts.private_voting_config;

    require!(private_config.reveal_started, GovernanceError::RevealNotStarted);
    require!(!private_config.reveal_completed, GovernanceError::RevealAlreadyComplete);
    require!(
        private_config.shares_received >= private_config.decryption_threshold,
        GovernanceError::InsufficientDecryptionShares
    );

    let proposal_key = private_config.proposal;
    let committee_size = private_config.committee_size;
    let h = PodRistrettoPoint(GENERATOR_H);

    // =========================================================================
    // Load and verify DecryptionShare PDAs from remaining_accounts
    // =========================================================================
    let mut verified_shares: u8 = 0;
    let mut partials_for: Vec<[u8; 32]> = Vec::with_capacity(committee_size as usize);
    let mut partials_against: Vec<[u8; 32]> = Vec::with_capacity(committee_size as usize);
    let mut partials_abstain: Vec<[u8; 32]> = Vec::with_capacity(committee_size as usize);
    let mut collected_share_bytes: Vec<[u8; 32]> = Vec::with_capacity(committee_size as usize);

    for account_info in ctx.remaining_accounts.iter() {
        require!(
            account_info.owner == ctx.program_id,
            GovernanceError::InvalidDecryptionSharePDA
        );

        let share_data = account_info.try_borrow_data()?;

        // DecryptionShare::LEN = 242
        if share_data.len() < 242 {
            continue;
        }

        // Parse committee_index (offset 72: after 8 disc + 32 proposal + 32 member)
        let committee_index = share_data[72];
        require!(
            committee_index < committee_size,
            GovernanceError::InvalidCommitteeIndex
        );

        // Verify PDA derivation
        let (expected_pda, _bump) = Pubkey::find_program_address(
            &[DECRYPTION_SHARE_SEED, proposal_key.as_ref(), &[committee_index]],
            ctx.program_id,
        );
        require!(
            account_info.key() == expected_pda,
            GovernanceError::InvalidDecryptionSharePDA
        );

        // Verify share belongs to our proposal (bytes 8..40)
        let mut proposal_bytes = [0u8; 32];
        proposal_bytes.copy_from_slice(&share_data[8..40]);
        require!(
            Pubkey::from(proposal_bytes) == proposal_key,
            GovernanceError::InvalidDecryptionShare
        );

        // Verify committee member matches (bytes 40..72)
        let mut member_bytes = [0u8; 32];
        member_bytes.copy_from_slice(&share_data[40..72]);
        require!(
            private_config.decryption_committee[committee_index as usize] == Pubkey::from(member_bytes),
            GovernanceError::InvalidDecryptionShare
        );

        // Extract partial decryptions (after committee_index at 72):
        // partial_for: 73..105, partial_against: 105..137, partial_abstain: 137..169
        let mut p_for = [0u8; 32];
        let mut p_against = [0u8; 32];
        let mut p_abstain = [0u8; 32];
        p_for.copy_from_slice(&share_data[73..105]);
        p_against.copy_from_slice(&share_data[105..137]);
        p_abstain.copy_from_slice(&share_data[137..169]);

        partials_for.push(p_for);
        partials_against.push(p_against);
        partials_abstain.push(p_abstain);

        // Collect first partial for verification hash
        collected_share_bytes.push(p_for);

        verified_shares += 1;
    }

    require!(
        verified_shares >= private_config.decryption_threshold,
        GovernanceError::InsufficientDecryptionShares
    );
    require!(
        lagrange_coefficients.len() == verified_shares as usize,
        GovernanceError::InvalidDecryptionShare
    );

    // =========================================================================
    // Combine partial decryptions using Lagrange interpolation
    // D_cat = sum(L_i * partial_i) for each category
    // =========================================================================
    let d_for = combine_partials(&lagrange_coefficients, &partials_for)?;
    let d_against = combine_partials(&lagrange_coefficients, &partials_against)?;
    let d_abstain = combine_partials(&lagrange_coefficients, &partials_abstain)?;

    // =========================================================================
    // Verify tally: tally * H == C_sum - D for each category
    // =========================================================================
    verify_tally(
        tally_for,
        &h,
        &PodRistrettoPoint(private_config.accumulated_ct_for_c),
        &d_for,
    )?;
    verify_tally(
        tally_against,
        &h,
        &PodRistrettoPoint(private_config.accumulated_ct_against_c),
        &d_against,
    )?;
    verify_tally(
        tally_abstain,
        &h,
        &PodRistrettoPoint(private_config.accumulated_ct_abstain_c),
        &d_abstain,
    )?;

    // =========================================================================
    // Compute verification hash for audit trail
    // =========================================================================
    let for_bytes = (tally_for as u128).to_le_bytes();
    let against_bytes = (tally_against as u128).to_le_bytes();
    let abstain_bytes = (tally_abstain as u128).to_le_bytes();

    let mut hash_inputs: Vec<&[u8]> = Vec::with_capacity(3 + collected_share_bytes.len());
    hash_inputs.push(&for_bytes);
    hash_inputs.push(&against_bytes);
    hash_inputs.push(&abstain_bytes);
    for share in &collected_share_bytes {
        hash_inputs.push(share);
    }
    let verification_hash = hashv(&hash_inputs);

    // =========================================================================
    // Store results
    // =========================================================================
    private_config.aggregated_for = tally_for as u128;
    private_config.aggregated_against = tally_against as u128;
    private_config.aggregated_abstain = tally_abstain as u128;
    private_config.verification_hash = verification_hash.to_bytes();
    private_config.reveal_completed = true;

    proposal.votes_for = tally_for as u128;
    proposal.votes_against = tally_against as u128;
    proposal.votes_abstain = tally_abstain as u128;

    emit!(ZKRevealComplete {
        proposal_id: proposal.id,
        votes_for: tally_for as u128,
        votes_against: tally_against as u128,
        votes_abstain: tally_abstain as u128,
        verification_hash: verification_hash.to_bytes(),
        shares_verified: verified_shares,
    });

    msg!("ZK reveal complete: For={}, Against={}, Abstain={} (cryptographically verified)",
        tally_for, tally_against, tally_abstain);
    msg!("Shares verified: {}/{}", verified_shares, private_config.decryption_threshold);
    Ok(())
}

/// Combine partial decryptions using Lagrange coefficients:
/// D = sum(L_i * partial_i)
fn combine_partials(
    lagrange_coefficients: &[[u8; 32]],
    partials: &[[u8; 32]],
) -> Result<PodRistrettoPoint> {
    // Start with identity (all zeros = Ristretto identity)
    let mut combined = PodRistrettoPoint([0u8; 32]);

    for (coeff, partial) in lagrange_coefficients.iter().zip(partials.iter()) {
        let scalar = PodScalar(*coeff);
        let point = PodRistrettoPoint(*partial);

        let weighted = multiply_ristretto(&scalar, &point)
            .ok_or(GovernanceError::InvalidDecryptionShare)?;

        combined = add_ristretto_safe(&combined, &weighted)?;
    }

    Ok(combined)
}

/// Verify: tally * H == C_sum - D
fn verify_tally(
    tally: u64,
    h: &PodRistrettoPoint,
    c_sum: &PodRistrettoPoint,
    d: &PodRistrettoPoint,
) -> Result<()> {
    // Left side: tally * H
    let tally_scalar = u64_to_scalar(tally);
    let expected = multiply_ristretto(&tally_scalar, h)
        .ok_or(GovernanceError::TallyVerificationFailed)?;

    // Right side: C_sum - D
    let actual = subtract_ristretto(c_sum, d)
        .ok_or(GovernanceError::TallyVerificationFailed)?;

    require!(
        expected.0 == actual.0,
        GovernanceError::TallyVerificationFailed
    );

    Ok(())
}

/// Safe point addition that handles identity
fn add_ristretto_safe(
    a: &PodRistrettoPoint,
    b: &PodRistrettoPoint,
) -> Result<PodRistrettoPoint> {
    use solana_curve25519::ristretto::add_ristretto;

    // If a is identity (all zeros), return b
    if a.0 == [0u8; 32] {
        return Ok(*b);
    }

    add_ristretto(a, b).ok_or_else(|| GovernanceError::InvalidRistrettoPoint.into())
}

fn u64_to_scalar(value: u64) -> PodScalar {
    let mut bytes = [0u8; 32];
    bytes[..8].copy_from_slice(&value.to_le_bytes());
    PodScalar(bytes)
}
