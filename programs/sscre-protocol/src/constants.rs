/// Seeds
pub const POOL_CONFIG_SEED: &[u8] = b"pool-config";
pub const EPOCH_SEED: &[u8] = b"epoch";
pub const USER_CLAIM_SEED: &[u8] = b"user-claim";
pub const FUNDING_LAYER_SEED: &[u8] = b"funding-layer";
pub const CIRCUIT_BREAKER_SEED: &[u8] = b"circuit-breaker";

/// Pool configuration
pub const PRIMARY_RESERVES: u64 = 350_000_000 * 1_000_000_000;  // 350M VCoin (35% of 1B)
pub const SECONDARY_RESERVES: u64 = 40_000_000 * 1_000_000_000; // 40M VCoin buyback buffer
pub const EPOCH_DURATION: i64 = 30 * 24 * 60 * 60;              // 30 days
pub const CLAIM_WINDOW: i64 = 90 * 24 * 60 * 60;                // 90 days to claim

/// Fee deduction for gasless claims
pub const GASLESS_FEE_BPS: u16 = 100; // 1% deducted for gas

/// Minimum claim amount
pub const MIN_CLAIM_AMOUNT: u64 = 1_000_000_000; // 1 VCoin minimum

/// Circuit breaker thresholds
pub const MAX_EPOCH_EMISSION: u64 = 10_000_000 * 1_000_000_000; // 10M VCoin max per epoch
pub const MAX_SINGLE_CLAIM: u64 = 100_000 * 1_000_000_000;      // 100K VCoin max single claim

/// 5A Score multipliers (x1000 for precision)
pub const SCORE_MULT_0_20: u64 = 100;   // 0.1x (10%)
pub const SCORE_MULT_20_40: u64 = 400;  // 0.4x (40%)
pub const SCORE_MULT_40_60: u64 = 700;  // 0.7x (70%)
pub const SCORE_MULT_60_80: u64 = 1000; // 1.0x (100%)
pub const SCORE_MULT_80_100: u64 = 1200; // 1.2x (120%)

