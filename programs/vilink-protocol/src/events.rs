use anchor_lang::prelude::*;

#[event]
pub struct ViLinkConfigInitialized {
    pub authority: Pubkey,
    pub vcoin_mint: Pubkey,
    pub enabled_actions: u8,
}

#[event]
pub struct ActionCreated {
    pub action_id: [u8; 32],
    pub creator: Pubkey,
    pub target: Pubkey,
    pub action_type: u8,
    pub amount: u64,
    pub expires_at: i64,
}

#[event]
pub struct ActionExecuted {
    pub action_id: [u8; 32],
    pub executor: Pubkey,
    pub target: Pubkey,
    pub action_type: u8,
    pub amount: u64,
    pub fee_paid: u64,
}

#[event]
pub struct DAppRegistered {
    pub dapp_id: [u8; 32],
    pub authority: Pubkey,
    pub allowed_actions: u8,
}

#[event]
pub struct BatchCreated {
    pub batch_id: [u8; 32],
    pub creator: Pubkey,
    pub action_count: u8,
}

