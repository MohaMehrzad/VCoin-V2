use anchor_lang::prelude::*;

use crate::state::{ScoreValues, VouchOutcome};

#[event]
pub struct FiveAInitialized {
    pub authority: Pubkey,
    pub required_consensus: u8,
}

#[event]
pub struct OracleRegistered {
    pub oracle: Pubkey,
    pub active_from: i64,
}

#[event]
pub struct OracleRemoved {
    pub oracle: Pubkey,
}

#[event]
pub struct ConsensusSet {
    pub required_consensus: u8,
}

#[event]
pub struct PauseChanged {
    pub paused: bool,
}

#[event]
pub struct ScoreSubmitted {
    pub user: Pubkey,
    pub oracle: Pubkey,
    pub values: ScoreValues,
    pub support: u8,
}

#[event]
pub struct ScoreApplied {
    pub user: Pubkey,
    pub values: ScoreValues,
    pub composite_score: u16,
    pub update_count: u32,
}

#[event]
pub struct ScoreRoundExpired {
    pub user: Pubkey,
}

#[event]
pub struct VouchCreated {
    pub voucher: Pubkey,
    pub vouchee: Pubkey,
    pub vouched_at: i64,
}

#[event]
pub struct VouchClosed {
    pub voucher: Pubkey,
    pub vouchee: Pubkey,
    pub outcome: VouchOutcome,
    pub vouchee_score: u16,
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
