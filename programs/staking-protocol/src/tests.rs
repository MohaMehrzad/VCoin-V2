//! Unit tests for Staking Protocol
//! 
//! These tests run against the ACTUAL program code.

#[cfg(test)]
mod tests {
    use crate::constants::*;
    use crate::state::StakingTier;
    use crate::utils::calculate_vevcoin;

    // ========================================================================
    // Tier Tests - Testing ACTUAL StakingTier::from_amount
    // ========================================================================

    #[test]
    fn test_tier_none() {
        assert_eq!(StakingTier::from_amount(0), StakingTier::None);
        assert_eq!(StakingTier::from_amount(999 * 1_000_000_000), StakingTier::None);
    }

    #[test]
    fn test_tier_bronze() {
        assert_eq!(StakingTier::from_amount(BRONZE_THRESHOLD), StakingTier::Bronze);
        assert_eq!(StakingTier::from_amount(BRONZE_THRESHOLD + 1), StakingTier::Bronze);
        assert_eq!(StakingTier::from_amount(SILVER_THRESHOLD - 1), StakingTier::Bronze);
    }

    #[test]
    fn test_tier_silver() {
        assert_eq!(StakingTier::from_amount(SILVER_THRESHOLD), StakingTier::Silver);
        assert_eq!(StakingTier::from_amount(GOLD_THRESHOLD - 1), StakingTier::Silver);
    }

    #[test]
    fn test_tier_gold() {
        assert_eq!(StakingTier::from_amount(GOLD_THRESHOLD), StakingTier::Gold);
        assert_eq!(StakingTier::from_amount(PLATINUM_THRESHOLD - 1), StakingTier::Gold);
    }

    #[test]
    fn test_tier_platinum() {
        assert_eq!(StakingTier::from_amount(PLATINUM_THRESHOLD), StakingTier::Platinum);
        assert_eq!(StakingTier::from_amount(PLATINUM_THRESHOLD * 10), StakingTier::Platinum);
    }

    #[test]
    fn test_tier_thresholds_correct() {
        assert_eq!(BRONZE_THRESHOLD, 1_000 * 1_000_000_000);
        assert_eq!(SILVER_THRESHOLD, 5_000 * 1_000_000_000);
        assert_eq!(GOLD_THRESHOLD, 20_000 * 1_000_000_000);
        assert_eq!(PLATINUM_THRESHOLD, 100_000 * 1_000_000_000);
    }

    #[test]
    fn test_tier_monotonic() {
        let amounts = [
            0u64,
            BRONZE_THRESHOLD - 1,
            BRONZE_THRESHOLD,
            SILVER_THRESHOLD - 1,
            SILVER_THRESHOLD,
            GOLD_THRESHOLD - 1,
            GOLD_THRESHOLD,
            PLATINUM_THRESHOLD - 1,
            PLATINUM_THRESHOLD,
        ];

        let mut prev_tier = 0u8;
        for amount in amounts {
            let tier = StakingTier::from_amount(amount).as_u8();
            assert!(tier >= prev_tier, "Tier should increase with amount");
            prev_tier = tier;
        }
    }

    // ========================================================================
    // Boost Multiplier Tests
    // ========================================================================

    #[test]
    fn test_boost_multipliers() {
        assert_eq!(StakingTier::None.boost_multiplier(), 1000);     // 1.0x
        assert_eq!(StakingTier::Bronze.boost_multiplier(), 1100);   // 1.1x
        assert_eq!(StakingTier::Silver.boost_multiplier(), 1200);   // 1.2x
        assert_eq!(StakingTier::Gold.boost_multiplier(), 1300);     // 1.3x
        assert_eq!(StakingTier::Platinum.boost_multiplier(), 1400); // 1.4x
    }

    #[test]
    fn test_fee_discounts() {
        assert_eq!(StakingTier::None.fee_discount_bps(), 0);       // 0%
        assert_eq!(StakingTier::Bronze.fee_discount_bps(), 1000);  // 10%
        assert_eq!(StakingTier::Silver.fee_discount_bps(), 2000);  // 20%
        assert_eq!(StakingTier::Gold.fee_discount_bps(), 3000);    // 30%
        assert_eq!(StakingTier::Platinum.fee_discount_bps(), 5000);// 50%
    }

    // ========================================================================
    // veVCoin Calculation Tests - Testing ACTUAL calculate_vevcoin
    // ========================================================================

    #[test]
    fn test_vevcoin_zero_stake() {
        let result = calculate_vevcoin(0, MAX_LOCK_DURATION, StakingTier::Platinum);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), 0);
    }

    #[test]
    fn test_vevcoin_max_lock_no_tier() {
        // 1000 VCoin for 4 years with no tier
        // ve_vcoin = 1000 * (4y/4y * 1000) * 1000 / 1_000_000 = 1000 * 1
        let amount = 1000 * 1_000_000_000u64; // 1000 VCoin
        let result = calculate_vevcoin(amount, MAX_LOCK_DURATION, StakingTier::None);
        assert!(result.is_ok());
        
        // Should equal stake amount at max lock with no boost
        let vevcoin = result.unwrap();
        assert_eq!(vevcoin, amount);
    }

    #[test]
    fn test_vevcoin_max_lock_platinum() {
        // 100,000 VCoin for 4 years with Platinum tier (1.4x boost)
        let amount = 100_000 * 1_000_000_000u64; // 100,000 VCoin
        let result = calculate_vevcoin(amount, MAX_LOCK_DURATION, StakingTier::Platinum);
        assert!(result.is_ok());
        
        let vevcoin = result.unwrap();
        // Expected: amount * 1.4 = 140,000 VCoin worth of veVCoin
        let expected = amount * 1400 / 1000;
        assert_eq!(vevcoin, expected);
    }

    #[test]
    fn test_vevcoin_half_lock_duration() {
        // 1000 VCoin for 2 years with no tier
        // duration_factor = (2y * 1000) / 4y = 500
        // ve_vcoin = 1000 * 500 * 1000 / 1_000_000 = 500 VCoin
        let amount = 1000 * 1_000_000_000u64;
        let two_years = MAX_LOCK_DURATION / 2;
        let result = calculate_vevcoin(amount, two_years, StakingTier::None);
        assert!(result.is_ok());
        
        let vevcoin = result.unwrap();
        assert_eq!(vevcoin, amount / 2);
    }

    #[test]
    fn test_vevcoin_min_lock() {
        let amount = 1000 * 1_000_000_000u64;
        let result = calculate_vevcoin(amount, MIN_LOCK_DURATION, StakingTier::None);
        assert!(result.is_ok());
        
        let vevcoin = result.unwrap();
        // Min lock is 1 week out of 4 years ≈ 0.48%
        // Using actual formula: ve_vcoin = amount * (duration * 1000 / 4y) * tier_boost / 1_000_000
        // duration_factor = MIN_LOCK_DURATION * 1000 / FOUR_YEARS = 604800 * 1000 / 126144000 = 4
        // vevcoin = amount * 4 * 1000 / 1_000_000 = amount * 0.004
        let duration_factor = (MIN_LOCK_DURATION as u128) * 1000 / (MAX_LOCK_DURATION as u128);
        let tier_boost = StakingTier::None.boost_multiplier() as u128;
        let expected = (amount as u128 * duration_factor * tier_boost / 1_000_000) as u64;
        assert_eq!(vevcoin, expected);
    }

    #[test]
    fn test_vevcoin_increases_with_duration() {
        let amount = 10_000 * 1_000_000_000u64;
        
        let vevcoin_1y = calculate_vevcoin(amount, MAX_LOCK_DURATION / 4, StakingTier::None).unwrap();
        let vevcoin_2y = calculate_vevcoin(amount, MAX_LOCK_DURATION / 2, StakingTier::None).unwrap();
        let vevcoin_4y = calculate_vevcoin(amount, MAX_LOCK_DURATION, StakingTier::None).unwrap();
        
        assert!(vevcoin_2y > vevcoin_1y);
        assert!(vevcoin_4y > vevcoin_2y);
    }

    #[test]
    fn test_vevcoin_increases_with_tier() {
        let amount = 100_000 * 1_000_000_000u64;
        let duration = MAX_LOCK_DURATION;
        
        let ve_none = calculate_vevcoin(amount, duration, StakingTier::None).unwrap();
        let ve_bronze = calculate_vevcoin(amount, duration, StakingTier::Bronze).unwrap();
        let ve_silver = calculate_vevcoin(amount, duration, StakingTier::Silver).unwrap();
        let ve_gold = calculate_vevcoin(amount, duration, StakingTier::Gold).unwrap();
        let ve_platinum = calculate_vevcoin(amount, duration, StakingTier::Platinum).unwrap();
        
        assert!(ve_bronze > ve_none);
        assert!(ve_silver > ve_bronze);
        assert!(ve_gold > ve_silver);
        assert!(ve_platinum > ve_gold);
    }

    // ========================================================================
    // Overflow Protection Tests
    // ========================================================================

    #[test]
    fn test_vevcoin_no_overflow_max_values() {
        // Test with very large values
        let max_reasonable_stake = 1_000_000_000 * 1_000_000_000u64; // 1 billion VCoin
        let result = calculate_vevcoin(max_reasonable_stake, MAX_LOCK_DURATION, StakingTier::Platinum);
        assert!(result.is_ok());
    }

    // ========================================================================
    // Lock Duration Constants
    // ========================================================================

    #[test]
    fn test_lock_duration_constants() {
        assert_eq!(MIN_LOCK_DURATION, 7 * 24 * 60 * 60); // 1 week in seconds
        assert_eq!(MAX_LOCK_DURATION, 4 * 365 * 24 * 60 * 60); // 4 years
        assert_eq!(FOUR_YEARS_SECONDS, MAX_LOCK_DURATION);
    }

    // ========================================================================
    // Property Tests
    // ========================================================================

    #[test]
    fn test_vevcoin_proportional_to_stake() {
        let base_amount = 1000 * 1_000_000_000u64;
        let duration = MAX_LOCK_DURATION;
        let tier = StakingTier::None;
        
        let ve1 = calculate_vevcoin(base_amount, duration, tier).unwrap();
        let ve2 = calculate_vevcoin(base_amount * 2, duration, tier).unwrap();
        let ve10 = calculate_vevcoin(base_amount * 10, duration, tier).unwrap();
        
        // veVCoin should scale linearly with stake
        assert_eq!(ve2, ve1 * 2);
        assert_eq!(ve10, ve1 * 10);
    }

    #[test]
    fn test_tier_boost_exact_ratios() {
        let amount = 100_000 * 1_000_000_000u64;
        let duration = MAX_LOCK_DURATION;
        
        let ve_none = calculate_vevcoin(amount, duration, StakingTier::None).unwrap();
        let ve_bronze = calculate_vevcoin(amount, duration, StakingTier::Bronze).unwrap();
        let ve_platinum = calculate_vevcoin(amount, duration, StakingTier::Platinum).unwrap();
        
        // Bronze = 1.1x of None
        assert_eq!(ve_bronze * 1000, ve_none * 1100);
        
        // Platinum = 1.4x of None
        assert_eq!(ve_platinum * 1000, ve_none * 1400);
    }
}

