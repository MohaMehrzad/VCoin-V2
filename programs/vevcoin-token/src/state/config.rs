use anchor_lang::prelude::*;

/// veVCoin Configuration Account (Singleton PDA)
#[account]
#[derive(Default)]
pub struct VeVCoinConfig {
    /// The admin authority (can update staking protocol address)
    pub authority: Pubkey,
    /// Pending authority for two-step transfer (H-02 security fix)
    pub pending_authority: Pubkey,
    /// The veVCoin mint address
    pub mint: Pubkey,
    /// The authorized staking protocol that can mint/burn
    pub staking_protocol: Pubkey,
    /// Total veVCoin currently in circulation
    pub total_supply: u64,
    /// Total unique holders
    pub total_holders: u64,
    /// Bump seed for PDA
    pub bump: u8,
}

impl VeVCoinConfig {
    pub const LEN: usize = 8 + // discriminator
        32 + // authority
        32 + // pending_authority (NEW - H-02)
        32 + // mint
        32 + // staking_protocol
        8 +  // total_supply
        8 +  // total_holders
        1;   // bump
}

