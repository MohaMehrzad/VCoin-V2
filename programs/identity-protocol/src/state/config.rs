use anchor_lang::prelude::*;

/// Global identity protocol configuration
#[account]
#[derive(Default)]
pub struct IdentityConfig {
    /// Admin authority
    pub authority: Pubkey,
    /// SAS program ID (Solana Attestation Service)
    pub sas_program: Pubkey,
    /// USDC mint for subscriptions
    pub usdc_mint: Pubkey,
    /// Treasury for subscription payments
    pub treasury: Pubkey,
    /// Trusted attesters (max 10)
    pub trusted_attesters: [Pubkey; 10],
    /// Number of active trusted attesters
    pub attester_count: u8,
    /// Total registered identities
    pub total_identities: u64,
    /// Total verified identities (Level 1+)
    pub verified_identities: u64,
    /// Whether protocol is paused
    pub paused: bool,
    /// PDA bump
    pub bump: u8,
}

impl IdentityConfig {
    pub const LEN: usize = 8 + // discriminator
        32 + // authority
        32 + // sas_program
        32 + // usdc_mint
        32 + // treasury
        (32 * 10) + // trusted_attesters
        1 +  // attester_count
        8 +  // total_identities
        8 +  // verified_identities
        1 +  // paused
        1;   // bump
}

