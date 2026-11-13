//! Gasless: session-key registry + VCoin fee collection.
//!
//! * **Session keys** (MagicBlock gpl-session model): a scoped, ≤24h ephemeral signer for
//!   one target program. Consuming programs (content-registry, vilink) load the
//!   [`state::SessionToken`] with `seeds::program = gasless_protocol::ID` and call
//!   `SessionToken::verify`. Sessions only cover low-risk social actions; transfers, tips,
//!   claims, staking and votes always need the main wallet.
//! * **Fees**: `pay_vcoin_fee(max_fee)`; the user always caps the fee. Collected fees go
//!   to a treasury that can only change behind a 24h delay.
//! * **SOL sponsorship** happens off-chain: a relayer (Kora is recommended) pays
//!   transaction fees and rent. The v2 on-chain "subsidy budget" moved no SOL and could
//!   brick itself after 3 idle days (audit GAS-01/02); it is removed.

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

declare_id!("FcXJAjzJs8eVY2WTRFXynQBpC7WZUqKZppyp9xS6PaB3");

#[program]
pub mod gasless_protocol {
    use super::*;

    /// Upgrade authority only.
    pub fn initialize(ctx: Context<Initialize>, vcoin_fee_per_tx: u64) -> Result<()> {
        instructions::admin::initialize(ctx, vcoin_fee_per_tx)
    }

    pub fn create_session(
        ctx: Context<CreateSession>,
        target_program: Pubkey,
        scope: u16,
        valid_until: i64,
    ) -> Result<()> {
        instructions::session::create_session(ctx, target_program, scope, valid_until)
    }

    pub fn revoke_session(ctx: Context<RevokeSession>) -> Result<()> {
        instructions::session::revoke_session(ctx)
    }

    pub fn pay_vcoin_fee(ctx: Context<PayVCoinFee>, max_fee: u64) -> Result<()> {
        instructions::fee::pay_vcoin_fee(ctx, max_fee)
    }

    pub fn withdraw_fees(ctx: Context<WithdrawFees>, amount: u64) -> Result<()> {
        instructions::fee::withdraw_fees(ctx, amount)
    }

    pub fn schedule_fee(ctx: Context<UpdateConfig>, new_fee: u64) -> Result<()> {
        instructions::admin::schedule_fee(ctx, new_fee)
    }

    pub fn apply_fee(ctx: Context<ApplyFee>) -> Result<()> {
        instructions::admin::apply_fee(ctx)
    }

    pub fn schedule_treasury(ctx: Context<UpdateConfig>, new_treasury: Pubkey) -> Result<()> {
        instructions::admin::schedule_treasury(ctx, new_treasury)
    }

    pub fn apply_treasury(ctx: Context<ApplyTreasury>) -> Result<()> {
        instructions::admin::apply_treasury(ctx)
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
}
