//! Thin wrappers over `solana-curve25519` (syscalls on-chain, dalek off-chain) that turn
//! `None` into [`ZkError`].

use solana_curve25519::ristretto::{
    add_ristretto, multiply_ristretto, multiscalar_multiply_ristretto, subtract_ristretto,
    validate_ristretto, PodRistrettoPoint,
};
use solana_curve25519::scalar::PodScalar;

use crate::scalar;
use crate::{ZkError, ZkResult};

pub type Point = [u8; 32];
pub type Scalar = [u8; 32];

pub fn validate(p: &Point) -> ZkResult<()> {
    if validate_ristretto(&PodRistrettoPoint(*p)) {
        Ok(())
    } else {
        Err(ZkError::InvalidPoint)
    }
}

pub fn add(a: &Point, b: &Point) -> ZkResult<Point> {
    add_ristretto(&PodRistrettoPoint(*a), &PodRistrettoPoint(*b))
        .map(|p| p.0)
        .ok_or(ZkError::InvalidPoint)
}

pub fn sub(a: &Point, b: &Point) -> ZkResult<Point> {
    subtract_ristretto(&PodRistrettoPoint(*a), &PodRistrettoPoint(*b))
        .map(|p| p.0)
        .ok_or(ZkError::InvalidPoint)
}

pub fn mul(s: &Scalar, p: &Point) -> ZkResult<Point> {
    if !scalar::is_canonical(s) {
        return Err(ZkError::NonCanonicalScalar);
    }
    multiply_ristretto(&PodScalar(*s), &PodRistrettoPoint(*p))
        .map(|p| p.0)
        .ok_or(ZkError::InvalidPoint)
}

/// `Σ s_i·P_i` in one call.
pub fn msm(scalars: &[Scalar], points: &[Point]) -> ZkResult<Point> {
    if scalars.iter().any(|s| !scalar::is_canonical(s)) {
        return Err(ZkError::NonCanonicalScalar);
    }
    let s: Vec<PodScalar> = scalars.iter().map(|s| PodScalar(*s)).collect();
    let p: Vec<PodRistrettoPoint> = points.iter().map(|p| PodRistrettoPoint(*p)).collect();
    multiscalar_multiply_ristretto(&s, &p)
        .map(|p| p.0)
        .ok_or(ZkError::InvalidPoint)
}

/// Schnorr-style commitment recomputation `s·X − c·Y`.
pub fn s_x_minus_c_y(s: &Scalar, x: &Point, c: &Scalar, y: &Point) -> ZkResult<Point> {
    if !scalar::is_canonical(c) {
        return Err(ZkError::NonCanonicalScalar);
    }
    msm(&[*s, scalar::neg(c)], &[*x, *y])
}
