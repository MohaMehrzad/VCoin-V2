use anchor_lang::prelude::*;
use crate::constants::*;

/// User energy account
#[account]
#[derive(Default)]
pub struct UserEnergy {
    /// User wallet
    pub user: Pubkey,
    /// Current energy (scales with tier)
    pub current_energy: u16,
    /// Max energy (tier-based)
    pub max_energy: u16,
    /// Last regeneration time
    pub last_regen_time: i64,
    /// Regen rate per hour
    pub regen_rate: u16,
    /// Energy spent today
    pub energy_spent_today: u32,
    /// Energy refunded today
    pub energy_refunded_today: u32,
    /// Last daily reset
    pub last_reset: i64,
    /// User's staking tier (0-4)
    pub tier: u8,
    /// PDA bump
    pub bump: u8,
}

impl UserEnergy {
    pub const LEN: usize = 8 + // discriminator
        32 + // user
        2 +  // current_energy
        2 +  // max_energy
        8 +  // last_regen_time
        2 +  // regen_rate
        4 +  // energy_spent_today
        4 +  // energy_refunded_today
        8 +  // last_reset
        1 +  // tier
        1;   // bump
    
    pub fn max_energy_for_tier(tier: u8) -> u16 {
        match tier {
            0 => MAX_ENERGY_NONE,
            1 => MAX_ENERGY_BRONZE,
            2 => MAX_ENERGY_SILVER,
            3 => MAX_ENERGY_GOLD,
            4 => MAX_ENERGY_PLATINUM,
            _ => MAX_ENERGY_NONE,
        }
    }
    
    pub fn regen_rate_for_tier(tier: u8) -> u16 {
        match tier {
            0 => REGEN_RATE_NONE,
            1 => REGEN_RATE_BRONZE,
            2 => REGEN_RATE_SILVER,
            3 => REGEN_RATE_GOLD,
            4 => REGEN_RATE_PLATINUM,
            _ => REGEN_RATE_NONE,
        }
    }
}
