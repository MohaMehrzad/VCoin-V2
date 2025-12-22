use anchor_lang::prelude::*;

/// Global 5A protocol configuration
#[account]
#[derive(Default)]
pub struct FiveAConfig {
    /// Admin authority
    pub authority: Pubkey,
    /// Pending authority for two-step transfer (H-02 security fix)
    pub pending_authority: Pubkey,
    /// Identity protocol program
    pub identity_program: Pubkey,
    /// VCoin mint for vouch stakes
    pub vcoin_mint: Pubkey,
    /// Vouch stake vault
    pub vouch_vault: Pubkey,
    /// Registered oracles (max 10)
    pub oracles: [Pubkey; 10],
    /// Number of active oracles
    pub oracle_count: u8,
    /// Required consensus (e.g., 5 of 7) - H-05: Used for oracle consensus
    pub required_consensus: u8,
    /// Total users with scores
    pub total_users: u64,
    /// Current snapshot epoch
    pub current_epoch: u64,
    /// Last snapshot timestamp
    pub last_snapshot_time: i64,
    /// Whether protocol is paused
    pub paused: bool,
    /// PDA bump
    pub bump: u8,
}

impl FiveAConfig {
    pub const LEN: usize = 8 + // discriminator
        32 + // authority
        32 + // pending_authority (NEW - H-02)
        32 + // identity_program
        32 + // vcoin_mint
        32 + // vouch_vault
        (32 * 10) + // oracles
        1 +  // oracle_count
        1 +  // required_consensus
        8 +  // total_users
        8 +  // current_epoch
        8 +  // last_snapshot_time
        1 +  // paused
        1;   // bump
}

