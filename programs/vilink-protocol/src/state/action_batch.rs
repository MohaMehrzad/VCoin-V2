use anchor_lang::prelude::*;

/// Batch action container
#[account]
#[derive(Default)]
pub struct ActionBatch {
    /// Batch ID
    pub batch_id: [u8; 32],
    /// Creator
    pub creator: Pubkey,
    /// Action IDs in this batch
    pub action_ids: Vec<[u8; 32]>,
    /// Batch created timestamp
    pub created_at: i64,
    /// Total actions in batch
    pub total_actions: u8,
    /// Executed actions count
    pub executed_count: u8,
    /// PDA bump
    pub bump: u8,
}

impl ActionBatch {
    pub const LEN: usize = 8 + // discriminator
        32 + // batch_id
        32 + // creator
        4 + (32 * 10) + // action_ids (Vec with max 10)
        8 +  // created_at
        1 +  // total_actions
        1 +  // executed_count
        1;   // bump
}

