use anchor_lang::prelude::*;
use super::GaslessConfig;

/// User gasless statistics
#[account]
#[derive(Default)]
pub struct UserGaslessStats {
    /// User wallet
    pub user: Pubkey,
    /// Total gasless transactions
    pub total_gasless_tx: u64,
    /// Total subsidized transactions
    pub total_subsidized: u64,
    /// Total VCoin paid as fees
    pub total_vcoin_fees: u64,
    /// Total SSCRE deductions
    pub total_sscre_deductions: u64,
    /// Sessions created
    pub sessions_created: u32,
    /// Active session (if any)
    pub active_session: Pubkey,
    /// Current day for daily limits
    pub current_day: u32,
    /// Today's subsidized tx count
    pub today_subsidized: u32,
    /// First gasless tx timestamp
    pub first_gasless_at: i64,
    /// Last gasless tx timestamp
    pub last_gasless_at: i64,
    /// PDA bump
    pub bump: u8,
}

impl UserGaslessStats {
    pub const LEN: usize = 8 + // discriminator
        32 + // user
        8 +  // total_gasless_tx
        8 +  // total_subsidized
        8 +  // total_vcoin_fees
        8 +  // total_sscre_deductions
        4 +  // sessions_created
        32 + // active_session
        4 +  // current_day
        4 +  // today_subsidized
        8 +  // first_gasless_at
        8 +  // last_gasless_at
        1;   // bump
    
    /// Reset daily limits if new day
    pub fn check_daily_reset(&mut self, current_timestamp: i64) {
        let day = GaslessConfig::get_day_number(current_timestamp);
        if day > self.current_day {
            self.current_day = day;
            self.today_subsidized = 0;
        }
    }
}

