//! VCoin token constants.

use anchor_lang::prelude::*;

/// Config PDA seed. `-v2` avoids the stale v1 `vcoin-config` account left on devnet.
#[constant]
pub const VCOIN_CONFIG_SEED: &[u8] = b"vcoin-config-v2";

/// Mint PDA seed. The VCoin mint is created by this program at this address.
#[constant]
pub const VCOIN_MINT_SEED: &[u8] = b"vcoin-mint";

/// VCoin decimals.
#[constant]
pub const VCOIN_DECIMALS: u8 = vcoin_common::VCOIN_DECIMALS;

/// Hard supply cap: 1,000,000,000 VCoin in base units. Enforced against the mint's
/// real on-chain supply, not a program-side counter (audit MON-8 / TOK-04).
#[constant]
pub const TOTAL_SUPPLY: u64 = 1_000_000_000 * 10u64.pow(VCOIN_DECIMALS as u32);

/// Metadata field length limits (bytes).
pub const MAX_NAME_LEN: usize = 32;
pub const MAX_SYMBOL_LEN: usize = 10;
pub const MAX_URI_LEN: usize = 200;
