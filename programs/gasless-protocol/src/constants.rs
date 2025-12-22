/// Seeds
pub const GASLESS_CONFIG_SEED: &[u8] = b"gasless-config";
pub const SESSION_KEY_SEED: &[u8] = b"session-key";
pub const USER_GASLESS_SEED: &[u8] = b"user-gasless";
pub const FEE_VAULT_SEED: &[u8] = b"fee-vault";
pub const DAILY_BUDGET_SEED: &[u8] = b"daily-budget";

/// Session configuration
pub const SESSION_DURATION: i64 = 24 * 60 * 60;   // 24 hours
pub const MAX_SESSION_ACTIONS: u32 = 1000;        // Max actions per session
pub const MAX_SESSION_SPEND: u64 = 100_000_000_000_000; // 100,000 VCoin max per session

/// Fee configuration
pub const DEFAULT_SOL_FEE: u64 = 5_000;          // 0.000005 SOL per tx
pub const VCOIN_FEE_MULTIPLIER: u64 = 100;      // 100x VCoin equivalent
pub const SSCRE_DEDUCTION_BPS: u16 = 100;       // 1% from SSCRE claims

/// Daily budget
pub const DAILY_SUBSIDY_BUDGET_SOL: u64 = 10_000_000_000; // 10 SOL per day
pub const MAX_SUBSIDIZED_TX_PER_USER: u32 = 50; // Max 50 free tx per user per day

/// Action scope bits
pub const SCOPE_TIP: u16 = 1 << 0;
pub const SCOPE_VOUCH: u16 = 1 << 1;
pub const SCOPE_CONTENT: u16 = 1 << 2;
pub const SCOPE_GOVERNANCE: u16 = 1 << 3;
pub const SCOPE_TRANSFER: u16 = 1 << 4;
pub const SCOPE_STAKE: u16 = 1 << 5;
pub const SCOPE_CLAIM: u16 = 1 << 6;
pub const SCOPE_FOLLOW: u16 = 1 << 7;
pub const SCOPE_ALL: u16 = 0xFFFF;

