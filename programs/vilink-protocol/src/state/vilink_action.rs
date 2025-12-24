use anchor_lang::prelude::*;

/// Individual action link
#[account]
#[derive(Default)]
pub struct ViLinkAction {
    /// Unique action ID (hash)
    pub action_id: [u8; 32],
    /// Action creator
    pub creator: Pubkey,
    /// Target user (recipient of tip, vouch target, etc.)
    pub target: Pubkey,
    /// Action type
    pub action_type: u8,
    /// Amount (for tips, stakes)
    pub amount: u64,
    /// Optional metadata hash (IPFS CID, etc.)
    pub metadata_hash: [u8; 32],
    /// Creation timestamp
    pub created_at: i64,
    /// Expiry timestamp
    pub expires_at: i64,
    /// Whether action has been executed
    pub executed: bool,
    /// Executor (who executed the action)
    pub executor: Pubkey,
    /// Execution timestamp
    pub executed_at: i64,
    /// Associated content ID (for content reactions)
    pub content_id: Option<[u8; 32]>,
    /// Source dApp
    pub source_dapp: Pubkey,
    /// One-time use?
    pub one_time: bool,
    /// Execution count (for reusable actions)
    pub execution_count: u32,
    /// Max executions (0 = unlimited)
    pub max_executions: u32,
    /// M-04 Security Fix: Nonce used for deterministic PDA derivation
    /// Replaces timestamp-based derivation to prevent collisions
    pub action_nonce: u64,
    /// PDA bump
    pub bump: u8,
}

impl ViLinkAction {
    pub const LEN: usize = 8 + // discriminator
        32 + // action_id
        32 + // creator
        32 + // target
        1 +  // action_type
        8 +  // amount
        32 + // metadata_hash
        8 +  // created_at
        8 +  // expires_at
        1 +  // executed
        32 + // executor
        8 +  // executed_at
        (1 + 32) + // content_id (Option)
        32 + // source_dapp
        1 +  // one_time
        4 +  // execution_count
        4 +  // max_executions
        8 +  // action_nonce (M-04)
        1;   // bump
}

