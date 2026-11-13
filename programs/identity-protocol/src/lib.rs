//! Portable identity: DID hash, unique username, attester-signed verification levels and
//! USDC subscriptions.
//!
//! * Levels come from attestations co-signed by a trusted attester ("attester-signed"; the
//!   program does not read Solana Attestation Service accounts). A level carries its
//!   attester and expiry; consumers read [`state::Identity::effective_level`], which drops to
//!   0 on expiry or when the attester is removed. Attestations can be refreshed, revoked by
//!   their attester or the authority, and the authority may only lower levels (audit ID-01/02).
//! * One attestation id binds to one wallet; KYC level requires the KYC claim (ID-03/04).
//! * Subscriptions extend, prorate upgrades and refuse active downgrades (ID-05); payment
//!   config is timelocked; init is upgrade-authority gated (ID-06/07); pause covers every user
//!   path (ID-08).

use anchor_lang::prelude::*;

pub mod constants;
pub mod contexts;
pub mod errors;
pub mod events;
pub mod instructions;
pub mod state;

#[cfg(test)]
mod tests;

use constants::NUM_TIERS;
use contexts::*;

declare_id!("3egAds3pFR5oog6iQCN42KPvgih8HQz2FGybNjiVWixG");

#[program]
pub mod identity_protocol {
    use super::*;

    /// Upgrade authority only; USDC mint (6 decimals) and treasury token account fixed here.
    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        instructions::admin::initialize(ctx)
    }

    pub fn add_attester(ctx: Context<UpdateConfig>, attester: Pubkey) -> Result<()> {
        instructions::admin::add_attester(ctx, attester)
    }

    pub fn remove_attester(ctx: Context<UpdateConfig>, attester: Pubkey) -> Result<()> {
        instructions::admin::remove_attester(ctx, attester)
    }

    pub fn propose_payment(ctx: Context<ProposePayment>, prices: [u64; NUM_TIERS]) -> Result<()> {
        instructions::admin::propose_payment(ctx, prices)
    }

    pub fn apply_payment(ctx: Context<ApplyPayment>) -> Result<()> {
        instructions::admin::apply_payment(ctx)
    }

    pub fn cancel_payment(ctx: Context<UpdateConfig>) -> Result<()> {
        instructions::admin::cancel_payment(ctx)
    }

    pub fn set_paused(ctx: Context<UpdateConfig>, paused: bool) -> Result<()> {
        instructions::admin::set_paused(ctx, paused)
    }

    pub fn propose_authority(ctx: Context<UpdateConfig>, new_authority: Pubkey) -> Result<()> {
        instructions::admin::propose_authority(ctx, new_authority)
    }

    pub fn cancel_authority_transfer(ctx: Context<UpdateConfig>) -> Result<()> {
        instructions::admin::cancel_authority_transfer(ctx)
    }

    pub fn accept_authority(ctx: Context<AcceptAuthority>) -> Result<()> {
        instructions::admin::accept_authority(ctx)
    }

    pub fn create_identity(
        ctx: Context<CreateIdentity>,
        did_hash: [u8; 32],
        username: String,
    ) -> Result<()> {
        instructions::identity::create_identity(ctx, did_hash, username)
    }

    pub fn update_did_hash(ctx: Context<UpdateIdentity>, did_hash: [u8; 32]) -> Result<()> {
        instructions::identity::update_did_hash(ctx, did_hash)
    }

    pub fn close_identity(ctx: Context<CloseIdentity>) -> Result<()> {
        instructions::identity::close_identity(ctx)
    }

    pub fn link_attestation(
        ctx: Context<LinkAttestation>,
        attestation_id: [u8; 32],
        claims: u16,
        expires_at: i64,
    ) -> Result<()> {
        instructions::identity::link_attestation(ctx, attestation_id, claims, expires_at)
    }

    pub fn revoke_attestation(ctx: Context<RevokeAttestation>) -> Result<()> {
        instructions::identity::revoke_attestation(ctx)
    }

    pub fn admin_lower_level(ctx: Context<AdminLowerLevel>, new_level: u8) -> Result<()> {
        instructions::identity::admin_lower_level(ctx, new_level)
    }

    pub fn subscribe(ctx: Context<Subscribe>, tier: u8, max_payment: u64) -> Result<()> {
        instructions::subscription::subscribe(ctx, tier, max_payment)
    }
}
