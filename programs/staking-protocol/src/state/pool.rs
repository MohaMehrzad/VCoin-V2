use anchor_lang::prelude::*;

/// Staking Pool Account (Singleton PDA)
#[account]
#[derive(Default)]
pub struct StakingPool {
    /// Admin authority
    pub authority: Pubkey,
    /// Pending authority for two-step transfer (H-02 security fix)
    pub pending_authority: Pubkey,
    /// H-NEW-01: Timestamp when pending authority was proposed (for timelock)
    pub pending_authority_activated_at: i64,
    /// VCoin mint address
    pub vcoin_mint: Pubkey,
    /// veVCoin mint address
    pub vevcoin_mint: Pubkey,
    /// veVCoin program address
    pub vevcoin_program: Pubkey,
    /// Pool vault for staked VCoin
    pub pool_vault: Pubkey,
    /// Total VCoin staked in the pool
    pub total_staked: u64,
    /// Total number of stakers
    pub total_stakers: u64,
    /// Whether the pool is paused
    pub paused: bool,
    /// Bump seed for PDA
    pub bump: u8,
    /// Vault bump seed
    pub vault_bump: u8,
    /// M-01 Security Fix: Reentrancy guard for CPI protection
    /// Set to true during stake/unstake operations to prevent reentrancy
    pub reentrancy_guard: bool,
}

impl StakingPool {
    pub const LEN: usize = 8 + // discriminator
        32 + // authority
        32 + // pending_authority (H-02)
        8 +  // pending_authority_activated_at (H-NEW-01)
        32 + // vcoin_mint
        32 + // vevcoin_mint
        32 + // vevcoin_program
        32 + // pool_vault
        8 +  // total_staked
        8 +  // total_stakers
        1 +  // paused
        1 +  // bump
        1 +  // vault_bump
        1;   // reentrancy_guard (M-01)
}

