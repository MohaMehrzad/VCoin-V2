use anchor_lang::prelude::*;

/// Epoch distribution account
#[account]
#[derive(Default)]
pub struct EpochDistribution {
    /// Epoch number
    pub epoch: u64,
    /// Merkle root of all user allocations
    pub merkle_root: [u8; 32],
    /// Total VCoin allocated for this epoch
    pub total_allocation: u64,
    /// Total VCoin claimed so far
    pub total_claimed: u64,
    /// Number of users who claimed
    pub claims_count: u64,
    /// Epoch start timestamp
    pub start_time: i64,
    /// Epoch end timestamp
    pub end_time: i64,
    /// Claim window expiry
    pub claim_expiry: i64,
    /// Whether epoch is finalized (merkle root set)
    pub is_finalized: bool,
    /// Oracle that submitted the merkle root
    pub submitter: Pubkey,
    /// Average 5A score for this epoch
    pub avg_five_a_score: u16,
    /// Total eligible users
    pub eligible_users: u64,
    /// PDA bump
    pub bump: u8,
}

impl EpochDistribution {
    pub const LEN: usize = 8 + // discriminator
        8 +  // epoch
        32 + // merkle_root
        8 +  // total_allocation
        8 +  // total_claimed
        8 +  // claims_count
        8 +  // start_time
        8 +  // end_time
        8 +  // claim_expiry
        1 +  // is_finalized
        32 + // submitter
        2 +  // avg_five_a_score
        8 +  // eligible_users
        1;   // bump
}

