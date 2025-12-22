//! Unit tests for 5A Protocol
//! 
//! These tests run against the ACTUAL program code.

#[cfg(test)]
mod tests {
    use crate::constants::*;
    use crate::state::UserScore;
    use anchor_lang::prelude::Pubkey;

    // ========================================================================
    // Weight Constants Tests
    // ========================================================================

    #[test]
    fn test_weights_sum_to_100_percent() {
        let total = AUTHENTICITY_WEIGHT + ACCURACY_WEIGHT + AGILITY_WEIGHT + 
                   ACTIVITY_WEIGHT + APPROVED_WEIGHT;
        assert_eq!(total, 10000, "Weights should sum to 10000 (100%)");
    }

    #[test]
    fn test_individual_weights() {
        assert_eq!(AUTHENTICITY_WEIGHT, 2500);  // 25%
        assert_eq!(ACCURACY_WEIGHT, 2000);      // 20%
        assert_eq!(AGILITY_WEIGHT, 1500);       // 15%
        assert_eq!(ACTIVITY_WEIGHT, 2500);      // 25%
        assert_eq!(APPROVED_WEIGHT, 1500);      // 15%
    }

    // ========================================================================
    // Vouch System Constants Tests
    // ========================================================================

    #[test]
    fn test_vouch_constants() {
        assert_eq!(MIN_VOUCHER_SCORE, 6000, "Min voucher score should be 60%");
        assert_eq!(VOUCH_STAKE_AMOUNT, 5_000_000_000, "Vouch stake should be 5 VCoin");
        assert_eq!(VOUCHES_REQUIRED, 3, "Should require 3 vouches");
        assert_eq!(VOUCH_EVALUATION_PERIOD, 90 * 24 * 60 * 60, "90 days in seconds");
        assert_eq!(VOUCH_REWARD, 10_000_000_000, "Vouch reward should be 10 VCoin");
    }

    #[test]
    fn test_snapshot_interval() {
        assert_eq!(SNAPSHOT_INTERVAL, 24 * 60 * 60, "Daily snapshot = 86400 seconds");
    }

    // ========================================================================
    // UserScore Tests
    // ========================================================================

    fn create_test_score(a: u16, b: u16, c: u16, d: u16, e: u16) -> UserScore {
        UserScore {
            user: Pubkey::new_unique(),
            authenticity: a,
            accuracy: b,
            agility: c,
            activity: d,
            approved: e,
            composite_score: 0,
            last_updated: 0,
            last_snapshot_epoch: 0,
            update_count: 0,
            is_private: false,
            bump: 0,
        }
    }

    #[test]
    fn test_composite_perfect_score() {
        let score = create_test_score(10000, 10000, 10000, 10000, 10000);
        assert_eq!(score.calculate_composite(), 10000);
    }

    #[test]
    fn test_composite_zero_score() {
        let score = create_test_score(0, 0, 0, 0, 0);
        assert_eq!(score.calculate_composite(), 0);
    }

    #[test]
    fn test_composite_average_score() {
        let score = create_test_score(5000, 5000, 5000, 5000, 5000);
        assert_eq!(score.calculate_composite(), 5000);
    }

    #[test]
    fn test_composite_mixed_scores() {
        // Auth=8000, Acc=6000, Agi=4000, Act=10000, App=2000
        // Weighted = (8000*2500 + 6000*2000 + 4000*1500 + 10000*2500 + 2000*1500) / 10000
        //          = (20000000 + 12000000 + 6000000 + 25000000 + 3000000) / 10000
        //          = 66000000 / 10000 = 6600
        let score = create_test_score(8000, 6000, 4000, 10000, 2000);
        assert_eq!(score.calculate_composite(), 6600);
    }

    #[test]
    fn test_composite_weighted_properly() {
        // Test that higher weighted dimensions have more impact
        
        // High authenticity (25% weight) vs low approved (15% weight)
        let high_auth = create_test_score(10000, 0, 0, 0, 0);
        let high_app = create_test_score(0, 0, 0, 0, 10000);
        
        assert_eq!(high_auth.calculate_composite(), 2500); // 10000 * 0.25
        assert_eq!(high_app.calculate_composite(), 1500);  // 10000 * 0.15
        
        // Auth should contribute more than Approved
        assert!(high_auth.calculate_composite() > high_app.calculate_composite());
    }

    #[test]
    fn test_composite_activity_and_authenticity_equal_weight() {
        // Both are 25%
        let high_auth = create_test_score(10000, 0, 0, 0, 0);
        let high_act = create_test_score(0, 0, 0, 10000, 0);
        
        assert_eq!(high_auth.calculate_composite(), high_act.calculate_composite());
    }

    #[test]
    fn test_composite_agility_and_approved_equal_weight() {
        // Both are 15%
        let high_agi = create_test_score(0, 0, 10000, 0, 0);
        let high_app = create_test_score(0, 0, 0, 0, 10000);
        
        assert_eq!(high_agi.calculate_composite(), high_app.calculate_composite());
    }

    // ========================================================================
    // Boundary Tests
    // ========================================================================

    #[test]
    fn test_composite_no_overflow() {
        // Max values shouldn't overflow u32 multiplication
        let max_score = create_test_score(10000, 10000, 10000, 10000, 10000);
        
        // Max calculation: 10000 * 10000 = 100_000_000 per dimension
        // Total: 5 * 100_000_000 = 500_000_000
        // Fits easily in u32
        let composite = max_score.calculate_composite();
        assert!(composite <= 10000);
    }

    #[test]
    fn test_score_bounds() {
        // Scores should be 0-10000
        for i in [0u16, 1, 5000, 9999, 10000] {
            let score = create_test_score(i, i, i, i, i);
            let composite = score.calculate_composite();
            assert!(composite <= 10000, "Composite should not exceed 10000");
            assert_eq!(composite, i, "With all same scores, composite equals individual");
        }
    }

    // ========================================================================
    // Vouch Eligibility Tests
    // ========================================================================

    #[test]
    fn test_vouch_eligibility_threshold() {
        // User needs 60% (6000) to vouch
        let eligible_score = create_test_score(6000, 6000, 6000, 6000, 6000);
        let ineligible_score = create_test_score(5999, 5999, 5999, 5999, 5999);
        
        assert!(eligible_score.calculate_composite() >= MIN_VOUCHER_SCORE);
        assert!(ineligible_score.calculate_composite() < MIN_VOUCHER_SCORE);
    }

    // ========================================================================
    // Account Size Tests
    // ========================================================================

    #[test]
    fn test_user_score_size() {
        // Verify the account size calculation is correct
        let expected_size = 8 + 32 + 2 + 2 + 2 + 2 + 2 + 2 + 8 + 8 + 4 + 1 + 1;
        assert_eq!(UserScore::LEN, expected_size);
    }
}

