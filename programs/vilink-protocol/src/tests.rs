//! Unit tests: fee rounding, action ids, sizes. Instruction behaviour:
//! `integration-tests/tests/vilink.rs`.

use anchor_lang::prelude::*;

use crate::constants::*;
use crate::state::*;

#[test]
fn fees_round_up_and_never_exceed_the_amount() {
    assert_eq!(tip_fee(1_000_000_000, 250), 25_000_000);
    assert_eq!(tip_fee(101, 250), 3, "2.525 rounds up");
    assert_eq!(tip_fee(1, MAX_FEE_BPS), 1);
    assert_eq!(tip_fee(MAX_TIP_AMOUNT, MAX_FEE_BPS), MAX_TIP_AMOUNT / 10);
}

#[test]
fn action_ids_differ_by_nonce() {
    let (c, t) = (Pubkey::new_unique(), Pubkey::new_unique());
    assert_ne!(
        action_id(&c, 0, ACTION_TIP, &t, 5, 100),
        action_id(&c, 1, ACTION_TIP, &t, 5, 100),
        "audit VL-11"
    );
}

#[test]
fn only_three_types_can_ever_be_enabled() {
    assert_eq!(ALL_ACTIONS_MASK, 0b111);
    let mut config_mask = ALL_ACTIONS_MASK;
    config_mask &= !(1 << ACTION_VOUCH);
    let enabled = |t: u8| t < NUM_ACTION_TYPES && config_mask & (1 << t) != 0;
    assert!(
        enabled(ACTION_TIP)
            && !enabled(ACTION_VOUCH)
            && enabled(ACTION_FOLLOW)
            && !enabled(3)
            && !enabled(7)
    );
    let mut buf = Vec::new();
    UserActionStats::default().serialize(&mut buf).unwrap();
    assert_eq!(UserActionStats::LEN, 8 + buf.len());
}
