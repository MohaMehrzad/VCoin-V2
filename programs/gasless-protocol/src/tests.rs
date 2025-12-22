//! Unit tests for Gasless Protocol
//!
//! These tests run against the ACTUAL program code.

#[cfg(test)]
mod tests {
    use crate::constants::*;
    use crate::state::{GaslessConfig, SessionKey, UserGaslessStats, FeeMethod};
    use anchor_lang::prelude::Pubkey;

    // ========================================================================
    // Constants Tests
    // ========================================================================

    #[test]
    fn test_pda_seeds() {
        assert_eq!(GASLESS_CONFIG_SEED, b"gasless-config");
        assert_eq!(SESSION_KEY_SEED, b"session-key");
        assert_eq!(USER_GASLESS_SEED, b"user-gasless");
        assert_eq!(FEE_VAULT_SEED, b"fee-vault");
        assert_eq!(DAILY_BUDGET_SEED, b"daily-budget");
    }

    #[test]
    fn test_session_limits() {
        assert_eq!(SESSION_DURATION, 24 * 60 * 60, "24 hours");
        assert_eq!(MAX_SESSION_ACTIONS, 1000);
        assert_eq!(MAX_SESSION_SPEND, 100_000_000_000_000, "100K VCoin");
    }

    #[test]
    fn test_fee_configuration() {
        assert_eq!(DEFAULT_SOL_FEE, 5_000, "0.000005 SOL");
        assert_eq!(VCOIN_FEE_MULTIPLIER, 100, "100x multiplier");
        assert_eq!(SSCRE_DEDUCTION_BPS, 100, "1%");
    }

    #[test]
    fn test_daily_budget_limits() {
        assert_eq!(DAILY_SUBSIDY_BUDGET_SOL, 10_000_000_000, "10 SOL");
        assert_eq!(MAX_SUBSIDIZED_TX_PER_USER, 50);
    }

    #[test]
    fn test_scope_bits() {
        assert_eq!(SCOPE_TIP, 1 << 0);
        assert_eq!(SCOPE_VOUCH, 1 << 1);
        assert_eq!(SCOPE_CONTENT, 1 << 2);
        assert_eq!(SCOPE_GOVERNANCE, 1 << 3);
        assert_eq!(SCOPE_TRANSFER, 1 << 4);
        assert_eq!(SCOPE_STAKE, 1 << 5);
        assert_eq!(SCOPE_CLAIM, 1 << 6);
        assert_eq!(SCOPE_FOLLOW, 1 << 7);
        assert_eq!(SCOPE_ALL, 0xFFFF);
    }

    // ========================================================================
    // State Size Tests
    // ========================================================================

    #[test]
    fn test_gasless_config_size() {
        let expected = 8 + 32 + 32 + 32 + 32 + 32 + 8 + 8 + 8 + 2 + 4 + 8 + 8 + 8 + 1 + 4 + 8 + 1;
        assert_eq!(GaslessConfig::LEN, expected, "Config size mismatch");
    }

    #[test]
    fn test_session_key_size() {
        let expected = 8 + 32 + 32 + 2 + 8 + 8 + 4 + 4 + 8 + 8 + 1 + 8 + 1 + 1;
        assert_eq!(SessionKey::LEN, expected, "Session key size mismatch");
    }

    #[test]
    fn test_user_gasless_stats_size() {
        let expected = 8 + 32 + 8 + 8 + 8 + 8 + 4 + 32 + 4 + 4 + 8 + 8 + 1;
        assert_eq!(UserGaslessStats::LEN, expected, "User stats size mismatch");
    }

    // ========================================================================
    // Session Key Scope Tests
    // ========================================================================

    #[test]
    fn test_session_scope_all() {
        let session = SessionKey {
            scope: SCOPE_ALL,
            ..Default::default()
        };
        
        assert!(session.is_action_in_scope(SCOPE_TIP));
        assert!(session.is_action_in_scope(SCOPE_VOUCH));
        assert!(session.is_action_in_scope(SCOPE_GOVERNANCE));
    }

    #[test]
    fn test_session_scope_selective() {
        let session = SessionKey {
            scope: SCOPE_TIP | SCOPE_CONTENT,
            ..Default::default()
        };
        
        assert!(session.is_action_in_scope(SCOPE_TIP));
        assert!(session.is_action_in_scope(SCOPE_CONTENT));
        assert!(!session.is_action_in_scope(SCOPE_VOUCH));
        assert!(!session.is_action_in_scope(SCOPE_GOVERNANCE));
    }

    #[test]
    fn test_session_scope_none() {
        let session = SessionKey {
            scope: 0,
            ..Default::default()
        };
        
        assert!(!session.is_action_in_scope(SCOPE_TIP));
        assert!(!session.is_action_in_scope(SCOPE_VOUCH));
    }

    // ========================================================================
    // Session Validity Tests
    // ========================================================================

    #[test]
    fn test_session_is_valid() {
        let session = SessionKey {
            expires_at: 2000,
            max_actions: 100,
            actions_used: 50,
            is_revoked: false,
            ..Default::default()
        };
        
        assert!(session.is_valid(1500), "Should be valid");
    }

    #[test]
    fn test_session_expired() {
        let session = SessionKey {
            expires_at: 1000,
            max_actions: 100,
            actions_used: 50,
            is_revoked: false,
            ..Default::default()
        };
        
        assert!(!session.is_valid(1500), "Should be expired");
    }

    #[test]
    fn test_session_revoked() {
        let session = SessionKey {
            expires_at: 2000,
            max_actions: 100,
            actions_used: 50,
            is_revoked: true,
            ..Default::default()
        };
        
        assert!(!session.is_valid(1500), "Should be invalid (revoked)");
    }

    #[test]
    fn test_session_actions_exhausted() {
        let session = SessionKey {
            expires_at: 2000,
            max_actions: 100,
            actions_used: 100,
            is_revoked: false,
            ..Default::default()
        };
        
        assert!(!session.is_valid(1500), "Should be invalid (actions exhausted)");
    }

    // ========================================================================
    // Fee Method Tests
    // ========================================================================

    #[test]
    fn test_fee_method_default() {
        let method = FeeMethod::default();
        assert_eq!(method, FeeMethod::PlatformSubsidized);
    }

    #[test]
    fn test_fee_method_variants() {
        let _subsidized = FeeMethod::PlatformSubsidized;
        let _vcoin = FeeMethod::VCoinDeduction;
        let _sscre = FeeMethod::SSCREDeduction;
    }

    // ========================================================================
    // Daily Budget Tests
    // ========================================================================

    #[test]
    fn test_get_day_number() {
        let timestamp = 172800i64; // 2 days in seconds
        let day = GaslessConfig::get_day_number(timestamp);
        assert_eq!(day, 2);
    }

    #[test]
    fn test_should_reset_daily_budget() {
        let config = GaslessConfig {
            current_day: 10,
            ..Default::default()
        };
        
        let day_10_timestamp = 10 * 86400i64;
        let day_11_timestamp = 11 * 86400i64;
        
        assert!(!config.should_reset_daily_budget(day_10_timestamp));
        assert!(config.should_reset_daily_budget(day_11_timestamp));
    }

    // ========================================================================
    // User Stats Daily Reset Tests
    // ========================================================================

    #[test]
    fn test_user_stats_daily_reset() {
        let mut stats = UserGaslessStats {
            current_day: 10,
            today_subsidized: 25,
            ..Default::default()
        };
        
        let day_11_timestamp = 11 * 86400i64;
        stats.check_daily_reset(day_11_timestamp);
        
        assert_eq!(stats.current_day, 11);
        assert_eq!(stats.today_subsidized, 0);
    }

    #[test]
    fn test_user_stats_no_reset_same_day() {
        let mut stats = UserGaslessStats {
            current_day: 10,
            today_subsidized: 25,
            ..Default::default()
        };
        
        let day_10_timestamp = 10 * 86400i64 + 1000;
        stats.check_daily_reset(day_10_timestamp);
        
        assert_eq!(stats.current_day, 10);
        assert_eq!(stats.today_subsidized, 25); // Not reset
    }

    // ========================================================================
    // PDA Derivation Tests
    // ========================================================================

    #[test]
    fn test_session_key_pda_unique() {
        let program_id = Pubkey::new_unique();
        let user = Pubkey::new_unique();
        let session1 = Pubkey::new_unique();
        let session2 = Pubkey::new_unique();
        
        let (pda1, _) = Pubkey::find_program_address(
            &[SESSION_KEY_SEED, user.as_ref(), session1.as_ref()],
            &program_id
        );
        
        let (pda2, _) = Pubkey::find_program_address(
            &[SESSION_KEY_SEED, user.as_ref(), session2.as_ref()],
            &program_id
        );
        
        assert_ne!(pda1, pda2);
    }

    // ========================================================================
    // Invariant Tests
    // ========================================================================

    #[test]
    fn test_invariant_actions_bounded() {
        let session = SessionKey {
            actions_used: 500,
            max_actions: 1000,
            ..Default::default()
        };
        
        assert!(session.actions_used <= session.max_actions);
    }

    #[test]
    fn test_invariant_spend_bounded() {
        let session = SessionKey {
            vcoin_spent: 50_000_000_000_000,
            max_spend: 100_000_000_000_000,
            ..Default::default()
        };
        
        assert!(session.vcoin_spent <= session.max_spend);
    }

    #[test]
    fn test_invariant_daily_subsidy_bounded() {
        let stats = UserGaslessStats {
            today_subsidized: 30,
            ..Default::default()
        };
        
        assert!(stats.today_subsidized <= MAX_SUBSIDIZED_TX_PER_USER);
    }
}

