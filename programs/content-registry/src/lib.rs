//! Content registry: proof-of-existence records rate-limited by stake-tiered energy.
//!
//! * A post's record lives at `["content-record", author, tracking_id]` (no squatting) and
//!   costs energy by type; edits cost energy too and every version's hash is evented.
//! * Energy tiers come from VCoin locked in staking-protocol, refreshed on every post (users
//!   cannot pick their tier, audit CR-01); regeneration keeps partial progress (CR-03).
//! * A separate engagement oracle reports engagement; refunds of up to 100% open after 24h,
//!   close after 7 days and never consume a non-qualifying claim (CR-06/07).
//! * Posts and edits may be signed by a gasless-protocol session key (scope CONTENT); deletes
//!   and refund claims need the author's wallet.
//! * No global counters (CR-05); deleting closes the record (CR-11).

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

declare_id!("MJn1A4MPCBPJGWWuZrtq7bHSo2G289sUwW3ej2wcmLV");

#[program]
pub mod content_registry {
    use super::*;

    /// Upgrade authority only.
    pub fn initialize(ctx: Context<Initialize>, engagement_oracle: Pubkey) -> Result<()> {
        instructions::admin::initialize(ctx, engagement_oracle)
    }

    pub fn propose_oracle(ctx: Context<UpdateConfig>, oracle: Pubkey) -> Result<()> {
        instructions::admin::propose_oracle(ctx, oracle)
    }

    pub fn apply_oracle(ctx: Context<ApplyOracle>) -> Result<()> {
        instructions::admin::apply_oracle(ctx)
    }

    pub fn cancel_oracle(ctx: Context<UpdateConfig>) -> Result<()> {
        instructions::admin::cancel_oracle(ctx)
    }

    pub fn set_paused(ctx: Context<UpdateConfig>, paused: bool) -> Result<()> {
        instructions::admin::set_paused(ctx, paused)
    }

    /// Sets or clears the emergency guardian (may pause, never unpause).
    pub fn set_guardian(ctx: Context<UpdateConfig>, guardian: Pubkey) -> Result<()> {
        instructions::admin::set_guardian(ctx, guardian)
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

    pub fn init_user_energy(ctx: Context<InitUserEnergy>) -> Result<()> {
        instructions::content::init_user_energy(ctx)
    }

    pub fn create_content(
        ctx: Context<CreateContent>,
        tracking_id: [u8; 32],
        content_hash: [u8; 32],
        uri: String,
        content_type: u8,
    ) -> Result<()> {
        instructions::content::create_content(ctx, tracking_id, content_hash, uri, content_type)
    }

    pub fn edit_content(
        ctx: Context<EditContent>,
        content_hash: [u8; 32],
        uri: String,
    ) -> Result<()> {
        instructions::content::edit_content(ctx, content_hash, uri)
    }

    pub fn delete_content(ctx: Context<DeleteContent>) -> Result<()> {
        instructions::content::delete_content(ctx)
    }

    pub fn update_engagement(ctx: Context<UpdateEngagement>, engagement_count: u32) -> Result<()> {
        instructions::content::update_engagement(ctx, engagement_count)
    }

    pub fn claim_refund(ctx: Context<ClaimRefund>) -> Result<()> {
        instructions::content::claim_refund(ctx)
    }
}
