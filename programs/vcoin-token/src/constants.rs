/// VCoin Token Constants
/// 
/// Token configuration and PDA seeds

/// Token decimals (9 for VCoin)
pub const VCOIN_DECIMALS: u8 = 9;

/// Total supply: 1 billion tokens with 9 decimals
pub const TOTAL_SUPPLY: u64 = 1_000_000_000 * 1_000_000_000;

/// Token metadata
pub const TOKEN_NAME: &str = "VCoin";
pub const TOKEN_SYMBOL: &str = "VIWO";
pub const TOKEN_URI: &str = "https://viwoapp.com/vcoin-metadata.json";

/// PDA Seeds
pub const VCOIN_CONFIG_SEED: &[u8] = b"vcoin-config";
pub const SLASH_REQUEST_SEED: &[u8] = b"slash-request"; // H-01

/// H-01: Slashing governance timelock
pub const SLASH_TIMELOCK_SECONDS: i64 = 48 * 60 * 60; // 48 hours after governance approval

/// H-01: Slash request status
pub const SLASH_STATUS_PENDING: u8 = 0;
pub const SLASH_STATUS_APPROVED: u8 = 1;
pub const SLASH_STATUS_EXECUTED: u8 = 2;
pub const SLASH_STATUS_REJECTED: u8 = 3;
pub const SLASH_STATUS_CANCELLED: u8 = 4;

/// H-NEW-01: Authority transfer timelock (24 hours in seconds)
pub const AUTHORITY_TRANSFER_TIMELOCK: i64 = 24 * 60 * 60;

