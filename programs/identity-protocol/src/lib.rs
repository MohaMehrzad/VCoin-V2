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

declare_id!("3egAds3pFR5oog6iQCN42KPvgih8HQz2FGybNjiVWixG");

/// ViWoApp Identity Protocol
/// 
/// Minimal on-chain DID anchor with Solana Attestation Service (SAS) integration.
/// Provides portable identity verification across all Solana dApps.
/// 
/// Verification Levels:
/// - Level 0 (None): Wallet connected only
/// - Level 1 (Basic): Email + phone verified  
/// - Level 2 (KYC): Identity documents verified
/// - Level 3 (Full): KYC + biometric verification
/// - Level 4 (Enhanced): Full + UniqueHuman attestation

#[program]
pub mod identity_protocol {
    use super::*;

    /// Initialize the identity protocol
    pub fn initialize(ctx: Context<Initialize>, sas_program: Pubkey, usdc_mint: Pubkey) -> Result<()> {
        instructions::admin::initialize::handler(ctx, sas_program, usdc_mint)
    }
    
    /// Create a new identity for a user
    pub fn create_identity(ctx: Context<CreateIdentity>, did_hash: [u8; 32], username: String) -> Result<()> {
        instructions::user::create_identity::handler(ctx, did_hash, username)
    }
    
    /// Update DID document hash
    pub fn update_did_hash(ctx: Context<UpdateIdentity>, new_did_hash: [u8; 32]) -> Result<()> {
        instructions::user::update_did_hash::handler(ctx, new_did_hash)
    }
    
    /// Update verification level (admin only)
    pub fn update_verification(ctx: Context<AdminUpdateIdentity>, new_level: u8, verification_hash: [u8; 32]) -> Result<()> {
        instructions::admin::update_verification::handler(ctx, new_level, verification_hash)
    }
    
    /// Link SAS attestation to identity
    pub fn link_sas_attestation(
        ctx: Context<LinkSASAttestation>,
        sas_attestation_id: Pubkey,
        attestation_type: u8,
        verified_claims: u16,
        expires_at: i64,
        portable_score: u16,
    ) -> Result<()> {
        instructions::user::link_sas_attestation::handler(ctx, sas_attestation_id, attestation_type, verified_claims, expires_at, portable_score)
    }
    
    /// Subscribe to a tier
    pub fn subscribe(ctx: Context<Subscribe>, tier: u8) -> Result<()> {
        instructions::user::subscribe::handler(ctx, tier)
    }
    
    /// Add trusted attester (admin only)
    pub fn add_trusted_attester(ctx: Context<UpdateConfig>, attester: Pubkey) -> Result<()> {
        instructions::admin::add_trusted_attester::handler(ctx, attester)
    }
    
    /// Remove trusted attester (admin only)
    pub fn remove_trusted_attester(ctx: Context<UpdateConfig>, attester: Pubkey) -> Result<()> {
        instructions::admin::remove_trusted_attester::handler(ctx, attester)
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
    
    /// Get identity info
    pub fn get_identity(ctx: Context<GetIdentity>) -> Result<()> {
        instructions::query::get_identity::handler(ctx)
    }
}
