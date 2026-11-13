//! A committee member's partial decryption of the poll's aggregate ciphertexts.

use curve25519_dalek::scalar::Scalar;
use rand::{CryptoRng, RngCore};
use zeroize::Zeroizing;
use zk_poll_core::{verify_dleq, DleqContext, DLEQ_PROOF_LEN, NUM_CHOICES};

use crate::dkg::KeyShare;
use crate::error::SdkResult;
use crate::points::{bytes, challenge, decompress, G};

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct PartialDecryption {
    /// 1-based committee index.
    pub member_index: u8,
    /// `D_k = x_i·R_k` for each choice.
    pub shares: [[u8; 32]; NUM_CHOICES],
    pub proof: [u8; DLEQ_PROOF_LEN],
}

/// Decrypts the aggregate `R_k` (sum of all ballots' `R` per choice) with `key`'s share and
/// proves it (batched DLEQ over the three choices, bound to program, poll and member).
pub fn partial_decrypt<R: RngCore + CryptoRng>(
    key: &KeyShare,
    program_id: [u8; 32],
    poll: [u8; 32],
    aggregate_r: &[[u8; 32]; NUM_CHOICES],
    rng: &mut R,
) -> SdkResult<PartialDecryption> {
    let x = key.secret();
    let mut d = [[0u8; 32]; NUM_CHOICES];
    let mut b = [[0u8; 32]; NUM_CHOICES];
    let t = Zeroizing::new(Scalar::random(rng));
    for k in 0..NUM_CHOICES {
        let r_k = decompress(&aggregate_r[k])?;
        d[k] = bytes(&(x * r_k));
        b[k] = bytes(&(*t * r_k));
    }
    let ctx = DleqContext {
        program_id,
        poll,
        member_index: key.index,
        member_pk: key.public,
    };
    let c = challenge(ctx.challenge(aggregate_r, &d, &bytes(&(*t * G)), &b));
    let s = *t + c * x;
    let mut proof = [0u8; DLEQ_PROOF_LEN];
    proof[..32].copy_from_slice(c.as_bytes());
    proof[32..].copy_from_slice(s.as_bytes());

    verify_dleq(&ctx, aggregate_r, &d, &proof)?;
    Ok(PartialDecryption {
        member_index: key.index,
        shares: d,
        proof,
    })
}
