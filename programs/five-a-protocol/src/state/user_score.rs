use anchor_lang::prelude::*;

use crate::constants::*;

/// Individual user's 5A score
#[account]
#[derive(Default)]
pub struct UserScore {
    /// User wallet
    pub user: Pubkey,
    /// Authenticity score (0-10000)
    pub authenticity: u16,
    /// Accuracy score (0-10000)
    pub accuracy: u16,
    /// Agility score (0-10000)
    pub agility: u16,
    /// Activity score (0-10000)
    pub activity: u16,
    /// Approved score (0-10000)
    pub approved: u16,
    /// Weighted composite score (0-10000)
    pub composite_score: u16,
    /// Score last updated
    pub last_updated: i64,
    /// Last snapshot epoch this user was included in
    pub last_snapshot_epoch: u64,
    /// Number of score updates
    pub update_count: u32,
    /// Whether user has private score mode enabled
    pub is_private: bool,
    /// PDA bump
    pub bump: u8,
}

impl UserScore {
    pub const LEN: usize = 8 + // discriminator
        32 + // user
        2 +  // authenticity
        2 +  // accuracy
        2 +  // agility
        2 +  // activity
        2 +  // approved
        2 +  // composite_score
        8 +  // last_updated
        8 +  // last_snapshot_epoch
        4 +  // update_count
        1 +  // is_private
        1;   // bump
    
    /// Calculate weighted composite score
    pub fn calculate_composite(&self) -> u16 {
        let weighted = 
            (self.authenticity as u32 * AUTHENTICITY_WEIGHT as u32 +
             self.accuracy as u32 * ACCURACY_WEIGHT as u32 +
             self.agility as u32 * AGILITY_WEIGHT as u32 +
             self.activity as u32 * ACTIVITY_WEIGHT as u32 +
             self.approved as u32 * APPROVED_WEIGHT as u32) / 10000;
        
        weighted as u16
    }
}

