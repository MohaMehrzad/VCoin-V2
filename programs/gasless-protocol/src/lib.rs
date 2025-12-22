use anchor_lang::prelude::*;

declare_id!("FcXJAjzJs8eVY2WTRFXynQBpC7WZUqKZppyp9xS6PaB3");

/// Gasless Layer - Paymaster + Session Keys for Zero-Friction UX
/// 
/// Enables gasless transactions for ViWoApp users.
/// 
/// Key Features:
/// - Paymaster: Platform subsidizes transaction fees
/// - Session Keys: Temporary 24h signing keys with limited scope
/// - VCoin Deduction: Automatic fee deduction from VCoin balance
/// - SSCRE Integration: 1% deduction from reward claims
/// 
/// Fee Deduction Methods:
/// 1. Platform Subsidized (onboarding, governance)
/// 2. VCoin Deduction (tips, transfers)
/// 3. Reward Deduction (1% from SSCRE claims)
/// 
/// Session Key Scopes:
/// - Tip actions
/// - Vouch actions
/// - Content interactions
/// - Governance voting

pub mod constants;
pub mod errors;
pub mod events;
pub mod state;
pub mod contexts;
pub mod instructions;

#[cfg(test)]
mod tests;

use contexts::*;
use instructions::*;

#[program]
pub mod gasless_protocol {
    use super::*;

    /// Initialize gasless configuration
    pub fn initialize(
        ctx: Context<Initialize>,
        fee_payer: Pubkey,
        daily_budget: u64,
    ) -> Result<()> {
        admin::initialize::handler(ctx, fee_payer, daily_budget)
    }
    
    /// Create a session key for gasless transactions
    pub fn create_session_key(
        ctx: Context<CreateSessionKey>,
        session_pubkey: Pubkey,
        scope: u16,
        duration_seconds: i64,
        max_actions: u32,
        max_spend: u64,
        fee_method: u8,
    ) -> Result<()> {
        session::create::handler(ctx, session_pubkey, scope, duration_seconds, max_actions, max_spend, fee_method)
    }
    
    /// Execute an action using session key
    pub fn execute_session_action(
        ctx: Context<ExecuteSessionAction>,
        action_type: u16,
        spend_amount: u64,
    ) -> Result<()> {
        session::execute_action::handler(ctx, action_type, spend_amount)
    }
    
    /// Deduct VCoin fee for gasless transaction
    pub fn deduct_vcoin_fee(
        ctx: Context<DeductVCoinFee>,
        amount: u64,
    ) -> Result<()> {
        fee::deduct_vcoin::handler(ctx, amount)
    }
    
    /// Revoke a session key
    pub fn revoke_session_key(ctx: Context<RevokeSessionKey>) -> Result<()> {
        session::revoke::handler(ctx)
    }
    
    /// Update fee configuration
    pub fn update_fee_config(
        ctx: Context<UpdateConfig>,
        sol_fee_per_tx: u64,
        vcoin_fee_multiplier: u64,
        sscre_deduction_bps: u16,
    ) -> Result<()> {
        admin::update_fee_config::handler(ctx, sol_fee_per_tx, vcoin_fee_multiplier, sscre_deduction_bps)
    }
    
    /// Update daily budget
    pub fn update_daily_budget(
        ctx: Context<UpdateConfig>,
        daily_budget: u64,
        max_per_user: u32,
    ) -> Result<()> {
        admin::update_daily_budget::handler(ctx, daily_budget, max_per_user)
    }
    
    /// Set fee payer (paymaster wallet)
    pub fn set_fee_payer(ctx: Context<UpdateConfig>, new_fee_payer: Pubkey) -> Result<()> {
        admin::set_fee_payer::handler(ctx, new_fee_payer)
    }
    
    /// Set SSCRE program reference
    pub fn set_sscre_program(ctx: Context<UpdateConfig>, sscre_program: Pubkey) -> Result<()> {
        admin::set_sscre_program::handler(ctx, sscre_program)
    }
    
    /// Pause/unpause protocol
    pub fn set_paused(ctx: Context<UpdateConfig>, paused: bool) -> Result<()> {
        admin::set_paused::handler(ctx, paused)
    }
    
    /// Update authority
    pub fn update_authority(ctx: Context<UpdateAuthority>, new_authority: Pubkey) -> Result<()> {
        admin::update_authority::handler(ctx, new_authority)
    }
    
    /// Get session key info
    pub fn get_session_info(ctx: Context<GetSessionInfo>) -> Result<()> {
        query::get_session_info::handler(ctx)
    }
    
    /// Get user gasless stats
    pub fn get_user_gasless_stats(ctx: Context<GetUserStats>) -> Result<()> {
        query::get_user_stats::handler(ctx)
    }
    
    /// Get config stats
    pub fn get_config_stats(ctx: Context<GetConfigStats>) -> Result<()> {
        query::get_config_stats::handler(ctx)
    }
}
