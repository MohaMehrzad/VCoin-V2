use anchor_lang::prelude::*;

declare_id!("MJn1A4MPCBPJGWWuZrtq7bHSo2G289sUwW3ej2wcmLV");

/// Content Registry Protocol
/// 
/// On-chain tracking hash with state management.
/// Each piece of content gets a unique tracking hash stored on-chain.
/// 
/// Content Types:
/// - Post (text/image)
/// - Article (long-form)
/// - Media (video/audio)
/// - NFT (tokenized content)
/// - Thread (multi-post)
/// 
/// State Flow: Active → Edited → Deleted (soft) → Archived

pub mod constants;
pub mod errors;
pub mod events;
pub mod state;
pub mod contexts;
pub mod instructions;
pub mod utils;

#[cfg(test)]
mod tests;

use contexts::*;
use instructions::*;

#[program]
pub mod content_registry {
    use super::*;

    /// Initialize the content registry
    pub fn initialize(
        ctx: Context<Initialize>,
        identity_program: Pubkey,
        staking_program: Pubkey,
    ) -> Result<()> {
        admin::initialize::handler(ctx, identity_program, staking_program)
    }
    
    /// Initialize energy system config
    pub fn initialize_energy(ctx: Context<InitializeEnergy>) -> Result<()> {
        admin::initialize_energy::handler(ctx)
    }
    
    /// Create content record
    pub fn create_content(
        ctx: Context<CreateContent>,
        tracking_id: [u8; 32],
        content_hash: [u8; 32],
        content_uri: String,
        content_type: u8,
    ) -> Result<()> {
        content::create::handler(ctx, tracking_id, content_hash, content_uri, content_type)
    }
    
    /// Edit content (update hash and URI)
    pub fn edit_content(
        ctx: Context<EditContent>,
        new_content_hash: [u8; 32],
        new_content_uri: String,
    ) -> Result<()> {
        content::edit::handler(ctx, new_content_hash, new_content_uri)
    }
    
    /// Soft delete content
    pub fn delete_content(ctx: Context<DeleteContent>) -> Result<()> {
        content::delete::handler(ctx)
    }
    
    /// Update engagement count (oracle/backend call)
    pub fn update_engagement(
        ctx: Context<UpdateEngagement>,
        engagement_count: u32,
    ) -> Result<()> {
        content::update_engagement::handler(ctx, engagement_count)
    }
    
    /// Claim energy refund based on engagement
    pub fn claim_energy_refund(ctx: Context<ClaimRefund>) -> Result<()> {
        energy::claim_refund::handler(ctx)
    }
    
    /// Initialize user energy account
    pub fn initialize_user_energy(
        ctx: Context<InitializeUserEnergy>,
        tier: u8,
    ) -> Result<()> {
        energy::initialize_user::handler(ctx, tier)
    }
    
    /// Update user tier (from staking program)
    pub fn update_user_tier(
        ctx: Context<UpdateUserTier>,
        new_tier: u8,
    ) -> Result<()> {
        energy::update_tier::handler(ctx, new_tier)
    }
    
    /// Pause/unpause registry
    pub fn set_paused(ctx: Context<UpdateConfig>, paused: bool) -> Result<()> {
        admin::set_paused::handler(ctx, paused)
    }
    
    /// Propose a new authority (step 1 of two-step transfer - H-02 security fix)
    pub fn propose_authority(ctx: Context<UpdateAuthority>, new_authority: Pubkey) -> Result<()> {
        admin::update_authority::handler(ctx, new_authority)
    }
    
    /// Accept authority transfer (step 2 of two-step transfer - H-02 security fix)
    pub fn accept_authority(ctx: Context<AcceptAuthority>) -> Result<()> {
        admin::accept_authority::handler(ctx)
    }
    
    /// Cancel a pending authority transfer (H-02 security fix)
    pub fn cancel_authority_transfer(ctx: Context<UpdateAuthority>) -> Result<()> {
        admin::cancel_authority_transfer::handler(ctx)
    }
    
    /// Get content info
    pub fn get_content(ctx: Context<GetContent>) -> Result<()> {
        query::get_content::handler(ctx)
    }
    
    /// Get user energy stats
    pub fn get_energy(ctx: Context<GetEnergy>) -> Result<()> {
        query::get_energy::handler(ctx)
    }
}
