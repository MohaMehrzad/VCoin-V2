//! Recovering the three totals from at least `threshold` partial decryptions.

use std::collections::HashMap;

use curve25519_dalek::{ristretto::RistrettoPoint, scalar::Scalar};
use zk_poll_core::{verify_tally, MAX_TALLY, NUM_CHOICES};

use crate::decrypt::PartialDecryption;
use crate::error::{SdkError, SdkResult};
use crate::points::{bytes, decompress, h};

/// Totals plus the Lagrange coefficients (aligned with the input partials) to submit
/// on-chain with them.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct TallyResult {
    pub tally: [u64; NUM_CHOICES],
    pub lagrange: Vec<[u8; 32]>,
}

/// `λ_j = Π_{m≠j} x_m / (x_m − x_j)` at x = 0 for 1-based indices. Rejects an empty set, a
/// zero index and duplicates (a duplicate would make a denominator zero; audit ZK-10).
pub fn lagrange_at_zero(indices: &[u8]) -> SdkResult<Vec<Scalar>> {
    if indices.is_empty() {
        return Err(SdkError::NotEnoughShares { needed: 1, got: 0 });
    }
    for (i, &idx) in indices.iter().enumerate() {
        if idx == 0 {
            return Err(SdkError::InvalidIndex(0));
        }
        if indices[..i].contains(&idx) {
            return Err(SdkError::DuplicateIndex(idx));
        }
    }
    let xs: Vec<Scalar> = indices.iter().map(|&i| Scalar::from(i as u64)).collect();
    Ok(xs
        .iter()
        .enumerate()
        .map(|(j, xj)| {
            xs.iter()
                .enumerate()
                .filter(|(m, _)| *m != j)
                .fold(Scalar::ONE, |acc, (_, xm)| acc * xm * (xm - xj).invert())
        })
        .collect())
}

/// Baby-step table for `j·H`, `0 <= j < m`.
struct DiscreteLog {
    table: HashMap<[u8; 32], u64>,
    m: u64,
    giant: RistrettoPoint,
    max: u64,
}

impl DiscreteLog {
    fn new(max: u64) -> Self {
        let m = (max as f64 + 1.0).sqrt().ceil() as u64;
        let h = h();
        let mut table = HashMap::with_capacity(m as usize);
        let mut p = RistrettoPoint::default();
        for j in 0..m {
            table.entry(bytes(&p)).or_insert(j);
            p += h;
        }
        Self {
            table,
            m,
            giant: -(Scalar::from(m) * h),
            max,
        }
    }

    fn solve(&self, target: &RistrettoPoint) -> Option<u64> {
        let mut gamma = *target;
        for i in 0..=self.m {
            if let Some(&j) = self.table.get(&bytes(&gamma)) {
                let v = i * self.m + j;
                return (v <= self.max).then_some(v);
            }
            gamma += self.giant;
        }
        None
    }
}

/// Combines partial decryptions (already DLEQ-verified on-chain) into the three totals.
/// `max_tally` bounds the search: use the total weight cast in the poll, since no choice can
/// exceed it (audit ZK-10: the vote count is the wrong bound).
pub fn combine(
    threshold: u8,
    aggregate_c: &[[u8; 32]; NUM_CHOICES],
    partials: &[PartialDecryption],
    max_tally: u64,
) -> SdkResult<TallyResult> {
    if partials.len() < threshold as usize {
        return Err(SdkError::NotEnoughShares {
            needed: threshold,
            got: partials.len(),
        });
    }
    if max_tally > MAX_TALLY {
        return Err(SdkError::TallyOutOfRange(MAX_TALLY));
    }
    let indices: Vec<u8> = partials.iter().map(|p| p.member_index).collect();
    let lambdas = lagrange_at_zero(&indices)?;
    let dlog = DiscreteLog::new(max_tally);

    let mut tally = [0u64; NUM_CHOICES];
    for k in 0..NUM_CHOICES {
        let mut combined = RistrettoPoint::default();
        for (l, p) in lambdas.iter().zip(partials) {
            combined += l * decompress(&p.shares[k])?;
        }
        let message = decompress(&aggregate_c[k])? - combined;
        tally[k] = dlog
            .solve(&message)
            .ok_or(SdkError::TallyOutOfRange(max_tally))?;
    }

    let lagrange: Vec<[u8; 32]> = lambdas.iter().map(|l| l.to_bytes()).collect();
    let pairs: Vec<([u8; 32], [[u8; 32]; NUM_CHOICES])> = lagrange
        .iter()
        .zip(partials)
        .map(|(l, p)| (*l, p.shares))
        .collect();
    verify_tally(aggregate_c, &pairs, &tally)?;
    Ok(TallyResult { tally, lagrange })
}
