use anchor_lang::prelude::*;

/// Rate limit account per user
#[account]
#[derive(Default)]
pub struct UserRateLimit {
    /// User wallet
    pub user: Pubkey,
    /// Posts today
    pub posts_today: u16,
    /// Edits this hour
    pub edits_this_hour: u8,
    /// Last post time
    pub last_post_time: i64,
    /// Day reset time
    pub day_reset_time: i64,
    /// Hour reset time
    pub hour_reset_time: i64,
    /// PDA bump
    pub bump: u8,
}

impl UserRateLimit {
    pub const LEN: usize = 8 + // discriminator
        32 + // user
        2 +  // posts_today
        1 +  // edits_this_hour
        8 +  // last_post_time
        8 +  // day_reset_time
        8 +  // hour_reset_time
        1;   // bump
    
    /// Get daily cap based on tier
    pub fn daily_cap_for_tier(tier: u8) -> u16 {
        match tier {
            0 => 50,
            1 => 100,
            2 => 200,
            3 => 400,
            4 => 1000,
            _ => 50,
        }
    }
}
