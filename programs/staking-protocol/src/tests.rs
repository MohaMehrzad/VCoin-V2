//! Unit tests for staking state. Weight math tests live in `weight.rs`; instruction
//! behaviour is covered by `integration-tests/tests/staking.rs`.

use crate::constants::*;
use crate::state::{DepositEntry, Registrar, Voter};
use anchor_lang::prelude::*;

#[test]
fn account_lens_match_borsh_size() {
    let mut buf = Vec::new();
    Registrar::default().serialize(&mut buf).unwrap();
    assert_eq!(Registrar::LEN, 8 + buf.len());
    buf.clear();
    Voter::default().serialize(&mut buf).unwrap();
    assert_eq!(Voter::LEN, 8 + buf.len());
}

#[test]
fn registrar_seed_matches_the_vevcoin_pinned_seed() {
    // vevcoin-token pins `["registrar"]` under this program as its only mint/burn signer.
    assert_eq!(
        REGISTRAR_SEED,
        vevcoin_token::constants::STAKING_REGISTRAR_SEED
    );
    assert_eq!(crate::ID, vevcoin_token::constants::STAKING_PROGRAM_ID);
}

#[test]
fn locked_amount_counts_only_unexpired_deposits() {
    let mut v = Voter::default();
    v.deposits[0] = DepositEntry {
        amount: 10,
        lock_start: 0,
        lock_end: 100,
        is_used: true,
    };
    v.deposits[1] = DepositEntry {
        amount: 5,
        lock_start: 0,
        lock_end: 50,
        is_used: true,
    };
    assert_eq!(v.total_amount(), 15);
    assert_eq!(v.locked_amount(60), 10);
    assert_eq!(v.locked_amount(100), 0);
    assert!(!v.is_empty());
}
