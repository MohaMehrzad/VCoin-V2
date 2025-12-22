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

