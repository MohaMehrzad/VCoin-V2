//! Gasless (session-key registry + VCoin fee) constants.

use anchor_lang::prelude::*;

#[constant]
pub const GASLESS_CONFIG_SEED: &[u8] = b"gasless-config";
/// Session PDA: `["session", target_program, session_signer, authority]`.
#[constant]
pub const SESSION_SEED: &[u8] = b"session";
#[constant]
pub const FEE_VAULT_SEED: &[u8] = b"fee-vault";

/// Longest allowed session (24 h), following MagicBlock gpl-session practice.
#[constant]
pub const MAX_SESSION_DURATION: i64 = 24 * 60 * 60;

/// Session scopes. Sessions may only authorize low-risk, non-value-moving social actions:
/// never transfers, tips, withdrawals, claims, staking or votes (those always need the
/// main wallet). Bit positions are kept from v2 for client compatibility.
#[constant]
pub const SCOPE_VOUCH: u16 = 1 << 1;
#[constant]
pub const SCOPE_CONTENT: u16 = 1 << 2;
#[constant]
pub const SCOPE_FOLLOW: u16 = 1 << 7;
/// Union of every allowed scope bit; any other bit is rejected.
#[constant]
pub const VALID_SCOPE_MASK: u16 = SCOPE_VOUCH | SCOPE_CONTENT | SCOPE_FOLLOW;

/// Upper bound on the per-transaction VCoin fee the admin may schedule (100 VCoin).
#[constant]
pub const MAX_VCOIN_FEE_PER_TX: u64 = 100_000_000_000u64;
/// Delay before a fee or treasury change takes effect.
#[constant]
pub const PARAM_CHANGE_DELAY: i64 = 24 * 60 * 60;
