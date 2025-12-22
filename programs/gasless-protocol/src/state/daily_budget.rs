use anchor_lang::prelude::*;

/// Daily budget tracking
#[account]
#[derive(Default)]
pub struct DailyBudget {
    /// Day number
    pub day: u32,
    /// Total budget allocated
    pub total_budget: u64,
    /// Amount spent
    pub spent: u64,
    /// Transactions subsidized
    pub tx_count: u64,
    /// Unique users subsidized
    pub unique_users: u32,
    /// PDA bump
    pub bump: u8,
}

impl DailyBudget {
    pub const LEN: usize = 8 + // discriminator
        4 +  // day
        8 +  // total_budget
        8 +  // spent
        8 +  // tx_count
        4 +  // unique_users
        1;   // bump
}

