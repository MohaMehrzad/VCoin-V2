use anchor_lang::prelude::*;

pub mod constants;
pub mod contexts;
pub mod errors;
pub mod events;
pub mod instructions;
pub mod state;
pub mod utils;

#[cfg(test)]
mod tests;

use contexts::*;
use instructions::query::get_stake_info::UserStakeInfo;

declare_id!("6EFcistyr2E81adLUcuBJRr8W2xzpt3D3dFYEcMewpWu");

/// VCoin Staking Protocol
/// 
/// Stake VCoin → Get veVCoin (voting power)
/// - Longer stake = More voting power
/// - Higher tier = Bonus veVCoin
/// 
/// Staking Tiers:
/// - None:     0 VCoin        → 0% fee discount, 1.0x veVCoin boost
/// - Bronze:   1,000 VCoin    → 10% fee discount, 1.1x veVCoin boost
/// - Silver:   5,000 VCoin    → 20% fee discount, 1.2x veVCoin boost
/// - Gold:     20,000 VCoin   → 30% fee discount, 1.3x veVCoin boost
/// - Platinum: 100,000 VCoin  → 50% fee discount, 1.4x veVCoin boost

#[program]
pub mod staking_protocol {
    use super::*;

    /// Initialize the staking pool
    pub fn initialize_pool(ctx: Context<InitializePool>, vevcoin_program: Pubkey) -> Result<()> {
        instructions::admin::initialize_pool::handler(ctx, vevcoin_program)
    }

    /// Stake VCoin with a lock duration
    pub fn stake(ctx: Context<Stake>, amount: u64, lock_duration: i64) -> Result<()> {
        instructions::user::stake::handler(ctx, amount, lock_duration)
    }

    /// Extend lock duration to increase veVCoin
    pub fn extend_lock(ctx: Context<ExtendLock>, new_lock_duration: i64) -> Result<()> {
        instructions::user::extend_lock::handler(ctx, new_lock_duration)
    }

    /// Unstake VCoin after lock expires
    pub fn unstake(ctx: Context<Unstake>, amount: u64) -> Result<()> {
        instructions::user::unstake::handler(ctx, amount)
    }

    /// Update user's tier based on current stake
    pub fn update_tier(ctx: Context<UpdateTier>) -> Result<()> {
        instructions::user::update_tier::handler(ctx)
    }

    /// Pause/unpause the staking pool
    pub fn set_paused(ctx: Context<AdminAction>, paused: bool) -> Result<()> {
        instructions::admin::set_paused::handler(ctx, paused)
    }

    /// Propose a new authority (step 1 of two-step transfer - H-02 security fix)
    pub fn propose_authority(ctx: Context<AdminAction>, new_authority: Pubkey) -> Result<()> {
        instructions::admin::update_authority::handler(ctx, new_authority)
    }

    /// Accept authority transfer (step 2 of two-step transfer - H-02 security fix)
    pub fn accept_authority(ctx: Context<AcceptAuthority>) -> Result<()> {
        instructions::admin::accept_authority::handler(ctx)
    }

    /// Cancel a pending authority transfer (H-02 security fix)
    pub fn cancel_authority_transfer(ctx: Context<AdminAction>) -> Result<()> {
        instructions::admin::cancel_authority_transfer::handler(ctx)
    }

    /// Get user's staking info (view function)
    pub fn get_stake_info(ctx: Context<GetStakeInfo>) -> Result<UserStakeInfo> {
        instructions::query::get_stake_info::handler(ctx)
    }
}
