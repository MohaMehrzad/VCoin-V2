//! Verifiers. Each returns `Ok(())` only for a valid statement; any malformed input is an
//! error, never a panic.

use crate::constants::*;
use crate::point::{self, s_x_minus_c_y, Point, Scalar};
use crate::scalar;
use crate::transcript::{BallotContext, DleqContext};
use crate::{ZkError, ZkResult};

/// `(R, C) = (r·G, r·PK + v·H)`.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct Ciphertext {
    pub r: Point,
    pub c: Point,
}

/// One ciphertext per choice (For, Against, Abstain).
pub type Ballot = [Ciphertext; NUM_CHOICES];

fn read32(b: &[u8], offset: usize) -> [u8; 32] {
    b[offset..offset + 32].try_into().unwrap()
}

fn canonical(scalars: &[&Scalar]) -> ZkResult<()> {
    if scalars.iter().all(|s| scalar::is_canonical(s)) {
        Ok(())
    } else {
        Err(ZkError::NonCanonicalScalar)
    }
}

/// A key anyone may encrypt to: a valid Ristretto point other than the identity (with the
/// identity key every ballot is readable, audit ZK-03b).
pub fn check_public_key(pk: &Point) -> ZkResult<()> {
    point::validate(pk)?;
    if *pk == IDENTITY {
        return Err(ZkError::InvalidPoint);
    }
    Ok(())
}

/// `MIN_THRESHOLD <= threshold <= size <= MAX_COMMITTEE`.
pub fn check_committee_shape(threshold: u8, size: u8) -> ZkResult<()> {
    if MIN_THRESHOLD <= threshold && threshold <= size && size <= MAX_COMMITTEE {
        Ok(())
    } else {
        Err(ZkError::InvalidCommittee)
    }
}

/// Feldman check: `member_pk == Σ_k index^k · A_k`, with the 1-based `member_index`.
pub fn verify_feldman(commitments: &[Point], member_index: u8, member_pk: &Point) -> ZkResult<()> {
    // At most MAX_COMMITTEE coefficients, so the powers below stay tiny (≤ 5^4) and a
    // direct caller cannot overflow them with an over-long list.
    if commitments.is_empty()
        || commitments.len() > MAX_COMMITTEE as usize
        || member_index == 0
        || member_index > MAX_COMMITTEE
    {
        return Err(ZkError::InvalidCommittee);
    }
    check_public_key(member_pk)?;
    let mut powers = Vec::with_capacity(commitments.len());
    let mut x = 1u64;
    for a in commitments {
        point::validate(a)?;
        powers.push(scalar::from_u64(x));
        x *= member_index as u64; // ≤ 5^4
    }
    if point::msm(&powers, commitments)? != *member_pk {
        return Err(ZkError::FeldmanMismatch);
    }
    Ok(())
}

/// Validates a whole committee produced by a Pedersen DKG: shape, `A_0 == election_pk`,
/// distinct members, and every member key consistent with the commitments (audit ZK-03).
/// `member_pks[i]` belongs to index `i + 1`.
pub fn verify_committee(
    threshold: u8,
    election_pk: &Point,
    commitments: &[Point],
    member_pks: &[Point],
) -> ZkResult<()> {
    let size = u8::try_from(member_pks.len()).map_err(|_| ZkError::InvalidCommittee)?;
    check_committee_shape(threshold, size)?;
    if commitments.len() != threshold as usize || commitments[0] != *election_pk {
        return Err(ZkError::InvalidCommittee);
    }
    check_public_key(election_pk)?;
    if member_pks
        .iter()
        .enumerate()
        .any(|(i, pk)| member_pks[..i].contains(pk))
    {
        return Err(ZkError::InvalidCommittee);
    }
    for (i, pk) in member_pks.iter().enumerate() {
        verify_feldman(commitments, i as u8 + 1, pk)?;
    }
    Ok(())
}

/// Verifies a ballot proof (`BALLOT_PROOF_LEN` bytes: three CDS OR proofs, then the sum
/// proof) for the context's voter, poll and weight.
pub fn verify_ballot(
    ctx: &BallotContext,
    ballot: &Ballot,
    proof: &[u8; BALLOT_PROOF_LEN],
) -> ZkResult<()> {
    check_public_key(&ctx.election_pk)?;
    let pk = ctx.election_pk;
    let w_h = point::mul(&scalar::from_u64(ctx.weight), &GENERATOR_H)?;
    let mut sum_r = IDENTITY;
    let mut sum_c = IDENTITY;

    for (k, ct) in ballot.iter().enumerate() {
        point::validate(&ct.r)?;
        point::validate(&ct.c)?;
        let p = &proof[k * OR_PROOF_LEN..(k + 1) * OR_PROOF_LEN];
        let (c0, c1, s0, s1) = (read32(p, 0), read32(p, 32), read32(p, 64), read32(p, 96));
        canonical(&[&c0, &c1, &s0, &s1])?;

        // Branch 0: C = r·PK.  Branch 1: C − w·H = r·PK.
        let c_minus_wh = point::sub(&ct.c, &w_h)?;
        let a0 = s_x_minus_c_y(&s0, &BASEPOINT_G, &c0, &ct.r)?;
        let b0 = s_x_minus_c_y(&s0, &pk, &c0, &ct.c)?;
        let a1 = s_x_minus_c_y(&s1, &BASEPOINT_G, &c1, &ct.r)?;
        let b1 = s_x_minus_c_y(&s1, &pk, &c1, &c_minus_wh)?;
        let e = ctx.or_challenge(k as u8, &ct.r, &ct.c, &a0, &b0, &a1, &b1);
        if scalar::add(&c0, &c1) != e {
            return Err(ZkError::InvalidBallotProof);
        }
        sum_r = point::add(&sum_r, &ct.r)?;
        sum_c = point::add(&sum_c, &ct.c)?;
    }

    // Σv = w:  ΣC − w·H = (Σr)·PK  with  ΣR = (Σr)·G.
    let p = &proof[NUM_CHOICES * OR_PROOF_LEN..];
    let (c, s) = (read32(p, 0), read32(p, 32));
    canonical(&[&c, &s])?;
    let target = point::sub(&sum_c, &w_h)?;
    let a = s_x_minus_c_y(&s, &BASEPOINT_G, &c, &sum_r)?;
    let b = s_x_minus_c_y(&s, &pk, &c, &target)?;
    if ctx.sum_challenge(&sum_r, &sum_c, &a, &b) != c {
        return Err(ZkError::InvalidBallotProof);
    }
    Ok(())
}

/// Verifies that `d[k] = x·r[k]` for every choice, where `ctx.member_pk = x·G`.
pub fn verify_dleq(
    ctx: &DleqContext,
    r: &[Point; NUM_CHOICES],
    d: &[Point; NUM_CHOICES],
    proof: &[u8; DLEQ_PROOF_LEN],
) -> ZkResult<()> {
    check_public_key(&ctx.member_pk)?;
    let (c, s) = (read32(proof, 0), read32(proof, 32));
    canonical(&[&c, &s])?;
    let a = s_x_minus_c_y(&s, &BASEPOINT_G, &c, &ctx.member_pk)?;
    let mut b = [IDENTITY; NUM_CHOICES];
    for k in 0..NUM_CHOICES {
        point::validate(&r[k])?;
        point::validate(&d[k])?;
        b[k] = s_x_minus_c_y(&s, &r[k], &c, &d[k])?;
    }
    if ctx.challenge(r, d, &a, &b) != c {
        return Err(ZkError::InvalidDleqProof);
    }
    Ok(())
}

/// Checks `tally[k]·H == C[k] − Σ_i λ_i·D_i[k]` for every choice. `partials` pairs each
/// participating member's Lagrange coefficient with its DLEQ-verified partial decryptions.
/// Caller-chosen coefficients cannot forge a tally: an H-component cannot be built from
/// multiples of R without knowing log_G(H).
pub fn verify_tally(
    aggregate_c: &[Point; NUM_CHOICES],
    partials: &[(Scalar, [Point; NUM_CHOICES])],
    tally: &[u64; NUM_CHOICES],
) -> ZkResult<()> {
    if partials.is_empty() {
        return Err(ZkError::InvalidCommittee);
    }
    let lambdas: Vec<Scalar> = partials.iter().map(|(l, _)| *l).collect();
    for k in 0..NUM_CHOICES {
        let d_k: Vec<Point> = partials.iter().map(|(_, d)| d[k]).collect();
        let combined = point::msm(&lambdas, &d_k)?;
        let message = point::sub(&aggregate_c[k], &combined)?;
        if point::mul(&scalar::from_u64(tally[k]), &GENERATOR_H)? != message {
            return Err(ZkError::TallyMismatch);
        }
    }
    Ok(())
}

/// Splits a serialized ballot (`NUM_CHOICES` × `R ‖ C`).
pub fn ballot_from_bytes(bytes: &[u8; NUM_CHOICES * 64]) -> Ballot {
    let mut ballot = [Ciphertext::default(); NUM_CHOICES];
    for (k, ct) in ballot.iter_mut().enumerate() {
        ct.r = read32(bytes, k * 64);
        ct.c = read32(bytes, k * 64 + 32);
    }
    ballot
}

pub fn ballot_to_bytes(ballot: &Ballot) -> [u8; NUM_CHOICES * 64] {
    let mut out = [0u8; NUM_CHOICES * 64];
    for (k, ct) in ballot.iter().enumerate() {
        out[k * 64..k * 64 + 32].copy_from_slice(&ct.r);
        out[k * 64 + 32..k * 64 + 64].copy_from_slice(&ct.c);
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;
    use curve25519_dalek::{
        constants::RISTRETTO_BASEPOINT_POINT, ristretto::RistrettoPoint, scalar::Scalar as DScalar,
    };
    use rand::rngs::OsRng;
    use sha2::{Digest, Sha512};

    #[test]
    fn generators_match_their_derivation() {
        assert_eq!(BASEPOINT_G, RISTRETTO_BASEPOINT_POINT.compress().to_bytes());
        let hash: [u8; 64] = Sha512::new()
            .chain_update(H_DERIVATION_TAG)
            .finalize()
            .into();
        assert_eq!(
            GENERATOR_H,
            RistrettoPoint::from_uniform_bytes(&hash)
                .compress()
                .to_bytes()
        );
    }

    #[test]
    fn scalar_arithmetic_matches_dalek() {
        for _ in 0..2_000 {
            let a = DScalar::random(&mut OsRng);
            let b = DScalar::random(&mut OsRng);
            assert_eq!(scalar::add(a.as_bytes(), b.as_bytes()), (a + b).to_bytes());
            assert_eq!(scalar::neg(a.as_bytes()), (-a).to_bytes());
            let h = scalar::hs(&[a.as_bytes()]);
            assert!(scalar::is_canonical(&h));
            assert_eq!(
                DScalar::from_canonical_bytes(h)
                    .into_option()
                    .map(|s| s.to_bytes()),
                Some(h)
            );
        }
        assert_eq!(scalar::neg(&[0u8; 32]), [0u8; 32]);
    }

    #[test]
    fn canonicity_boundary() {
        let l_minus_1 = (-DScalar::ONE).to_bytes();
        assert!(scalar::is_canonical(&l_minus_1));
        let mut l = l_minus_1;
        l[0] += 1;
        assert!(!scalar::is_canonical(&l));
        let mut l_plus_1 = l;
        l_plus_1[0] += 1;
        assert!(!scalar::is_canonical(&l_plus_1));
        assert!(!scalar::is_canonical(&[0xff; 32]));
        assert_eq!(
            point::mul(&l, &BASEPOINT_G),
            Err(ZkError::NonCanonicalScalar)
        );
    }

    #[test]
    fn keys_and_committee_shape() {
        assert_eq!(check_public_key(&IDENTITY), Err(ZkError::InvalidPoint));
        assert_eq!(check_public_key(&[0xff; 32]), Err(ZkError::InvalidPoint));
        assert_eq!(check_public_key(&GENERATOR_H), Ok(()));
        for (t, n, ok) in [
            (3, 3, true),
            (3, 5, true),
            (5, 5, true),
            (2, 3, false),
            (4, 3, false),
            (3, 6, false),
            (0, 0, false),
        ] {
            assert_eq!(check_committee_shape(t, n).is_ok(), ok, "t={t} n={n}");
        }
    }

    #[test]
    fn ballot_bytes_round_trip() {
        let mut bytes = [0u8; NUM_CHOICES * 64];
        bytes.iter_mut().enumerate().for_each(|(i, b)| *b = i as u8);
        assert_eq!(ballot_to_bytes(&ballot_from_bytes(&bytes)), bytes);
    }

    #[test]
    fn malformed_inputs_are_errors_not_panics() {
        let ctx = BallotContext {
            program_id: [1; 32],
            poll: [2; 32],
            voter: [3; 32],
            election_pk: GENERATOR_H,
            weight: 5,
        };
        let ballot = [Ciphertext {
            r: [0xff; 32],
            c: IDENTITY,
        }; NUM_CHOICES];
        assert_eq!(
            verify_ballot(&ctx, &ballot, &[0u8; BALLOT_PROOF_LEN]),
            Err(ZkError::InvalidPoint)
        );
        let ballot = [Ciphertext::default(); NUM_CHOICES];
        assert_eq!(
            verify_ballot(&ctx, &ballot, &[0xff; BALLOT_PROOF_LEN]),
            Err(ZkError::NonCanonicalScalar)
        );
        assert_eq!(
            verify_ballot(&ctx, &ballot, &[0u8; BALLOT_PROOF_LEN]),
            Err(ZkError::InvalidBallotProof)
        );
        assert_eq!(
            verify_tally(&[IDENTITY; 3], &[], &[0; 3]),
            Err(ZkError::InvalidCommittee)
        );
        assert_eq!(
            verify_feldman(&[], 1, &GENERATOR_H),
            Err(ZkError::InvalidCommittee)
        );
        assert_eq!(
            verify_feldman(&[GENERATOR_H], 0, &GENERATOR_H),
            Err(ZkError::InvalidCommittee)
        );
    }
}
