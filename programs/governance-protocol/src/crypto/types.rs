use anchor_lang::prelude::*;

/// Twisted ElGamal ciphertext: (R, C) where R = r*G, C = m*H + r*pk
/// 64 bytes total
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, Default, Debug, PartialEq)]
pub struct ElGamalCiphertext {
    /// Randomness component: r * G (compressed Ristretto point)
    pub r: [u8; 32],
    /// Ciphertext component: m * H + r * pk (compressed Ristretto point)
    pub c: [u8; 32],
}

/// Compressed 1-of-2 OR proof for vote validity
/// Proves: ciphertext encrypts 0 OR ciphertext encrypts weight*H
/// 96 bytes total
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, Default, Debug, PartialEq)]
pub struct CompressedOrProof {
    /// Challenge for branch 0
    pub c0: [u8; 32],
    /// Response for branch 0
    pub s0: [u8; 32],
    /// Response for branch 1
    pub s1: [u8; 32],
}

/// Sum proof: DLEQ proving that sum of encrypted values equals weight
/// 64 bytes total
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, Default, Debug, PartialEq)]
pub struct SumProof {
    /// Challenge
    pub c: [u8; 32],
    /// Response
    pub s: [u8; 32],
}

/// Complete vote validity proof: 3 OR proofs + 1 sum proof
/// 352 bytes total (3 * 96 + 64)
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, Default, Debug, PartialEq)]
pub struct VoteValidityProof {
    /// OR proof for "for" ciphertext
    pub or_proof_for: CompressedOrProof,
    /// OR proof for "against" ciphertext
    pub or_proof_against: CompressedOrProof,
    /// OR proof for "abstain" ciphertext
    pub or_proof_abstain: CompressedOrProof,
    /// Sum proof: exactly one category is 1
    pub sum_proof: SumProof,
}
