use anchor_lang::prelude::*;
use solana_curve25519::ristretto::{
    subtract_ristretto, multiply_ristretto, validate_ristretto,
    PodRistrettoPoint,
};
use solana_curve25519::scalar::PodScalar;
use solana_sha256_hasher::hashv;

use crate::crypto::constants::{RISTRETTO_BASEPOINT_COMPRESSED, DOMAIN_DLEQ_PROOF};
use crate::errors::GovernanceError;

/// Verify a batched DLEQ proof for a committee member's decryption shares.
///
/// Proves that for a committee member with public key pk_j = sk_j * G:
///   partial_for    = sk_j * R_for_sum
///   partial_against = sk_j * R_against_sum
///   partial_abstain = sk_j * R_abstain_sum
///
/// This is a single batched DLEQ proof over 4 equations:
///   DLOG_G(pk_j) == DLOG_{R_for}(partial_for)
///                 == DLOG_{R_against}(partial_against)
///                 == DLOG_{R_abstain}(partial_abstain)
#[inline(never)]
pub fn verify_batched_dleq(
    pubkey: &[u8; 32],
    bases: &[[u8; 32]; 3],
    results: &[[u8; 32]; 3],
    challenge: &[u8; 32],
    response: &[u8; 32],
) -> Result<()> {
    let g = PodRistrettoPoint(RISTRETTO_BASEPOINT_COMPRESSED);
    let pk = PodRistrettoPoint(*pubkey);

    // Validate all points
    require!(validate_ristretto(&pk), GovernanceError::InvalidRistrettoPoint);
    for base in bases.iter() {
        require!(
            validate_ristretto(&PodRistrettoPoint(*base)),
            GovernanceError::InvalidRistrettoPoint
        );
    }
    for result in results.iter() {
        require!(
            validate_ristretto(&PodRistrettoPoint(*result)),
            GovernanceError::InvalidRistrettoPoint
        );
    }

    let e_scalar = PodScalar(*challenge);
    let s_scalar = PodScalar(*response);

    // Reconstruct commitment for base equation: A0 = s*G - e*pk
    let s_g = multiply_ristretto(&s_scalar, &g)
        .ok_or(GovernanceError::InvalidDleqProof)?;
    let e_pk = multiply_ristretto(&e_scalar, &pk)
        .ok_or(GovernanceError::InvalidDleqProof)?;
    let a0 = subtract_ristretto(&s_g, &e_pk)
        .ok_or(GovernanceError::InvalidDleqProof)?;

    // Reconstruct commitments for each category: A_i = s*R_sum_i - e*partial_i
    let mut a_commitments = [[0u8; 32]; 3];
    for i in 0..3 {
        let base_point = PodRistrettoPoint(bases[i]);
        let result_point = PodRistrettoPoint(results[i]);

        let s_base = multiply_ristretto(&s_scalar, &base_point)
            .ok_or(GovernanceError::InvalidDleqProof)?;
        let e_result = multiply_ristretto(&e_scalar, &result_point)
            .ok_or(GovernanceError::InvalidDleqProof)?;
        let a_i = subtract_ristretto(&s_base, &e_result)
            .ok_or(GovernanceError::InvalidDleqProof)?;
        a_commitments[i] = a_i.0;
    }

    // Verify: e == SHA256(domain || pk || R_for || partial_for || R_against || partial_against ||
    //                      R_abstain || partial_abstain || A0 || A1 || A2 || A3)
    let expected_hash = hashv(&[
        DOMAIN_DLEQ_PROOF,
        pubkey,
        &bases[0],
        &results[0],
        &bases[1],
        &results[1],
        &bases[2],
        &results[2],
        &a0.0,
        &a_commitments[0],
        &a_commitments[1],
        &a_commitments[2],
    ]);

    require!(
        expected_hash.to_bytes() == *challenge,
        GovernanceError::InvalidDleqProof
    );

    Ok(())
}
