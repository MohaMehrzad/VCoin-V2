//! ViLink: shareable action links. A creator publishes a link (tip, vouch or follow) that
//! anyone except the creator and the target can execute once each, until it expires or
//! reaches `max_executions`.
//!
//! * Tips transfer VCoin (Token-2022, no hook) from the executor to the target, minus the fee
//!   the link was created with (rounded up), and always need the executor's wallet.
//! * Vouches and follows only update stats and may be signed by a gasless-protocol session key
//!   (scope VOUCH / FOLLOW).
//! * Disabling a type stops creation and execution at once; fee and treasury changes wait
//!   24h; links and receipts can be closed when done.
//!
//! Removed: never-executable types 3-7, batches, the unenforced dApp registry, global
//! counters (audit VL-6/7/8/13).

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

declare_id!("CFGXTS2MueQwTYTMMTBQbRWzJtSTC2p4ZRuKPpLDmrv7");

#[program]
pub mod vilink_protocol {
    use super::*;

    /// Upgrade authority only; VCoin mint pinned.
    pub fn initialize(ctx: Context<Initialize>, fee_bps: u16) -> Result<()> {
        instructions::admin::initialize(ctx, fee_bps)
    }

    pub fn propose_config(ctx: Context<ProposeConfig>, fee_bps: u16) -> Result<()> {
        instructions::admin::propose_config(ctx, fee_bps)
    }

    pub fn apply_config(ctx: Context<ApplyConfig>) -> Result<()> {
        instructions::admin::apply_config(ctx)
    }

    pub fn cancel_config(ctx: Context<UpdateConfig>) -> Result<()> {
        instructions::admin::cancel_config(ctx)
    }

    pub fn set_enabled_actions(ctx: Context<UpdateConfig>, enabled_actions: u8) -> Result<()> {
        instructions::admin::set_enabled_actions(ctx, enabled_actions)
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

    #[allow(clippy::too_many_arguments)]
    pub fn create_action(
        ctx: Context<CreateAction>,
        action_type: u8,
        amount: u64,
        target: Pubkey,
        metadata_hash: [u8; 32],
        expiry_secs: i64,
        max_executions: u32,
        nonce: u64,
    ) -> Result<()> {
        instructions::action::create_action(
            ctx,
            action_type,
            amount,
            target,
            metadata_hash,
            expiry_secs,
            max_executions,
            nonce,
        )
    }

    pub fn execute_tip(ctx: Context<ExecuteTip>) -> Result<()> {
        instructions::action::execute_tip(ctx)
    }

    pub fn execute_vouch(ctx: Context<ExecuteSocial>) -> Result<()> {
        instructions::action::execute_vouch(ctx)
    }

    pub fn execute_follow(ctx: Context<ExecuteSocial>) -> Result<()> {
        instructions::action::execute_follow(ctx)
    }

    pub fn close_action(ctx: Context<CloseAction>) -> Result<()> {
        instructions::action::close_action(ctx)
    }

    pub fn close_receipt(ctx: Context<CloseReceipt>) -> Result<()> {
        instructions::action::close_receipt(ctx)
    }
}
