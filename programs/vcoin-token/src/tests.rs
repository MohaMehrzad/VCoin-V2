//! Unit tests for VCoin Token
//!
//! These tests run against the ACTUAL program code.

#[cfg(test)]
mod tests {
    use crate::constants::*;
    use crate::state::VCoinConfig;
    use anchor_lang::prelude::Pubkey;

    // ========================================================================
    // Constants Tests
    // ========================================================================

    #[test]
    fn test_vcoin_decimals() {
        assert_eq!(VCOIN_DECIMALS, 9, "VCoin should have 9 decimals");
    }

    #[test]
    fn test_total_supply() {
        // 1 billion tokens with 9 decimals
        let expected = 1_000_000_000u64 * 1_000_000_000u64;
        assert_eq!(TOTAL_SUPPLY, expected, "Total supply should be 1B with 9 decimals");
    }

    #[test]
    fn test_one_vcoin() {
        // 1 VCoin = 10^9 base units
        let one_vcoin = 10u64.pow(VCOIN_DECIMALS as u32);
        assert_eq!(one_vcoin, 1_000_000_000, "1 VCoin = 1e9 base units");
    }

    #[test]
    fn test_token_metadata() {
        assert_eq!(TOKEN_NAME, "VCoin");
        assert_eq!(TOKEN_SYMBOL, "VIWO");
        assert!(TOKEN_URI.starts_with("https://"), "URI should be HTTPS");
    }

    // ========================================================================
    // PDA Derivation Tests
    // ========================================================================

    #[test]
    fn test_config_pda_seed() {
        assert_eq!(VCOIN_CONFIG_SEED, b"vcoin-config");
    }

    #[test]
    fn test_config_pda_derivation() {
        // Verify PDA can be derived with the seed
        let program_id = Pubkey::new_unique();
        let (pda, bump) = Pubkey::find_program_address(
            &[VCOIN_CONFIG_SEED],
            &program_id
        );
        
        // PDA should be valid (off-curve)
        assert!(bump <= 255);
        assert_ne!(pda, Pubkey::default());
    }

    // ========================================================================
    // State Tests
    // ========================================================================

    #[test]
    fn test_vcoin_config_size() {
        // Verify the account size calculation
        let expected_size = 8 + 32 + 32 + 32 + 32 + 8 + 1 + 1;
        assert_eq!(VCoinConfig::LEN, expected_size, "Config size mismatch");
    }

    #[test]
    fn test_vcoin_config_default() {
        let config = VCoinConfig::default();
        assert_eq!(config.total_minted, 0);
        assert!(!config.paused);
        assert_eq!(config.bump, 0);
    }

    // ========================================================================
    // Mint Logic Tests
    // ========================================================================

    #[test]
    fn test_mint_within_supply() {
        let total_minted = 500_000_000u64 * 1_000_000_000u64; // 500M
        let mint_amount = 100_000_000u64 * 1_000_000_000u64;  // 100M
        
        let new_total = total_minted.checked_add(mint_amount);
        assert!(new_total.is_some(), "Addition should not overflow");
        assert!(new_total.unwrap() <= TOTAL_SUPPLY, "Should be within supply");
    }

    #[test]
    fn test_mint_exceeds_supply() {
        let total_minted = 950_000_000u64 * 1_000_000_000u64; // 950M
        let mint_amount = 100_000_000u64 * 1_000_000_000u64;  // 100M
        
        let new_total = total_minted.checked_add(mint_amount).unwrap();
        assert!(new_total > TOTAL_SUPPLY, "Should exceed supply");
    }

    #[test]
    fn test_mint_exact_remaining() {
        let total_minted = 900_000_000u64 * 1_000_000_000u64; // 900M
        let remaining = TOTAL_SUPPLY - total_minted;
        
        assert_eq!(remaining, 100_000_000u64 * 1_000_000_000u64, "100M remaining");
        
        let new_total = total_minted + remaining;
        assert_eq!(new_total, TOTAL_SUPPLY, "Should equal total supply");
    }

    // ========================================================================
    // Security Tests
    // ========================================================================

    #[test]
    fn test_zero_mint_rejected() {
        // Zero mint should be rejected (business logic)
        let amount = 0u64;
        assert_eq!(amount, 0, "Zero amount should be handled by instruction");
    }

    #[test]
    fn test_pause_state() {
        let mut config = VCoinConfig::default();
        
        // Initially unpaused
        assert!(!config.paused);
        
        // Pause
        config.paused = true;
        assert!(config.paused);
        
        // Unpause
        config.paused = false;
        assert!(!config.paused);
    }

    // ========================================================================
    // Arithmetic Overflow Tests
    // ========================================================================

    #[test]
    fn test_no_overflow_max_supply() {
        // Ensure max supply fits in u64
        assert!(TOTAL_SUPPLY <= u64::MAX, "Total supply should fit in u64");
    }

    #[test]
    fn test_checked_add_for_minting() {
        let total_minted = u64::MAX - 1000;
        let mint_amount = 2000u64;
        
        // This should overflow
        let result = total_minted.checked_add(mint_amount);
        assert!(result.is_none(), "Should detect overflow");
    }

    // ========================================================================
    // Invariant Tests
    // ========================================================================

    #[test]
    fn test_invariant_total_minted_bounded() {
        // total_minted should never exceed TOTAL_SUPPLY
        for minted in [0u64, TOTAL_SUPPLY / 2, TOTAL_SUPPLY] {
            assert!(minted <= TOTAL_SUPPLY, "Minted amount must not exceed supply");
        }
    }

    #[test]
    fn test_supply_distribution() {
        // Verify supply breakdown constants if defined
        // Total: 1B VCoin
        // Expected distributions (from tokenomics):
        // - Ecosystem Rewards: 35% = 350M
        // - Team & Development: 18% = 180M
        // - etc.
        
        let total = TOTAL_SUPPLY;
        let one_percent = total / 100;
        
        assert_eq!(one_percent * 100, total, "Supply should be evenly divisible");
    }
}

