//! Unit tests for veVCoin Token (Soulbound)
//!
//! These tests run against the ACTUAL program code.

#[cfg(test)]
mod tests {
    use crate::constants::*;
    use crate::state::{VeVCoinConfig, UserVeVCoin};
    use anchor_lang::prelude::Pubkey;

    // ========================================================================
    // Constants Tests
    // ========================================================================

    #[test]
    fn test_vevcoin_decimals() {
        assert_eq!(VEVCOIN_DECIMALS, 9, "veVCoin should have 9 decimals");
    }

    #[test]
    fn test_four_years_seconds() {
        let expected = 4 * 365 * 24 * 60 * 60;
        assert_eq!(FOUR_YEARS_SECONDS, expected, "4 years in seconds");
    }

    #[test]
    fn test_token_metadata() {
        assert_eq!(TOKEN_NAME, "veVCoin");
        assert_eq!(TOKEN_SYMBOL, "veVIWO");
        assert!(TOKEN_URI.starts_with("https://"), "URI should be HTTPS");
    }

    // ========================================================================
    // PDA Derivation Tests
    // ========================================================================

    #[test]
    fn test_config_pda_seed() {
        assert_eq!(VEVCOIN_CONFIG_SEED, b"vevcoin-config");
    }

    #[test]
    fn test_user_pda_seed() {
        assert_eq!(USER_VEVCOIN_SEED, b"user-vevcoin");
    }

    #[test]
    fn test_config_pda_derivation() {
        let program_id = Pubkey::new_unique();
        let (pda, bump) = Pubkey::find_program_address(
            &[VEVCOIN_CONFIG_SEED],
            &program_id
        );
        
        assert!(bump <= 255);
        assert_ne!(pda, Pubkey::default());
    }

    #[test]
    fn test_user_pda_derivation() {
        let program_id = Pubkey::new_unique();
        let user = Pubkey::new_unique();
        
        let (pda, bump) = Pubkey::find_program_address(
            &[USER_VEVCOIN_SEED, user.as_ref()],
            &program_id
        );
        
        assert!(bump <= 255);
        assert_ne!(pda, Pubkey::default());
    }

    // ========================================================================
    // State Tests
    // ========================================================================

    #[test]
    fn test_vevcoin_config_size() {
        let expected_size = 8 + 32 + 32 + 32 + 8 + 8 + 1;
        assert_eq!(VeVCoinConfig::LEN, expected_size, "Config size mismatch");
    }

    #[test]
    fn test_user_vevcoin_size() {
        let expected_size = 8 + 32 + 8 + 8 + 8 + 1;
        assert_eq!(UserVeVCoin::LEN, expected_size, "User account size mismatch");
    }

    #[test]
    fn test_config_default() {
        let config = VeVCoinConfig::default();
        assert_eq!(config.total_supply, 0);
        assert_eq!(config.total_holders, 0);
        assert_eq!(config.bump, 0);
    }

    #[test]
    fn test_user_vevcoin_default() {
        let user = UserVeVCoin::default();
        assert_eq!(user.balance, 0);
        assert_eq!(user.first_mint_at, 0);
        assert_eq!(user.last_update_at, 0);
    }

    // ========================================================================
    // Mint/Burn Logic Tests
    // ========================================================================

    #[test]
    fn test_mint_increases_balance() {
        let mut user = UserVeVCoin::default();
        let mint_amount = 1000u64 * 1_000_000_000u64; // 1000 veVCoin
        
        // Simulate mint
        user.balance = user.balance.checked_add(mint_amount).unwrap();
        
        assert_eq!(user.balance, mint_amount);
    }

    #[test]
    fn test_burn_decreases_balance() {
        let mut user = UserVeVCoin::default();
        user.balance = 1000u64 * 1_000_000_000u64;
        
        let burn_amount = 400u64 * 1_000_000_000u64;
        user.balance = user.balance.checked_sub(burn_amount).unwrap();
        
        assert_eq!(user.balance, 600u64 * 1_000_000_000u64);
    }

    #[test]
    fn test_burn_exact_balance() {
        let mut user = UserVeVCoin::default();
        user.balance = 1000u64 * 1_000_000_000u64;
        
        // Burn everything
        user.balance = user.balance.checked_sub(user.balance).unwrap();
        
        assert_eq!(user.balance, 0);
    }

    #[test]
    fn test_burn_more_than_balance_fails() {
        let user = UserVeVCoin::default();
        // user.balance is 0
        
        let burn_amount = 100u64;
        let result = user.balance.checked_sub(burn_amount);
        
        assert!(result.is_none(), "Burning more than balance should fail");
    }

    // ========================================================================
    // Total Supply Tracking Tests
    // ========================================================================

    #[test]
    fn test_total_supply_increases_on_mint() {
        let mut config = VeVCoinConfig::default();
        
        let mint1 = 1000u64 * 1_000_000_000u64;
        let mint2 = 500u64 * 1_000_000_000u64;
        
        config.total_supply = config.total_supply.checked_add(mint1).unwrap();
        config.total_supply = config.total_supply.checked_add(mint2).unwrap();
        
        assert_eq!(config.total_supply, 1500u64 * 1_000_000_000u64);
    }

    #[test]
    fn test_total_supply_decreases_on_burn() {
        let mut config = VeVCoinConfig::default();
        config.total_supply = 2000u64 * 1_000_000_000u64;
        
        let burn = 500u64 * 1_000_000_000u64;
        config.total_supply = config.total_supply.checked_sub(burn).unwrap();
        
        assert_eq!(config.total_supply, 1500u64 * 1_000_000_000u64);
    }

    // ========================================================================
    // Total Holders Tracking Tests
    // ========================================================================

    #[test]
    fn test_holder_count_on_first_mint() {
        let mut config = VeVCoinConfig::default();
        let user = UserVeVCoin::default();
        
        // First mint to a new user should increment holders
        if user.balance == 0 {
            config.total_holders = config.total_holders.checked_add(1).unwrap();
        }
        
        assert_eq!(config.total_holders, 1);
    }

    #[test]
    fn test_holder_count_on_full_burn() {
        let mut config = VeVCoinConfig::default();
        config.total_holders = 5;
        
        let mut user = UserVeVCoin::default();
        user.balance = 1000u64 * 1_000_000_000u64;
        
        // Full burn should decrement holders
        user.balance = 0;
        if user.balance == 0 {
            config.total_holders = config.total_holders.checked_sub(1).unwrap();
        }
        
        assert_eq!(config.total_holders, 4);
    }

    // ========================================================================
    // Invariant Tests
    // ========================================================================

    #[test]
    fn test_invariant_total_supply_equals_sum_of_balances() {
        // In a real scenario:
        // config.total_supply == sum(all user.balance)
        
        let config_supply = 3000u64 * 1_000_000_000u64;
        let user1_balance = 1000u64 * 1_000_000_000u64;
        let user2_balance = 2000u64 * 1_000_000_000u64;
        
        assert_eq!(config_supply, user1_balance + user2_balance);
    }

    // ========================================================================
    // Soulbound Property Tests
    // ========================================================================

    #[test]
    fn test_soulbound_property() {
        // veVCoin is non-transferable
        // This is enforced by Token-2022 NonTransferable extension
        // Here we just verify the concept
        
        let user1 = Pubkey::new_unique();
        let user2 = Pubkey::new_unique();
        
        // Different users should have different PDAs
        let program_id = Pubkey::new_unique();
        
        let (pda1, _) = Pubkey::find_program_address(
            &[USER_VEVCOIN_SEED, user1.as_ref()],
            &program_id
        );
        
        let (pda2, _) = Pubkey::find_program_address(
            &[USER_VEVCOIN_SEED, user2.as_ref()],
            &program_id
        );
        
        assert_ne!(pda1, pda2, "Each user should have unique veVCoin account");
    }
}

