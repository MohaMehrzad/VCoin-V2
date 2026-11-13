//! Voting-weight math (Voter-Stake-Registry model). Pure functions, shared by the Realms
//! voter-weight plugin and the private-poll program.
//!
//! Per deposit, at reference time `t`:
//! * `remaining = lock_end - t`; if `remaining < min_lock_remaining_to_vote` the deposit
//!   carries **no** weight (an unlocking stake cannot vote and then exit; audit GOV-03).
//! * `weight = amount·baseline/S + amount·max_extra·min(remaining, saturation)/(saturation·S)`
//!   where `S` = [`FACTOR_SCALE`].
//!
//! Weight is linear in `amount`, so splitting tokens across wallets gains nothing
//! (audit GOV-02: the old per-wallet square root let 10 wallets × 10 VCoin reach quorum).

use crate::constants::FACTOR_SCALE;
use crate::state::{DepositEntry, Voter, WeightParams};

/// Weight of one deposit at time `t`; `None` on arithmetic overflow.
pub fn deposit_weight(deposit: &DepositEntry, params: &WeightParams, t: i64) -> Option<u64> {
    if !deposit.is_used || deposit.amount == 0 {
        return Some(0);
    }
    let remaining = deposit.lock_end.checked_sub(t)?;
    if remaining <= 0 || (remaining as u64) < params.min_lock_remaining_to_vote {
        return Some(0);
    }
    let amount = deposit.amount as u128;
    let scale = FACTOR_SCALE as u128;
    let saturation = params.saturation_secs as u128;
    let capped = (remaining as u128).min(saturation);

    let baseline = amount.checked_mul(params.baseline_factor as u128)? / scale;
    let extra = amount
        .checked_mul(params.max_extra_factor as u128)?
        .checked_mul(capped)?
        / saturation.checked_mul(scale)?;
    u64::try_from(baseline.checked_add(extra)?).ok()
}

/// Total voting weight of a voter at time `t`.
pub fn voter_weight(voter: &Voter, params: &WeightParams, t: i64) -> Option<u64> {
    voter.deposits.iter().try_fold(0u64, |acc, d| {
        acc.checked_add(deposit_weight(d, params, t)?)
    })
}

/// Upper bound on any voter's weight per unit locked; used for MaxVoterWeightRecord so
/// percentage thresholds are never overstated.
pub fn max_weight_for(total_locked: u64, params: &WeightParams) -> Option<u64> {
    let factor = (params.baseline_factor as u128).checked_add(params.max_extra_factor as u128)?;
    u64::try_from((total_locked as u128).checked_mul(factor)? / FACTOR_SCALE as u128).ok()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::constants::*;

    const VCOIN: u64 = 1_000_000_000;

    fn dep(amount: u64, lock_end: i64) -> DepositEntry {
        DepositEntry {
            amount,
            lock_start: 0,
            lock_end,
            is_used: true,
        }
    }

    fn no_min() -> WeightParams {
        WeightParams {
            min_lock_remaining_to_vote: 0,
            ..WeightParams::default()
        }
    }

    #[test]
    fn weight_is_linear_and_decays_to_zero_at_unlock() {
        let p = no_min();
        let four_years = MAX_LOCK_SECS;
        // Full saturation: 1.0× baseline + 1.0× extra = 2.0×.
        assert_eq!(
            deposit_weight(&dep(100 * VCOIN, four_years), &p, 0),
            Some(200 * VCOIN)
        );
        // Half way: 1.0× + 0.5×.
        assert_eq!(
            deposit_weight(&dep(100 * VCOIN, four_years), &p, four_years / 2),
            Some(150 * VCOIN)
        );
        // Beyond saturation is clamped.
        assert_eq!(
            deposit_weight(&dep(100 * VCOIN, 2 * four_years), &p, 0),
            Some(200 * VCOIN)
        );
        // At and after unlock: zero.
        assert_eq!(
            deposit_weight(&dep(100 * VCOIN, four_years), &p, four_years),
            Some(0)
        );
        assert_eq!(
            deposit_weight(&dep(100 * VCOIN, four_years), &p, four_years + 1),
            Some(0)
        );
    }

    #[test]
    fn unlocking_deposits_carry_no_weight() {
        let p = WeightParams::default(); // 4-day minimum remaining lock
        let t = 1_000_000;
        let min = p.min_lock_remaining_to_vote as i64;
        assert_eq!(deposit_weight(&dep(VCOIN, t + min - 1), &p, t), Some(0));
        assert!(deposit_weight(&dep(VCOIN, t + min), &p, t).unwrap() >= VCOIN);
    }

    #[test]
    fn splitting_across_wallets_gains_nothing() {
        // Audit GOV-02 regression: 10 wallets × 10 VCoin == 1 wallet × 100 VCoin.
        let p = no_min();
        let one = deposit_weight(&dep(100 * VCOIN, MAX_LOCK_SECS), &p, 0).unwrap();
        let ten: u64 = (0..10)
            .map(|_| deposit_weight(&dep(10 * VCOIN, MAX_LOCK_SECS), &p, 0).unwrap())
            .sum();
        assert_eq!(one, ten);
        // And the whole supply massively outweighs a handful of dust wallets.
        let whale = deposit_weight(&dep(1_000_000_000 * VCOIN, MAX_LOCK_SECS), &p, 0).unwrap();
        assert!(whale > 1_000_000 * ten);
    }

    #[test]
    fn extreme_inputs_do_not_overflow_or_panic() {
        let p = WeightParams {
            max_extra_factor: MAX_EXTRA_FACTOR_CAP,
            ..no_min()
        };
        let max_supply = 1_000_000_000 * VCOIN;
        let w = deposit_weight(&dep(max_supply, MAX_LOCK_SECS), &p, 0).unwrap();
        assert_eq!(w, 4 * max_supply);
        assert_eq!(
            deposit_weight(&dep(u64::MAX, i64::MAX), &p, i64::MIN),
            None,
            "overflow is reported"
        );
        assert_eq!(max_weight_for(max_supply, &p), Some(4 * max_supply));
    }

    #[test]
    fn voter_weight_sums_deposits_and_bounds_hold() {
        let p = no_min();
        let mut v = Voter::default();
        v.deposits[0] = dep(10 * VCOIN, MAX_LOCK_SECS);
        v.deposits[3] = dep(5 * VCOIN, MAX_LOCK_SECS / 2);
        let w = voter_weight(&v, &p, 0).unwrap();
        assert_eq!(w, 20 * VCOIN + 5 * VCOIN + 5 * VCOIN / 2);
        assert!(w <= max_weight_for(v.total_amount(), &p).unwrap());
    }
}
