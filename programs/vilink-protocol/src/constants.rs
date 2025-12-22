/// Seeds
pub const CONFIG_SEED: &[u8] = b"vilink-config";
pub const ACTION_SEED: &[u8] = b"action";
pub const DAPP_REGISTRY_SEED: &[u8] = b"dapp";
pub const USER_STATS_SEED: &[u8] = b"user-stats";
pub const BATCH_SEED: &[u8] = b"batch";

/// Action types
pub const ACTION_TIP: u8 = 0;
pub const ACTION_VOUCH: u8 = 1;
pub const ACTION_FOLLOW: u8 = 2;
pub const ACTION_CHALLENGE: u8 = 3;
pub const ACTION_STAKE: u8 = 4;
pub const ACTION_CONTENT_REACT: u8 = 5;
pub const ACTION_DELEGATE: u8 = 6;
pub const ACTION_VOTE: u8 = 7;

/// Limits
pub const MAX_ACTIONS_PER_BATCH: usize = 10;
pub const MAX_ACTION_EXPIRY: i64 = 7 * 24 * 60 * 60; // 7 days
pub const MIN_TIP_AMOUNT: u64 = 100_000_000; // 0.1 VCoin
pub const MAX_TIP_AMOUNT: u64 = 10_000_000_000_000; // 10,000 VCoin

/// Fee configuration
pub const PLATFORM_FEE_BPS: u16 = 250; // 2.5%

/// M-02 Security Fix: Platform fee bounds
/// Maximum platform fee (10%)
pub const MAX_PLATFORM_FEE_BPS: u16 = 1000;
/// Minimum platform fee (0.1%)
pub const MIN_PLATFORM_FEE_BPS: u16 = 10;

