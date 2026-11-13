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

#[test]
fn realms_addin_records_match_the_deployed_governance_layout() {
    // Discriminators and maximal sizes observed against the deployed spl-governance
    // program (mainnet + devnet builds) in LiteSVM.
    use crate::state::{MaxVoterWeightRecord, VoterWeightRecord};
    use anchor_lang::Discriminator;
    assert_eq!(
        VoterWeightRecord::DISCRIMINATOR,
        [0x2e, 0xf9, 0x9b, 0x4b, 0x99, 0xf8, 0x74, 0x09]
    );
    assert_eq!(
        MaxVoterWeightRecord::DISCRIMINATOR,
        [0x9d, 0x5f, 0xf2, 0x97, 0x10, 0x62, 0x1a, 0x76]
    );
    assert_eq!(VoterWeightRecord::LEN, 164);
    assert_eq!(MaxVoterWeightRecord::LEN, 97);
}
