use anchor_lang::prelude::*;

/// Registered external dApp
#[account]
#[derive(Default)]
pub struct RegisteredDApp {
    /// dApp identifier (domain hash or pubkey)
    pub dapp_id: [u8; 32],
    /// dApp name
    pub name: [u8; 32],
    /// dApp authority
    pub authority: Pubkey,
    /// dApp webhook URL hash
    pub webhook_hash: [u8; 32],
    /// Whether dApp is active
    pub is_active: bool,
    /// Registration timestamp
    pub registered_at: i64,
    /// Total actions from this dApp
    pub action_count: u64,
    /// Allowed action types bitmap
    pub allowed_actions: u8,
    /// Fee share (for affiliate model)
    pub fee_share_bps: u16,
    /// PDA bump
    pub bump: u8,
}

impl RegisteredDApp {
    pub const LEN: usize = 8 + // discriminator
        32 + // dapp_id
        32 + // name
        32 + // authority
        32 + // webhook_hash
        1 +  // is_active
        8 +  // registered_at
        8 +  // action_count
        1 +  // allowed_actions
        2 +  // fee_share_bps
        1;   // bump
}

