use anchor_lang::prelude::*;

#[event]
pub struct PoolInitialized {
    pub authority: Pubkey,
    pub vcoin_mint: Pubkey,
    pub initial_reserves: u64,
}

#[event]
pub struct EpochStarted {
    pub epoch: u64,
    pub start_time: i64,
    pub end_time: i64,
}

#[event]
pub struct EpochFinalized {
    pub epoch: u64,
    pub merkle_root: [u8; 32],
    pub total_allocation: u64,
    pub eligible_users: u64,
}

#[event]
pub struct RewardsClaimed {
    pub user: Pubkey,
    pub epoch: u64,
    pub gross_amount: u64,
    pub fee_deducted: u64,
    pub net_amount: u64,
}

#[event]
pub struct CircuitBreakerTriggered {
    pub reason: u8,
    pub value: u64,
    pub threshold: u64,
    pub timestamp: i64,
}

#[event]
pub struct FundingLayerSwitch {
    pub from_layer: u8,
    pub to_layer: u8,
    pub reason: String,
    pub timestamp: i64,
}

