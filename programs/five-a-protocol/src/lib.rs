use anchor_lang::prelude::*;

pub mod constants;
pub mod contexts;
pub mod errors;
pub mod events;
pub mod instructions;
pub mod state;

#[cfg(test)]
mod tests;

use contexts::*;

declare_id!("783PbtJw5cc7yatnr9fsvTGSnkKaV6iJe6E8VUPTYrT8");

/// 5A Reputation Protocol
/// 
/// Anti-bot reputation scoring with oracle model and periodic snapshots.
/// 
/// The 5 Stars (Open Source - MIT Licensed - Public Good):
/// - A1: Authenticity (25%) - "Are you a real person?"
/// - A2: Accuracy (20%) - "Is your content quality?"
/// - A3: Agility (15%) - "Are you fast?"
/// - A4: Activity (25%) - "Do you show up daily?"
/// - A5: Approved (15%) - "Does the community like you?"
/// 
/// Score Range: 0-100 (stored as 0-10000 for precision)

#[program]
pub mod five_a_protocol {
    use super::*;

    /// Initialize the 5A protocol
    pub fn initialize(ctx: Context<Initialize>, identity_program: Pubkey, vcoin_mint: Pubkey) -> Result<()> {
        instructions::admin::initialize::handler(ctx, identity_program, vcoin_mint)
    }
    
    /// Register an oracle
    pub fn register_oracle(ctx: Context<RegisterOracle>, name: String) -> Result<()> {
        instructions::admin::register_oracle::handler(ctx, name)
    }
    
    /// Submit score update (oracle only)
    pub fn submit_score(
        ctx: Context<SubmitScore>,
        authenticity: u16,
        accuracy: u16,
        agility: u16,
        activity: u16,
        approved: u16,
    ) -> Result<()> {
        instructions::oracle::submit_score::handler(ctx, authenticity, accuracy, agility, activity, approved)
    }
    
    /// Create a score snapshot (oracle only)
    pub fn create_snapshot(
        ctx: Context<CreateSnapshot>,
        merkle_root: [u8; 32],
        user_count: u64,
        avg_score: u16,
    ) -> Result<()> {
        instructions::oracle::create_snapshot::handler(ctx, merkle_root, user_count, avg_score)
    }
    
    /// Vouch for a new user
    pub fn vouch_for_user(ctx: Context<VouchForUser>) -> Result<()> {
        instructions::vouch::vouch_for_user::handler(ctx)
    }
    
    /// Evaluate vouch outcome after 90 days
    pub fn evaluate_vouch(ctx: Context<EvaluateVouch>) -> Result<()> {
        instructions::vouch::evaluate_vouch::handler(ctx)
    }
    
    /// Enable private score mode
    pub fn enable_private_score(ctx: Context<UpdateUserScore>) -> Result<()> {
        instructions::user::enable_private_score::handler(ctx)
    }
    
    /// Disable private score mode
    pub fn disable_private_score(ctx: Context<UpdateUserScore>) -> Result<()> {
        instructions::user::disable_private_score::handler(ctx)
    }
    
    /// Pause/unpause protocol
    pub fn set_paused(ctx: Context<UpdateConfig>, paused: bool) -> Result<()> {
        instructions::admin::set_paused::handler(ctx, paused)
    }
    
    /// Propose a new authority (step 1 of two-step transfer - H-02 security fix)
    pub fn propose_authority(ctx: Context<UpdateAuthority>, new_authority: Pubkey) -> Result<()> {
        instructions::admin::update_authority::handler(ctx, new_authority)
    }
    
    /// Accept authority transfer (step 2 of two-step transfer - H-02 security fix)
    pub fn accept_authority(ctx: Context<AcceptAuthority>) -> Result<()> {
        instructions::admin::accept_authority::handler(ctx)
    }
    
    /// Cancel a pending authority transfer (H-02 security fix)
    pub fn cancel_authority_transfer(ctx: Context<UpdateAuthority>) -> Result<()> {
        instructions::admin::cancel_authority_transfer::handler(ctx)
    }
    
    /// Get user score
    pub fn get_score(ctx: Context<GetScore>) -> Result<()> {
        instructions::query::get_score::handler(ctx)
    }
}
