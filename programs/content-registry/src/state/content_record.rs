use anchor_lang::prelude::*;

/// Individual content record
#[account]
pub struct ContentRecord {
    /// Unique tracking ID (hash)
    pub tracking_id: [u8; 32],
    /// Content author
    pub author: Pubkey,
    /// SHA256 of current content
    pub content_hash: [u8; 32],
    /// IPFS/Arweave CID (max 128)
    pub content_uri: [u8; 128],
    /// URI length
    pub uri_len: u8,
    /// Content type
    pub content_type: u8,
    /// Current state
    pub state: u8,
    /// Version (edit count)
    pub version: u16,
    /// Original creation timestamp
    pub created_at: i64,
    /// Last state change
    pub updated_at: i64,
    /// Hash before last edit (for history)
    pub previous_hash: [u8; 32],
    /// Energy spent on creation
    pub energy_spent: u16,
    /// Whether energy refund was claimed
    pub refund_claimed: bool,
    /// Engagement count (likes) for refund calculation
    pub engagement_count: u32,
    /// PDA bump
    pub bump: u8,
}

impl Default for ContentRecord {
    fn default() -> Self {
        Self {
            tracking_id: [0u8; 32],
            author: Pubkey::default(),
            content_hash: [0u8; 32],
            content_uri: [0u8; 128],
            uri_len: 0,
            content_type: 0,
            state: 0,
            version: 0,
            created_at: 0,
            updated_at: 0,
            previous_hash: [0u8; 32],
            energy_spent: 0,
            refund_claimed: false,
            engagement_count: 0,
            bump: 0,
        }
    }
}

impl ContentRecord {
    pub const LEN: usize = 8 + // discriminator
        32 + // tracking_id
        32 + // author
        32 + // content_hash
        128 + // content_uri
        1 +  // uri_len
        1 +  // content_type
        1 +  // state
        2 +  // version
        8 +  // created_at
        8 +  // updated_at
        32 + // previous_hash
        2 +  // energy_spent
        1 +  // refund_claimed
        4 +  // engagement_count
        1;   // bump
}
