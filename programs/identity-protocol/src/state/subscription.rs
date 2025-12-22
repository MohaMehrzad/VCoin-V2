use anchor_lang::prelude::*;

/// User subscription account
#[account]
#[derive(Default)]
pub struct Subscription {
    /// User wallet
    pub user: Pubkey,
    /// Current subscription tier
    pub tier: u8,
    /// Subscription start timestamp
    pub started_at: i64,
    /// Subscription expiry timestamp
    pub expires_at: i64,
    /// Auto-renew enabled
    pub auto_renew: bool,
    /// Total payments made (USDC)
    pub total_paid: u64,
    /// PDA bump
    pub bump: u8,
}

impl Subscription {
    pub const LEN: usize = 8 + // discriminator
        32 + // user
        1 +  // tier
        8 +  // started_at
        8 +  // expires_at
        1 +  // auto_renew
        8 +  // total_paid
        1;   // bump
}

