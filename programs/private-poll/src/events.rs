use anchor_lang::prelude::*;

use crate::constants::{BALLOT_LEN, NUM_CHOICES};
use crate::state::{PollOutcome, PollParams};

#[event]
pub struct ConfigInitialized {
    pub authority: Pubkey,
    pub registrar: Pubkey,
}

#[event]
pub struct CommitteeSet {
    pub epoch: u32,
    pub threshold: u8,
    pub size: u8,
    pub election_pk: [u8; 32],
}

#[event]
pub struct PollParamsUpdated {
    pub params: PollParams,
}

#[event]
pub struct PauseChanged {
    pub paused: bool,
}

#[event]
pub struct PollCreated {
    pub poll: Pubkey,
    pub id: u64,
    pub creator: Pubkey,
    pub consumer: Pubkey,
    pub description_hash: [u8; 32],
    pub start_time: i64,
    pub end_time: i64,
    pub reveal_deadline: i64,
    pub committee_epoch: u32,
    pub quorum_weight: u64,
}

/// Carries the full ballot so anyone can recompute the poll's aggregates.
#[event]
pub struct BallotCast {
    pub poll: Pubkey,
    pub voter: Pubkey,
    pub weight: u64,
    pub ballot: [u8; BALLOT_LEN],
    pub total_weight: u64,
}

#[event]
pub struct ShareSubmitted {
    pub poll: Pubkey,
    pub member_index: u8,
    pub shares: [[u8; 32]; NUM_CHOICES],
}

#[event]
pub struct PollFinalized {
    pub poll: Pubkey,
    pub outcome: PollOutcome,
    pub tally: [u64; NUM_CHOICES],
    pub total_weight: u64,
    pub quorum_weight: u64,
}

#[event]
pub struct PollConsumed {
    pub poll: Pubkey,
    pub consumer: Pubkey,
}

#[event]
pub struct AuthorityProposed {
    pub current: Pubkey,
    pub proposed: Pubkey,
    pub ready_at: i64,
}

#[event]
pub struct AuthorityTransferCancelled {
    pub authority: Pubkey,
}

#[event]
pub struct AuthorityTransferred {
    pub previous: Pubkey,
    pub new_authority: Pubkey,
}
