/// veVCoin Token Constants

/// Token decimals
pub const VEVCOIN_DECIMALS: u8 = 9;

/// Token metadata
pub const TOKEN_NAME: &str = "veVCoin";
pub const TOKEN_SYMBOL: &str = "veVIWO";
pub const TOKEN_URI: &str = "https://viwoapp.com/vevcoin-metadata.json";

/// PDA Seeds
pub const VEVCOIN_CONFIG_SEED: &[u8] = b"vevcoin-config";
pub const USER_VEVCOIN_SEED: &[u8] = b"user-vevcoin";

/// 4 years in seconds (for veVCoin calculation)
pub const FOUR_YEARS_SECONDS: i64 = 4 * 365 * 24 * 60 * 60;

