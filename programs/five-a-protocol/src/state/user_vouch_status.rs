use anchor_lang::prelude::*;

/// User's vouch status (how many vouches received)
#[account]
#[derive(Default)]
pub struct UserVouchStatus {
    /// User wallet
    pub user: Pubkey,
    /// Number of vouches received (0-3)
    pub vouches_received: u8,
    /// Who vouched (max 3)
    pub vouchers: [Pubkey; 3],
    /// Timestamp when 3 vouches received
    pub vouch_completed_at: i64,
    /// Reward multiplier (0-10000 = 0-100%)
    pub reward_multiplier: u16,
    /// Whether fully vouched
    pub is_fully_vouched: bool,
    /// PDA bump
    pub bump: u8,
}

impl UserVouchStatus {
    pub const LEN: usize = 8 + // discriminator
        32 + // user
        1 +  // vouches_received
        (32 * 3) + // vouchers
        8 +  // vouch_completed_at
        2 +  // reward_multiplier
        1 +  // is_fully_vouched
        1;   // bump
    
    /// Get reward multiplier based on vouch count
    pub fn get_multiplier(&self) -> u16 {
        match self.vouches_received {
            0 => 1000,  // 10%
            1 => 4000,  // 40%
            2 => 7000,  // 70%
            _ => 10000, // 100%
        }
    }
}

