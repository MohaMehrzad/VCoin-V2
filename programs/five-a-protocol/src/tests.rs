//! Unit tests for score math and state sizes. Instruction behaviour:
//! `integration-tests/tests/five_a.rs`.

use anchor_lang::prelude::*;

use crate::constants::*;
use crate::state::*;

#[test]
fn weights_sum_to_full_score_and_composite_is_bounded() {
    assert_eq!(
        AUTHENTICITY_WEIGHT + ACCURACY_WEIGHT + AGILITY_WEIGHT + ACTIVITY_WEIGHT + APPROVED_WEIGHT,
        MAX_SCORE
    );
    let max = ScoreValues {
        authenticity: MAX_SCORE,
        accuracy: MAX_SCORE,
        agility: MAX_SCORE,
        activity: MAX_SCORE,
        approved: MAX_SCORE,
    };
    assert_eq!(max.composite(), MAX_SCORE);
    assert_eq!(ScoreValues::default().composite(), 0);
    let v = ScoreValues {
        authenticity: 8000,
        accuracy: 6000,
        agility: 4000,
        activity: 10000,
        approved: 0,
    };
    assert_eq!(
        v.composite(),
        ((8000u32 * 2500 + 6000 * 2000 + 4000 * 1500 + 10000 * 2500) / 10000) as u16
    );
    assert!(!ScoreValues {
        approved: MAX_SCORE + 1,
        ..v
    }
    .is_valid());
}

/// Vouchees start below the success bar, which sits below the voucher bar.
const _: () =
    assert!(MAX_VOUCHEE_SCORE < VOUCH_SUCCESS_SCORE && VOUCH_SUCCESS_SCORE < MIN_VOUCHER_SCORE);

#[test]
fn vouch_slots_follow_the_voucher_score() {
    for (score, slots) in [
        (5999, 0),
        (6000, 3),
        (7000, 5),
        (8000, 8),
        (9000, 10),
        (10000, 10),
    ] {
        assert_eq!(VoucherStats::max_vouches_for_score(score), slots, "{score}");
    }
}

#[test]
fn account_lens_match_borsh_size() {
    let mut buf = Vec::new();
    UserScore::default().serialize(&mut buf).unwrap();
    assert_eq!(UserScore::LEN, 8 + buf.len());
    buf.clear();
    VoucherStats::default().serialize(&mut buf).unwrap();
    assert_eq!(VoucherStats::LEN, 8 + buf.len());
}
