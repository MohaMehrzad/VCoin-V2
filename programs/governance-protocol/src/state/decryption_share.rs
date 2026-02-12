use anchor_lang::prelude::*;

use crate::constants::DECRYPTION_SHARE_SEED;

/// Stored decryption share from committee member
/// Each committee member computes partial decryptions and proves correctness via DLEQ
#[account]
#[derive(Default)]
pub struct DecryptionShare {
    /// Proposal this share is for
    pub proposal: Pubkey,
    /// Committee member who submitted
    pub committee_member: Pubkey,
    /// Committee index (0-4)
    pub committee_index: u8,
    /// Partial decryption for "for" category: sk_j * R_for_sum
    pub partial_for: [u8; 32],
    /// Partial decryption for "against" category: sk_j * R_against_sum
    pub partial_against: [u8; 32],
    /// Partial decryption for "abstain" category: sk_j * R_abstain_sum
    pub partial_abstain: [u8; 32],
    /// Batched DLEQ proof challenge
    pub dleq_challenge: [u8; 32],
    /// Batched DLEQ proof response
    pub dleq_response: [u8; 32],
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
        32 +  // partial_for
        32 +  // partial_against
        32 +  // partial_abstain
        32 +  // dleq_challenge
        32 +  // dleq_response
        8 +   // submitted_at
        1;    // bump
    // Total: 242 bytes

    pub const SEED: &'static [u8] = DECRYPTION_SHARE_SEED;
}
