use anchor_lang::prelude::*;

#[event]
pub struct ContentCreated {
    pub tracking_id: [u8; 32],
    pub author: Pubkey,
    pub content_type: u8,
    pub content_hash: [u8; 32],
    pub timestamp: i64,
}

#[event]
pub struct ContentEdited {
    pub tracking_id: [u8; 32],
    pub author: Pubkey,
    pub version: u16,
    pub new_hash: [u8; 32],
    pub timestamp: i64,
}

#[event]
pub struct ContentDeleted {
    pub tracking_id: [u8; 32],
    pub author: Pubkey,
    pub timestamp: i64,
}

#[event]
pub struct EnergySpent {
    pub user: Pubkey,
    pub amount: u16,
    pub action: String,
    pub remaining: u16,
}

#[event]
pub struct EnergyRefunded {
    pub user: Pubkey,
    pub content_id: [u8; 32],
    pub refund_amount: u16,
    pub engagement_count: u32,
}
