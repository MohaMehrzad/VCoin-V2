//! Unit tests for session-scope logic. Instruction behaviour: `integration-tests/tests/gasless.rs`.

use crate::constants::*;
use crate::state::{validate_scope, GaslessConfig, SessionToken};
use anchor_lang::prelude::*;

fn session(scope: u16) -> (SessionToken, Pubkey, Pubkey, Pubkey) {
    let (a, s, p) = (
        Pubkey::new_unique(),
        Pubkey::new_unique(),
        Pubkey::new_unique(),
    );
    let tok = SessionToken {
        authority: a,
        target_program: p,
        session_signer: s,
        scope,
        valid_until: 1_000,
        payer: a,
        bump: 255,
    };
    (tok, a, s, p)
}

#[test]
fn lens_match_borsh_size() {
    let mut buf = Vec::new();
    GaslessConfig::default().serialize(&mut buf).unwrap();
    assert_eq!(GaslessConfig::LEN, 8 + buf.len());
    buf.clear();
    SessionToken::default().serialize(&mut buf).unwrap();
    assert_eq!(SessionToken::LEN, 8 + buf.len());
}

#[test]
fn scope_validation_rejects_empty_and_unknown_bits() {
    assert!(validate_scope(SCOPE_CONTENT).is_ok());
    assert!(validate_scope(VALID_SCOPE_MASK).is_ok());
    assert!(validate_scope(0).is_err());
    assert!(
        validate_scope(1 << 0).is_err(),
        "tip scope no longer exists"
    );
    assert!(validate_scope(0xFFFF).is_err());
}

#[test]
fn verify_requires_exactly_one_in_scope_bit() {
    // Audit GAS-04: `scope & action != 0` accepted action 0xFFFF for any non-empty scope.
    let (tok, a, s, p) = session(SCOPE_CONTENT);
    assert!(tok.verify(&a, &s, &p, SCOPE_CONTENT, 0).is_ok());
    assert!(tok.verify(&a, &s, &p, 0xFFFF, 0).is_err());
    assert!(tok
        .verify(&a, &s, &p, SCOPE_CONTENT | SCOPE_FOLLOW, 0)
        .is_err());
    assert!(tok.verify(&a, &s, &p, SCOPE_FOLLOW, 0).is_err());
    assert!(tok.verify(&a, &s, &p, 0, 0).is_err());
}

#[test]
fn verify_checks_identity_and_expiry() {
    let (tok, a, s, p) = session(SCOPE_FOLLOW);
    let other = Pubkey::new_unique();
    assert!(tok.verify(&other, &s, &p, SCOPE_FOLLOW, 0).is_err());
    assert!(tok.verify(&a, &other, &p, SCOPE_FOLLOW, 0).is_err());
    assert!(tok.verify(&a, &s, &other, SCOPE_FOLLOW, 0).is_err());
    assert!(tok.verify(&a, &s, &p, SCOPE_FOLLOW, 999).is_ok());
    assert!(
        tok.verify(&a, &s, &p, SCOPE_FOLLOW, 1_000).is_err(),
        "expired at valid_until"
    );
}
