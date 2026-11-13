//! Fiat–Shamir transcripts. Every challenge binds the program, the poll and the prover's
//! identity, so a proof cannot be replayed by another voter, in another poll or against
//! another deployment (audit ZK-06).

use crate::constants::*;
use crate::point::{Point, Scalar};
use crate::scalar::hs;

/// Public inputs of one ballot.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct BallotContext {
    pub program_id: [u8; 32],
    pub poll: [u8; 32],
    pub voter: [u8; 32],
    pub election_pk: Point,
    /// Poll weight units (see [`poll_weight`]).
    pub weight: u64,
}

impl BallotContext {
    /// Challenge for the OR proof of `choice`.
    #[allow(clippy::too_many_arguments)]
    pub fn or_challenge(
        &self,
        choice: u8,
        r: &Point,
        c: &Point,
        a0: &Point,
        b0: &Point,
        a1: &Point,
        b1: &Point,
    ) -> Scalar {
        let w = self.weight.to_le_bytes();
        hs(&[
            DOMAIN_BALLOT_OR,
            &self.program_id,
            &self.poll,
            &self.voter,
            &self.election_pk,
            &GENERATOR_H,
            &w,
            &[choice],
            r,
            c,
            a0,
            b0,
            a1,
            b1,
        ])
    }

    /// Challenge for the sum proof over `ΣR`, `ΣC`.
    pub fn sum_challenge(&self, sum_r: &Point, sum_c: &Point, a: &Point, b: &Point) -> Scalar {
        let w = self.weight.to_le_bytes();
        hs(&[
            DOMAIN_BALLOT_SUM,
            &self.program_id,
            &self.poll,
            &self.voter,
            &self.election_pk,
            &GENERATOR_H,
            &w,
            sum_r,
            sum_c,
            a,
            b,
        ])
    }
}

/// Public inputs of one committee member's partial decryption.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct DleqContext {
    pub program_id: [u8; 32],
    pub poll: [u8; 32],
    /// 1-based committee index (the Shamir evaluation point).
    pub member_index: u8,
    pub member_pk: Point,
}

impl DleqContext {
    /// Challenge over the aggregate `R_k`, partials `D_k` and commitments.
    pub fn challenge(
        &self,
        r: &[Point; NUM_CHOICES],
        d: &[Point; NUM_CHOICES],
        a: &Point,
        b: &[Point; NUM_CHOICES],
    ) -> Scalar {
        hs(&[
            DOMAIN_DLEQ,
            &self.program_id,
            &self.poll,
            &[self.member_index],
            &self.member_pk,
            &r[0],
            &r[1],
            &r[2],
            &d[0],
            &d[1],
            &d[2],
            a,
            &b[0],
            &b[1],
            &b[2],
        ])
    }
}
