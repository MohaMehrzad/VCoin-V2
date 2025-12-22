use anchor_lang::prelude::*;

/// Global gasless configuration
#[account]
#[derive(Default)]
pub struct GaslessConfig {
    /// Admin authority
    pub authority: Pubkey,
    /// Fee payer wallet (paymaster)
    pub fee_payer: Pubkey,
    /// VCoin mint
    pub vcoin_mint: Pubkey,
    /// Fee vault for VCoin fees
    pub fee_vault: Pubkey,
    /// SSCRE program for reward deduction
    pub sscre_program: Pubkey,
    /// Daily subsidy budget (SOL lamports)
    pub daily_subsidy_budget: u64,
    /// SOL fee per transaction (lamports)
    pub sol_fee_per_tx: u64,
    /// VCoin fee multiplier
    pub vcoin_fee_multiplier: u64,
    /// SSCRE deduction rate (bps)
    pub sscre_deduction_bps: u16,
    /// Max subsidized tx per user per day
    pub max_subsidized_per_user: u32,
    /// Total transactions subsidized
    pub total_subsidized_tx: u64,
    /// Total SOL spent on subsidies
    pub total_sol_spent: u64,
    /// Total VCoin collected as fees
    pub total_vcoin_collected: u64,
    /// Whether protocol is paused
    pub paused: bool,
    /// Current day (for daily budget reset)
    pub current_day: u32,
    /// Day's spent budget
    pub day_spent: u64,
    /// PDA bump
    pub bump: u8,
}

impl GaslessConfig {
    pub const LEN: usize = 8 + // discriminator
        32 + // authority
        32 + // fee_payer
        32 + // vcoin_mint
        32 + // fee_vault
        32 + // sscre_program
        8 +  // daily_subsidy_budget
        8 +  // sol_fee_per_tx
        8 +  // vcoin_fee_multiplier
        2 +  // sscre_deduction_bps
        4 +  // max_subsidized_per_user
        8 +  // total_subsidized_tx
        8 +  // total_sol_spent
        8 +  // total_vcoin_collected
        1 +  // paused
        4 +  // current_day
        8 +  // day_spent
        1;   // bump
    
    /// Get current day number
    pub fn get_day_number(timestamp: i64) -> u32 {
        (timestamp / 86400) as u32
    }
    
    /// Check if daily budget reset needed
    pub fn should_reset_daily_budget(&self, current_timestamp: i64) -> bool {
        Self::get_day_number(current_timestamp) > self.current_day
    }
}

