use anchor_lang::prelude::*;

#[event]
pub struct TransferProcessed {
    pub sender: Pubkey,
    pub receiver: Pubkey,
    pub amount: u64,
    pub timestamp: i64,
    pub is_tip: bool,
}

#[event]
pub struct WashTradingDetected {
    pub sender: Pubkey,
    pub receiver: Pubkey,
    pub amount: u64,
    pub timestamp: i64,
    pub pair_transfers_24h: u16,
}

#[event]
pub struct ActivityScoreUpdated {
    pub user: Pubkey,
    pub new_contribution: u16,
    pub transfers_this_hour: u8,
}

