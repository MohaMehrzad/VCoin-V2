use anchor_lang::prelude::*;
use solana_curve25519::ristretto::{
    add_ristretto, subtract_ristretto, multiply_ristretto, validate_ristretto,
    PodRistrettoPoint,
};
use solana_curve25519::scalar::PodScalar;
use solana_sha256_hasher::hashv;

use crate::crypto::constants::{RISTRETTO_BASEPOINT_COMPRESSED, GENERATOR_H, DOMAIN_VOTE_PROOF};
use crate::crypto::types::{ElGamalCiphertext, VoteValidityProof, CompressedOrProof};
use crate::errors::GovernanceError;

/// Verify a complete vote validity proof for a private vote.
#[inline(never)]
pub fn verify_vote_validity_proof(
    encryption_pubkey: &[u8; 32],
    ct_for: &ElGamalCiphertext,
    ct_against: &ElGamalCiphertext,
    ct_abstain: &ElGamalCiphertext,
    weight: u64,
    proof: &VoteValidityProof,
) -> Result<()> {
    let g = PodRistrettoPoint(RISTRETTO_BASEPOINT_COMPRESSED);
    let h = PodRistrettoPoint(GENERATOR_H);
    let pk = PodRistrettoPoint(*encryption_pubkey);

    require!(validate_ristretto(&pk), GovernanceError::InvalidRistrettoPoint);

    let weight_scalar = u64_to_scalar(weight);
    let weight_h = multiply_ristretto(&weight_scalar, &h)
        .ok_or(GovernanceError::InvalidRistrettoPoint)?;

    verify_or_proof(&g, &pk, &weight_h, ct_for, &proof.or_proof_for, b"for")?;
    verify_or_proof(&g, &pk, &weight_h, ct_against, &proof.or_proof_against, b"against")?;
    verify_or_proof(&g, &pk, &weight_h, ct_abstain, &proof.or_proof_abstain, b"abstain")?;

    verify_sum_proof(
        &g, &pk, &weight_h,
        ct_for, ct_against, ct_abstain,
        &proof.sum_proof.c, &proof.sum_proof.s,
    )
}

/// Verify a 1-of-2 OR proof for a single ciphertext.
#[inline(never)]
fn verify_or_proof(
    g: &PodRistrettoPoint,
    pk: &PodRistrettoPoint,
    weight_h: &PodRistrettoPoint,
    ct: &ElGamalCiphertext,
    proof: &CompressedOrProof,
    label: &[u8],
) -> Result<()> {
    let r_point = PodRistrettoPoint(ct.r);
    let c_point = PodRistrettoPoint(ct.c);

    require!(validate_ristretto(&r_point), GovernanceError::InvalidRistrettoPoint);
    require!(validate_ristretto(&c_point), GovernanceError::InvalidRistrettoPoint);

    let c0_scalar = PodScalar(proof.c0);
    let s0_scalar = PodScalar(proof.s0);
    let s1_scalar = PodScalar(proof.s1);

    // Branch 0: A0 = s0*G - c0*R, B0 = s0*pk - c0*C
    let a0 = sub_mul(g, &s0_scalar, &r_point, &c0_scalar)?;
    let b0 = sub_mul(pk, &s0_scalar, &c_point, &c0_scalar)?;

    // c0*R (reused for branch 1)
    let c0_r = multiply_ristretto(&c0_scalar, &r_point)
        .ok_or(GovernanceError::InvalidOrProof)?;

    // C_adj = C - weight*H
    let c_adj = subtract_ristretto(&c_point, weight_h)
        .ok_or(GovernanceError::InvalidOrProof)?;

    // Compute challenge hash from branch 0 commitments
    let challenge_bytes = hashv(&[
        DOMAIN_VOTE_PROOF, label,
        &r_point.0, &c_point.0, &a0.0, &b0.0,
    ]).to_bytes();
    let challenge_scalar = PodScalar(challenge_bytes);

    // Branch 1: A1 = s1*G - challenge*R + c0*R (since c1 = challenge - c0)
    let a1 = compute_branch1_commitment(g, &s1_scalar, &r_point, &challenge_scalar, &c0_r)?;
    // B1 = s1*pk - challenge*C_adj + c0*C_adj
    let b1 = compute_branch1_commitment(pk, &s1_scalar, &c_adj, &challenge_scalar,
        &multiply_ristretto(&c0_scalar, &c_adj).ok_or(GovernanceError::InvalidOrProof)?)?;

    // Verify full challenge
    let full_hash = hashv(&[
        DOMAIN_VOTE_PROOF, label,
        &r_point.0, &c_point.0, &a0.0, &b0.0, &a1.0, &b1.0,
    ]);

    require!(
        full_hash.to_bytes() == challenge_bytes,
        GovernanceError::InvalidOrProof
    );

    Ok(())
}

/// Compute branch 1 commitment: s*base - challenge*target + c0_target
#[inline(never)]
fn compute_branch1_commitment(
    base: &PodRistrettoPoint,
    s: &PodScalar,
    target: &PodRistrettoPoint,
    challenge: &PodScalar,
    c0_target: &PodRistrettoPoint,
) -> Result<PodRistrettoPoint> {
    let s_base = multiply_ristretto(s, base)
        .ok_or(GovernanceError::InvalidOrProof)?;
    let ch_target = multiply_ristretto(challenge, target)
        .ok_or(GovernanceError::InvalidOrProof)?;
    let partial = subtract_ristretto(&s_base, &ch_target)
        .ok_or(GovernanceError::InvalidOrProof)?;
    add_ristretto(&partial, c0_target)
        .ok_or_else(|| GovernanceError::InvalidOrProof.into())
}

/// Compute s*base1 - c*base2 (used for DLEQ commitments)
#[inline(never)]
fn sub_mul(
    base1: &PodRistrettoPoint,
    s: &PodScalar,
    base2: &PodRistrettoPoint,
    c: &PodScalar,
) -> Result<PodRistrettoPoint> {
    let left = multiply_ristretto(s, base1)
        .ok_or(GovernanceError::InvalidOrProof)?;
    let right = multiply_ristretto(c, base2)
        .ok_or(GovernanceError::InvalidOrProof)?;
    subtract_ristretto(&left, &right)
        .ok_or_else(|| GovernanceError::InvalidOrProof.into())
}

/// Verify the sum proof.
#[inline(never)]
fn verify_sum_proof(
    g: &PodRistrettoPoint,
    pk: &PodRistrettoPoint,
    weight_h: &PodRistrettoPoint,
    ct_for: &ElGamalCiphertext,
    ct_against: &ElGamalCiphertext,
    ct_abstain: &ElGamalCiphertext,
    challenge: &[u8; 32],
    response: &[u8; 32],
) -> Result<()> {
    // R_sum = R_for + R_against + R_abstain
    let r_sum = add_three(
        &PodRistrettoPoint(ct_for.r),
        &PodRistrettoPoint(ct_against.r),
        &PodRistrettoPoint(ct_abstain.r),
    )?;

    // C_sum = C_for + C_against + C_abstain
    let c_sum = add_three(
        &PodRistrettoPoint(ct_for.c),
        &PodRistrettoPoint(ct_against.c),
        &PodRistrettoPoint(ct_abstain.c),
    )?;

    // C_adj = C_sum - weight*H
    let c_adj = subtract_ristretto(&c_sum, weight_h)
        .ok_or(GovernanceError::InvalidSumProof)?;

    let c_scalar = PodScalar(*challenge);
    let s_scalar = PodScalar(*response);

    let a = sub_mul_sum(g, &s_scalar, &r_sum, &c_scalar)?;
    let b = sub_mul_sum(pk, &s_scalar, &c_adj, &c_scalar)?;

    let expected_hash = hashv(&[
        DOMAIN_VOTE_PROOF, b"sum",
        &r_sum.0, &c_adj.0, &a.0, &b.0,
    ]);

    require!(
        expected_hash.to_bytes() == *challenge,
        GovernanceError::InvalidSumProof
    );

    Ok(())
}

#[inline(never)]
fn add_three(
    a: &PodRistrettoPoint,
    b: &PodRistrettoPoint,
    c: &PodRistrettoPoint,
) -> Result<PodRistrettoPoint> {
    let ab = add_ristretto(a, b)
        .ok_or(GovernanceError::InvalidSumProof)?;
    add_ristretto(&ab, c)
        .ok_or_else(|| GovernanceError::InvalidSumProof.into())
}

#[inline(never)]
fn sub_mul_sum(
    base1: &PodRistrettoPoint,
    s: &PodScalar,
    base2: &PodRistrettoPoint,
    c: &PodScalar,
) -> Result<PodRistrettoPoint> {
    let left = multiply_ristretto(s, base1)
        .ok_or(GovernanceError::InvalidSumProof)?;
    let right = multiply_ristretto(c, base2)
        .ok_or(GovernanceError::InvalidSumProof)?;
    subtract_ristretto(&left, &right)
        .ok_or_else(|| GovernanceError::InvalidSumProof.into())
}

fn u64_to_scalar(value: u64) -> PodScalar {
    let mut bytes = [0u8; 32];
    bytes[..8].copy_from_slice(&value.to_le_bytes());
    PodScalar(bytes)
}
