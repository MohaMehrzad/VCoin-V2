use anchor_lang::prelude::*;

use crate::constants::DECRYPTION_SHARE_SEED;

/// Stored decryption share from committee member (C-02 fix)
/// Each committee member's share is stored on-chain for verification
#[account]
#[derive(Default)]
pub struct DecryptionShare {
    /// Proposal this share is for
    pub proposal: Pubkey,
    /// Committee member who submitted
    pub committee_member: Pubkey,
    /// Committee index (0-4)
    pub committee_index: u8,
    /// The actual decryption share (32 bytes)
    pub share: [u8; 32],
    /// Submission timestamp
    pub submitted_at: i64,
    /// PDA bump
    pub bump: u8,
}

impl DecryptionShare {
    pub const LEN: usize = 8 +   // discriminator
        32 +  // proposal
        32 +  // committee_member
        1 +   // committee_index
        32 +  // share
        8 +   // submitted_at
        1;    // bump
    
    pub const SEED: &'static [u8] = DECRYPTION_SHARE_SEED;
}

