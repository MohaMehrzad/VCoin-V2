use anchor_lang::prelude::*;

#[constant]
pub const FIVE_A_CONFIG_SEED: &[u8] = b"five-a-config";
#[constant]
pub const USER_SCORE_SEED: &[u8] = b"user-score";
#[constant]
pub const PENDING_SCORE_SEED: &[u8] = b"pending-score";
#[constant]
pub const VOUCH_RECORD_SEED: &[u8] = b"vouch-record";
#[constant]
pub const VOUCH_STATUS_SEED: &[u8] = b"vouch-status";
#[constant]
pub const VOUCHER_STATS_SEED: &[u8] = b"voucher-stats";

pub const DAY: i64 = 24 * 60 * 60;

/// Every dimension and the composite are in `0..=MAX_SCORE` (10000 = 100%).
pub const MAX_SCORE: u16 = 10_000;
/// Composite weights (sum 10000).
pub const AUTHENTICITY_WEIGHT: u16 = 2500;
pub const ACCURACY_WEIGHT: u16 = 2000;
pub const AGILITY_WEIGHT: u16 = 1500;
pub const ACTIVITY_WEIGHT: u16 = 2500;
pub const APPROVED_WEIGHT: u16 = 1500;

/// Oracles: up to 7 registered; a score applies when identical values are submitted by
/// `required_consensus` (2..=5) of the active ones and by a strict majority of them,
/// whichever is larger (audit 5A-01).
pub const MAX_ORACLES: usize = 7;
pub const MIN_CONSENSUS: u8 = 2;
pub const MAX_CONSENSUS: u8 = 5;
pub const DEFAULT_CONSENSUS: u8 = 2;
/// A newly registered oracle counts only after this delay.
pub const ORACLE_ACTIVATION_DELAY: i64 = DAY;
/// A submission counts toward consensus for this long (a sliding window per oracle, so no
/// single oracle's timing decides which submissions are still alive).
pub const SCORE_UPDATE_EXPIRY: i64 = 60 * 60;
/// Minimum time between two applied updates for the same user.
pub const MIN_SCORE_UPDATE_INTERVAL: i64 = 60 * 60;

/// Vouching (reputation only: no stake, no reward).
pub const MIN_VOUCHER_SCORE: u16 = 6000;
/// A vouch is meaningful only for users not already established (audit 5A-05).
pub const MAX_VOUCHEE_SCORE: u16 = 4999;
/// At evaluation, the vouchee's composite must reach this for the vouch to succeed.
pub const VOUCH_SUCCESS_SCORE: u16 = 5000;
/// Live vouches a user can hold at once; the vouchee may decline any of them.
pub const MAX_VOUCHES_RECEIVED: u8 = 3;
pub const VOUCH_EVALUATION_PERIOD: i64 = 90 * DAY;
pub const MAX_VOUCH_AGE: i64 = 365 * DAY;
