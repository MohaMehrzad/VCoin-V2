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

declare_id!("9K14FcDRrBeHKD9FPNYeVJaEqJQTac2xspJyb1mM6m48");

/// VCoin Transfer Hook Program
/// 
/// Implements Token-2022 Transfer Hook for:
/// 1. Auto-updating 5A Activity scores on transfers
/// 2. Recording tip transactions for SSCRE calculations
/// 3. Detecting wash trading patterns
/// 4. Updating engagement trust scores

#[program]
pub mod transfer_hook {
    use super::*;

    /// Initialize the transfer hook configuration
    pub fn initialize(
        ctx: Context<Initialize>,
        five_a_program: Pubkey,
        min_activity_amount: u64,
    ) -> Result<()> {
        instructions::admin::initialize::handler(ctx, five_a_program, min_activity_amount)
    }
    
    /// Execute transfer hook - called automatically on every VCoin transfer
    pub fn execute(ctx: Context<Execute>, amount: u64) -> Result<()> {
        instructions::hook::execute::handler(ctx, amount)
    }
    
    /// Initialize extra account metas for the transfer hook
    pub fn initialize_extra_account_meta_list(
        ctx: Context<InitializeExtraAccountMetaList>,
    ) -> Result<()> {
        instructions::hook::initialize_extra_accounts::handler(ctx)
    }
    
    /// Update hook configuration
    pub fn update_config(
        ctx: Context<UpdateConfig>,
        new_five_a_program: Option<Pubkey>,
        new_min_activity_amount: Option<u64>,
        block_wash_trading: Option<bool>,
    ) -> Result<()> {
        instructions::admin::update_config::handler(ctx, new_five_a_program, new_min_activity_amount, block_wash_trading)
    }
    
    /// Pause/unpause the hook
    pub fn set_paused(ctx: Context<UpdateConfig>, paused: bool) -> Result<()> {
        instructions::admin::set_paused::handler(ctx, paused)
    }
    
    /// Update authority
    pub fn update_authority(ctx: Context<UpdateAuthority>, new_authority: Pubkey) -> Result<()> {
        instructions::admin::update_authority::handler(ctx, new_authority)
    }
    
    /// Query user activity stats
    pub fn get_user_activity(ctx: Context<GetUserActivity>) -> Result<()> {
        instructions::query::get_user_activity::handler(ctx)
    }
    
    /// Query pair tracking stats
    pub fn get_pair_stats(ctx: Context<GetPairStats>) -> Result<()> {
        instructions::query::get_pair_stats::handler(ctx)
    }
}
