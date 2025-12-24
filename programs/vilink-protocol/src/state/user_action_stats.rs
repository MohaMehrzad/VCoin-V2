use anchor_lang::prelude::*;

/// User action statistics
#[account]
#[derive(Default)]
pub struct UserActionStats {
    /// User wallet
    pub user: Pubkey,
    /// Total actions created
    pub actions_created: u64,
    /// Total actions executed
    pub actions_executed: u64,
    /// Total tips sent
    pub tips_sent: u64,
    /// Total tips received
    pub tips_received: u64,
    /// Total VCoin sent via tips
    pub vcoin_sent: u64,
    /// Total VCoin received via tips
    pub vcoin_received: u64,
    /// Total vouches given via actions
    pub vouches_given: u64,
    /// Total follows via actions
    pub follows_given: u64,
    /// First action timestamp
    pub first_action_at: i64,
    /// Last action timestamp
    pub last_action_at: i64,
    /// M-04 Security Fix: Nonce for deterministic action PDA derivation
    /// Incremented each time a new action is created
    pub action_nonce: u64,
    /// PDA bump
    pub bump: u8,
}

impl UserActionStats {
    pub const LEN: usize = 8 + // discriminator
        32 + // user
        8 +  // actions_created
        8 +  // actions_executed
        8 +  // tips_sent
        8 +  // tips_received
        8 +  // vcoin_sent
        8 +  // vcoin_received
        8 +  // vouches_given
        8 +  // follows_given
        8 +  // first_action_at
        8 +  // last_action_at
        8 +  // action_nonce (M-04)
        1;   // bump
}

