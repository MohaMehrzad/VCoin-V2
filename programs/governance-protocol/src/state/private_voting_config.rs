use anchor_lang::prelude::*;

/// ZK Private voting configuration (per proposal)
#[account]
#[derive(Default)]
pub struct PrivateVotingConfig {
    /// Proposal
    pub proposal: Pubkey,
    /// Whether private voting is enabled
    pub is_enabled: bool,
    /// Threshold encryption public key
    pub encryption_pubkey: Pubkey,
    /// Decryption threshold (e.g., 3-of-5)
    pub decryption_threshold: u8,
    /// Decryption committee (max 5)
    pub decryption_committee: [Pubkey; 5],
    /// Committee size
    pub committee_size: u8,
    /// Decryption shares received
    pub shares_received: u8,
    /// Whether reveal has started
    pub reveal_started: bool,
    /// Whether reveal is complete
    pub reveal_completed: bool,
    /// Aggregated votes for (revealed)
    pub aggregated_for: u128,
    /// Aggregated votes against
    pub aggregated_against: u128,
    /// Aggregated abstain
    pub aggregated_abstain: u128,
    /// PDA bump
    pub bump: u8,
}

impl PrivateVotingConfig {
    pub const LEN: usize = 8 + // discriminator
        32 + // proposal
        1 +  // is_enabled
        32 + // encryption_pubkey
        1 +  // decryption_threshold
        (32 * 5) + // decryption_committee
        1 +  // committee_size
        1 +  // shares_received
        1 +  // reveal_started
        1 +  // reveal_completed
        16 + // aggregated_for
        16 + // aggregated_against
        16 + // aggregated_abstain
        1;   // bump
}

