//! Unit tests: claim levels, usernames, subscription pricing, sizes. Instruction behaviour:
//! `integration-tests/tests/identity.rs`.

use crate::constants::*;
use crate::state::*;

#[test]
fn kyc_level_needs_the_kyc_claim() {
    assert_eq!(
        level_for_claims(CLAIM_EMAIL | CLAIM_PHONE | CLAIM_SOCIAL),
        LEVEL_BASIC,
        "audit ID-04"
    );
    assert_eq!(level_for_claims(CLAIM_EMAIL), LEVEL_NONE);
    assert_eq!(level_for_claims(CLAIM_KYC), LEVEL_KYC);
    assert_eq!(level_for_claims(CLAIM_KYC | CLAIM_BIOMETRIC), LEVEL_FULL);
    assert_eq!(
        level_for_claims(CLAIM_KYC | CLAIM_BIOMETRIC | CLAIM_UNIQUE_HUMAN),
        LEVEL_ENHANCED
    );
    assert_eq!(
        level_for_claims(CLAIM_BIOMETRIC | CLAIM_UNIQUE_HUMAN),
        LEVEL_NONE
    );
}

#[test]
fn usernames() {
    for ok in ["abc", "a_b_c", "user123", &"x".repeat(32)] {
        assert!(is_valid_username(ok), "{ok}");
    }
    for bad in ["ab", "Abc", "a-b", "a b", "ünï", "", &"x".repeat(33)] {
        assert!(!is_valid_username(bad), "{bad}");
    }
}

fn sub(tier: u8, expires_at: i64) -> Subscription {
    Subscription {
        user: Default::default(),
        tier,
        started_at: 0,
        expires_at,
        total_paid: 0,
        bump: 0,
    }
}

#[test]
fn subscription_pricing() {
    let p = DEFAULT_PRICES;
    let now = 1_000_000;
    // New or lapsed: full price, 30 days from now.
    assert_eq!(
        subscription_quote(&sub(0, 0), 2, &p, now),
        Some((p[1], now + SUBSCRIPTION_DURATION))
    );
    assert_eq!(
        subscription_quote(&sub(3, now), 1, &p, now),
        Some((p[0], now + SUBSCRIPTION_DURATION))
    );
    // Same tier: extends from the current expiry, no time lost.
    let exp = now + 5 * DAY;
    assert_eq!(
        subscription_quote(&sub(1, exp), 1, &p, now),
        Some((p[0], exp + SUBSCRIPTION_DURATION))
    );
    // Downgrade while active: refused.
    assert_eq!(subscription_quote(&sub(3, exp), 1, &p, now), None);
    // Upgrade halfway through: credit = half of the old price.
    let half = now + SUBSCRIPTION_DURATION / 2;
    let (cost, until) = subscription_quote(&sub(1, half), 3, &p, now).unwrap();
    assert_eq!(
        (cost, until),
        (p[2] - p[0] / 2, now + SUBSCRIPTION_DURATION)
    );

    // Re-review C4: four prepaid periods of tier 1 are worth more than one period of
    // tier 2. The upgrade is free and the time carries over at the price ratio.
    let prepaid = now + 4 * SUBSCRIPTION_DURATION;
    let (cost, until) = subscription_quote(&sub(1, prepaid), 2, &p, now).unwrap();
    let carried = (4 * SUBSCRIPTION_DURATION) as u128 * p[0] as u128 / p[1] as u128;
    assert_eq!((cost, until), (0, now + carried as i64));
    assert!(until > now + SUBSCRIPTION_DURATION, "no paid time dropped");
    // Value before and after is the same (up to rounding down by under a second's worth).
    let before = (4 * SUBSCRIPTION_DURATION) as u128 * p[0] as u128;
    let after = (until - now) as u128 * p[1] as u128;
    assert!(after <= before && before - after < p[1] as u128);
    // Exactly one period's worth: free, one period.
    let exact = now + SUBSCRIPTION_DURATION * p[1] as i64 / p[0] as i64;
    assert_eq!(
        subscription_quote(&sub(1, exact), 2, &p, now),
        Some((0, now + SUBSCRIPTION_DURATION))
    );
}
