use anchor_lang::prelude::*;

#[constant]
pub const IDENTITY_CONFIG_SEED: &[u8] = b"identity-config";
#[constant]
pub const IDENTITY_SEED: &[u8] = b"identity";
#[constant]
pub const USERNAME_SEED: &[u8] = b"username";
#[constant]
pub const ATTESTATION_SEED: &[u8] = b"attestation";
#[constant]
pub const SUBSCRIPTION_SEED: &[u8] = b"subscription";

pub const DAY: i64 = 24 * 60 * 60;
pub const MAX_ATTESTERS: usize = 10;
/// An attestation may be valid for at most two years.
pub const MAX_ATTESTATION_VALIDITY: i64 = 730 * DAY;

/// Usernames: `[a-z0-9_]{3,32}`, unique via a PDA.
pub const MIN_USERNAME_LEN: usize = 3;
pub const MAX_USERNAME_LEN: usize = 32;

/// Claim bits an attester can certify.
pub const CLAIM_EMAIL: u16 = 1 << 0;
pub const CLAIM_PHONE: u16 = 1 << 1;
pub const CLAIM_SOCIAL: u16 = 1 << 2;
pub const CLAIM_KYC: u16 = 1 << 3;
pub const CLAIM_BIOMETRIC: u16 = 1 << 4;
pub const CLAIM_UNIQUE_HUMAN: u16 = 1 << 5;
pub const VALID_CLAIMS_MASK: u16 = 0x3f;

/// Verification levels.
pub const LEVEL_NONE: u8 = 0;
pub const LEVEL_BASIC: u8 = 1; // email + phone
pub const LEVEL_KYC: u8 = 2; // identity documents
pub const LEVEL_FULL: u8 = 3; // KYC + biometric
pub const LEVEL_ENHANCED: u8 = 4; // full + unique human

/// Subscriptions: tiers 1..=3, 30 days each, priced in USDC (6 decimals).
pub const SUBSCRIPTION_DURATION: i64 = 30 * DAY;
pub const NUM_TIERS: usize = 3;
pub const DEFAULT_PRICES: [u64; NUM_TIERS] = [4_000_000, 12_000_000, 59_000_000];
pub const USDC_DECIMALS: u8 = 6;
/// Payment configuration changes take effect after this delay.
pub const PAYMENT_CONFIG_DELAY: i64 = DAY;
