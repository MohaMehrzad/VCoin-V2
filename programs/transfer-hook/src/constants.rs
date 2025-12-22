/// Transfer Hook Constants

/// Minimum transfer amount to count as activity (1 VCoin)
pub const MIN_ACTIVITY_THRESHOLD: u64 = 1_000_000_000;

/// Maximum transfers per hour before diminishing activity score
pub const MAX_TRANSFERS_PER_HOUR: u8 = 20;

/// Wash trading detection: minimum time between transfers to same recipient
pub const WASH_TRADING_COOLDOWN_SECONDS: i64 = 3600; // 1 hour

/// PDA Seeds
pub const HOOK_CONFIG_SEED: &[u8] = b"hook-config";
pub const TRANSFER_RECORD_SEED: &[u8] = b"transfer-record";
pub const USER_ACTIVITY_SEED: &[u8] = b"user-activity";
pub const PAIR_TRACKING_SEED: &[u8] = b"pair-tracking";

