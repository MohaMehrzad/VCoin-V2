//! Ballot encryption with the proof `zk_poll_core::verify_ballot` checks.

use curve25519_dalek::{ristretto::RistrettoPoint, scalar::Scalar};
use rand::{CryptoRng, RngCore};
use zeroize::Zeroizing;
use zk_poll_core::{
    ballot_to_bytes, check_public_key, verify_ballot, Ballot, BallotContext, Ciphertext,
    BALLOT_PROOF_LEN, NUM_CHOICES, OR_PROOF_LEN,
};

use crate::error::{SdkError, SdkResult};
use crate::points::{bytes, challenge, decompress, h, G};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Choice {
    For = 0,
    Against = 1,
    Abstain = 2,
}

impl TryFrom<u8> for Choice {
    type Error = SdkError;
    fn try_from(v: u8) -> SdkResult<Self> {
        match v {
            0 => Ok(Choice::For),
            1 => Ok(Choice::Against),
            2 => Ok(Choice::Abstain),
            _ => Err(SdkError::InvalidChoice(v)),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct EncryptedBallot {
    pub ballot: Ballot,
    pub proof: [u8; BALLOT_PROOF_LEN],
}

impl EncryptedBallot {
    /// `R ‖ C` per choice, the layout the program takes.
    pub fn ballot_bytes(&self) -> [u8; NUM_CHOICES * 64] {
        ballot_to_bytes(&self.ballot)
    }
}

/// Encrypts `choice` with weight `ctx.weight` (poll weight units, see
/// `zk_poll_core::poll_weight`) and proves it. The weight must equal what the program
/// computes for this voter, or the proof is rejected.
pub fn encrypt_ballot<R: RngCore + CryptoRng>(
    ctx: &BallotContext,
    choice: Choice,
    rng: &mut R,
) -> SdkResult<EncryptedBallot> {
    if ctx.weight == 0 {
        return Err(SdkError::ZeroWeight);
    }
    check_public_key(&ctx.election_pk)?;
    let pk = decompress(&ctx.election_pk)?;
    let w_h = Scalar::from(ctx.weight) * h();

    let mut ballot = [Ciphertext::default(); NUM_CHOICES];
    let mut proof = [0u8; BALLOT_PROOF_LEN];
    let mut r_sum = Zeroizing::new(Scalar::ZERO);
    let mut sum_r = RistrettoPoint::default();
    let mut sum_c = RistrettoPoint::default();

    for k in 0..NUM_CHOICES {
        let real = usize::from(k == choice as usize); // branch 1 iff this choice carries w
        let r = Zeroizing::new(Scalar::random(rng));
        let big_r = *r * G;
        let big_c = *r * pk
            + if real == 1 {
                w_h
            } else {
                RistrettoPoint::default()
            };
        let targets = [big_c, big_c - w_h];

        // Simulate the false branch, commit honestly on the true one.
        let fake = 1 - real;
        let c_fake = Scalar::random(rng);
        let s_fake = Scalar::random(rng);
        let t = Zeroizing::new(Scalar::random(rng));
        let mut a = [RistrettoPoint::default(); 2];
        let mut b = [RistrettoPoint::default(); 2];
        a[fake] = s_fake * G - c_fake * big_r;
        b[fake] = s_fake * pk - c_fake * targets[fake];
        a[real] = *t * G;
        b[real] = *t * pk;

        let (r_bytes, c_bytes) = (bytes(&big_r), bytes(&big_c));
        let e = challenge(ctx.or_challenge(
            k as u8,
            &r_bytes,
            &c_bytes,
            &bytes(&a[0]),
            &bytes(&b[0]),
            &bytes(&a[1]),
            &bytes(&b[1]),
        ));
        let c_real = e - c_fake;
        let s_real = *t + c_real * *r;
        let (mut c, mut s) = ([Scalar::ZERO; 2], [Scalar::ZERO; 2]);
        (c[real], s[real], c[fake], s[fake]) = (c_real, s_real, c_fake, s_fake);

        let out = &mut proof[k * OR_PROOF_LEN..(k + 1) * OR_PROOF_LEN];
        for (i, v) in [c[0], c[1], s[0], s[1]].iter().enumerate() {
            out[i * 32..i * 32 + 32].copy_from_slice(v.as_bytes());
        }
        ballot[k] = Ciphertext {
            r: r_bytes,
            c: c_bytes,
        };
        *r_sum += *r;
        sum_r += big_r;
        sum_c += big_c;
    }

    // Σv = w: prove log_G(ΣR) = log_PK(ΣC − wH).
    let t = Zeroizing::new(Scalar::random(rng));
    let (a, b) = (*t * G, *t * pk);
    let c = challenge(ctx.sum_challenge(&bytes(&sum_r), &bytes(&sum_c), &bytes(&a), &bytes(&b)));
    let s = *t + c * *r_sum;
    let off = NUM_CHOICES * OR_PROOF_LEN;
    proof[off..off + 32].copy_from_slice(c.as_bytes());
    proof[off + 32..off + 64].copy_from_slice(s.as_bytes());

    verify_ballot(ctx, &ballot, &proof)?;
    Ok(EncryptedBallot { ballot, proof })
}
