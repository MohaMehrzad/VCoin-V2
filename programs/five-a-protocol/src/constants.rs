/// 5A Protocol Constants

/// PDA Seeds
pub const FIVE_A_CONFIG_SEED: &[u8] = b"five-a-config";
pub const USER_SCORE_SEED: &[u8] = b"user-score";
pub const SCORE_SNAPSHOT_SEED: &[u8] = b"score-snapshot";
pub const VOUCH_RECORD_SEED: &[u8] = b"vouch-record";
pub const VOUCH_STATUS_SEED: &[u8] = b"vouch-status";
pub const VOUCHER_STATS_SEED: &[u8] = b"voucher-stats";
pub const ORACLE_SEED: &[u8] = b"oracle";

/// Score weights (out of 10000)
pub const AUTHENTICITY_WEIGHT: u16 = 2500;  // 25%
pub const ACCURACY_WEIGHT: u16 = 2000;      // 20%
pub const AGILITY_WEIGHT: u16 = 1500;       // 15%
pub const ACTIVITY_WEIGHT: u16 = 2500;      // 25%
pub const APPROVED_WEIGHT: u16 = 1500;      // 15%

/// Vouch system
pub const MIN_VOUCHER_SCORE: u16 = 6000;    // 60% 5A score to vouch
pub const VOUCH_STAKE_AMOUNT: u64 = 5_000_000_000; // 5 VCoin
pub const VOUCHES_REQUIRED: u8 = 3;
pub const VOUCH_EVALUATION_PERIOD: i64 = 90 * 24 * 60 * 60; // 90 days
pub const VOUCH_REWARD: u64 = 10_000_000_000; // 10 VCoin bonus for successful vouch

/// Score update intervals
pub const SNAPSHOT_INTERVAL: i64 = 24 * 60 * 60; // Daily snapshots

