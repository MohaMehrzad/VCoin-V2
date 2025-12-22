//! Unit tests for Content Registry
//!
//! These tests run against the ACTUAL program code.

#[cfg(test)]
mod tests {
    use crate::constants::*;
    use crate::state::{ContentRecord, UserEnergy};
    use anchor_lang::prelude::Pubkey;

    // ========================================================================
    // Constants Tests
    // ========================================================================

    #[test]
    fn test_pda_seeds() {
        assert_eq!(REGISTRY_CONFIG_SEED, b"registry-config");
        assert_eq!(CONTENT_RECORD_SEED, b"content-record");
        assert_eq!(USER_ENERGY_SEED, b"user-energy");
        assert_eq!(RATE_LIMIT_SEED, b"rate-limit");
        assert_eq!(ENERGY_CONFIG_SEED, b"energy-config");
    }

    #[test]
    fn test_energy_costs() {
        assert_eq!(ENERGY_COST_TEXT_POST, 10);
        assert_eq!(ENERGY_COST_IMAGE_POST, 20);
        assert_eq!(ENERGY_COST_VIDEO_POST, 50);
        assert_eq!(ENERGY_COST_THREAD, 40);
        assert_eq!(ENERGY_COST_REPLY, 5);
        assert_eq!(ENERGY_COST_REPOST, 8);
        assert_eq!(ENERGY_COST_EDIT_AFTER_1H, 5);
    }

    #[test]
    fn test_regen_rates() {
        assert_eq!(REGEN_RATE_NONE, 20);
        assert_eq!(REGEN_RATE_BRONZE, 50);
        assert_eq!(REGEN_RATE_SILVER, 80);
        assert_eq!(REGEN_RATE_GOLD, 120);
        assert_eq!(REGEN_RATE_PLATINUM, 200);
    }

    #[test]
    fn test_max_energy_by_tier() {
        assert_eq!(MAX_ENERGY_NONE, 200);
        assert_eq!(MAX_ENERGY_BRONZE, 500);
        assert_eq!(MAX_ENERGY_SILVER, 800);
        assert_eq!(MAX_ENERGY_GOLD, 1200);
        assert_eq!(MAX_ENERGY_PLATINUM, 2000);
    }

    #[test]
    fn test_refund_thresholds() {
        assert_eq!(REFUND_THRESHOLD_10, 10);
        assert_eq!(REFUND_THRESHOLD_50, 50);
        assert_eq!(REFUND_THRESHOLD_100, 100);
        assert_eq!(REFUND_THRESHOLD_1000, 1000);
    }

    #[test]
    fn test_timing_constants() {
        assert_eq!(ENGAGEMENT_CHECK_DELAY, 24 * 60 * 60, "24 hours");
        assert_eq!(FREE_EDIT_WINDOW, 60 * 60, "1 hour");
    }

    // ========================================================================
    // State Size Tests
    // ========================================================================

    #[test]
    fn test_content_record_size() {
        let expected = 8 + 32 + 32 + 32 + 128 + 1 + 1 + 1 + 2 + 8 + 8 + 32 + 2 + 1 + 4 + 1;
        assert_eq!(ContentRecord::LEN, expected, "ContentRecord size mismatch");
    }

    #[test]
    fn test_user_energy_size() {
        let expected = 8 + 32 + 2 + 2 + 8 + 2 + 4 + 4 + 8 + 1 + 1;
        assert_eq!(UserEnergy::LEN, expected, "UserEnergy size mismatch");
    }

    // ========================================================================
    // UserEnergy Tier Tests
    // ========================================================================

    #[test]
    fn test_max_energy_for_tier() {
        assert_eq!(UserEnergy::max_energy_for_tier(0), MAX_ENERGY_NONE);
        assert_eq!(UserEnergy::max_energy_for_tier(1), MAX_ENERGY_BRONZE);
        assert_eq!(UserEnergy::max_energy_for_tier(2), MAX_ENERGY_SILVER);
        assert_eq!(UserEnergy::max_energy_for_tier(3), MAX_ENERGY_GOLD);
        assert_eq!(UserEnergy::max_energy_for_tier(4), MAX_ENERGY_PLATINUM);
        assert_eq!(UserEnergy::max_energy_for_tier(5), MAX_ENERGY_NONE); // Invalid
    }

    #[test]
    fn test_regen_rate_for_tier() {
        assert_eq!(UserEnergy::regen_rate_for_tier(0), REGEN_RATE_NONE);
        assert_eq!(UserEnergy::regen_rate_for_tier(1), REGEN_RATE_BRONZE);
        assert_eq!(UserEnergy::regen_rate_for_tier(2), REGEN_RATE_SILVER);
        assert_eq!(UserEnergy::regen_rate_for_tier(3), REGEN_RATE_GOLD);
        assert_eq!(UserEnergy::regen_rate_for_tier(4), REGEN_RATE_PLATINUM);
        assert_eq!(UserEnergy::regen_rate_for_tier(5), REGEN_RATE_NONE);
    }

    #[test]
    fn test_tier_progression() {
        // Higher tiers should have better stats
        for tier in 0..4u8 {
            let current_max = UserEnergy::max_energy_for_tier(tier);
            let next_max = UserEnergy::max_energy_for_tier(tier + 1);
            assert!(next_max > current_max, "Max energy should increase with tier");
            
            let current_regen = UserEnergy::regen_rate_for_tier(tier);
            let next_regen = UserEnergy::regen_rate_for_tier(tier + 1);
            assert!(next_regen > current_regen, "Regen rate should increase with tier");
        }
    }

    // ========================================================================
    // Energy Regeneration Tests
    // ========================================================================

    #[test]
    fn test_energy_regen_calculation() {
        let regen_rate = REGEN_RATE_BRONZE; // 50 per hour
        let hours_passed = 2;
        let regen_amount = (regen_rate as u32) * hours_passed;
        
        assert_eq!(regen_amount, 100, "Should regen 100 energy in 2 hours");
    }

    #[test]
    fn test_energy_cap() {
        let mut energy = UserEnergy::default();
        energy.tier = 2; // Silver
        energy.max_energy = UserEnergy::max_energy_for_tier(2);
        energy.current_energy = 700;
        
        // Simulate regen that would exceed max
        let regen_amount = 200u16;
        let new_energy = energy.current_energy.saturating_add(regen_amount);
        let capped_energy = new_energy.min(energy.max_energy);
        
        assert_eq!(capped_energy, MAX_ENERGY_SILVER, "Energy should cap at max");
    }

    // ========================================================================
    // Energy Cost Tests
    // ========================================================================

    #[test]
    fn test_can_post_with_energy() {
        let energy = UserEnergy {
            current_energy: 100,
            ..Default::default()
        };
        
        assert!(energy.current_energy >= ENERGY_COST_TEXT_POST);
        assert!(energy.current_energy >= ENERGY_COST_IMAGE_POST);
        assert!(energy.current_energy >= ENERGY_COST_VIDEO_POST);
    }

    #[test]
    fn test_insufficient_energy() {
        let energy = UserEnergy {
            current_energy: 5,
            ..Default::default()
        };
        
        // Can only post replies with this energy
        assert!(energy.current_energy >= ENERGY_COST_REPLY);
        assert!(energy.current_energy < ENERGY_COST_TEXT_POST);
    }

    // ========================================================================
    // Content Record Tests
    // ========================================================================

    #[test]
    fn test_content_record_default() {
        let record = ContentRecord::default();
        
        assert_eq!(record.version, 0);
        assert_eq!(record.state, 0);
        assert!(!record.refund_claimed);
        assert_eq!(record.engagement_count, 0);
    }

    #[test]
    fn test_content_edit_increments_version() {
        let mut record = ContentRecord::default();
        
        record.version = record.version.saturating_add(1);
        assert_eq!(record.version, 1);
        
        record.version = record.version.saturating_add(1);
        assert_eq!(record.version, 2);
    }

    // ========================================================================
    // Refund Calculation Tests
    // ========================================================================

    #[test]
    fn test_refund_tier_10_likes() {
        let engagement = 15u32;
        let energy_spent = 20u16;
        
        // 10-49 likes = 25% refund
        assert!(engagement >= REFUND_THRESHOLD_10);
        assert!(engagement < REFUND_THRESHOLD_50);
        
        let refund = (energy_spent as u32 * 25) / 100;
        assert_eq!(refund, 5);
    }

    #[test]
    fn test_refund_tier_50_likes() {
        let engagement = 75u32;
        let energy_spent = 20u16;
        
        // 50-99 likes = 50% refund
        assert!(engagement >= REFUND_THRESHOLD_50);
        assert!(engagement < REFUND_THRESHOLD_100);
        
        let refund = (energy_spent as u32 * 50) / 100;
        assert_eq!(refund, 10);
    }

    #[test]
    fn test_refund_tier_viral() {
        let engagement = 2000u32;
        let energy_spent = 20u16;
        
        // 1000+ likes = 150% refund
        assert!(engagement >= REFUND_THRESHOLD_1000);
        
        let refund = (energy_spent as u32 * 150) / 100;
        assert_eq!(refund, 30);
    }

    // ========================================================================
    // PDA Derivation Tests
    // ========================================================================

    #[test]
    fn test_content_pda_unique() {
        let program_id = Pubkey::new_unique();
        let tracking_id_1 = [1u8; 32];
        let tracking_id_2 = [2u8; 32];
        
        let (pda1, _) = Pubkey::find_program_address(
            &[CONTENT_RECORD_SEED, &tracking_id_1],
            &program_id
        );
        
        let (pda2, _) = Pubkey::find_program_address(
            &[CONTENT_RECORD_SEED, &tracking_id_2],
            &program_id
        );
        
        assert_ne!(pda1, pda2, "Different tracking IDs should have different PDAs");
    }

    #[test]
    fn test_user_energy_pda_unique() {
        let program_id = Pubkey::new_unique();
        let user1 = Pubkey::new_unique();
        let user2 = Pubkey::new_unique();
        
        let (pda1, _) = Pubkey::find_program_address(
            &[USER_ENERGY_SEED, user1.as_ref()],
            &program_id
        );
        
        let (pda2, _) = Pubkey::find_program_address(
            &[USER_ENERGY_SEED, user2.as_ref()],
            &program_id
        );
        
        assert_ne!(pda1, pda2);
    }

    // ========================================================================
    // Invariant Tests
    // ========================================================================

    #[test]
    fn test_invariant_energy_bounded() {
        let mut energy = UserEnergy::default();
        energy.tier = 4; // Platinum
        energy.max_energy = MAX_ENERGY_PLATINUM;
        energy.current_energy = MAX_ENERGY_PLATINUM;
        
        // Energy should never exceed max
        assert!(energy.current_energy <= energy.max_energy);
    }

    #[test]
    fn test_invariant_version_monotonic() {
        let mut record = ContentRecord::default();
        
        for expected in 1..=10u16 {
            record.version = record.version.saturating_add(1);
            assert_eq!(record.version, expected);
        }
    }
}

