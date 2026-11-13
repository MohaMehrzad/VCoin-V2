//! Off-chain SDK for VCoin private polls.
//!
//! * [`dkg`]: Pedersen distributed key generation with Feldman share checks. No function
//!   returns (or can compute) the joint secret; each member only ever holds its own share
//!   (audit ZK-03a).
//! * [`ballot`]: encrypts a choice under the election key with the 448-byte proof the
//!   program verifies.
//! * [`decrypt`]: a committee member's partial decryption of the aggregate, with DLEQ proof.
//! * [`tally`]: Lagrange combination (duplicate indices rejected) and bounded discrete-log
//!   recovery of the three totals (audit ZK-10).
//!
//! Every artifact is checked with `zk-poll-core` before it is returned, the same code the
//! program runs, so the SDK cannot emit something the chain would reject.
//!
//! Secrets (`ShareMessage`, `KeyShare`, `Dealer` coefficients, ballot randomness) are
//! zeroized on drop and never printed by `Debug`.

pub mod ballot;
pub mod decrypt;
pub mod dkg;
pub mod error;
mod points;
pub mod tally;
#[cfg(feature = "wasm")]
pub mod wasm;

pub use ballot::{encrypt_ballot, Choice, EncryptedBallot};
pub use decrypt::{partial_decrypt, PartialDecryption};
pub use dkg::{
    aggregate_dealings, check_revealed_share, finish, Dealer, GroupKey, KeyShare, PublicDealing,
    ShareMessage, DKG_POK_LEN,
};
pub use error::{SdkError, SdkResult};
pub use tally::{combine, lagrange_at_zero, TallyResult};
pub use zk_poll_core::{
    ballot_from_bytes, ballot_to_bytes, poll_weight, verify_ballot, verify_committee, verify_dleq,
    verify_feldman, verify_tally, Ballot, BallotContext, Ciphertext, DleqContext, ZkError,
    BALLOT_PROOF_LEN, DLEQ_PROOF_LEN, MAX_COMMITTEE, MAX_TALLY, MIN_THRESHOLD, NUM_CHOICES,
    WEIGHT_UNIT,
};
