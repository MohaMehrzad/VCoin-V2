use anchor_lang::prelude::*;

use crate::contexts::UpdateTier;
use crate::errors::StakingError;
use crate::state::StakingTier;
use crate::utils::calculate_vevcoin;

/// Update user's tier based on current stake
pub fn handler(ctx: Context<UpdateTier>) -> Result<()> {
    let user_stake = &mut ctx.accounts.user_stake;
    
    require!(user_stake.staked_amount > 0, StakingError::NoActiveStake);
    
    let new_tier = StakingTier::from_amount(user_stake.staked_amount);
    let old_tier = user_stake.tier;
    
    user_stake.tier = new_tier.as_u8();
    
    // Recalculate veVCoin with new tier
    let new_vevcoin = calculate_vevcoin(
        user_stake.staked_amount,
        user_stake.lock_duration,
        new_tier,
    )?;
    user_stake.ve_vcoin_amount = new_vevcoin;
    
    msg!("Tier updated from {} to {}", old_tier, new_tier.as_u8());
    msg!("veVCoin updated to: {}", new_vevcoin);
    
    Ok(())
}

