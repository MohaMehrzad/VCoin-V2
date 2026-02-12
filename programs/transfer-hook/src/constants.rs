/// Transfer Hook Constants
///
/// L-AUDIT-07: All constants below are hardcoded at compile time.
/// Changing any value requires a program upgrade (redeployment).
/// This is intentional for on-chain determinism and auditability.
/// Future versions may migrate select parameters to on-chain config
/// accounts to allow runtime governance updates.

/// Minimum transfer amount to count as activity (1 VCoin)
/// L-AUDIT-07: Changing this threshold requires a program upgrade.
pub const MIN_ACTIVITY_THRESHOLD: u64 = 1_000_000_000;

/// Maximum transfers per hour before diminishing activity score
/// L-AUDIT-07: Changing this limit requires a program upgrade.
pub const MAX_TRANSFERS_PER_HOUR: u8 = 20;

/// Wash trading detection: minimum time between transfers to same recipient
/// L-AUDIT-07: Changing this cooldown requires a program upgrade.
pub const WASH_TRADING_COOLDOWN_SECONDS: i64 = 3600; // 1 hour

/// H-AUDIT-09: Maximum activity score contribution per day.
/// Once a user reaches this cap, no further activity score is accrued
/// until the daily reset. Prevents score inflation from spam transfers.
/// L-AUDIT-07: Changing this cap requires a program upgrade.
pub const MAX_DAILY_ACTIVITY_SCORE: u16 = 5000;

/// M-AUDIT-10: Time period after which wash trading flags decay (1 week).
/// If no new wash trading is detected for this duration, accumulated
/// wash_flags are decremented, allowing pairs to rehabilitate over time.
/// L-AUDIT-07: Changing this period requires a program upgrade.
pub const WASH_FLAG_DECAY_PERIOD: i64 = 7 * 86400;

/// PDA Seeds
pub const HOOK_CONFIG_SEED: &[u8] = b"hook-config";
pub const TRANSFER_RECORD_SEED: &[u8] = b"transfer-record";
pub const USER_ACTIVITY_SEED: &[u8] = b"user-activity";
pub const PAIR_TRACKING_SEED: &[u8] = b"pair-tracking";

