use curve25519_dalek::ristretto::RistrettoPoint;
use curve25519_dalek::scalar::Scalar;

use crate::encrypt::generator_h;

/// Compute Lagrange coefficients for a set of committee member indices.
///
/// For members at indices [i_1, i_2, ..., i_k] (0-based), computes
/// L_j = product_{m != j} (x_m / (x_m - x_j))
/// where x_j = index_j + 1 (1-based evaluation points).
pub fn compute_lagrange_coefficients(indices: &[u8]) -> Vec<Scalar> {
    let points: Vec<Scalar> = indices.iter().map(|&i| Scalar::from((i as u64) + 1)).collect();

    points.iter().enumerate().map(|(j, xj)| {
        let mut coeff = Scalar::ONE;
        for (m, xm) in points.iter().enumerate() {
            if m != j {
                coeff *= xm * (xm - xj).invert();
            }
        }
        coeff
    }).collect()
}

/// Combine partial decryptions using Lagrange interpolation.
///
/// D = sum(L_i * partial_i)
pub fn combine_partials(
    lagrange_coefficients: &[Scalar],
    partials: &[RistrettoPoint],
) -> RistrettoPoint {
    lagrange_coefficients.iter()
        .zip(partials.iter())
        .fold(RistrettoPoint::default(), |acc, (coeff, partial)| {
            acc + coeff * partial
        })
}

/// Recover the plaintext tally from the combined decryption.
///
/// Given M = C_sum - D (where D is the combined decryption),
/// find t such that t * H == M using baby-step giant-step.
///
/// max_votes bounds the search space.
pub fn recover_tally(
    combined_decryption: &RistrettoPoint,
    accumulated_c_sum: &RistrettoPoint,
    max_votes: u64,
) -> Option<u64> {
    let m = accumulated_c_sum - combined_decryption;
    let h = generator_h();
    baby_step_giant_step(&m, &h, max_votes)
}

/// Baby-step giant-step discrete log solver for small values.
///
/// Finds x such that x * base == target, where 0 <= x <= max_value.
fn baby_step_giant_step(
    target: &RistrettoPoint,
    base: &RistrettoPoint,
    max_value: u64,
) -> Option<u64> {
    use std::collections::HashMap;

    // Handle the zero case
    if *target == RistrettoPoint::default() {
        return Some(0);
    }

    let m = (max_value as f64).sqrt().ceil() as u64 + 1;

    // Baby step: compute table of j * base for j in 0..m
    let mut table = HashMap::with_capacity(m as usize);
    let mut baby = RistrettoPoint::default();
    for j in 0..m {
        table.insert(baby.compress().to_bytes(), j);
        baby += base;
    }

    // Giant step: compute target - i*m*base for i in 0..m
    let giant_step = Scalar::from(m) * base;
    let neg_giant_step = -giant_step;
    let mut gamma = *target;
    for i in 0..m {
        if let Some(&j) = table.get(&gamma.compress().to_bytes()) {
            let result = i * m + j;
            if result <= max_value {
                return Some(result);
            }
        }
        gamma += neg_giant_step;
    }

    None
}

/// Serialize Lagrange coefficients to bytes for on-chain submission
pub fn lagrange_coefficients_to_bytes(coefficients: &[Scalar]) -> Vec<[u8; 32]> {
    coefficients.iter().map(|c| *c.as_bytes()).collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::keygen::{generate_elgamal_keypair, threshold_keygen};
    use crate::encrypt::{encrypt_and_prove, generator_h};
    use crate::decrypt::generate_partial_decryption;
    use crate::types::VoteChoice;
    use curve25519_dalek::constants::RISTRETTO_BASEPOINT_POINT;

    #[test]
    fn test_lagrange_coefficients() {
        let coeffs = compute_lagrange_coefficients(&[0, 1, 2]);
        assert_eq!(coeffs.len(), 3);
    }

    #[test]
    fn test_baby_step_giant_step() {
        let h = generator_h();
        let target = Scalar::from(42u64) * h;
        let result = baby_step_giant_step(&target, &h, 1000);
        assert_eq!(result, Some(42));
    }

    #[test]
    fn test_baby_step_giant_step_zero() {
        let h = generator_h();
        let target = RistrettoPoint::default();
        let result = baby_step_giant_step(&target, &h, 1000);
        assert_eq!(result, Some(0));
    }

    #[test]
    fn test_full_voting_flow_single_committee() {
        let keypair = generate_elgamal_keypair();
        let h = generator_h();

        // 3 voters: 2 vote For (weight 100), 1 votes Against (weight 50)
        let (ct_for_1, ct_against_1, ct_abstain_1, _) =
            encrypt_and_prove(&keypair.public, VoteChoice::For, 100);
        let (ct_for_2, ct_against_2, ct_abstain_2, _) =
            encrypt_and_prove(&keypair.public, VoteChoice::For, 100);
        let (ct_for_3, ct_against_3, ct_abstain_3, _) =
            encrypt_and_prove(&keypair.public, VoteChoice::Against, 50);

        // Accumulate homomorphically
        let acc_r_for = ct_for_1.r + ct_for_2.r + ct_for_3.r;
        let acc_c_for = ct_for_1.c + ct_for_2.c + ct_for_3.c;
        let acc_r_against = ct_against_1.r + ct_against_2.r + ct_against_3.r;
        let acc_c_against = ct_against_1.c + ct_against_2.c + ct_against_3.c;
        let acc_r_abstain = ct_abstain_1.r + ct_abstain_2.r + ct_abstain_3.r;
        let acc_c_abstain = ct_abstain_1.c + ct_abstain_2.c + ct_abstain_3.c;

        // Single committee member decrypts (threshold = 1)
        let d_for = keypair.secret * acc_r_for;
        let d_against = keypair.secret * acc_r_against;
        let d_abstain = keypair.secret * acc_r_abstain;

        // Recover tallies
        let tally_for = recover_tally(&d_for, &acc_c_for, 1000).unwrap();
        let tally_against = recover_tally(&d_against, &acc_c_against, 1000).unwrap();
        let tally_abstain = recover_tally(&d_abstain, &acc_c_abstain, 1000).unwrap();

        assert_eq!(tally_for, 200);     // 100 + 100
        assert_eq!(tally_against, 50);   // 50
        assert_eq!(tally_abstain, 0);    // 0

        // Verify tally equation: tally * H == C_sum - D
        assert_eq!(Scalar::from(tally_for) * h, acc_c_for - d_for);
        assert_eq!(Scalar::from(tally_against) * h, acc_c_against - d_against);
        assert_eq!(Scalar::from(tally_abstain) * h, acc_c_abstain - d_abstain);
    }

    #[test]
    fn test_full_voting_flow_threshold() {
        let (joint, shares) = threshold_keygen(5, 3);

        // Committee member public keys
        let g = RISTRETTO_BASEPOINT_POINT;
        let member_pubkeys: Vec<_> = shares.iter()
            .map(|(_, sk)| sk * g)
            .collect();

        // Vote: 1 voter, For, weight 100
        let (ct_for, ct_against, ct_abstain, _) =
            encrypt_and_prove(&joint.public, VoteChoice::For, 100);

        // 3 of 5 members submit partial decryptions
        let selected_indices = vec![0u8, 2, 4];
        let mut partials_for = Vec::new();
        let mut partials_against = Vec::new();
        let mut partials_abstain = Vec::new();

        for &idx in &selected_indices {
            let (_, secret_share) = &shares[idx as usize];
            let partial = generate_partial_decryption(
                secret_share,
                &member_pubkeys[idx as usize],
                &ct_for.r,
                &ct_against.r,
                &ct_abstain.r,
            );
            partials_for.push(partial.partial_for);
            partials_against.push(partial.partial_against);
            partials_abstain.push(partial.partial_abstain);
        }

        // Combine with Lagrange interpolation
        let lagrange = compute_lagrange_coefficients(&selected_indices);
        let d_for = combine_partials(&lagrange, &partials_for);
        let d_against = combine_partials(&lagrange, &partials_against);
        let d_abstain = combine_partials(&lagrange, &partials_abstain);

        // Recover tallies
        let tally_for = recover_tally(&d_for, &ct_for.c, 1000).unwrap();
        let tally_against = recover_tally(&d_against, &ct_against.c, 1000).unwrap();
        let tally_abstain = recover_tally(&d_abstain, &ct_abstain.c, 1000).unwrap();

        assert_eq!(tally_for, 100);
        assert_eq!(tally_against, 0);
        assert_eq!(tally_abstain, 0);
    }
}
