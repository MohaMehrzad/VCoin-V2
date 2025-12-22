use anchor_lang::prelude::*;

use crate::contexts::GetStakeInfo;
use crate::state::StakingTier;

/// Return type for get_stake_info
#[derive(AnchorSerialize, AnchorDeserialize)]
pub struct UserStakeInfo {
    pub staked_amount: u64,
    pub lock_end: i64,
    pub tier: u8,
    pub ve_vcoin_amount: u64,
    pub is_locked: bool,
    pub fee_discount_bps: u16,
}

/// Get user's staking info (view function)
pub fn handler(ctx: Context<GetStakeInfo>) -> Result<UserStakeInfo> {
    let user_stake = &ctx.accounts.user_stake;
    let clock = Clock::get()?;
    
    Ok(UserStakeInfo {
        staked_amount: user_stake.staked_amount,
        lock_end: user_stake.lock_end,
        tier: user_stake.tier,
        ve_vcoin_amount: user_stake.ve_vcoin_amount,
        is_locked: clock.unix_timestamp < user_stake.lock_end,
        fee_discount_bps: StakingTier::from_amount(user_stake.staked_amount).fee_discount_bps(),
    })
}

