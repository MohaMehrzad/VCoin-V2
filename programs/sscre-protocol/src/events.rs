use anchor_lang::prelude::*;

use crate::state::PoolParams;

#[event]
pub struct PoolInitialized {
    pub authority: Pubkey,
    pub reserve_vault: Pubkey,
    pub fee_vault: Pubkey,
    pub first_epoch_budget: u64,
    pub fee_bps: u16,
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
pub struct RootQuorumSet {
    pub quorum: u8,
}

#[event]
pub struct ParamsUpdated {
    pub params: PoolParams,
}

#[event]
pub struct NextEpochBudgetSet {
    pub budget: u64,
}

#[event]
pub struct FeeChangeProposed {
    pub fee_bps: u16,
    pub fee_destination: Pubkey,
    pub eta: i64,
}

#[event]
pub struct FeeChangeApplied {
    pub fee_bps: u16,
    pub fee_destination: Pubkey,
}

#[event]
pub struct FeeChangeCancelled {}

#[event]
pub struct PauseChanged {
    pub paused: bool,
}

#[event]
pub struct EpochStarted {
    pub index: u64,
    pub budget: u64,
    pub start_ts: i64,
    pub end_ts: i64,
    pub next_epoch_budget: u64,
}

#[event]
pub struct RootVoted {
    pub index: u64,
    pub oracle: Pubkey,
    pub root: [u8; 32],
    pub max_total_claim: u64,
    pub votes_for_candidate: u8,
}

#[event]
pub struct RootProposed {
    pub index: u64,
    pub root: [u8; 32],
    pub max_total_claim: u64,
    pub claimable_at: i64,
}

#[event]
pub struct RootVetoed {
    pub index: u64,
    pub root: [u8; 32],
}

#[event]
pub struct RootActivated {
    pub index: u64,
    pub root: [u8; 32],
    pub max_total_claim: u64,
    pub claim_deadline: i64,
}

#[event]
pub struct RewardClaimed {
    pub index: u64,
    pub claimant: Pubkey,
    pub gross: u64,
    pub fee: u64,
    pub total_claimed: u64,
}

#[event]
pub struct EpochClawedBack {
    pub index: u64,
    pub returned: u64,
    pub total_claimed: u64,
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

/// The emergency guardian may pause (never unpause); `Pubkey::default()` = none.
#[event]
pub struct GuardianSet {
    pub guardian: Pubkey,
}

#[event]
pub struct FeesWithdrawn {
    pub amount: u64,
    pub destination: Pubkey,
}
