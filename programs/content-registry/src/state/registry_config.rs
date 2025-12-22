use anchor_lang::prelude::*;

/// Global registry configuration
#[account]
#[derive(Default)]
pub struct RegistryConfig {
    /// Admin authority
    pub authority: Pubkey,
    /// Pending authority for two-step transfer (H-02 security fix)
    pub pending_authority: Pubkey,
    /// Identity protocol for verification
    pub identity_program: Pubkey,
    /// Staking program for tier lookup
    pub staking_program: Pubkey,
    /// Total content count
    pub total_content_count: u64,
    /// Total active content
    pub active_content_count: u64,
    /// Whether registry is paused
    pub paused: bool,
    /// PDA bump
    pub bump: u8,
}

impl RegistryConfig {
    pub const LEN: usize = 8 + // discriminator
        32 + // authority
        32 + // pending_authority (NEW - H-02)
        32 + // identity_program
        32 + // staking_program
        8 +  // total_content_count
        8 +  // active_content_count
        1 +  // paused
        1;   // bump
}

