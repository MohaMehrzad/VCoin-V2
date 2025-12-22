use anchor_lang::prelude::*;

/// Tracks transfer patterns between specific pairs for wash trading detection
#[account]
#[derive(Default)]
pub struct PairTracking {
    /// Sender wallet
    pub sender: Pubkey,
    /// Receiver wallet
    pub receiver: Pubkey,
    /// Last transfer timestamp
    pub last_transfer_time: i64,
    /// Transfer count in last 24 hours
    pub transfers_24h: u16,
    /// Day reset timestamp
    pub day_reset_time: i64,
    /// Total amount transferred in 24h
    pub amount_24h: u64,
    /// Wash trading flag count
    pub wash_flags: u16,
    /// Engagement trust score (0-10000)
    pub trust_score: u16,
    /// PDA bump
    pub bump: u8,
}

impl PairTracking {
    pub const LEN: usize = 8 + // discriminator
        32 + // sender
        32 + // receiver
        8 +  // last_transfer_time
        2 +  // transfers_24h
        8 +  // day_reset_time
        8 +  // amount_24h
        2 +  // wash_flags
        2 +  // trust_score
        1;   // bump
}

