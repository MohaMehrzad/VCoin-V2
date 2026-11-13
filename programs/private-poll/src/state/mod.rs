use anchor_lang::prelude::*;
use staking_protocol::state::WeightParams;
use vcoin_common::AdminState;

use crate::constants::*;

/// Decryption committee registered by the DAO (audit ZK-03: not chosen per poll). Polls
/// snapshot it at creation, so a later change never affects a running poll.
#[derive(
    AnchorSerialize, AnchorDeserialize, Clone, Copy, Default, Debug, PartialEq, Eq, InitSpace,
)]
pub struct Committee {
    /// Incremented on every change; 0 = never set.
    pub epoch: u32,
    pub threshold: u8,
    pub size: u8,
    /// `commitments[0]`.
    pub election_pk: [u8; 32],
    /// Feldman commitments `A_0..A_{threshold-1}` (unused slots zero).
    pub commitments: [[u8; 32]; MAX_COMMITTEE],
    /// `member_pks[i] = Σ_k (i+1)^k·A_k`, verified at registration.
    pub member_pks: [[u8; 32]; MAX_COMMITTEE],
    /// Wallet allowed to submit member `i + 1`'s decryption share.
    pub member_authorities: [Pubkey; MAX_COMMITTEE],
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, Debug, PartialEq, Eq, InitSpace)]
pub struct PollParams {
    /// Minimum total weight cast, as basis points of the maximum possible weight.
    pub quorum_bps: u16,
    /// For must exceed this share (bps) of For + Against.
    pub pass_bps: u16,
    pub min_voting_secs: i64,
    pub max_voting_secs: i64,
    pub reveal_window_secs: i64,
    /// Poll weight units the creator must hold.
    pub min_creator_weight: u64,
}

impl Default for PollParams {
    fn default() -> Self {
        Self {
            quorum_bps: DEFAULT_QUORUM_BPS,
            pass_bps: DEFAULT_PASS_BPS,
            min_voting_secs: DEFAULT_MIN_VOTING_SECS,
            max_voting_secs: DEFAULT_MAX_VOTING_SECS,
            reveal_window_secs: DEFAULT_REVEAL_WINDOW_SECS,
            min_creator_weight: DEFAULT_MIN_CREATOR_WEIGHT,
        }
    }
}

impl PollParams {
    pub fn is_valid(&self) -> bool {
        self.quorum_bps as u64 <= BPS
            && (MIN_PASS_BPS..BPS as u16).contains(&self.pass_bps)
            && MIN_VOTING_SECS_BOUND <= self.min_voting_secs
            && self.min_voting_secs <= self.max_voting_secs
            && self.max_voting_secs <= MAX_VOTING_SECS_BOUND
            && (MIN_REVEAL_WINDOW_BOUND..=MAX_REVEAL_WINDOW_BOUND)
                .contains(&self.reveal_window_secs)
    }
}

/// Singleton, PDA `["poll-config"]`.
#[account]
#[derive(InitSpace)]
pub struct PollConfig {
    /// Bootstrapped by the upgrade authority, handed to the Realms governance PDA.
    pub admin: AdminState,
    /// staking-protocol registrar whose voters vote here.
    pub registrar: Pubkey,
    pub committee: Committee,
    pub params: PollParams,
    pub poll_count: u64,
    pub paused: bool,
    pub paused_since: i64,
    /// Total seconds spent paused, so reveal deadlines can be extended by downtime.
    pub pause_accum: i64,
    pub bump: u8,
}

impl PollConfig {
    pub const LEN: usize = 8 + Self::INIT_SPACE;

    /// Paused seconds so far, including a pause in progress.
    pub fn paused_total(&self, now: i64) -> i64 {
        self.pause_accum
            + if self.paused {
                now.saturating_sub(self.paused_since)
            } else {
                0
            }
    }
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, Debug, PartialEq, Eq, InitSpace)]
pub enum PollOutcome {
    /// Created; voting, revealing or awaiting finalization depending on the clock.
    Open,
    Passed,
    Rejected,
    /// Reveal window closed without enough decryption shares.
    Expired,
}

/// One poll, PDA `["poll", id]`.
#[account]
#[derive(InitSpace)]
pub struct Poll {
    pub id: u64,
    pub creator: Pubkey,
    /// The only signer that may consume a pass (e.g. the Realms governance PDA).
    pub consumer: Pubkey,
    pub description_hash: [u8; 32],
    pub start_time: i64,
    pub end_time: i64,
    pub reveal_deadline: i64,
    /// Snapshot so every voter's weight uses the same parameters and timestamp.
    pub weight_params: WeightParams,
    pub quorum_weight: u64,
    pub pass_bps: u16,
    pub committee: Committee,
    /// Running sums of every ballot's `R` and `C` per choice.
    pub agg_r: [[u8; 32]; NUM_CHOICES],
    pub agg_c: [[u8; 32]; NUM_CHOICES],
    pub total_weight: u64,
    pub voter_count: u32,
    /// Bit `i` set = member `i + 1` submitted a verified share.
    pub shares_bitmap: u8,
    pub partials: [[[u8; 32]; NUM_CHOICES]; MAX_COMMITTEE],
    pub outcome: PollOutcome,
    pub tally: [u64; NUM_CHOICES],
    pub finalized_at: i64,
    pub consumed: bool,
    /// `PollConfig::pause_accum` when the poll was created.
    pub pause_accum_at_creation: i64,
    pub bump: u8,
}

impl Poll {
    pub const LEN: usize = 8 + Self::INIT_SPACE;

    /// The reveal window closes at `reveal_deadline` plus any time the program was paused
    /// since the poll was created: shares cannot be posted while paused, so a pause must
    /// never let a decryptable poll expire. The voting window is *not* extended, because
    /// weights are fixed at `end_time`.
    pub fn effective_reveal_deadline(&self, config: &PollConfig, now: i64) -> i64 {
        self.reveal_deadline.saturating_add(
            config
                .paused_total(now)
                .saturating_sub(self.pause_accum_at_creation),
        )
    }

    pub fn shares_submitted(&self) -> u32 {
        self.shares_bitmap.count_ones()
    }
}

/// One per (poll, voter): prevents double voting. PDA `["ballot", poll, voter]`.
#[account]
#[derive(InitSpace)]
pub struct BallotRecord {
    pub poll: Pubkey,
    pub voter: Pubkey,
    pub weight: u64,
    /// SHA-256 of the 192-byte ballot (the ballot itself is in the transaction and event).
    pub ballot_hash: [u8; 32],
    pub cast_at: i64,
    pub bump: u8,
}

impl BallotRecord {
    pub const LEN: usize = 8 + Self::INIT_SPACE;
}
