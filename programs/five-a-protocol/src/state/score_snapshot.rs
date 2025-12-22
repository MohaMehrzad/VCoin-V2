use anchor_lang::prelude::*;

/// Periodic score snapshot for epoch
#[account]
#[derive(Default)]
pub struct ScoreSnapshot {
    /// Epoch number
    pub epoch: u64,
    /// Merkle root of all scores in this epoch
    pub merkle_root: [u8; 32],
    /// Total users in snapshot
    pub user_count: u64,
    /// Average composite score
    pub avg_score: u16,
    /// Snapshot timestamp
    pub timestamp: i64,
    /// Oracle that submitted snapshot
    pub submitter: Pubkey,
    /// PDA bump
    pub bump: u8,
}

impl ScoreSnapshot {
    pub const LEN: usize = 8 + // discriminator
        8 +  // epoch
        32 + // merkle_root
        8 +  // user_count
        2 +  // avg_score
        8 +  // timestamp
        32 + // submitter
        1;   // bump
}

