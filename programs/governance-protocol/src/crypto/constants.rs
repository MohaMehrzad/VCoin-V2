/// Ristretto255 basepoint G (compressed encoding)
/// This is the canonical generator point for Ristretto255.
/// Source: curve25519-dalek RISTRETTO_BASEPOINT_COMPRESSED
pub const RISTRETTO_BASEPOINT_COMPRESSED: [u8; 32] = [
    0xe2, 0xf2, 0xae, 0x0a, 0x6a, 0xbc, 0x4e, 0x71,
    0xa8, 0x84, 0xa9, 0x61, 0xc5, 0x00, 0x51, 0x5f,
    0x58, 0xe3, 0x0b, 0x6a, 0xa5, 0x82, 0xdd, 0x8d,
    0xb6, 0xa6, 0x59, 0x45, 0xe0, 0x8d, 0x2d, 0x76,
];

/// Second generator H, derived from hashing a domain separator
/// H = SHA512("VCoin-Governance-ZK-Vote-GeneratorH-v1") → from_uniform_bytes
///
/// Precomputed using curve25519-dalek v4's RistrettoPoint::from_uniform_bytes
/// with SHA-512 of the domain string. This ensures H has unknown discrete log
/// relation to G, which is essential for the binding property of Twisted ElGamal.
///
/// To verify: In the SDK crate, run:
///   use sha2::{Sha512, Digest};
///   let hash: [u8; 64] = Sha512::new().chain_update(b"VCoin-Governance-ZK-Vote-GeneratorH-v1").finalize().into();
///   let h = RistrettoPoint::from_uniform_bytes(&hash);
///   assert_eq!(h.compress().to_bytes(), GENERATOR_H);
pub const GENERATOR_H: [u8; 32] = [
    0x78, 0x5c, 0x29, 0xe0, 0x13, 0x5c, 0xea, 0x2e,
    0x4f, 0x17, 0x1a, 0x3b, 0xef, 0x51, 0xc9, 0x83,
    0xfe, 0x45, 0x95, 0x6e, 0xec, 0x3d, 0xa4, 0x55,
    0xeb, 0xc2, 0xac, 0x09, 0xa2, 0xd0, 0x6e, 0x7b,
];

/// Domain separator for vote validity proofs (OR proofs + sum proof)
pub const DOMAIN_VOTE_PROOF: &[u8] = b"VCoin-ZK-VoteValidityProof-v1";

/// Domain separator for DLEQ proofs (decryption shares)
pub const DOMAIN_DLEQ_PROOF: &[u8] = b"VCoin-ZK-DLEQProof-v1";

/// Ristretto identity point (all zeros in compressed form)
pub const RISTRETTO_IDENTITY: [u8; 32] = [0u8; 32];
