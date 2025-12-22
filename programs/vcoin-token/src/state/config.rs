use anchor_lang::prelude::*;

/// VCoin Configuration Account (Singleton PDA)
/// Stores global configuration for the VCoin token
#[account]
#[derive(Default)]
pub struct VCoinConfig {
    /// The authority that can mint tokens and update config
    pub authority: Pubkey,
    /// Pending authority for two-step transfer (H-02 security fix)
    pub pending_authority: Pubkey,
    /// The VCoin mint address
    pub mint: Pubkey,
    /// The treasury token account that receives initial minted tokens
    pub treasury: Pubkey,
    /// The permanent delegate for slashing (governance multisig)
    pub permanent_delegate: Pubkey,
    /// Total tokens minted so far
    pub total_minted: u64,
    /// Whether token operations are paused
    pub paused: bool,
    /// Bump seed for PDA
    pub bump: u8,
}

impl VCoinConfig {
    pub const LEN: usize = 8 + // discriminator
        32 + // authority
        32 + // pending_authority (NEW - H-02)
        32 + // mint
        32 + // treasury
        32 + // permanent_delegate
        8 +  // total_minted
        1 +  // paused
        1;   // bump
}

