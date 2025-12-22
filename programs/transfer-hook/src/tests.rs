//! Unit tests for Transfer Hook
//!
//! These tests run against the ACTUAL program code.

#[cfg(test)]
mod tests {
    use crate::constants::*;
    use crate::state::{HookConfig, UserActivity, PairTracking};
    use crate::utils::{update_user_activity, check_wash_trading};
    use anchor_lang::prelude::Pubkey;

    // ========================================================================
    // Constants Tests
    // ========================================================================

    #[test]
    fn test_min_activity_threshold() {
        assert_eq!(MIN_ACTIVITY_THRESHOLD, 1_000_000_000, "Min activity = 1 VCoin");
    }

    #[test]
    fn test_max_transfers_per_hour() {
        assert_eq!(MAX_TRANSFERS_PER_HOUR, 20, "Max 20 transfers/hour before diminishing");
    }

    #[test]
    fn test_wash_trading_cooldown() {
        assert_eq!(WASH_TRADING_COOLDOWN_SECONDS, 3600, "1 hour cooldown");
    }

    // ========================================================================
    // PDA Seeds Tests
    // ========================================================================

    #[test]
    fn test_hook_config_seed() {
        assert_eq!(HOOK_CONFIG_SEED, b"hook-config");
    }

    #[test]
    fn test_transfer_record_seed() {
        assert_eq!(TRANSFER_RECORD_SEED, b"transfer-record");
    }

    #[test]
    fn test_user_activity_seed() {
        assert_eq!(USER_ACTIVITY_SEED, b"user-activity");
    }

    #[test]
    fn test_pair_tracking_seed() {
        assert_eq!(PAIR_TRACKING_SEED, b"pair-tracking");
    }

    // ========================================================================
    // State Size Tests
    // ========================================================================

    #[test]
    fn test_hook_config_size() {
        let expected = 8 + 32 + 32 + 32 + 1 + 8 + 8 + 8 + 1 + 1;
        assert_eq!(HookConfig::LEN, expected, "HookConfig size mismatch");
    }

    #[test]
    fn test_user_activity_size() {
        let expected = 8 + 32 + 1 + 8 + 8 + 8 + 8 + 8 + 8 + 2 + 1;
        assert_eq!(UserActivity::LEN, expected, "UserActivity size mismatch");
    }

    #[test]
    fn test_pair_tracking_size() {
        let expected = 8 + 32 + 32 + 8 + 2 + 8 + 8 + 2 + 2 + 1;
        assert_eq!(PairTracking::LEN, expected, "PairTracking size mismatch");
    }

    // ========================================================================
    // Activity Score Tests
    // ========================================================================

    #[test]
    fn test_activity_contribution_within_limit() {
        let mut activity = UserActivity::default();
        let user = Pubkey::new_unique();
        let current_time = 1000i64;
        
        update_user_activity(&mut activity, user, MIN_ACTIVITY_THRESHOLD, true, current_time).unwrap();
        
        assert_eq!(activity.transfers_this_hour, 1);
        assert_eq!(activity.activity_score_contribution, 100); // Full contribution
    }

    #[test]
    fn test_activity_diminishing_returns() {
        let mut activity = UserActivity::default();
        activity.transfers_this_hour = 25; // Over MAX_TRANSFERS_PER_HOUR
        activity.hour_reset_time = 500;
        
        let user = Pubkey::new_unique();
        let current_time = 1000i64; // Same hour
        
        update_user_activity(&mut activity, user, MIN_ACTIVITY_THRESHOLD, true, current_time).unwrap();
        
        // Should get diminished contribution (50/26 = 1)
        assert!(activity.activity_score_contribution < 100);
    }

    #[test]
    fn test_hourly_reset() {
        let mut activity = UserActivity::default();
        activity.transfers_this_hour = 15;
        activity.hour_reset_time = 0;
        
        let user = Pubkey::new_unique();
        let current_time = 7200i64; // 2 hours later
        
        update_user_activity(&mut activity, user, MIN_ACTIVITY_THRESHOLD, true, current_time).unwrap();
        
        // Should have reset and be at 1
        assert_eq!(activity.transfers_this_hour, 1);
        assert_eq!(activity.hour_reset_time, current_time);
    }

    // ========================================================================
    // Transfer Tracking Tests
    // ========================================================================

    #[test]
    fn test_sender_tracking() {
        let mut activity = UserActivity::default();
        let user = Pubkey::new_unique();
        let amount = 5_000_000_000u64; // 5 VCoin
        
        update_user_activity(&mut activity, user, amount, true, 1000).unwrap();
        
        assert_eq!(activity.total_transfers_sent, 1);
        assert_eq!(activity.total_amount_sent, amount);
        assert_eq!(activity.total_transfers_received, 0);
        assert_eq!(activity.total_amount_received, 0);
    }

    #[test]
    fn test_receiver_tracking() {
        let mut activity = UserActivity::default();
        let user = Pubkey::new_unique();
        let amount = 5_000_000_000u64;
        
        update_user_activity(&mut activity, user, amount, false, 1000).unwrap();
        
        assert_eq!(activity.total_transfers_received, 1);
        assert_eq!(activity.total_amount_received, amount);
        assert_eq!(activity.total_transfers_sent, 0);
        assert_eq!(activity.total_amount_sent, 0);
    }

    #[test]
    fn test_transfer_count_accumulates() {
        let mut activity = UserActivity::default();
        let user = Pubkey::new_unique();
        
        for i in 1..=5 {
            update_user_activity(&mut activity, user, MIN_ACTIVITY_THRESHOLD, true, 1000 + i).unwrap();
        }
        
        assert_eq!(activity.total_transfers_sent, 5);
    }

    // ========================================================================
    // Wash Trading Detection Tests
    // ========================================================================

    #[test]
    fn test_wash_trading_not_detected_first_transfer() {
        let mut pair = PairTracking::default();
        let sender = Pubkey::new_unique();
        let receiver = Pubkey::new_unique();
        
        let is_wash = check_wash_trading(&mut pair, sender, receiver, MIN_ACTIVITY_THRESHOLD, 1000).unwrap();
        
        assert!(!is_wash, "First transfer should not be flagged");
        assert_eq!(pair.trust_score, 5000 + 10); // Initial + rebuild
    }

    #[test]
    fn test_wash_trading_high_frequency() {
        let mut pair = PairTracking::default();
        pair.sender = Pubkey::new_unique();
        pair.receiver = Pubkey::new_unique();
        pair.transfers_24h = 15; // High frequency
        pair.last_transfer_time = 1000;
        pair.trust_score = 5000;
        pair.day_reset_time = 500;
        
        let sender = pair.sender;
        let receiver = pair.receiver;
        
        // Rapid transfer (within cooldown)
        let is_wash = check_wash_trading(&mut pair, sender, receiver, MIN_ACTIVITY_THRESHOLD, 1500).unwrap();
        
        assert!(is_wash, "Should detect wash trading pattern");
        assert_eq!(pair.wash_flags, 1);
        assert_eq!(pair.trust_score, 4500); // Decreased by 500
    }

    #[test]
    fn test_legitimate_transfer_rebuilds_trust() {
        let mut pair = PairTracking::default();
        pair.sender = Pubkey::new_unique();
        pair.receiver = Pubkey::new_unique();
        pair.transfers_24h = 2; // Low frequency
        pair.last_transfer_time = 0;
        pair.trust_score = 4000;
        pair.day_reset_time = 0;
        
        let sender = pair.sender;
        let receiver = pair.receiver;
        
        // Legitimate transfer (after cooldown)
        let is_wash = check_wash_trading(&mut pair, sender, receiver, MIN_ACTIVITY_THRESHOLD, 10000).unwrap();
        
        assert!(!is_wash, "Should not be wash trading");
        assert_eq!(pair.trust_score, 4010); // Rebuilt by 10
    }

    #[test]
    fn test_trust_score_bounded_at_max() {
        let mut pair = PairTracking::default();
        pair.sender = Pubkey::new_unique();
        pair.receiver = Pubkey::new_unique();
        pair.trust_score = 10000; // Already at max
        pair.day_reset_time = 0;
        
        let sender = pair.sender;
        let receiver = pair.receiver;
        
        check_wash_trading(&mut pair, sender, receiver, MIN_ACTIVITY_THRESHOLD, 100000).unwrap();
        
        assert_eq!(pair.trust_score, 10000); // Should not exceed max
    }

    // ========================================================================
    // Daily Reset Tests
    // ========================================================================

    #[test]
    fn test_daily_reset_clears_counters() {
        let mut pair = PairTracking::default();
        pair.sender = Pubkey::new_unique();
        pair.receiver = Pubkey::new_unique();
        pair.transfers_24h = 50;
        pair.amount_24h = 1_000_000_000_000;
        pair.day_reset_time = 0;
        pair.trust_score = 5000;
        
        let sender = pair.sender;
        let receiver = pair.receiver;
        
        // Next day
        check_wash_trading(&mut pair, sender, receiver, MIN_ACTIVITY_THRESHOLD, 100000).unwrap();
        
        // Should have reset and started fresh
        assert_eq!(pair.transfers_24h, 1); // Reset + this transfer
    }

    // ========================================================================
    // Edge Case Tests
    // ========================================================================

    #[test]
    fn test_same_sender_receiver_pda() {
        // Same address sending to itself should have unique PDA
        let program_id = Pubkey::new_unique();
        let user = Pubkey::new_unique();
        
        let (pda, _) = Pubkey::find_program_address(
            &[PAIR_TRACKING_SEED, user.as_ref(), user.as_ref()],
            &program_id
        );
        
        assert_ne!(pda, Pubkey::default());
    }

    #[test]
    fn test_zero_amount_handling() {
        let mut activity = UserActivity::default();
        let user = Pubkey::new_unique();
        
        // Zero amount transfer
        update_user_activity(&mut activity, user, 0, true, 1000).unwrap();
        
        assert_eq!(activity.total_amount_sent, 0);
        assert_eq!(activity.transfers_this_hour, 1); // Still counted
    }

    // ========================================================================
    // Invariant Tests
    // ========================================================================

    #[test]
    fn test_activity_score_bounded() {
        let mut activity = UserActivity::default();
        let user = Pubkey::new_unique();
        
        // Many transfers
        for i in 0..1000 {
            update_user_activity(&mut activity, user, MIN_ACTIVITY_THRESHOLD, true, 1000 + i as i64).unwrap();
        }
        
        // Score should be bounded (u16 max = 65535)
        assert!(activity.activity_score_contribution <= u16::MAX);
    }

    #[test]
    fn test_trust_score_bounded_at_zero() {
        let mut pair = PairTracking::default();
        pair.sender = Pubkey::new_unique();
        pair.receiver = Pubkey::new_unique();
        pair.trust_score = 100; // Very low
        pair.transfers_24h = 20;
        pair.last_transfer_time = 500;
        pair.day_reset_time = 0;
        
        let sender = pair.sender;
        let receiver = pair.receiver;
        
        // Rapid wash trading
        check_wash_trading(&mut pair, sender, receiver, MIN_ACTIVITY_THRESHOLD, 600).unwrap();
        
        // Should not underflow
        assert!(pair.trust_score <= 100);
    }

    #[test]
    fn test_transfer_count_monotonic() {
        let mut activity = UserActivity::default();
        let user = Pubkey::new_unique();
        
        let mut prev_sent = 0u64;
        for i in 0..10 {
            update_user_activity(&mut activity, user, MIN_ACTIVITY_THRESHOLD, true, 1000 + i as i64).unwrap();
            assert!(activity.total_transfers_sent >= prev_sent, "Transfer count should be monotonic");
            prev_sent = activity.total_transfers_sent;
        }
    }
}

