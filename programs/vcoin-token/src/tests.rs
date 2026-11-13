//! Unit tests for pure VCoin logic. Instruction behaviour is covered by the
//! LiteSVM suite in `integration-tests/tests/vcoin_token.rs`.

use crate::constants::*;
use crate::state::VCoinConfig;
use anchor_lang::prelude::*;

#[test]
fn supply_cap_is_one_billion_whole_tokens() {
    assert_eq!(VCOIN_DECIMALS, 9);
    assert_eq!(TOTAL_SUPPLY, 1_000_000_000_000_000_000);
}

#[test]
fn config_len_matches_borsh_size() {
    let mut buf = Vec::new();
    VCoinConfig::default().serialize(&mut buf).unwrap();
    assert_eq!(VCoinConfig::LEN, 8 + buf.len());
}

#[test]
fn pdas_are_derived_from_versioned_seeds() {
    assert_eq!(VCOIN_CONFIG_SEED, b"vcoin-config-v2");
    let (config, _) = Pubkey::find_program_address(&[VCOIN_CONFIG_SEED], &crate::ID);
    let (legacy, _) = Pubkey::find_program_address(&[b"vcoin-config"], &crate::ID);
    assert_ne!(
        config, legacy,
        "v2 config must not collide with the stale v1 account"
    );
}
