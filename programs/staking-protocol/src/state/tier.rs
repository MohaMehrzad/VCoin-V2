use anchor_lang::prelude::*;

use crate::constants::*;

/// Staking tier enum
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, PartialEq, Eq, Default)]
pub enum StakingTier {
    #[default]
    None = 0,
    Bronze = 1,
    Silver = 2,
    Gold = 3,
    Platinum = 4,
}

impl StakingTier {
    pub fn from_amount(amount: u64) -> Self {
        if amount >= PLATINUM_THRESHOLD {
            StakingTier::Platinum
        } else if amount >= GOLD_THRESHOLD {
            StakingTier::Gold
        } else if amount >= SILVER_THRESHOLD {
            StakingTier::Silver
        } else if amount >= BRONZE_THRESHOLD {
            StakingTier::Bronze
        } else {
            StakingTier::None
        }
    }
    
    pub fn boost_multiplier(&self) -> u64 {
        match self {
            StakingTier::None => TIER_BOOST_NONE,
            StakingTier::Bronze => TIER_BOOST_BRONZE,
            StakingTier::Silver => TIER_BOOST_SILVER,
            StakingTier::Gold => TIER_BOOST_GOLD,
            StakingTier::Platinum => TIER_BOOST_PLATINUM,
        }
    }
    
    pub fn fee_discount_bps(&self) -> u16 {
        match self {
            StakingTier::None => FEE_DISCOUNT_NONE,
            StakingTier::Bronze => FEE_DISCOUNT_BRONZE,
            StakingTier::Silver => FEE_DISCOUNT_SILVER,
            StakingTier::Gold => FEE_DISCOUNT_GOLD,
            StakingTier::Platinum => FEE_DISCOUNT_PLATINUM,
        }
    }
    
    pub fn as_u8(&self) -> u8 {
        match self {
            StakingTier::None => 0,
            StakingTier::Bronze => 1,
            StakingTier::Silver => 2,
            StakingTier::Gold => 3,
            StakingTier::Platinum => 4,
        }
    }
}

