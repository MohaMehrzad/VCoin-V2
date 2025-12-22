use anchor_lang::prelude::*;

/// Individual transfer record for audit trail
#[account]
#[derive(Default)]
pub struct TransferRecord {
    /// Transfer ID (sequential)
    pub transfer_id: u64,
    /// Sender
    pub sender: Pubkey,
    /// Receiver
    pub receiver: Pubkey,
    /// Amount transferred
    pub amount: u64,
    /// Timestamp
    pub timestamp: i64,
    /// Whether this was flagged as wash trading
    pub wash_trading_flag: bool,
    /// Whether this was a tip transaction
    pub is_tip: bool,
    /// PDA bump
    pub bump: u8,
}

impl TransferRecord {
    pub const LEN: usize = 8 + // discriminator
        8 +  // transfer_id
        32 + // sender
        32 + // receiver
        8 +  // amount
        8 +  // timestamp
        1 +  // wash_trading_flag
        1 +  // is_tip
        1;   // bump
}

