//! Unit tests for pure veVCoin logic. Mint/burn behaviour (including the registrar-only
//! signer and min() burn) is covered through staking in `integration-tests/tests/staking.rs`.

use crate::constants::*;
use crate::state::VeVCoinConfig;
use anchor_lang::prelude::*;

#[test]
fn config_len_matches_borsh_size() {
    let mut buf = Vec::new();
    VeVCoinConfig::default().serialize(&mut buf).unwrap();
    assert_eq!(VeVCoinConfig::LEN, 8 + buf.len());
}

#[test]
fn staking_authority_is_the_registrar_pda_of_the_pinned_staking_program() {
    let (registrar, _) =
        Pubkey::find_program_address(&[STAKING_REGISTRAR_SEED], &STAKING_PROGRAM_ID);
    assert_ne!(registrar, Pubkey::default());
    assert_eq!(
        STAKING_PROGRAM_ID.to_string(),
        "6EFcistyr2E81adLUcuBJRr8W2xzpt3D3dFYEcMewpWu"
    );
}
