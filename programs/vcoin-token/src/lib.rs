//! VCoin: the ViWo utility token (Token-2022).
//!
//! The program creates and controls the VCoin mint:
//! * mint PDA `["vcoin-mint"]`, 9 decimals, **metadata only**: no transfer hook, no
//!   permanent delegate, no freeze authority (audit TOK-01, TH-*). Nobody can seize,
//!   freeze or hook-block holder balances.
//! * supply hard-capped at 1,000,000,000 VCoin, checked against the real mint supply;
//! * `initialize` callable only by the program's upgrade authority (audit X-1, TOK-04);
//! * 24h two-step authority transfer (audit X-2).
//!
//! Slashing was removed from the token: slashing applies to staked deposits in
//! `staking-protocol`, behind a DAO decision and a timelock (audit TOK-02/03).

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
use state::MetadataField;

declare_id!("Gg1dtrjAfGYi6NLC31WaJjZNBoucvD98rK2h1u9qrUjn");

#[program]
pub mod vcoin_token {
    use super::*;

    /// Creates the config and the metadata-only VCoin mint. Upgrade authority only.
    pub fn initialize(
        ctx: Context<Initialize>,
        name: String,
        symbol: String,
        uri: String,
    ) -> Result<()> {
        instructions::admin::initialize::handler(ctx, name, symbol, uri)
    }

    /// Mints VCoin to `destination`; authority only, capped at the 1B supply.
    pub fn mint_tokens(ctx: Context<MintTokens>, amount: u64) -> Result<()> {
        instructions::token::mint::handler(ctx, amount)
    }

    /// Updates a token-metadata field (name, symbol or URI).
    pub fn update_metadata(
        ctx: Context<UpdateMetadata>,
        field: MetadataField,
        value: String,
    ) -> Result<()> {
        instructions::admin::update_metadata::handler(ctx, field, value)
    }

    /// Pauses or resumes `mint_tokens`.
    pub fn set_paused(ctx: Context<UpdateConfig>, paused: bool) -> Result<()> {
        instructions::admin::set_paused::handler(ctx, paused)
    }

    /// Starts a 24h two-step authority transfer.
    pub fn propose_authority(ctx: Context<UpdateConfig>, new_authority: Pubkey) -> Result<()> {
        instructions::admin::authority::propose(ctx, new_authority)
    }

    /// Cancels a pending authority transfer.
    pub fn cancel_authority_transfer(ctx: Context<UpdateConfig>) -> Result<()> {
        instructions::admin::authority::cancel(ctx)
    }

    /// Completes a pending authority transfer after the 24h delay.
    pub fn accept_authority(ctx: Context<AcceptAuthority>) -> Result<()> {
        instructions::admin::authority::accept(ctx)
    }
}
