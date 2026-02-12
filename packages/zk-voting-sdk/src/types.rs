use curve25519_dalek::ristretto::{CompressedRistretto, RistrettoPoint};
use curve25519_dalek::scalar::Scalar;

/// ElGamal ciphertext (R, C)
#[derive(Clone, Debug)]
pub struct ElGamalCiphertext {
    pub r: RistrettoPoint,
    pub c: RistrettoPoint,
}

impl ElGamalCiphertext {
    /// Serialize to 64 bytes: R_compressed || C_compressed
    pub fn to_bytes(&self) -> [u8; 64] {
        let mut bytes = [0u8; 64];
        bytes[..32].copy_from_slice(self.r.compress().as_bytes());
        bytes[32..64].copy_from_slice(self.c.compress().as_bytes());
        bytes
    }

    /// Deserialize from 64 bytes
    pub fn from_bytes(bytes: &[u8; 64]) -> Option<Self> {
        let r = CompressedRistretto::from_slice(&bytes[..32]).ok()?.decompress()?;
        let c = CompressedRistretto::from_slice(&bytes[32..64]).ok()?.decompress()?;
        Some(Self { r, c })
    }
}

/// Compressed 1-of-2 OR proof (96 bytes)
#[derive(Clone, Debug)]
pub struct CompressedOrProof {
    pub c0: Scalar,
    pub s0: Scalar,
    pub s1: Scalar,
}

impl CompressedOrProof {
    pub fn to_bytes(&self) -> [u8; 96] {
        let mut bytes = [0u8; 96];
        bytes[..32].copy_from_slice(self.c0.as_bytes());
        bytes[32..64].copy_from_slice(self.s0.as_bytes());
        bytes[64..96].copy_from_slice(self.s1.as_bytes());
        bytes
    }
}

/// Sum proof (64 bytes)
#[derive(Clone, Debug)]
pub struct SumProof {
    pub c: Scalar,
    pub s: Scalar,
}

impl SumProof {
    pub fn to_bytes(&self) -> [u8; 64] {
        let mut bytes = [0u8; 64];
        bytes[..32].copy_from_slice(self.c.as_bytes());
        bytes[32..64].copy_from_slice(self.s.as_bytes());
        bytes
    }
}

/// Complete vote validity proof (352 bytes)
#[derive(Clone, Debug)]
pub struct VoteValidityProof {
    pub or_proof_for: CompressedOrProof,
    pub or_proof_against: CompressedOrProof,
    pub or_proof_abstain: CompressedOrProof,
    pub sum_proof: SumProof,
}

impl VoteValidityProof {
    /// Serialize to 352 bytes for on-chain submission
    pub fn to_bytes(&self) -> Vec<u8> {
        let mut bytes = Vec::with_capacity(352);
        bytes.extend_from_slice(&self.or_proof_for.to_bytes());
        bytes.extend_from_slice(&self.or_proof_against.to_bytes());
        bytes.extend_from_slice(&self.or_proof_abstain.to_bytes());
        bytes.extend_from_slice(&self.sum_proof.to_bytes());
        bytes
    }
}

/// Vote choice
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum VoteChoice {
    For,
    Against,
    Abstain,
}

/// ElGamal keypair
#[derive(Clone, Debug)]
pub struct ElGamalKeypair {
    pub secret: Scalar,
    pub public: RistrettoPoint,
}

/// Partial decryption result from a committee member
#[derive(Clone, Debug)]
pub struct PartialDecryption {
    pub partial_for: RistrettoPoint,
    pub partial_against: RistrettoPoint,
    pub partial_abstain: RistrettoPoint,
    pub dleq_challenge: Scalar,
    pub dleq_response: Scalar,
}

impl PartialDecryption {
    /// Serialize partials to bytes for on-chain submission
    pub fn partial_for_bytes(&self) -> [u8; 32] {
        *self.partial_for.compress().as_bytes()
    }

    pub fn partial_against_bytes(&self) -> [u8; 32] {
        *self.partial_against.compress().as_bytes()
    }

    pub fn partial_abstain_bytes(&self) -> [u8; 32] {
        *self.partial_abstain.compress().as_bytes()
    }

    pub fn challenge_bytes(&self) -> [u8; 32] {
        *self.dleq_challenge.as_bytes()
    }

    pub fn response_bytes(&self) -> [u8; 32] {
        *self.dleq_response.as_bytes()
    }
}
