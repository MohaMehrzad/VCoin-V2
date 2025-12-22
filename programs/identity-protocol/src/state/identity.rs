use anchor_lang::prelude::*;

/// Individual user identity (on-chain DID anchor)
#[account]
#[derive(Default)]
pub struct Identity {
    /// Owner wallet
    pub owner: Pubkey,
    /// SHA256 hash of full DID document (stored off-chain)
    pub did_hash: [u8; 32],
    /// Current verification level
    pub verification_level: u8,
    /// Hash of verification proof
    pub verification_hash: [u8; 32],
    /// Username (max 32 chars)
    pub username: [u8; 32],
    /// Username length
    pub username_len: u8,
    /// Account creation timestamp
    pub created_at: i64,
    /// Last update timestamp
    pub updated_at: i64,
    /// Whether identity is active
    pub is_active: bool,
    /// PDA bump
    pub bump: u8,
}

impl Identity {
    pub const LEN: usize = 8 + // discriminator
        32 + // owner
        32 + // did_hash
        1 +  // verification_level
        32 + // verification_hash
        32 + // username
        1 +  // username_len
        8 +  // created_at
        8 +  // updated_at
        1 +  // is_active
        1;   // bump
}

