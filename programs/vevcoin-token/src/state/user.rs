use anchor_lang::prelude::*;

use crate::constants::USER_VEVCOIN_SEED;

/// User veVCoin Account (PDA per user)
/// Tracks individual veVCoin balance and metadata
#[account]
#[derive(Default)]
pub struct UserVeVCoin {
    /// The user's wallet address
    pub owner: Pubkey,
    /// Current veVCoin balance
    pub balance: u64,
    /// When veVCoin was first minted to this user
    pub first_mint_at: i64,
    /// When veVCoin was last updated
    pub last_update_at: i64,
    /// Bump seed for PDA
    pub bump: u8,
}

impl UserVeVCoin {
    pub const LEN: usize = 8 + // discriminator
        32 + // owner
        8 +  // balance
        8 +  // first_mint_at
        8 +  // last_update_at
        1;   // bump
        
    pub const SEED: &'static [u8] = USER_VEVCOIN_SEED;
}

