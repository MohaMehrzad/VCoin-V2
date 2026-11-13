use anchor_lang::prelude::*;

#[constant]
pub const CONFIG_SEED: &[u8] = b"poll-config";
#[constant]
pub const POLL_SEED: &[u8] = b"poll";
#[constant]
pub const BALLOT_SEED: &[u8] = b"ballot";

pub const BPS: u64 = 10_000;

/// Defaults; the DAO may change them within the bounds below.
pub const DEFAULT_QUORUM_BPS: u16 = 1_000; // 10% of the maximum possible weight
pub const DEFAULT_PASS_BPS: u16 = 5_000; // For must exceed 50% of For + Against
pub const DEFAULT_MIN_VOTING_SECS: i64 = 3 * DAY;
pub const DEFAULT_MAX_VOTING_SECS: i64 = 14 * DAY;
pub const DEFAULT_REVEAL_WINDOW_SECS: i64 = 7 * DAY;
/// Poll weight units (whole VCoin of lock-weighted stake) needed to open a poll.
pub const DEFAULT_MIN_CREATOR_WEIGHT: u64 = 1_000;

pub const DAY: i64 = 24 * 60 * 60;
pub const MIN_PASS_BPS: u16 = 5_000;
pub const MIN_VOTING_SECS_BOUND: i64 = DAY;
pub const MAX_VOTING_SECS_BOUND: i64 = 60 * DAY;
pub const MIN_REVEAL_WINDOW_BOUND: i64 = DAY;
pub const MAX_REVEAL_WINDOW_BOUND: i64 = 30 * DAY;
/// Polls must start within this delay of creation.
pub const MAX_START_DELAY_SECS: i64 = 30 * DAY;
/// A passed poll nobody consumed may be closed after this long.
pub const PASSED_CLOSE_GRACE_SECS: i64 = 90 * DAY;

pub const MAX_COMMITTEE: usize = zk_poll_core::MAX_COMMITTEE as usize;
pub const NUM_CHOICES: usize = zk_poll_core::NUM_CHOICES;
pub const BALLOT_LEN: usize = NUM_CHOICES * 64;
