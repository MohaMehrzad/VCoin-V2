//! SSCRE rewards: a per-epoch merkle distributor (Jito / Saber pattern).
//!
//! * The reserve vault holds undistributed rewards. Each epoch, a permissionless crank moves
//!   `min(next_epoch_budget, reserve)` into that epoch's own vault and decays the next
//!   budget by `decay_bps`; epochs start at most once per `epoch_duration`.
//! * After an epoch ends, registered oracles (24h activation) vote on
//!   `(merkle root, max_total_claim ≤ budget)`. A candidate with `root_quorum` matching votes
//!   becomes claimable after the dispute delay unless the DAO vetoes it.
//! * Claims verify a keccak leaf (format in [`merkle`]), are capped by the epoch's
//!   `max_total_claim` and its vault, and are recorded per (epoch, claimant).
//! * After the claim window (extended by any pause), the remainder returns to the reserve.
//!
//! Audit: SSC-01..13 (old circuit breaker, funding layers and 5A multipliers removed).

use anchor_lang::prelude::*;

pub mod constants;
pub mod contexts;
pub mod errors;
pub mod events;
pub mod instructions;
pub mod merkle;
pub mod state;

#[cfg(test)]
mod tests;

use contexts::*;
use state::PoolParams;

declare_id!("6AJNcQSfoiE2UAeUDyJUBumS9SBwhAdSznoAeYpXrxXZ");

#[program]
pub mod sscre_protocol {
    use super::*;

    /// Creates the pool and reserve vault. Upgrade authority only; VCoin mint pinned.
    pub fn initialize_pool(
        ctx: Context<InitializePool>,
        params: PoolParams,
        first_epoch_budget: u64,
        fee_bps: u16,
    ) -> Result<()> {
        instructions::admin::initialize_pool(ctx, params, first_epoch_budget, fee_bps)
    }

    pub fn register_oracle(ctx: Context<UpdateConfig>, oracle: Pubkey) -> Result<()> {
        instructions::admin::register_oracle(ctx, oracle)
    }

    pub fn remove_oracle(ctx: Context<UpdateConfig>, oracle: Pubkey) -> Result<()> {
        instructions::admin::remove_oracle(ctx, oracle)
    }

    pub fn set_root_quorum(ctx: Context<UpdateConfig>, quorum: u8) -> Result<()> {
        instructions::admin::set_root_quorum(ctx, quorum)
    }

    pub fn set_params(ctx: Context<UpdateConfig>, params: PoolParams) -> Result<()> {
        instructions::admin::set_params(ctx, params)
    }

    pub fn set_next_epoch_budget(ctx: Context<UpdateConfig>, budget: u64) -> Result<()> {
        instructions::admin::set_next_epoch_budget(ctx, budget)
    }

    pub fn propose_fee(ctx: Context<ProposeFee>, fee_bps: u16) -> Result<()> {
        instructions::admin::propose_fee(ctx, fee_bps)
    }

    pub fn apply_fee(ctx: Context<ApplyFee>) -> Result<()> {
        instructions::admin::apply_fee(ctx)
    }

    pub fn cancel_fee(ctx: Context<UpdateConfig>) -> Result<()> {
        instructions::admin::cancel_fee(ctx)
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

    /// Funds the next epoch from the reserve (permissionless, rate-limited).
    pub fn start_epoch(ctx: Context<StartEpoch>) -> Result<()> {
        instructions::epoch::start_epoch(ctx)
    }

    pub fn vote_root(ctx: Context<VoteRoot>, root: [u8; 32], max_total_claim: u64) -> Result<()> {
        instructions::epoch::vote_root(ctx, root, max_total_claim)
    }

    pub fn veto_root(ctx: Context<VetoRoot>) -> Result<()> {
        instructions::epoch::veto_root(ctx)
    }

    pub fn activate_root(ctx: Context<ActivateRoot>) -> Result<()> {
        instructions::epoch::activate_root(ctx)
    }

    pub fn claim(ctx: Context<Claim>, amount: u64, proof: Vec<[u8; 32]>) -> Result<()> {
        instructions::epoch::claim(ctx, amount, proof)
    }

    pub fn clawback(ctx: Context<Clawback>) -> Result<()> {
        instructions::epoch::clawback(ctx)
    }

    pub fn close_claim_status(ctx: Context<CloseClaimStatus>) -> Result<()> {
        instructions::epoch::close_claim_status(ctx)
    }
}
