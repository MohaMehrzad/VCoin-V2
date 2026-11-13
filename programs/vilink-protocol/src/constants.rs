use anchor_lang::prelude::*;

#[constant]
pub const CONFIG_SEED: &[u8] = b"vilink-config";
#[constant]
pub const ACTION_SEED: &[u8] = b"action";
#[constant]
pub const USER_STATS_SEED: &[u8] = b"user-stats";
#[constant]
pub const RECEIPT_SEED: &[u8] = b"exec";

/// Executable action types. Others (challenge, stake, react, delegate, vote) were never
/// executable and are rejected (audit VL-6).
pub const ACTION_TIP: u8 = 0;
pub const ACTION_VOUCH: u8 = 1;
pub const ACTION_FOLLOW: u8 = 2;
pub const NUM_ACTION_TYPES: u8 = 3;
pub const ALL_ACTIONS_MASK: u8 = 0b111;

pub const HOUR: i64 = 60 * 60;
pub const DAY: i64 = 24 * HOUR;
pub const MIN_ACTION_EXPIRY: i64 = HOUR;
pub const MAX_ACTION_EXPIRY: i64 = 7 * DAY;
pub const MAX_EXECUTIONS: u32 = 10_000;

pub const MIN_TIP_AMOUNT: u64 = 100_000_000; // 0.1 VCoin
pub const MAX_TIP_AMOUNT: u64 = 10_000_000_000_000; // 10,000 VCoin

pub const BPS: u64 = 10_000;
pub const DEFAULT_FEE_BPS: u16 = 250; // 2.5%
pub const MIN_FEE_BPS: u16 = 10; // 0.1%
pub const MAX_FEE_BPS: u16 = 1_000; // 10%
/// Fee and treasury changes take effect after this delay.
pub const CONFIG_CHANGE_DELAY: i64 = DAY;
