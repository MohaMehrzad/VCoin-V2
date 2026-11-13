//! Unit tests: tiers, regeneration remainder, refunds, sizes. Instruction behaviour:
//! `integration-tests/tests/content.rs`.

use anchor_lang::prelude::*;

use crate::constants::*;
use crate::state::*;

#[test]
fn tiers_follow_locked_stake() {
    for (locked, tier) in [
        (0, 0),
        (999 * VCOIN, 0),
        (1_000 * VCOIN, 1),
        (5_000 * VCOIN, 2),
        (20_000 * VCOIN, 3),
        (100_000 * VCOIN, 4),
        (u64::MAX, 4),
    ] {
        assert_eq!(tier_for_locked(locked), tier, "{locked}");
    }
}

fn energy(tier: u8, current: u16, t0: i64) -> UserEnergy {
    let mut e = UserEnergy {
        last_regen_time: t0,
        ..Default::default()
    };
    e.set_tier(tier, t0);
    e.current_energy = current;
    e
}

#[test]
fn regeneration_keeps_partial_progress() {
    // Tier 0: 20/hour = one unit per 180 s. Acting every 120 s must still regenerate.
    let mut e = energy(0, 0, 0);
    let mut t = 0;
    for _ in 0..30 {
        t += 120;
        e.regenerate(t);
    }
    assert_eq!(
        e.current_energy, 20,
        "an hour of 2-minute actions earns the full hour"
    );
    // Never exceeds max, and a full bar does not bank time.
    let mut full = energy(0, 199, 0);
    full.regenerate(10 * HOUR);
    assert_eq!(full.current_energy, 200);
    full.current_energy = 0;
    full.regenerate(10 * HOUR + 180);
    assert_eq!(full.current_energy, 1);
}

#[test]
fn a_tier_change_settles_regeneration_at_the_old_rate() {
    let mut e = energy(0, 0, 0);
    e.set_tier(4, HOUR); // one hour at 20/h, then Platinum
    assert_eq!(
        (e.current_energy, e.max_energy, e.regen_per_hour),
        (20, 2_000, 200)
    );
    e.set_tier(0, HOUR);
    assert_eq!(e.max_energy, 200);
}

#[test]
fn refunds_are_capped_at_one_hundred_percent() {
    assert_eq!(refund_percent(0), 0);
    assert_eq!(refund_percent(9), 0);
    assert_eq!(refund_percent(10), 25);
    assert_eq!(refund_percent(50), 50);
    assert_eq!(refund_percent(100), 100);
    assert_eq!(refund_percent(1_000_000), 100);
    let mut buf = Vec::new();
    UserEnergy::default().serialize(&mut buf).unwrap();
    assert_eq!(UserEnergy::LEN, 8 + buf.len());
}
