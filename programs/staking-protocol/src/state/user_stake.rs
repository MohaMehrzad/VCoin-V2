use anchor_lang::prelude::*;

/// User Stake Account (PDA per user)
#[account]
#[derive(Default)]
pub struct UserStake {
    /// Owner of this stake
    pub owner: Pubkey,
    /// Amount of VCoin staked
    pub staked_amount: u64,
    /// Lock duration in seconds
    pub lock_duration: i64,
    /// Timestamp when lock ends
    pub lock_end: i64,
    /// When the stake was created
    pub stake_start: i64,
    /// Current staking tier
    pub tier: u8,
    /// Current veVCoin amount minted
    pub ve_vcoin_amount: u64,
    /// Bump seed for PDA
    pub bump: u8,
}

impl UserStake {
    pub const LEN: usize = 8 + // discriminator
        32 + // owner
        8 +  // staked_amount
        8 +  // lock_duration
        8 +  // lock_end
        8 +  // stake_start
        1 +  // tier
        8 +  // ve_vcoin_amount
        1;   // bump
}

