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

declare_id!("FB39ae9x53FxVL3pER9LqCPEx2TRnEnQP55i838Upnjx");

/// veVCoin Token - Vote-Escrowed VCoin (Soulbound)
/// 
/// Token-2022 with Non-Transferable extension making it truly soulbound.
/// Users receive veVCoin when they stake VCoin, representing their voting power.
/// 
/// Key Properties:
/// - Non-Transferable: Cannot be traded on secondary markets
/// - Mint Authority: Only Staking Protocol can mint
/// - Burn Authority: Only Staking Protocol can burn (on unstake)
/// - Prevents governance power markets

#[program]
pub mod vevcoin_token {
    use super::*;

    /// Initialize the veVCoin mint with Token-2022 Non-Transferable extension
    pub fn initialize_mint(
        ctx: Context<InitializeMint>,
        staking_protocol: Pubkey,
    ) -> Result<()> {
        instructions::admin::initialize::handler(ctx, staking_protocol)
    }

    /// Mint veVCoin to a user (only callable by staking protocol)
    pub fn mint_vevcoin(ctx: Context<MintVeVCoin>, amount: u64) -> Result<()> {
        instructions::token::mint::handler(ctx, amount)
    }

    /// Burn veVCoin from a user (only callable by staking protocol)
    pub fn burn_vevcoin(ctx: Context<BurnVeVCoin>, amount: u64) -> Result<()> {
        instructions::token::burn::handler(ctx, amount)
    }

    /// Update the staking protocol address (only authority)
    pub fn update_staking_protocol(
        ctx: Context<UpdateConfig>,
        new_staking_protocol: Pubkey,
    ) -> Result<()> {
        instructions::admin::update_staking_protocol::handler(ctx, new_staking_protocol)
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

    /// Get user's veVCoin balance (view function)
    pub fn get_balance(ctx: Context<GetBalance>) -> Result<u64> {
        instructions::token::get_balance::handler(ctx)
    }
}
