use anchor_lang::prelude::*;

/// Registered oracle
#[account]
#[derive(Default)]
pub struct Oracle {
    /// Oracle wallet
    pub wallet: Pubkey,
    /// Oracle name (max 32 chars)
    pub name: [u8; 32],
    /// Whether oracle is active
    pub is_active: bool,
    /// Total score submissions
    pub total_submissions: u64,
    /// Accuracy rate (0-10000)
    pub accuracy_rate: u16,
    /// Last submission timestamp
    pub last_submission: i64,
    /// PDA bump
    pub bump: u8,
}

impl Oracle {
    pub const LEN: usize = 8 + // discriminator
        32 + // wallet
        32 + // name
        1 +  // is_active
        8 +  // total_submissions
        2 +  // accuracy_rate
        8 +  // last_submission
        1;   // bump
}

