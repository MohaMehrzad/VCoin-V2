use anchor_lang::prelude::*;
use super::FeeMethod;

/// Session key for temporary signing
#[account]
#[derive(Default)]
pub struct SessionKey {
    /// User who owns this session
    pub user: Pubkey,
    /// The session key pubkey
    pub session_pubkey: Pubkey,
    /// Allowed action scope bitmap
    pub scope: u16,
    /// Session creation timestamp
    pub created_at: i64,
    /// Session expiry timestamp
    pub expires_at: i64,
    /// Actions executed in this session
    pub actions_used: u32,
    /// Max actions allowed
    pub max_actions: u32,
    /// VCoin spent via this session
    pub vcoin_spent: u64,
    /// Max VCoin spend allowed
    pub max_spend: u64,
    /// Whether session is revoked
    pub is_revoked: bool,
    /// Last action timestamp
    pub last_action_at: i64,
    /// Fee method for this session
    pub fee_method: FeeMethod,
    /// PDA bump
    pub bump: u8,
}

impl SessionKey {
    pub const LEN: usize = 8 + // discriminator
        32 + // user
        32 + // session_pubkey
        2 +  // scope
        8 +  // created_at
        8 +  // expires_at
        4 +  // actions_used
        4 +  // max_actions
        8 +  // vcoin_spent
        8 +  // max_spend
        1 +  // is_revoked
        8 +  // last_action_at
        1 +  // fee_method
        1;   // bump
    
    /// Check if action is in scope
    pub fn is_action_in_scope(&self, action_type: u16) -> bool {
        (self.scope & action_type) != 0
    }
    
    /// Check if session is valid
    pub fn is_valid(&self, current_timestamp: i64) -> bool {
        !self.is_revoked && 
        current_timestamp <= self.expires_at &&
        self.actions_used < self.max_actions
    }
}

