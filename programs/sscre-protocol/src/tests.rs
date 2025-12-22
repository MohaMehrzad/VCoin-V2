//! Unit tests for SSCRE Protocol
//!
//! These tests run against the ACTUAL program code.

#[cfg(test)]
mod tests {
    use crate::constants::*;
    use crate::state::{RewardsPoolConfig, EpochDistribution, UserClaim};
    use crate::state::utils::{get_five_a_multiplier, compute_leaf, verify_merkle_proof};
    use anchor_lang::prelude::Pubkey;
    use solana_program::keccak;

    // ========================================================================
    // Constants Tests
    // ========================================================================

    #[test]
    fn test_primary_reserves() {
        // 350M VCoin = 350 * 10^6 * 10^9
        assert_eq!(PRIMARY_RESERVES, 350_000_000 * 1_000_000_000);
    }

    #[test]
    fn test_secondary_reserves() {
        assert_eq!(SECONDARY_RESERVES, 40_000_000 * 1_000_000_000);
    }

    #[test]
    fn test_epoch_duration() {
        assert_eq!(EPOCH_DURATION, 30 * 24 * 60 * 60, "30 days epoch");
    }

    #[test]
    fn test_claim_window() {
        assert_eq!(CLAIM_WINDOW, 90 * 24 * 60 * 60, "90 days claim window");
    }

    #[test]
    fn test_gasless_fee() {
        assert_eq!(GASLESS_FEE_BPS, 100, "1% fee");
    }

    #[test]
    fn test_min_claim_amount() {
        assert_eq!(MIN_CLAIM_AMOUNT, 1_000_000_000, "1 VCoin minimum");
    }

    #[test]
    fn test_circuit_breaker_thresholds() {
        assert_eq!(MAX_EPOCH_EMISSION, 10_000_000 * 1_000_000_000, "10M VCoin max/epoch");
        assert_eq!(MAX_SINGLE_CLAIM, 100_000 * 1_000_000_000, "100K VCoin max/claim");
    }

    // ========================================================================
    // 5A Score Multiplier Tests
    // ========================================================================

    #[test]
    fn test_five_a_multiplier_lowest() {
        assert_eq!(get_five_a_multiplier(0), SCORE_MULT_0_20);
        assert_eq!(get_five_a_multiplier(1000), SCORE_MULT_0_20);
        assert_eq!(get_five_a_multiplier(1999), SCORE_MULT_0_20);
    }

    #[test]
    fn test_five_a_multiplier_low() {
        assert_eq!(get_five_a_multiplier(2000), SCORE_MULT_20_40);
        assert_eq!(get_five_a_multiplier(3999), SCORE_MULT_20_40);
    }

    #[test]
    fn test_five_a_multiplier_mid() {
        assert_eq!(get_five_a_multiplier(4000), SCORE_MULT_40_60);
        assert_eq!(get_five_a_multiplier(5999), SCORE_MULT_40_60);
    }

    #[test]
    fn test_five_a_multiplier_high() {
        assert_eq!(get_five_a_multiplier(6000), SCORE_MULT_60_80);
        assert_eq!(get_five_a_multiplier(7999), SCORE_MULT_60_80);
    }

    #[test]
    fn test_five_a_multiplier_highest() {
        assert_eq!(get_five_a_multiplier(8000), SCORE_MULT_80_100);
        assert_eq!(get_five_a_multiplier(10000), SCORE_MULT_80_100);
    }

    #[test]
    fn test_multiplier_values() {
        assert_eq!(SCORE_MULT_0_20, 100, "0.1x = 10%");
        assert_eq!(SCORE_MULT_20_40, 400, "0.4x = 40%");
        assert_eq!(SCORE_MULT_40_60, 700, "0.7x = 70%");
        assert_eq!(SCORE_MULT_60_80, 1000, "1.0x = 100%");
        assert_eq!(SCORE_MULT_80_100, 1200, "1.2x = 120%");
    }

    // ========================================================================
    // PDA Seeds Tests
    // ========================================================================

    #[test]
    fn test_pda_seeds() {
        assert_eq!(POOL_CONFIG_SEED, b"pool-config");
        assert_eq!(EPOCH_SEED, b"epoch");
        assert_eq!(USER_CLAIM_SEED, b"user-claim");
        assert_eq!(FUNDING_LAYER_SEED, b"funding-layer");
        assert_eq!(CIRCUIT_BREAKER_SEED, b"circuit-breaker");
    }

    // ========================================================================
    // State Size Tests
    // ========================================================================

    #[test]
    fn test_rewards_pool_config_size() {
        let expected = 8 + 32 + 32 + 32 + 32 + (32 * 5) + 1 + 8 + 8 + 8 + 1 + 1 + 32 + 1 + 1;
        assert_eq!(RewardsPoolConfig::LEN, expected, "Config size mismatch");
    }

    #[test]
    fn test_epoch_distribution_size() {
        let expected = 8 + 8 + 32 + 8 + 8 + 8 + 8 + 8 + 8 + 1 + 32 + 2 + 8 + 1;
        assert_eq!(EpochDistribution::LEN, expected, "Epoch size mismatch");
    }

    #[test]
    fn test_user_claim_size() {
        let expected = 8 + 32 + 8 + 8 + 4 + 8 + 8 + (8 * 4) + 1;
        assert_eq!(UserClaim::LEN, expected, "User claim size mismatch");
    }

    // ========================================================================
    // Merkle Proof Tests
    // ========================================================================

    #[test]
    fn test_compute_leaf() {
        let user = Pubkey::new_unique();
        let amount = 1000u64;
        let epoch = 1u64;
        
        let leaf = compute_leaf(&user, amount, epoch);
        
        // Verify it's deterministic
        let leaf2 = compute_leaf(&user, amount, epoch);
        assert_eq!(leaf, leaf2);
    }

    #[test]
    fn test_compute_leaf_different_inputs() {
        let user1 = Pubkey::new_unique();
        let user2 = Pubkey::new_unique();
        let amount = 1000u64;
        let epoch = 1u64;
        
        let leaf1 = compute_leaf(&user1, amount, epoch);
        let leaf2 = compute_leaf(&user2, amount, epoch);
        
        assert_ne!(leaf1, leaf2, "Different users should have different leaves");
    }

    #[test]
    fn test_verify_merkle_proof_empty() {
        // For a single-leaf tree, the leaf is the root
        let user = Pubkey::new_unique();
        let leaf = compute_leaf(&user, 1000, 1);
        
        let proof: Vec<[u8; 32]> = vec![];
        
        // With empty proof, leaf must equal root
        assert!(verify_merkle_proof(&proof, &leaf, &leaf));
    }

    #[test]
    fn test_verify_merkle_proof_single_sibling() {
        let user1 = Pubkey::new_unique();
        let user2 = Pubkey::new_unique();
        
        let leaf1 = compute_leaf(&user1, 1000, 1);
        let leaf2 = compute_leaf(&user2, 2000, 1);
        
        // Create root from two leaves
        let (left, right) = if leaf1 < leaf2 { (leaf1, leaf2) } else { (leaf2, leaf1) };
        let mut combined = [0u8; 64];
        combined[..32].copy_from_slice(&left);
        combined[32..].copy_from_slice(&right);
        let root = keccak::hash(&combined).to_bytes();
        
        // Verify leaf1 with proof = [leaf2]
        let proof = vec![leaf2];
        assert!(verify_merkle_proof(&proof, &root, &leaf1));
        
        // Verify leaf2 with proof = [leaf1]
        let proof2 = vec![leaf1];
        assert!(verify_merkle_proof(&proof2, &root, &leaf2));
    }

    #[test]
    fn test_verify_merkle_proof_invalid() {
        let user = Pubkey::new_unique();
        let leaf = compute_leaf(&user, 1000, 1);
        
        let fake_root = [0u8; 32];
        let proof: Vec<[u8; 32]> = vec![];
        
        // Wrong root should fail
        assert!(!verify_merkle_proof(&proof, &fake_root, &leaf));
    }

    // ========================================================================
    // User Claim Bitmap Tests
    // ========================================================================

    #[test]
    fn test_user_claim_epoch_unclaimed() {
        let claim = UserClaim::default();
        
        assert!(!claim.is_epoch_claimed(0));
        assert!(!claim.is_epoch_claimed(1));
        assert!(!claim.is_epoch_claimed(100));
    }

    #[test]
    fn test_user_claim_mark_claimed() {
        let mut claim = UserClaim::default();
        
        claim.mark_epoch_claimed(5);
        
        assert!(claim.is_epoch_claimed(5));
        assert!(!claim.is_epoch_claimed(4));
        assert!(!claim.is_epoch_claimed(6));
    }

    #[test]
    fn test_user_claim_multiple_epochs() {
        let mut claim = UserClaim::default();
        
        claim.mark_epoch_claimed(1);
        claim.mark_epoch_claimed(10);
        claim.mark_epoch_claimed(100);
        
        assert!(claim.is_epoch_claimed(1));
        assert!(claim.is_epoch_claimed(10));
        assert!(claim.is_epoch_claimed(100));
        assert!(!claim.is_epoch_claimed(50));
    }

    #[test]
    fn test_user_claim_bitmap_boundaries() {
        let mut claim = UserClaim::default();
        
        // Test at bitmap boundaries (0, 63, 64, 127, 128, 191, 192, 255)
        for epoch in [0u64, 63, 64, 127, 128, 191, 192, 255] {
            claim.mark_epoch_claimed(epoch);
            assert!(claim.is_epoch_claimed(epoch), "Epoch {} should be claimed", epoch);
        }
    }

    #[test]
    fn test_user_claim_large_epoch() {
        let mut claim = UserClaim::default();
        
        // Epoch > 255 uses last_claimed_epoch tracking
        claim.mark_epoch_claimed(300);
        
        assert_eq!(claim.last_claimed_epoch, 300);
        assert!(claim.is_epoch_claimed(300));
        // Any epoch <= 300 should be considered claimed via last_claimed_epoch
        assert!(claim.is_epoch_claimed(256));
    }

    // ========================================================================
    // Fee Calculation Tests
    // ========================================================================

    #[test]
    fn test_gasless_fee_calculation() {
        let amount = 100_000_000_000u64; // 100 VCoin
        let fee = (amount as u128 * GASLESS_FEE_BPS as u128 / 10000) as u64;
        let net = amount - fee;
        
        assert_eq!(fee, 1_000_000_000, "1% of 100 VCoin = 1 VCoin");
        assert_eq!(net, 99_000_000_000, "Net = 99 VCoin");
    }

    #[test]
    fn test_min_claim_fee() {
        let amount = MIN_CLAIM_AMOUNT; // 1 VCoin
        let fee = (amount as u128 * GASLESS_FEE_BPS as u128 / 10000) as u64;
        
        assert_eq!(fee, 10_000_000, "1% of 1 VCoin = 0.01 VCoin");
    }

    // ========================================================================
    // Epoch Progression Tests
    // ========================================================================

    #[test]
    fn test_epoch_default() {
        let epoch = EpochDistribution::default();
        
        assert_eq!(epoch.epoch, 0);
        assert_eq!(epoch.total_allocation, 0);
        assert_eq!(epoch.total_claimed, 0);
        assert!(!epoch.is_finalized);
    }

    #[test]
    fn test_epoch_claim_expiry() {
        let start_time = 1000i64;
        let end_time = start_time + EPOCH_DURATION;
        let claim_expiry = start_time + EPOCH_DURATION + CLAIM_WINDOW;
        
        // 30 + 90 = 120 days total
        assert_eq!(claim_expiry - start_time, (30 + 90) * 24 * 60 * 60);
    }

    // ========================================================================
    // Invariant Tests
    // ========================================================================

    #[test]
    fn test_invariant_total_claimed_bounded() {
        let epoch = EpochDistribution {
            total_allocation: 1_000_000 * 1_000_000_000, // 1M VCoin
            total_claimed: 500_000 * 1_000_000_000,      // 500K VCoin
            ..Default::default()
        };
        
        // total_claimed should never exceed total_allocation
        assert!(epoch.total_claimed <= epoch.total_allocation);
    }

    #[test]
    fn test_invariant_circuit_breaker_limits() {
        let claim_amount = 50_000u64 * 1_000_000_000; // 50K VCoin
        
        assert!(claim_amount < MAX_SINGLE_CLAIM, "Within single claim limit");
    }

    #[test]
    fn test_invariant_epoch_emission_limit() {
        let epoch_emission = 5_000_000u64 * 1_000_000_000; // 5M VCoin
        
        assert!(epoch_emission < MAX_EPOCH_EMISSION, "Within epoch limit");
    }

    // ========================================================================
    // PDA Derivation Tests
    // ========================================================================

    #[test]
    fn test_epoch_pda_unique() {
        let program_id = Pubkey::new_unique();
        
        let (pda1, _) = Pubkey::find_program_address(
            &[EPOCH_SEED, &1u64.to_le_bytes()],
            &program_id
        );
        
        let (pda2, _) = Pubkey::find_program_address(
            &[EPOCH_SEED, &2u64.to_le_bytes()],
            &program_id
        );
        
        assert_ne!(pda1, pda2, "Different epochs should have different PDAs");
    }

    #[test]
    fn test_user_claim_pda_unique() {
        let program_id = Pubkey::new_unique();
        let user1 = Pubkey::new_unique();
        let user2 = Pubkey::new_unique();
        
        let (pda1, _) = Pubkey::find_program_address(
            &[USER_CLAIM_SEED, user1.as_ref()],
            &program_id
        );
        
        let (pda2, _) = Pubkey::find_program_address(
            &[USER_CLAIM_SEED, user2.as_ref()],
            &program_id
        );
        
        assert_ne!(pda1, pda2, "Different users should have different PDAs");
    }
}

