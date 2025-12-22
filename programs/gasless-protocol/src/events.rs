use anchor_lang::prelude::*;

#[event]
pub struct GaslessConfigInitialized {
    pub authority: Pubkey,
    pub fee_payer: Pubkey,
    pub daily_budget: u64,
}

#[event]
pub struct SessionKeyCreated {
    pub user: Pubkey,
    pub session_pubkey: Pubkey,
    pub scope: u16,
    pub expires_at: i64,
    pub fee_method: u8,
}

#[event]
pub struct SessionKeyRevoked {
    pub user: Pubkey,
    pub session_pubkey: Pubkey,
    pub actions_used: u32,
}

#[event]
pub struct SessionActionExecuted {
    pub user: Pubkey,
    pub session_pubkey: Pubkey,
    pub action_type: u16,
    pub fee_method: u8,
    pub fee_amount: u64,
}

#[event]
pub struct DailyBudgetReset {
    pub day: u32,
    pub previous_spent: u64,
    pub new_budget: u64,
}

#[event]
pub struct FeeCollected {
    pub user: Pubkey,
    pub fee_method: u8,
    pub amount: u64,
    pub is_vcoin: bool,
}

