use anchor_lang::prelude::*;

pub mod constants;
pub mod contexts;
pub mod errors;
pub mod instructions;
pub mod state;

use contexts::*;

declare_id!("Gg1dtrjAfGYi6NLC31WaJjZNBoucvD98rK2h1u9qrUjn");

/// VCoin Token-2022 with Extensions
/// - Metadata: On-chain token metadata (Name: VCoin, Symbol: VIWO)
/// - Permanent Delegate: Slashing authority for governance multisig
/// - Transfer Hook: Auto-update 5A scores (prepared, hook program separate)
/// 
/// Total Supply: 1,000,000,000 (1B)
/// Decimals: 9

#[program]
pub mod vcoin_token {
    use super::*;

    /// Initialize the VCoin mint with Token-2022 extensions
    pub fn initialize_mint(
        ctx: Context<InitializeMint>,
        permanent_delegate: Pubkey,
    ) -> Result<()> {
        instructions::admin::initialize::handler(ctx, permanent_delegate)
    }

    /// Mint VCoin tokens to a specified account
    pub fn mint_tokens(ctx: Context<MintTokens>, amount: u64) -> Result<()> {
        instructions::token::mint::handler(ctx, amount)
    }

    /// Slash tokens from an account using permanent delegate authority
    pub fn slash_tokens(ctx: Context<SlashTokens>, amount: u64) -> Result<()> {
        instructions::token::slash::handler(ctx, amount)
    }

    /// Pause/unpause token operations
    pub fn set_paused(ctx: Context<UpdateConfig>, paused: bool) -> Result<()> {
        instructions::admin::set_paused::handler(ctx, paused)
    }

    /// Update the authority
    pub fn update_authority(ctx: Context<UpdateConfig>, new_authority: Pubkey) -> Result<()> {
        instructions::admin::update_authority::handler(ctx, new_authority)
    }

    /// Update the permanent delegate (for slashing)
    pub fn update_permanent_delegate(
        ctx: Context<UpdateConfig>,
        new_delegate: Pubkey,
    ) -> Result<()> {
        instructions::admin::update_delegate::handler(ctx, new_delegate)
    }
}
