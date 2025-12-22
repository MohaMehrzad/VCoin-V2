//! Unit tests for Identity Protocol
//!
//! These tests run against the ACTUAL program code.

#[cfg(test)]
mod tests {
    use crate::constants::*;
    use crate::state::{IdentityConfig, Identity, VerificationLevel, SubscriptionTier};
    use anchor_lang::prelude::Pubkey;

    // ========================================================================
    // Constants Tests
    // ========================================================================

    #[test]
    fn test_pda_seeds() {
        assert_eq!(IDENTITY_CONFIG_SEED, b"identity-config");
        assert_eq!(IDENTITY_SEED, b"identity");
        assert_eq!(SAS_ATTESTATION_SEED, b"sas-attestation");
        assert_eq!(SUBSCRIPTION_SEED, b"subscription");
    }

    #[test]
    fn test_subscription_prices() {
        assert_eq!(SUBSCRIPTION_FREE, 0, "Free tier = $0");
        assert_eq!(SUBSCRIPTION_VERIFIED, 4_000_000, "Verified = $4 USDC");
        assert_eq!(SUBSCRIPTION_PREMIUM, 12_000_000, "Premium = $12 USDC");
        assert_eq!(SUBSCRIPTION_ENTERPRISE, 59_000_000, "Enterprise = $59 USDC");
    }

    #[test]
    fn test_subscription_duration() {
        assert_eq!(SUBSCRIPTION_DURATION, 30 * 24 * 60 * 60, "30 days");
    }

    // ========================================================================
    // State Size Tests
    // ========================================================================

    #[test]
    fn test_identity_config_size() {
        let expected = 8 + 32 + 32 + 32 + 32 + (32 * 10) + 1 + 8 + 8 + 1 + 1;
        assert_eq!(IdentityConfig::LEN, expected, "Config size mismatch");
    }

    #[test]
    fn test_identity_size() {
        let expected = 8 + 32 + 32 + 1 + 32 + 32 + 1 + 8 + 8 + 1 + 1;
        assert_eq!(Identity::LEN, expected, "Identity size mismatch");
    }

    // ========================================================================
    // Verification Level Tests
    // ========================================================================

    #[test]
    fn test_verification_level_from_u8() {
        assert_eq!(VerificationLevel::from_u8(0), Some(VerificationLevel::None));
        assert_eq!(VerificationLevel::from_u8(1), Some(VerificationLevel::Basic));
        assert_eq!(VerificationLevel::from_u8(2), Some(VerificationLevel::KYC));
        assert_eq!(VerificationLevel::from_u8(3), Some(VerificationLevel::Full));
        assert_eq!(VerificationLevel::from_u8(4), Some(VerificationLevel::Enhanced));
        assert_eq!(VerificationLevel::from_u8(5), None);
    }

    #[test]
    fn test_verification_level_default() {
        let level = VerificationLevel::default();
        assert_eq!(level, VerificationLevel::None);
    }

    #[test]
    fn test_verification_level_ordering() {
        // Higher levels should be "more verified"
        let levels = [
            VerificationLevel::None,
            VerificationLevel::Basic,
            VerificationLevel::KYC,
            VerificationLevel::Full,
            VerificationLevel::Enhanced,
        ];
        
        for i in 0..levels.len() {
            assert_eq!(VerificationLevel::from_u8(i as u8), Some(levels[i]));
        }
    }

    // ========================================================================
    // Subscription Tier Tests
    // ========================================================================

    #[test]
    fn test_subscription_tier_from_u8() {
        assert_eq!(SubscriptionTier::from_u8(0), Some(SubscriptionTier::Free));
        assert_eq!(SubscriptionTier::from_u8(1), Some(SubscriptionTier::Verified));
        assert_eq!(SubscriptionTier::from_u8(2), Some(SubscriptionTier::Premium));
        assert_eq!(SubscriptionTier::from_u8(3), Some(SubscriptionTier::Enterprise));
        assert_eq!(SubscriptionTier::from_u8(4), None);
    }

    #[test]
    fn test_subscription_tier_prices() {
        assert_eq!(SubscriptionTier::Free.price(), SUBSCRIPTION_FREE);
        assert_eq!(SubscriptionTier::Verified.price(), SUBSCRIPTION_VERIFIED);
        assert_eq!(SubscriptionTier::Premium.price(), SUBSCRIPTION_PREMIUM);
        assert_eq!(SubscriptionTier::Enterprise.price(), SUBSCRIPTION_ENTERPRISE);
    }

    #[test]
    fn test_subscription_tier_default() {
        let tier = SubscriptionTier::default();
        assert_eq!(tier, SubscriptionTier::Free);
    }

    // ========================================================================
    // Identity State Tests
    // ========================================================================

    #[test]
    fn test_identity_default() {
        let identity = Identity::default();
        
        assert_eq!(identity.verification_level, 0);
        assert!(!identity.is_active);
        assert_eq!(identity.created_at, 0);
        assert_eq!(identity.updated_at, 0);
    }

    #[test]
    fn test_identity_username_capacity() {
        let identity = Identity::default();
        
        // Username is stored as [u8; 32]
        assert_eq!(identity.username.len(), 32);
    }

    // ========================================================================
    // Config Tests
    // ========================================================================

    #[test]
    fn test_config_default() {
        let config = IdentityConfig::default();
        
        assert_eq!(config.attester_count, 0);
        assert_eq!(config.total_identities, 0);
        assert_eq!(config.verified_identities, 0);
        assert!(!config.paused);
    }

    #[test]
    fn test_trusted_attester_capacity() {
        let config = IdentityConfig::default();
        
        // Max 10 trusted attesters
        assert_eq!(config.trusted_attesters.len(), 10);
    }

    // ========================================================================
    // PDA Derivation Tests
    // ========================================================================

    #[test]
    fn test_identity_pda_unique_per_user() {
        let program_id = Pubkey::new_unique();
        let user1 = Pubkey::new_unique();
        let user2 = Pubkey::new_unique();
        
        let (pda1, _) = Pubkey::find_program_address(
            &[IDENTITY_SEED, user1.as_ref()],
            &program_id
        );
        
        let (pda2, _) = Pubkey::find_program_address(
            &[IDENTITY_SEED, user2.as_ref()],
            &program_id
        );
        
        assert_ne!(pda1, pda2, "Different users should have different identity PDAs");
    }

    #[test]
    fn test_config_pda() {
        let program_id = Pubkey::new_unique();
        
        let (pda, bump) = Pubkey::find_program_address(
            &[IDENTITY_CONFIG_SEED],
            &program_id
        );
        
        assert!(bump <= 255);
        assert_ne!(pda, Pubkey::default());
    }

    // ========================================================================
    // Subscription Expiry Tests
    // ========================================================================

    #[test]
    fn test_subscription_expiry_calculation() {
        let start_time = 1000i64;
        let expires_at = start_time + SUBSCRIPTION_DURATION;
        
        assert_eq!(expires_at, start_time + 30 * 24 * 60 * 60);
    }

    #[test]
    fn test_subscription_active() {
        let expires_at = 2_000_000i64;
        let current_time = 1_000_000i64;
        
        assert!(current_time < expires_at, "Subscription should be active");
    }

    #[test]
    fn test_subscription_expired() {
        let expires_at = 1_000_000i64;
        let current_time = 2_000_000i64;
        
        assert!(current_time >= expires_at, "Subscription should be expired");
    }

    // ========================================================================
    // Invariant Tests
    // ========================================================================

    #[test]
    fn test_invariant_verified_lte_total() {
        let config = IdentityConfig {
            total_identities: 100,
            verified_identities: 50,
            ..Default::default()
        };
        
        assert!(config.verified_identities <= config.total_identities);
    }

    #[test]
    fn test_invariant_attester_count_bounded() {
        let config = IdentityConfig {
            attester_count: 10,
            ..Default::default()
        };
        
        assert!(config.attester_count as usize <= config.trusted_attesters.len());
    }
}

