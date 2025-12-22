use anchor_lang::prelude::*;

/// Voucher statistics
#[account]
#[derive(Default)]
pub struct VoucherStats {
    /// Voucher wallet
    pub user: Pubkey,
    /// Total vouches given
    pub total_vouches_given: u32,
    /// Successful vouches (vouchee reached 50%+)
    pub successful_vouches: u32,
    /// Failed vouches (vouchee banned/inactive)
    pub failed_vouches: u32,
    /// Vouch accuracy (0-10000)
    pub vouch_accuracy: u16,
    /// Current active vouches
    pub vouches_active: u8,
    /// Max concurrent vouches (based on 5A score)
    pub max_concurrent_vouches: u8,
    /// Total rewards earned
    pub total_rewards_earned: u64,
    /// Total stake lost
    pub total_stake_lost: u64,
    /// PDA bump
    pub bump: u8,
}

impl VoucherStats {
    pub const LEN: usize = 8 + // discriminator
        32 + // user
        4 +  // total_vouches_given
        4 +  // successful_vouches
        4 +  // failed_vouches
        2 +  // vouch_accuracy
        1 +  // vouches_active
        1 +  // max_concurrent_vouches
        8 +  // total_rewards_earned
        8 +  // total_stake_lost
        1;   // bump
    
    /// Calculate max vouches based on 5A score
    pub fn max_vouches_for_score(score: u16) -> u8 {
        if score >= 9000 { 10 }
        else if score >= 8000 { 8 }
        else if score >= 7000 { 5 }
        else if score >= 6000 { 3 }
        else { 0 }
    }
}

