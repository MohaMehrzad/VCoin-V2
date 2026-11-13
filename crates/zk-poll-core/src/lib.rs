//! Shared cryptography for VCoin private polls.
//!
//! One implementation of every check, used by the on-chain `private-poll` program (where
//! `solana-curve25519` calls the Ristretto syscalls) and by the off-chain SDK (where the same
//! crate runs curve25519-dalek). A proof that verifies here verifies on-chain.
//!
//! Scheme: exponential ElGamal on Ristretto255. A ballot holds one ciphertext per choice,
//! `(R, C) = (r·G, r·PK + v·H)` with `v ∈ {0, w}` and `Σv = w`, where `w` is the voter's
//! weight computed on-chain. Proofs:
//! * per choice, a CDS 1-of-2 OR proof that `v ∈ {0, w}` (audit ZK-01);
//! * a Chaum–Pedersen proof that the three values sum to `w`;
//! * per committee member, a DLEQ proof that partial decryptions use their key share;
//! * Feldman checks binding member keys to the election key (ZK-03);
//! * a tally check `t·H == C − Σ λ_i·D_i`.
//!
//! Every challenge is [`scalar::hs`] (SHA-256 masked to 252 bits, always canonical) over a
//! transcript that starts with the program, poll, voter, election key, H and weight
//! (ZK-02, ZK-06).

pub mod constants;
pub mod point;
pub mod scalar;
pub mod transcript;
pub mod verify;

pub use constants::*;
pub use transcript::{BallotContext, DleqContext};
pub use verify::*;

/// Why a check failed. Programs map these onto their own error codes.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ZkError {
    /// A point is not a canonical Ristretto encoding (or is the identity where forbidden).
    InvalidPoint,
    /// A scalar is not reduced modulo the group order.
    NonCanonicalScalar,
    /// Ballot validity proof rejected.
    InvalidBallotProof,
    /// DLEQ (decryption share) proof rejected.
    InvalidDleqProof,
    /// A member key does not match the Feldman commitments.
    FeldmanMismatch,
    /// Committee parameters outside `MIN_THRESHOLD <= t <= n <= MAX_COMMITTEE`.
    InvalidCommittee,
    /// Claimed tally does not match the ciphertexts and partial decryptions.
    TallyMismatch,
}

impl core::fmt::Display for ZkError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(self, f)
    }
}

impl std::error::Error for ZkError {}

pub type ZkResult<T> = Result<T, ZkError>;
