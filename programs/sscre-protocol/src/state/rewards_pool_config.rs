use anchor_lang::prelude::*;

/// Global rewards pool configuration
#[account]
#[derive(Default)]
pub struct RewardsPoolConfig {
    /// Admin authority
    pub authority: Pubkey,
    /// VCoin mint
    pub vcoin_mint: Pubkey,
    /// Pool vault holding VCoin rewards
    pub pool_vault: Pubkey,
    /// 5A Protocol for score verification
    pub five_a_program: Pubkey,
    /// Registered oracles (max 5)
    pub oracles: [Pubkey; 5],
    /// Number of active oracles
    pub oracle_count: u8,
    /// Current epoch number
    pub current_epoch: u64,
    /// Total VCoin distributed all-time
    pub total_distributed: u64,
    /// Remaining primary reserves
    pub remaining_reserves: u64,
    /// Whether protocol is paused
    pub paused: bool,
    /// Whether circuit breaker is active
    pub circuit_breaker_active: bool,
    /// Fee recipient for gasless fee
    pub fee_recipient: Pubkey,
    /// PDA bump
    pub bump: u8,
    /// Vault bump
    pub vault_bump: u8,
}

impl RewardsPoolConfig {
    pub const LEN: usize = 8 + // discriminator
        32 + // authority
        32 + // vcoin_mint
        32 + // pool_vault
        32 + // five_a_program
        (32 * 5) + // oracles
        1 +  // oracle_count
        8 +  // current_epoch
        8 +  // total_distributed
        8 +  // remaining_reserves
        1 +  // paused
        1 +  // circuit_breaker_active
        32 + // fee_recipient
        1 +  // bump
        1;   // vault_bump
}

