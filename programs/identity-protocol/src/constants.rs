/// Identity Protocol Constants

/// PDA Seeds
pub const IDENTITY_CONFIG_SEED: &[u8] = b"identity-config";
pub const IDENTITY_SEED: &[u8] = b"identity";
pub const SAS_ATTESTATION_SEED: &[u8] = b"sas-attestation";
pub const SUBSCRIPTION_SEED: &[u8] = b"subscription";

/// Subscription prices in USDC (6 decimals)
pub const SUBSCRIPTION_FREE: u64 = 0;
pub const SUBSCRIPTION_VERIFIED: u64 = 4_000_000;   // $4 USDC
pub const SUBSCRIPTION_PREMIUM: u64 = 12_000_000;   // $12 USDC
pub const SUBSCRIPTION_ENTERPRISE: u64 = 59_000_000; // $59 USDC

/// Subscription duration (30 days in seconds)
pub const SUBSCRIPTION_DURATION: i64 = 30 * 24 * 60 * 60;

