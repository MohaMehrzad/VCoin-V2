//! VCoin staking: a Voter-Stake-Registry–style lockup program.
//!
//! * Users lock VCoin in up to 8 deposits (1 week – 4 years) and receive a 1:1
//!   non-transferable veVCoin receipt (audit STK-03: the receipt can never drift).
//! * Voting weight is computed, not minted: linear in amount plus a lock-time bonus that
//!   decays to zero at unlock (see [`weight`]); it feeds SPL Governance (Realms) as a
//!   voter-weight plugin.
//! * Withdrawals are never paused; deposits credit exactly what the vault receives and the
//!   VCoin mint must be hook/fee/freeze-free (audit STK-01).
//! * The registrar is created only by the upgrade authority (audit X-1) and its authority
//!   moves through a 24h two-step transfer (audit X-2).

use anchor_lang::prelude::*;

pub mod constants;
pub mod contexts;
pub mod errors;
pub mod events;
pub mod governance;
pub mod instructions;
pub mod state;
pub mod weight;

#[cfg(test)]
mod tests;

use contexts::*;
use state::{VoterWeightAction, WeightParams};

declare_id!("6EFcistyr2E81adLUcuBJRr8W2xzpt3D3dFYEcMewpWu");

#[program]
pub mod staking_protocol {
    use super::*;

    /// Creates the registrar and VCoin vault. Upgrade authority only.
    pub fn create_registrar(ctx: Context<CreateRegistrar>) -> Result<()> {
        instructions::admin::create_registrar(ctx)
    }

    /// Creates the caller's voter account.
    pub fn create_voter(ctx: Context<CreateVoter>) -> Result<()> {
        instructions::stake::create_voter(ctx)
    }

    /// Locks VCoin into deposit slot `index` for at least `lock_secs` seconds.
    pub fn deposit(ctx: Context<MoveStake>, index: u8, amount: u64, lock_secs: i64) -> Result<()> {
        instructions::stake::deposit(ctx, index, amount, lock_secs)
    }

    /// Moves a deposit's lock end to `now + lock_secs` (must be later than today's end).
    pub fn extend_lock(ctx: Context<ExtendLock>, index: u8, lock_secs: i64) -> Result<()> {
        instructions::stake::extend_lock(ctx, index, lock_secs)
    }

    /// Withdraws unlocked VCoin from deposit slot `index`.
    pub fn withdraw(ctx: Context<Withdraw>, index: u8, amount: u64) -> Result<()> {
        instructions::stake::withdraw(ctx, index, amount)
    }

    /// Closes an empty voter account and refunds its rent.
    pub fn close_voter(ctx: Context<CloseVoter>) -> Result<()> {
        instructions::stake::close_voter(ctx)
    }

    /// Wires the registrar to its Realms realm as voter-weight plugin (one-shot).
    pub fn configure_governance(
        ctx: Context<ConfigureGovernance>,
        governance_program_id: Pubkey,
        min_vote_score: u16,
        min_propose_score: u16,
    ) -> Result<()> {
        instructions::realms::configure_governance(
            ctx,
            governance_program_id,
            min_vote_score,
            min_propose_score,
        )
    }

    pub fn create_voter_weight_record(ctx: Context<CreateVoterWeightRecord>) -> Result<()> {
        instructions::realms::create_voter_weight_record(ctx)
    }

    /// Writes the voter's weight for one Realms action (valid for the current slot).
    pub fn update_voter_weight_record(
        ctx: Context<UpdateVoterWeightRecord>,
        action: VoterWeightAction,
        target: Option<Pubkey>,
    ) -> Result<()> {
        instructions::realms::update_voter_weight_record(ctx, action, target)
    }

    pub fn update_max_voter_weight_record(ctx: Context<UpdateMaxVoterWeightRecord>) -> Result<()> {
        instructions::realms::update_max_voter_weight_record(ctx)
    }

    pub fn set_gate_params(
        ctx: Context<UpdateRegistrar>,
        min_vote_score: u16,
        min_propose_score: u16,
    ) -> Result<()> {
        instructions::realms::set_gate_params(ctx, min_vote_score, min_propose_score)
    }

    pub fn set_paused(ctx: Context<UpdateRegistrar>, paused: bool) -> Result<()> {
        instructions::admin::set_paused(ctx, paused)
    }

    pub fn set_weight_params(ctx: Context<UpdateRegistrar>, params: WeightParams) -> Result<()> {
        instructions::admin::set_weight_params(ctx, params)
    }

    pub fn propose_authority(ctx: Context<UpdateRegistrar>, new_authority: Pubkey) -> Result<()> {
        instructions::admin::propose_authority(ctx, new_authority)
    }

    pub fn cancel_authority_transfer(ctx: Context<UpdateRegistrar>) -> Result<()> {
        instructions::admin::cancel_authority_transfer(ctx)
    }

    pub fn accept_authority(ctx: Context<AcceptAuthority>) -> Result<()> {
        instructions::admin::accept_authority(ctx)
    }
}
