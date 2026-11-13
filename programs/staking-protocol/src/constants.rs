//! Staking / voter-stake-registry constants.

use anchor_lang::prelude::*;

/// Registrar PDA seed (must match `vevcoin_token::constants::STAKING_REGISTRAR_SEED`).
#[constant]
pub const REGISTRAR_SEED: &[u8] = b"registrar";
/// Per-owner voter PDA seed: `["voter", owner]`.
#[constant]
pub const VOTER_SEED: &[u8] = b"voter";
/// VCoin vault PDA seed (token account owned by the registrar).
#[constant]
pub const VAULT_SEED: &[u8] = b"vault";

/// Deposit slots per voter.
pub const MAX_DEPOSITS: usize = 8;

/// Lock bounds (seconds).
#[constant]
pub const MIN_LOCK_SECS: i64 = 7 * 24 * 60 * 60;
#[constant]
pub const MAX_LOCK_SECS: i64 = 4 * 365 * 24 * 60 * 60;

/// Weight factors are fixed-point with this scale (1e9 = 1.0×).
pub const FACTOR_SCALE: u64 = 1_000_000_000;

/// Default weight parameters (VSR model, linear in amount; Sybil-neutral):
/// weight = amount·baseline + amount·max_extra·min(remaining, saturation)/saturation.
pub const DEFAULT_BASELINE_FACTOR: u64 = FACTOR_SCALE; // 1.0×
pub const DEFAULT_MAX_EXTRA_FACTOR: u64 = FACTOR_SCALE; // up to +1.0× for a 4-year lock
pub const DEFAULT_SATURATION_SECS: u64 = MAX_LOCK_SECS as u64;
/// A deposit whose lock ends within this window carries no weight, so tokens cannot vote
/// and then be withdrawn while the proposal is still open. Must exceed the Realms voting
/// time + cool-off (set by the DAO via `set_weight_params`).
pub const DEFAULT_MIN_LOCK_REMAINING_TO_VOTE: u64 = 4 * 24 * 60 * 60;

/// Bounds the DAO may set.
pub const MAX_BASELINE_FACTOR: u64 = FACTOR_SCALE;
pub const MAX_EXTRA_FACTOR_CAP: u64 = 3 * FACTOR_SCALE;
pub const MIN_SATURATION_SECS: u64 = MIN_LOCK_SECS as u64;

/// Realms voter-weight records written by this plugin.
#[constant]
pub const VOTER_WEIGHT_RECORD_SEED: &[u8] = b"voter-weight-record";
#[constant]
pub const MAX_VOTER_WEIGHT_RECORD_SEED: &[u8] = b"max-voter-weight-record";

/// spl-governance `GovernanceAccountType` tags (verified against the deployed program).
pub const REALM_V2_ACCOUNT_TYPE: u8 = 16;
pub const TOKEN_OWNER_RECORD_V2_ACCOUNT_TYPE: u8 = 17;
/// spl-governance PDA prefix for realms, holdings and token owner records.
pub const GOVERNANCE_SEED: &[u8] = b"governance";

/// 5A composite score upper bound (scores are 0..=10000).
pub const MAX_FIVE_A_SCORE: u16 = 10_000;
