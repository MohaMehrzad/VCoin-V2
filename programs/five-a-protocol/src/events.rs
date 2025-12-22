use anchor_lang::prelude::*;

#[event]
pub struct ScoreUpdated {
    pub user: Pubkey,
    pub authenticity: u16,
    pub accuracy: u16,
    pub agility: u16,
    pub activity: u16,
    pub approved: u16,
    pub composite: u16,
    pub timestamp: i64,
}

#[event]
pub struct SnapshotCreated {
    pub epoch: u64,
    pub merkle_root: [u8; 32],
    pub user_count: u64,
    pub timestamp: i64,
}

#[event]
pub struct VouchCreated {
    pub voucher: Pubkey,
    pub vouchee: Pubkey,
    pub stake: u64,
    pub timestamp: i64,
}

#[event]
pub struct VouchEvaluated {
    pub voucher: Pubkey,
    pub vouchee: Pubkey,
    pub success: bool,
    pub reward_or_slash: u64,
}

