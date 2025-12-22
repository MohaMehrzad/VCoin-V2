use anchor_lang::prelude::*;

use crate::constants::FOUR_YEARS_SECONDS;
use crate::errors::StakingError;
use crate::state::StakingTier;

/// Calculate veVCoin amount based on staked amount, lock duration, and tier
/// Formula: ve_vcoin = staked_amount * (lock_duration / 4_years) * tier_boost
pub fn calculate_vevcoin(staked_amount: u64, lock_duration: i64, tier: StakingTier) -> Result<u64> {
    // ve_vcoin = staked_amount * (lock_duration / 4_years) * tier_boost
    // To avoid floating point, we multiply first then divide
    // tier_boost is already multiplied by 1000
    
    let duration_factor = (lock_duration as u128) * 1000 / (FOUR_YEARS_SECONDS as u128);
    let tier_boost = tier.boost_multiplier() as u128;
    
    let ve_vcoin = (staked_amount as u128)
        .checked_mul(duration_factor)
        .ok_or(StakingError::Overflow)?
        .checked_mul(tier_boost)
        .ok_or(StakingError::Overflow)?
        .checked_div(1_000_000) // Divide by 1000 * 1000 to normalize
        .ok_or(StakingError::Overflow)?;
    
    Ok(ve_vcoin as u64)
}

