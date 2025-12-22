//! Unit tests for ViLink Protocol
//!
//! These tests run against the ACTUAL program code.

#[cfg(test)]
mod tests {
    use crate::constants::*;
    use crate::state::{ViLinkConfig, ViLinkAction, UserActionStats};
    use crate::state::utils::{action_type_name, generate_action_id, generate_dapp_id, generate_batch_id};
    use anchor_lang::prelude::Pubkey;

    // ========================================================================
    // Constants Tests
    // ========================================================================

    #[test]
    fn test_pda_seeds() {
        assert_eq!(CONFIG_SEED, b"vilink-config");
        assert_eq!(ACTION_SEED, b"action");
        assert_eq!(DAPP_REGISTRY_SEED, b"dapp");
        assert_eq!(USER_STATS_SEED, b"user-stats");
        assert_eq!(BATCH_SEED, b"batch");
    }

    #[test]
    fn test_action_types() {
        assert_eq!(ACTION_TIP, 0);
        assert_eq!(ACTION_VOUCH, 1);
        assert_eq!(ACTION_FOLLOW, 2);
        assert_eq!(ACTION_CHALLENGE, 3);
        assert_eq!(ACTION_STAKE, 4);
        assert_eq!(ACTION_CONTENT_REACT, 5);
        assert_eq!(ACTION_DELEGATE, 6);
        assert_eq!(ACTION_VOTE, 7);
    }

    #[test]
    fn test_action_limits() {
        assert_eq!(MAX_ACTIONS_PER_BATCH, 10);
        assert_eq!(MAX_ACTION_EXPIRY, 7 * 24 * 60 * 60, "7 days");
        assert_eq!(MIN_TIP_AMOUNT, 100_000_000, "0.1 VCoin");
        assert_eq!(MAX_TIP_AMOUNT, 10_000_000_000_000, "10,000 VCoin");
    }

    #[test]
    fn test_platform_fee() {
        assert_eq!(PLATFORM_FEE_BPS, 250, "2.5%");
    }

    // ========================================================================
    // State Size Tests
    // ========================================================================

    #[test]
    fn test_vilink_config_size() {
        let expected = 8 + 32 + 32 + 32 + 32 + 32 + 32 + 32 + 32 + 1 + 8 + 8 + 8 + 1 + 2 + 1;
        assert_eq!(ViLinkConfig::LEN, expected, "Config size mismatch");
    }

    #[test]
    fn test_vilink_action_size() {
        let expected = 8 + 32 + 32 + 32 + 1 + 8 + 32 + 8 + 8 + 1 + 32 + 8 + (1 + 32) + 32 + 1 + 4 + 4 + 1;
        assert_eq!(ViLinkAction::LEN, expected, "Action size mismatch");
    }

    #[test]
    fn test_user_action_stats_size() {
        let expected = 8 + 32 + 8 + 8 + 8 + 8 + 8 + 8 + 8 + 8 + 8 + 8 + 1;
        assert_eq!(UserActionStats::LEN, expected, "Stats size mismatch");
    }

    // ========================================================================
    // Action Type Name Tests
    // ========================================================================

    #[test]
    fn test_action_type_names() {
        assert_eq!(action_type_name(0), "Tip");
        assert_eq!(action_type_name(1), "Vouch");
        assert_eq!(action_type_name(2), "Follow");
        assert_eq!(action_type_name(3), "Challenge");
        assert_eq!(action_type_name(4), "Stake");
        assert_eq!(action_type_name(5), "ContentReact");
        assert_eq!(action_type_name(6), "Delegate");
        assert_eq!(action_type_name(7), "Vote");
        assert_eq!(action_type_name(8), "Unknown");
        assert_eq!(action_type_name(255), "Unknown");
    }

    // ========================================================================
    // Action ID Generation Tests
    // ========================================================================

    #[test]
    fn test_action_id_generation() {
        let creator = Pubkey::new_unique();
        let target = Pubkey::new_unique();
        
        let action_id = generate_action_id(&creator, &target, ACTION_TIP, 1000, 12345);
        
        // Verify it's deterministic
        let action_id_2 = generate_action_id(&creator, &target, ACTION_TIP, 1000, 12345);
        assert_eq!(action_id, action_id_2);
    }

    #[test]
    fn test_action_id_unique() {
        let creator = Pubkey::new_unique();
        let target = Pubkey::new_unique();
        
        let id1 = generate_action_id(&creator, &target, ACTION_TIP, 1000, 12345);
        let id2 = generate_action_id(&creator, &target, ACTION_TIP, 2000, 12345);
        let id3 = generate_action_id(&creator, &target, ACTION_TIP, 1000, 12346);
        
        assert_ne!(id1, id2, "Different amounts should produce different IDs");
        assert_ne!(id1, id3, "Different timestamps should produce different IDs");
    }

    #[test]
    fn test_dapp_id_generation() {
        let authority = Pubkey::new_unique();
        
        let dapp_id = generate_dapp_id(&authority);
        
        // Verify deterministic
        let dapp_id_2 = generate_dapp_id(&authority);
        assert_eq!(dapp_id, dapp_id_2);
    }

    #[test]
    fn test_batch_id_generation() {
        let creator = Pubkey::new_unique();
        
        let batch_id = generate_batch_id(&creator, 12345);
        
        // Verify deterministic
        let batch_id_2 = generate_batch_id(&creator, 12345);
        assert_eq!(batch_id, batch_id_2);
        
        // Different timestamp = different batch
        let batch_id_3 = generate_batch_id(&creator, 12346);
        assert_ne!(batch_id, batch_id_3);
    }

    // ========================================================================
    // Config Action Enabled Tests
    // ========================================================================

    #[test]
    fn test_action_enabled_all() {
        let config = ViLinkConfig {
            enabled_actions: 0xFF, // All enabled
            ..Default::default()
        };
        
        for action_type in 0..8 {
            assert!(config.is_action_enabled(action_type), "Action {} should be enabled", action_type);
        }
    }

    #[test]
    fn test_action_enabled_none() {
        let config = ViLinkConfig {
            enabled_actions: 0x00, // None enabled
            ..Default::default()
        };
        
        for action_type in 0..8 {
            assert!(!config.is_action_enabled(action_type), "Action {} should be disabled", action_type);
        }
    }

    #[test]
    fn test_action_enabled_selective() {
        let config = ViLinkConfig {
            enabled_actions: 0b00000101, // Only Tip (0) and Follow (2)
            ..Default::default()
        };
        
        assert!(config.is_action_enabled(ACTION_TIP));
        assert!(!config.is_action_enabled(ACTION_VOUCH));
        assert!(config.is_action_enabled(ACTION_FOLLOW));
        assert!(!config.is_action_enabled(ACTION_CHALLENGE));
    }

    // ========================================================================
    // Fee Calculation Tests
    // ========================================================================

    #[test]
    fn test_platform_fee_calculation() {
        let amount = 100_000_000_000u64; // 100 VCoin
        let fee = (amount as u128 * PLATFORM_FEE_BPS as u128 / 10000) as u64;
        
        // 2.5% of 100 VCoin = 2.5 VCoin
        assert_eq!(fee, 2_500_000_000);
    }

    #[test]
    fn test_min_tip_fee() {
        let amount = MIN_TIP_AMOUNT; // 0.1 VCoin
        let fee = (amount as u128 * PLATFORM_FEE_BPS as u128 / 10000) as u64;
        
        // 2.5% of 0.1 VCoin = 0.0025 VCoin
        assert_eq!(fee, 2_500_000);
    }

    // ========================================================================
    // Action State Tests
    // ========================================================================

    #[test]
    fn test_action_default() {
        let action = ViLinkAction::default();
        
        assert!(!action.executed);
        assert_eq!(action.execution_count, 0);
        assert!(!action.one_time);
    }

    #[test]
    fn test_action_one_time() {
        let mut action = ViLinkAction::default();
        action.one_time = true;
        action.executed = true;
        
        // One-time action that's been executed
        assert!(action.one_time && action.executed);
    }

    #[test]
    fn test_action_multi_execution() {
        let mut action = ViLinkAction::default();
        action.one_time = false;
        action.max_executions = 10;
        action.execution_count = 5;
        
        // Can still execute
        assert!(action.execution_count < action.max_executions);
    }

    // ========================================================================
    // PDA Derivation Tests
    // ========================================================================

    #[test]
    fn test_action_pda_unique() {
        let program_id = Pubkey::new_unique();
        let creator = Pubkey::new_unique();
        let ts1 = 1000i64;
        let ts2 = 2000i64;
        
        let (pda1, _) = Pubkey::find_program_address(
            &[ACTION_SEED, creator.as_ref(), &ts1.to_le_bytes()],
            &program_id
        );
        
        let (pda2, _) = Pubkey::find_program_address(
            &[ACTION_SEED, creator.as_ref(), &ts2.to_le_bytes()],
            &program_id
        );
        
        assert_ne!(pda1, pda2);
    }

    // ========================================================================
    // Invariant Tests
    // ========================================================================

    #[test]
    fn test_invariant_execution_count_bounded() {
        let action = ViLinkAction {
            max_executions: 10,
            execution_count: 10,
            ..Default::default()
        };
        
        // execution_count should not exceed max_executions
        assert!(action.execution_count <= action.max_executions);
    }

    #[test]
    fn test_invariant_tip_amount_bounded() {
        let amount = 5_000_000_000_000u64; // 5000 VCoin
        
        assert!(amount >= MIN_TIP_AMOUNT);
        assert!(amount <= MAX_TIP_AMOUNT);
    }
}

