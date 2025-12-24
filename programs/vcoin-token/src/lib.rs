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
    /// DEPRECATED: Use propose_slash -> approve_slash -> execute_slash flow instead
    /// This legacy function is kept for backwards compatibility
    pub fn slash_tokens(ctx: Context<SlashTokens>, amount: u64) -> Result<()> {
        instructions::token::slash::handler(ctx, amount)
    }

    /// Propose a slash request (H-01 Security Fix - Step 1)
    /// Only permanent delegate can propose; requires governance approval
    /// request_id should be unique (e.g., current timestamp) to derive unique PDA
    pub fn propose_slash(
        ctx: Context<ProposeSlash>,
        target: Pubkey,
        request_id: u64,
        amount: u64,
        reason_hash: [u8; 32],
    ) -> Result<()> {
        instructions::token::propose_slash::handler(ctx, target, request_id, amount, reason_hash)
    }

    /// Approve a slash request (H-01 Security Fix - Step 2)
    /// Only governance authority can approve; starts 48h timelock
    pub fn approve_slash(ctx: Context<ApproveSlash>, proposal_id: u64) -> Result<()> {
        instructions::token::approve_slash::handler(ctx, proposal_id)
    }

    /// Execute an approved slash (H-01 Security Fix - Step 3)
    /// Requires 48h timelock to have expired after governance approval
    pub fn execute_slash(ctx: Context<ExecuteSlash>) -> Result<()> {
        instructions::token::execute_slash::handler(ctx)
    }

    /// Pause/unpause token operations
    pub fn set_paused(ctx: Context<UpdateConfig>, paused: bool) -> Result<()> {
        instructions::admin::set_paused::handler(ctx, paused)
    }

    /// Propose a new authority (step 1 of two-step transfer - H-02 security fix)
    pub fn propose_authority(ctx: Context<UpdateConfig>, new_authority: Pubkey) -> Result<()> {
        instructions::admin::update_authority::handler(ctx, new_authority)
    }

    /// Accept authority transfer (step 2 of two-step transfer - H-02 security fix)
    pub fn accept_authority(ctx: Context<AcceptAuthority>) -> Result<()> {
        instructions::admin::accept_authority::handler(ctx)
    }

    /// Cancel a pending authority transfer (H-02 security fix)
    pub fn cancel_authority_transfer(ctx: Context<UpdateConfig>) -> Result<()> {
        instructions::admin::cancel_authority_transfer::handler(ctx)
    }

    /// Update the permanent delegate (for slashing)
    pub fn update_permanent_delegate(
        ctx: Context<UpdateConfig>,
        new_delegate: Pubkey,
    ) -> Result<()> {
        instructions::admin::update_delegate::handler(ctx, new_delegate)
    }
}
