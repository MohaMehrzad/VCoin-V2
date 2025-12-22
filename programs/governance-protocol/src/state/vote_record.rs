use anchor_lang::prelude::*;

/// Vote record (PDA per user per proposal)
#[account]
pub struct VoteRecord {
    /// Voter
    pub voter: Pubkey,
    /// Proposal
    pub proposal: Pubkey,
    /// Vote weight (veVCoin * 5A boost)
    pub vote_weight: u64,
    /// Vote choice
    pub vote_choice: u8,
    /// Timestamp
    pub voted_at: i64,
    /// Whether this is a ZK encrypted vote
    pub is_private: bool,
    /// Encrypted choice (for ZK voting)
    pub encrypted_choice: [u8; 32],
    /// Encrypted weight (for ZK voting)
    pub encrypted_weight: [u8; 32],
    /// ZK proof
    pub zk_proof: [u8; 128],
    /// Whether vote has been revealed
    pub revealed: bool,
    /// PDA bump
    pub bump: u8,
}

impl Default for VoteRecord {
    fn default() -> Self {
        Self {
            voter: Pubkey::default(),
            proposal: Pubkey::default(),
            vote_weight: 0,
            vote_choice: 0,
            voted_at: 0,
            is_private: false,
            encrypted_choice: [0u8; 32],
            encrypted_weight: [0u8; 32],
            zk_proof: [0u8; 128],
            revealed: false,
            bump: 0,
        }
    }
}

impl VoteRecord {
    pub const LEN: usize = 8 + // discriminator
        32 + // voter
        32 + // proposal
        8 +  // vote_weight
        1 +  // vote_choice
        8 +  // voted_at
        1 +  // is_private
        32 + // encrypted_choice
        32 + // encrypted_weight
        128 + // zk_proof
        1 +  // revealed
        1;   // bump
}

