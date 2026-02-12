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
    /// ElGamal ciphertext for "for" vote: R (32 bytes) || C (32 bytes)
    pub ct_for: [u8; 64],
    /// ElGamal ciphertext for "against" vote: R (32 bytes) || C (32 bytes)
    pub ct_against: [u8; 64],
    /// ElGamal ciphertext for "abstain" vote: R (32 bytes) || C (32 bytes)
    pub ct_abstain: [u8; 64],
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
            ct_for: [0u8; 64],
            ct_against: [0u8; 64],
            ct_abstain: [0u8; 64],
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
        64 + // ct_for
        64 + // ct_against
        64 + // ct_abstain
        1 +  // revealed
        1;   // bump
    // Total: 284 bytes (same as before: 32 + 32 + 128 = 192 = 64 + 64 + 64)
}
