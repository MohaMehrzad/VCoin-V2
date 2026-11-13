//! Generators, sizes and domain tags. Changing any tag invalidates every proof.

/// Ristretto255 basepoint G (compressed).
pub const BASEPOINT_G: [u8; 32] = [
    0xe2, 0xf2, 0xae, 0x0a, 0x6a, 0xbc, 0x4e, 0x71, 0xa8, 0x84, 0xa9, 0x61, 0xc5, 0x00, 0x51, 0x5f,
    0x58, 0xe3, 0x0b, 0x6a, 0xa5, 0x82, 0xdd, 0x8d, 0xb6, 0xa6, 0x59, 0x45, 0xe0, 0x8d, 0x2d, 0x76,
];

/// Message generator H = `RistrettoPoint::from_uniform_bytes(SHA-512(H_DERIVATION_TAG))`;
/// nobody knows log_G(H). Re-derived in this crate's tests.
pub const GENERATOR_H: [u8; 32] = [
    0x78, 0x5c, 0x29, 0xe0, 0x13, 0x5c, 0xea, 0x2e, 0x4f, 0x17, 0x1a, 0x3b, 0xef, 0x51, 0xc9, 0x83,
    0xfe, 0x45, 0x95, 0x6e, 0xec, 0x3d, 0xa4, 0x55, 0xeb, 0xc2, 0xac, 0x09, 0xa2, 0xd0, 0x6e, 0x7b,
];
pub const H_DERIVATION_TAG: &[u8] = b"VCoin-Governance-ZK-Vote-GeneratorH-v1";

/// Compressed identity point.
pub const IDENTITY: [u8; 32] = [0u8; 32];

/// Choices per poll: 0 = For, 1 = Against, 2 = Abstain.
pub const NUM_CHOICES: usize = 3;

/// Per choice: `c0 ‖ c1 ‖ s0 ‖ s1`.
pub const OR_PROOF_LEN: usize = 128;
/// Sum proof: `c ‖ s`.
pub const SUM_PROOF_LEN: usize = 64;
/// Ballot proof: three OR proofs then the sum proof.
pub const BALLOT_PROOF_LEN: usize = NUM_CHOICES * OR_PROOF_LEN + SUM_PROOF_LEN;
/// DLEQ proof: `c ‖ s`.
pub const DLEQ_PROOF_LEN: usize = 64;

/// Committee bounds (audit ZK-03d): at least 3 members must cooperate to decrypt.
pub const MIN_THRESHOLD: u8 = 3;
pub const MAX_COMMITTEE: u8 = 5;

/// Poll weights are counted in whole VCoin so tallies stay small enough for discrete-log
/// recovery (2× total supply = 2·10⁹ < 2³²).
pub const WEIGHT_UNIT: u64 = 1_000_000_000;
/// Upper bound on any tally the SDK will search for.
pub const MAX_TALLY: u64 = 1 << 32;

pub const DOMAIN_BALLOT_OR: &[u8] = b"VCoin-PrivatePoll-BallotOR-v2";
pub const DOMAIN_BALLOT_SUM: &[u8] = b"VCoin-PrivatePoll-BallotSum-v2";
pub const DOMAIN_DLEQ: &[u8] = b"VCoin-PrivatePoll-DLEQ-v2";

/// Converts a staking voter weight (base units) to poll weight units.
pub const fn poll_weight(staking_weight: u64) -> u64 {
    staking_weight / WEIGHT_UNIT
}
