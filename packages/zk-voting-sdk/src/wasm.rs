//! wasm-bindgen exports for the TypeScript SDK (voter side). Committee tooling (DKG,
//! decryption, tally) stays in Rust.
//!
//! Byte layouts: keys and ids are 32 bytes; a ballot is `R ‖ C` per choice (192 bytes)
//! followed by the 448-byte proof.

use rand::rngs::OsRng;
use wasm_bindgen::prelude::*;
use zk_poll_core::{
    ballot_from_bytes, verify_ballot, BallotContext, BALLOT_PROOF_LEN, NUM_CHOICES,
};

use crate::ballot::{encrypt_ballot, Choice};

const BALLOT_LEN: usize = NUM_CHOICES * 64;

fn key(bytes: &[u8], what: &str) -> Result<[u8; 32], JsError> {
    bytes
        .try_into()
        .map_err(|_| JsError::new(&format!("{what} must be 32 bytes")))
}

fn context(
    program_id: &[u8],
    poll: &[u8],
    voter: &[u8],
    election_pk: &[u8],
    weight: u64,
) -> Result<BallotContext, JsError> {
    Ok(BallotContext {
        program_id: key(program_id, "program_id")?,
        poll: key(poll, "poll")?,
        voter: key(voter, "voter")?,
        election_pk: key(election_pk, "election_pk")?,
        weight,
    })
}

/// Encrypts `choice` (0 For, 1 Against, 2 Abstain) and returns ballot ‖ proof (640 bytes).
#[wasm_bindgen(js_name = encryptBallot)]
pub fn encrypt_ballot_js(
    program_id: &[u8],
    poll: &[u8],
    voter: &[u8],
    election_pk: &[u8],
    weight: u64,
    choice: u8,
) -> Result<Vec<u8>, JsError> {
    let ctx = context(program_id, poll, voter, election_pk, weight)?;
    let choice = Choice::try_from(choice).map_err(|e| JsError::new(&e.to_string()))?;
    let b = encrypt_ballot(&ctx, choice, &mut OsRng).map_err(|e| JsError::new(&e.to_string()))?;
    let mut out = b.ballot_bytes().to_vec();
    out.extend_from_slice(&b.proof);
    Ok(out)
}

/// Verifies ballot ‖ proof exactly as the program does.
#[wasm_bindgen(js_name = verifyBallot)]
pub fn verify_ballot_js(
    program_id: &[u8],
    poll: &[u8],
    voter: &[u8],
    election_pk: &[u8],
    weight: u64,
    ballot_and_proof: &[u8],
) -> Result<bool, JsError> {
    let ctx = context(program_id, poll, voter, election_pk, weight)?;
    if ballot_and_proof.len() != BALLOT_LEN + BALLOT_PROOF_LEN {
        return Err(JsError::new("expected 640 bytes"));
    }
    let ballot = ballot_from_bytes(ballot_and_proof[..BALLOT_LEN].try_into().unwrap());
    let proof: &[u8; BALLOT_PROOF_LEN] = ballot_and_proof[BALLOT_LEN..].try_into().unwrap();
    Ok(verify_ballot(&ctx, &ballot, proof).is_ok())
}

/// Poll weight units for a staking voter weight (base units).
#[wasm_bindgen(js_name = pollWeight)]
pub fn poll_weight_js(staking_weight: u64) -> u64 {
    zk_poll_core::poll_weight(staking_weight)
}
