//! dalek <-> byte conversions.

use curve25519_dalek::{
    constants::RISTRETTO_BASEPOINT_POINT,
    ristretto::{CompressedRistretto, RistrettoPoint},
    scalar::Scalar,
};
use zk_poll_core::GENERATOR_H;

use crate::error::{SdkError, SdkResult};

pub const G: RistrettoPoint = RISTRETTO_BASEPOINT_POINT;

pub fn h() -> RistrettoPoint {
    decompress(&GENERATOR_H).expect("GENERATOR_H is a valid point")
}

pub fn decompress(bytes: &[u8; 32]) -> SdkResult<RistrettoPoint> {
    CompressedRistretto(*bytes)
        .decompress()
        .ok_or(SdkError::InvalidPoint)
}

pub fn bytes(p: &RistrettoPoint) -> [u8; 32] {
    p.compress().to_bytes()
}

/// Challenges from `zk_poll_core::scalar::hs` are always canonical.
pub fn challenge(bytes: [u8; 32]) -> Scalar {
    Scalar::from_canonical_bytes(bytes).expect("hs output is canonical")
}
