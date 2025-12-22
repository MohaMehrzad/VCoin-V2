use anchor_lang::prelude::*;

/// Per-user activity tracking for rate limiting
#[account]
#[derive(Default)]
pub struct UserActivity {
    /// User wallet
    pub user: Pubkey,
    /// Transfers in current hour
    pub transfers_this_hour: u8,
    /// Hour reset timestamp
    pub hour_reset_time: i64,
    /// Total lifetime transfers sent
    pub total_transfers_sent: u64,
    /// Total lifetime transfers received
    pub total_transfers_received: u64,
    /// Total VCoin sent
    pub total_amount_sent: u64,
    /// Total VCoin received
    pub total_amount_received: u64,
    /// Last transfer timestamp
    pub last_transfer_time: i64,
    /// Activity score contribution (updated by 5A oracle)
    pub activity_score_contribution: u16,
    /// PDA bump
    pub bump: u8,
}

impl UserActivity {
    pub const LEN: usize = 8 + // discriminator
        32 + // user
        1 +  // transfers_this_hour
        8 +  // hour_reset_time
        8 +  // total_transfers_sent
        8 +  // total_transfers_received
        8 +  // total_amount_sent
        8 +  // total_amount_received
        8 +  // last_transfer_time
        2 +  // activity_score_contribution
        1;   // bump
}

