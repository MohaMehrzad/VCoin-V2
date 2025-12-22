/// Staking Protocol Constants

/// Tier thresholds in base units (9 decimals)
pub const BRONZE_THRESHOLD: u64 = 1_000 * 1_000_000_000;      // 1,000 VCoin
pub const SILVER_THRESHOLD: u64 = 5_000 * 1_000_000_000;      // 5,000 VCoin
pub const GOLD_THRESHOLD: u64 = 20_000 * 1_000_000_000;       // 20,000 VCoin
pub const PLATINUM_THRESHOLD: u64 = 100_000 * 1_000_000_000;  // 100,000 VCoin

/// Lock duration limits in seconds
pub const MIN_LOCK_DURATION: i64 = 7 * 24 * 60 * 60;          // 1 week
pub const MAX_LOCK_DURATION: i64 = 4 * 365 * 24 * 60 * 60;    // 4 years
pub const FOUR_YEARS_SECONDS: i64 = 4 * 365 * 24 * 60 * 60;   // 4 years

/// Tier boost multipliers (x1000 for precision)
pub const TIER_BOOST_NONE: u64 = 1000;     // 1.0x
pub const TIER_BOOST_BRONZE: u64 = 1100;   // 1.1x
pub const TIER_BOOST_SILVER: u64 = 1200;   // 1.2x
pub const TIER_BOOST_GOLD: u64 = 1300;     // 1.3x
pub const TIER_BOOST_PLATINUM: u64 = 1400; // 1.4x

/// Fee discount basis points
pub const FEE_DISCOUNT_NONE: u16 = 0;       // 0%
pub const FEE_DISCOUNT_BRONZE: u16 = 1000;  // 10%
pub const FEE_DISCOUNT_SILVER: u16 = 2000;  // 20%
pub const FEE_DISCOUNT_GOLD: u16 = 3000;    // 30%
pub const FEE_DISCOUNT_PLATINUM: u16 = 5000;// 50%

/// PDA Seeds
pub const STAKING_POOL_SEED: &[u8] = b"staking-pool";
pub const USER_STAKE_SEED: &[u8] = b"user-stake";
pub const POOL_VAULT_SEED: &[u8] = b"pool-vault";

