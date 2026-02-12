use curve25519_dalek::constants::RISTRETTO_BASEPOINT_POINT;
use curve25519_dalek::ristretto::{CompressedRistretto, RistrettoPoint};
use curve25519_dalek::scalar::Scalar;
use rand::rngs::OsRng;
use sha2::{Sha256, Digest};

use crate::constants::DOMAIN_DLEQ_PROOF;
use crate::types::PartialDecryption;

/// Generate partial decryption for a committee member's secret share.
///
/// For each accumulated ciphertext (R_sum, C_sum), the member computes:
///   partial = sk_j * R_sum
///
/// Also generates a batched DLEQ proof that all partials are correct.
pub fn generate_partial_decryption(
    secret_share: &Scalar,
    public_key: &RistrettoPoint,
    accumulated_r_for: &RistrettoPoint,
    accumulated_r_against: &RistrettoPoint,
    accumulated_r_abstain: &RistrettoPoint,
) -> PartialDecryption {
    let g = RISTRETTO_BASEPOINT_POINT;

    let partial_for = secret_share * accumulated_r_for;
    let partial_against = secret_share * accumulated_r_against;
    let partial_abstain = secret_share * accumulated_r_abstain;

    // Generate batched DLEQ proof
    let k = Scalar::random(&mut OsRng);

    // Commitments: k*G, k*R_for, k*R_against, k*R_abstain
    let a0 = k * g;
    let a1 = k * accumulated_r_for;
    let a2 = k * accumulated_r_against;
    let a3 = k * accumulated_r_abstain;

    // Challenge
    let challenge = hash_dleq_challenge(
        public_key,
        accumulated_r_for, &partial_for,
        accumulated_r_against, &partial_against,
        accumulated_r_abstain, &partial_abstain,
        &a0, &a1, &a2, &a3,
    );

    // Response
    let response = k + challenge * secret_share;

    PartialDecryption {
        partial_for,
        partial_against,
        partial_abstain,
        dleq_challenge: challenge,
        dleq_response: response,
    }
}

/// Decompress a Ristretto point from 32 bytes
pub fn decompress_point(bytes: &[u8; 32]) -> Option<RistrettoPoint> {
    CompressedRistretto::from_slice(bytes).ok()?.decompress()
}

fn hash_dleq_challenge(
    pk: &RistrettoPoint,
    r_for: &RistrettoPoint, partial_for: &RistrettoPoint,
    r_against: &RistrettoPoint, partial_against: &RistrettoPoint,
    r_abstain: &RistrettoPoint, partial_abstain: &RistrettoPoint,
    a0: &RistrettoPoint, a1: &RistrettoPoint, a2: &RistrettoPoint, a3: &RistrettoPoint,
) -> Scalar {
    let mut hasher = Sha256::new();
    hasher.update(DOMAIN_DLEQ_PROOF);
    hasher.update(pk.compress().as_bytes());
    hasher.update(r_for.compress().as_bytes());
    hasher.update(partial_for.compress().as_bytes());
    hasher.update(r_against.compress().as_bytes());
    hasher.update(partial_against.compress().as_bytes());
    hasher.update(r_abstain.compress().as_bytes());
    hasher.update(partial_abstain.compress().as_bytes());
    hasher.update(a0.compress().as_bytes());
    hasher.update(a1.compress().as_bytes());
    hasher.update(a2.compress().as_bytes());
    hasher.update(a3.compress().as_bytes());
    let hash = hasher.finalize();
    Scalar::from_bytes_mod_order(hash.into())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::keygen::generate_elgamal_keypair;
    use crate::encrypt::{encrypt_and_prove, generator_h};
    use crate::types::VoteChoice;

    #[test]
    fn test_partial_decryption() {
        let keypair = generate_elgamal_keypair();
        let (ct_for, ct_against, ct_abstain, _proof) =
            encrypt_and_prove(&keypair.public, VoteChoice::For, 100);

        let partial = generate_partial_decryption(
            &keypair.secret,
            &keypair.public,
            &ct_for.r,
            &ct_against.r,
            &ct_abstain.r,
        );

        // Verify partial: partial_for = sk * R_for
        assert_eq!(partial.partial_for, keypair.secret * ct_for.r);

        // Decrypt: M = C - D where D = partial_for (for single committee member)
        let m_for = ct_for.c - partial.partial_for;
        let h = generator_h();
        let expected_for = Scalar::from(100u64) * h;
        assert_eq!(m_for, expected_for);

        // Against should decrypt to 0
        let m_against = ct_against.c - partial.partial_against;
        let expected_against = Scalar::from(0u64) * h;
        assert_eq!(m_against, expected_against);
    }
}
