use curve25519_dalek::constants::RISTRETTO_BASEPOINT_POINT;
use curve25519_dalek::ristretto::RistrettoPoint;
use curve25519_dalek::scalar::Scalar;
use rand::rngs::OsRng;
use sha2::{Sha256, Sha512, Digest};

use crate::constants::{DOMAIN_VOTE_PROOF, GENERATOR_H_DOMAIN};
use crate::types::*;

/// Get the second generator H (hash-derived, unknown DL relation to G)
pub fn generator_h() -> RistrettoPoint {
    let mut hasher = Sha512::new();
    hasher.update(GENERATOR_H_DOMAIN);
    let hash: [u8; 64] = hasher.finalize().into();
    RistrettoPoint::from_uniform_bytes(&hash)
}

/// Encrypt a vote and generate validity proofs
///
/// Returns 3 ciphertexts (for/against/abstain) and a VoteValidityProof.
pub fn encrypt_and_prove(
    pubkey: &RistrettoPoint,
    choice: VoteChoice,
    weight: u64,
) -> (ElGamalCiphertext, ElGamalCiphertext, ElGamalCiphertext, VoteValidityProof) {
    let g = RISTRETTO_BASEPOINT_POINT;
    let h = generator_h();
    let weight_scalar = Scalar::from(weight);
    let weight_h = weight_scalar * h;

    // Determine vote values: exactly one is 1, others are 0
    let (v_for, v_against, v_abstain) = match choice {
        VoteChoice::For => (1u64, 0u64, 0u64),
        VoteChoice::Against => (0u64, 1u64, 0u64),
        VoteChoice::Abstain => (0u64, 0u64, 1u64),
    };

    // Encrypt each category
    let (ct_for, r_for) = encrypt(pubkey, &h, v_for * weight);
    let (ct_against, r_against) = encrypt(pubkey, &h, v_against * weight);
    let (ct_abstain, r_abstain) = encrypt(pubkey, &h, v_abstain * weight);

    // Generate OR proofs
    let or_proof_for = generate_or_proof(
        &g, pubkey, &weight_h, &ct_for, r_for, v_for == 1, b"for",
    );
    let or_proof_against = generate_or_proof(
        &g, pubkey, &weight_h, &ct_against, r_against, v_against == 1, b"against",
    );
    let or_proof_abstain = generate_or_proof(
        &g, pubkey, &weight_h, &ct_abstain, r_abstain, v_abstain == 1, b"abstain",
    );

    // Generate sum proof
    let r_sum = r_for + r_against + r_abstain;
    let sum_proof = generate_sum_proof(
        &g, pubkey, &weight_h, &ct_for, &ct_against, &ct_abstain, r_sum,
    );

    let proof = VoteValidityProof {
        or_proof_for,
        or_proof_against,
        or_proof_abstain,
        sum_proof,
    };

    (ct_for, ct_against, ct_abstain, proof)
}

/// Twisted ElGamal encryption: (r*G, m*H + r*pk)
fn encrypt(
    pk: &RistrettoPoint,
    h: &RistrettoPoint,
    message: u64,
) -> (ElGamalCiphertext, Scalar) {
    let r = Scalar::random(&mut OsRng);
    let ct = ElGamalCiphertext {
        r: r * RISTRETTO_BASEPOINT_POINT,
        c: Scalar::from(message) * h + r * pk,
    };
    (ct, r)
}

/// Generate a 1-of-2 OR proof
///
/// If `is_one` is true, the real branch is 1 (encrypts weight*H).
/// If `is_one` is false, the real branch is 0 (encrypts 0).
fn generate_or_proof(
    g: &RistrettoPoint,
    pk: &RistrettoPoint,
    weight_h: &RistrettoPoint,
    ct: &ElGamalCiphertext,
    randomness: Scalar,
    is_one: bool,
    label: &[u8],
) -> CompressedOrProof {
    let r_point = ct.r;
    let c_point = ct.c;

    if is_one {
        // Real branch is 1 (C encrypts weight*H), simulate branch 0
        let c0 = Scalar::random(&mut OsRng);
        let s0 = Scalar::random(&mut OsRng);
        let k = Scalar::random(&mut OsRng);

        // Simulated branch 0: A0 = s0*G - c0*R, B0 = s0*pk - c0*C
        let a0 = s0 * g - c0 * r_point;
        let b0 = s0 * pk - c0 * c_point;

        // Real branch 1: commitment A1 = k*G, B1 = k*pk
        let a1 = k * g;
        let b1 = k * pk;

        // Full challenge
        let challenge = hash_challenge_full(label, &r_point, &c_point, &a0, &b0, &a1, &b1);
        let c1 = challenge - c0;
        let s1 = k + c1 * randomness; // s1 = k + c1 * r (real response)

        CompressedOrProof { c0, s0, s1 }
    } else {
        // Real branch is 0 (C encrypts 0), simulate branch 1
        let c1 = Scalar::random(&mut OsRng);
        let s1 = Scalar::random(&mut OsRng);
        let k = Scalar::random(&mut OsRng);

        // Real branch 0: commitment A0 = k*G, B0 = k*pk
        let a0 = k * g;
        let b0 = k * pk;

        // Simulated branch 1: A1 = s1*G - c1*R, B1 = s1*pk - c1*(C - weight*H)
        let c_adj = c_point - weight_h;
        let a1 = s1 * g - c1 * r_point;
        let b1 = s1 * pk - c1 * c_adj;

        // Full challenge
        let challenge = hash_challenge_full(label, &r_point, &c_point, &a0, &b0, &a1, &b1);
        let c0 = challenge - c1;
        let s0 = k + c0 * randomness; // s0 = k + c0 * r (real response)

        CompressedOrProof { c0, s0, s1 }
    }
}

/// Generate sum proof: DLEQ that R_sum = (r_for+r_against+r_abstain)*G
/// and C_sum - weight*H = r_sum * pk (i.e. sum encrypts weight*H)
fn generate_sum_proof(
    g: &RistrettoPoint,
    pk: &RistrettoPoint,
    weight_h: &RistrettoPoint,
    ct_for: &ElGamalCiphertext,
    ct_against: &ElGamalCiphertext,
    ct_abstain: &ElGamalCiphertext,
    r_sum: Scalar,
) -> SumProof {
    let r_sum_point = ct_for.r + ct_against.r + ct_abstain.r;
    let c_sum = ct_for.c + ct_against.c + ct_abstain.c;
    let c_adj = c_sum - weight_h;

    let k = Scalar::random(&mut OsRng);
    let a = k * g;
    let b = k * pk;

    let c = hash_sum_challenge(&r_sum_point, &c_adj, &a, &b);
    let s = k + c * r_sum;

    SumProof { c, s }
}

fn hash_challenge_full(
    label: &[u8],
    r: &RistrettoPoint,
    c: &RistrettoPoint,
    a0: &RistrettoPoint,
    b0: &RistrettoPoint,
    a1: &RistrettoPoint,
    b1: &RistrettoPoint,
) -> Scalar {
    let mut hasher = Sha256::new();
    hasher.update(DOMAIN_VOTE_PROOF);
    hasher.update(label);
    hasher.update(r.compress().as_bytes());
    hasher.update(c.compress().as_bytes());
    hasher.update(a0.compress().as_bytes());
    hasher.update(b0.compress().as_bytes());
    hasher.update(a1.compress().as_bytes());
    hasher.update(b1.compress().as_bytes());
    let hash = hasher.finalize();
    Scalar::from_bytes_mod_order(hash.into())
}

fn hash_sum_challenge(
    r_sum: &RistrettoPoint,
    c_adj: &RistrettoPoint,
    a: &RistrettoPoint,
    b: &RistrettoPoint,
) -> Scalar {
    let mut hasher = Sha256::new();
    hasher.update(DOMAIN_VOTE_PROOF);
    hasher.update(b"sum");
    hasher.update(r_sum.compress().as_bytes());
    hasher.update(c_adj.compress().as_bytes());
    hasher.update(a.compress().as_bytes());
    hasher.update(b.compress().as_bytes());
    let hash = hasher.finalize();
    Scalar::from_bytes_mod_order(hash.into())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_encrypt_and_prove_for() {
        let keypair = crate::keygen::generate_elgamal_keypair();
        let (ct_for, ct_against, ct_abstain, proof) =
            encrypt_and_prove(&keypair.public, VoteChoice::For, 100);

        // Verify ciphertexts are valid points (non-identity)
        assert_ne!(ct_for.to_bytes(), [0u8; 64]);
        assert_ne!(ct_against.to_bytes(), [0u8; 64]);
        assert_ne!(ct_abstain.to_bytes(), [0u8; 64]);

        // Proof should serialize to 352 bytes
        assert_eq!(proof.to_bytes().len(), 352);
    }

    #[test]
    fn test_generator_h_matches_on_chain_constant() {
        let h = generator_h();
        let bytes = *h.compress().as_bytes();
        // Must match GENERATOR_H in governance-protocol/src/crypto/constants.rs
        let expected: [u8; 32] = [
            0x78, 0x5c, 0x29, 0xe0, 0x13, 0x5c, 0xea, 0x2e,
            0x4f, 0x17, 0x1a, 0x3b, 0xef, 0x51, 0xc9, 0x83,
            0xfe, 0x45, 0x95, 0x6e, 0xec, 0x3d, 0xa4, 0x55,
            0xeb, 0xc2, 0xac, 0x09, 0xa2, 0xd0, 0x6e, 0x7b,
        ];
        assert_eq!(bytes, expected, "SDK generator_h() must match on-chain GENERATOR_H constant");
    }

    #[test]
    fn test_encrypt_decrypt_roundtrip() {
        let keypair = crate::keygen::generate_elgamal_keypair();
        let h = generator_h();

        let (ct, _r) = encrypt(&keypair.public, &h, 42);

        // Decrypt: M = C - sk * R
        let m_point = ct.c - keypair.secret * ct.r;
        let expected = Scalar::from(42u64) * h;
        assert_eq!(m_point, expected);
    }
}
