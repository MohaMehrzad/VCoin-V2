use anchor_lang::prelude::*;

#[constant]
pub const POOL_CONFIG_SEED: &[u8] = b"pool-config";
#[constant]
pub const RESERVE_VAULT_SEED: &[u8] = b"reserve-vault";
#[constant]
pub const EPOCH_SEED: &[u8] = b"epoch";
#[constant]
pub const EPOCH_VAULT_SEED: &[u8] = b"epoch-vault";
#[constant]
pub const CLAIM_SEED: &[u8] = b"claim";
#[constant]
pub const FEE_VAULT_SEED: &[u8] = b"fee-vault";

pub const HOUR: i64 = 60 * 60;
pub const DAY: i64 = 24 * HOUR;
pub const BPS: u64 = 10_000;

/// Up to 7 oracles; a root needs matching votes from `root_quorum` (≥ 2) oracles and from a
/// strict majority of the active ones, whichever is larger.
pub const MAX_ORACLES: usize = 7;
pub const MIN_ROOT_QUORUM: u8 = 2;
/// A newly registered oracle can vote only after this delay.
pub const ORACLE_ACTIVATION_DELAY: i64 = DAY;
/// Fee changes take effect after this delay.
pub const FEE_CHANGE_DELAY: i64 = DAY;
/// Claim fee ceiling: 1%.
pub const MAX_FEE_BPS: u16 = 100;
/// Proofs longer than this (2^32 leaves) are rejected before hashing.
pub const MAX_PROOF_LEN: usize = 32;

/// Defaults (adjustable by the DAO within the bounds below).
pub const DEFAULT_EPOCH_DURATION: i64 = 30 * DAY;
pub const DEFAULT_CLAIM_WINDOW: i64 = 90 * DAY;
pub const DEFAULT_DISPUTE_DELAY: i64 = DAY;
pub const DEFAULT_DECAY_BPS: u16 = 200; // −2% per epoch
pub const DEFAULT_FEE_BPS: u16 = 100;
pub const DEFAULT_ROOT_QUORUM: u8 = 2;

pub const MIN_EPOCH_DURATION: i64 = DAY;
pub const MAX_EPOCH_DURATION: i64 = 90 * DAY;
pub const MIN_CLAIM_WINDOW: i64 = 7 * DAY;
pub const MAX_CLAIM_WINDOW: i64 = 365 * DAY;
pub const MIN_DISPUTE_DELAY: i64 = HOUR;
pub const MAX_DISPUTE_DELAY: i64 = 7 * DAY;
pub const MAX_DECAY_BPS: u16 = 5_000;
