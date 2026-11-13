//! 5A reputation: oracle-consensus scores and reputation-only vouching.
//!
//! * Up to 7 oracles (24h activation, removable). A user's five dimensions change only when
//!   `required_consensus` (2..=5) active oracles submit identical values within an hour;
//!   competing submissions are tallied separately (audit 5A-01/04).
//! * The composite score gates participation in staking-protocol (Realms) and private-poll.
//! * Vouching carries no stake or reward: an established user vouches for a new one, an oracle
//!   records the outcome after 90 days, and unevaluated vouches expire after a year.
//!
//! Removed: single-oracle "consensus", vouch stake/reward numbers, private mode, `get_score`,
//! unread snapshots (audit 5A-05/09, 5A-L2/L4).

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
use state::ScoreValues;

declare_id!("783PbtJw5cc7yatnr9fsvTGSnkKaV6iJe6E8VUPTYrT8");

#[program]
pub mod five_a_protocol {
    use super::*;

    /// Upgrade authority only.
    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        instructions::admin::initialize(ctx)
    }

    pub fn register_oracle(ctx: Context<UpdateConfig>, oracle: Pubkey) -> Result<()> {
        instructions::admin::register_oracle(ctx, oracle)
    }

    pub fn remove_oracle(ctx: Context<UpdateConfig>, oracle: Pubkey) -> Result<()> {
        instructions::admin::remove_oracle(ctx, oracle)
    }

    pub fn set_required_consensus(
        ctx: Context<UpdateConfig>,
        required_consensus: u8,
    ) -> Result<()> {
        instructions::admin::set_required_consensus(ctx, required_consensus)
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

    /// An active oracle submits (or changes) its values for `user`.
    pub fn submit_score(ctx: Context<SubmitScore>, values: ScoreValues) -> Result<()> {
        instructions::score::submit_score(ctx, values)
    }

    pub fn close_expired_round(ctx: Context<CloseExpiredRound>) -> Result<()> {
        instructions::score::close_expired_round(ctx)
    }

    pub fn vouch_for_user(ctx: Context<VouchForUser>) -> Result<()> {
        instructions::vouch::vouch_for_user(ctx)
    }

    pub fn evaluate_vouch(ctx: Context<EvaluateVouch>) -> Result<()> {
        instructions::vouch::evaluate_vouch(ctx)
    }

    pub fn expire_vouch(ctx: Context<CloseVouch>) -> Result<()> {
        instructions::vouch::expire_vouch(ctx)
    }

    /// The vouchee declines a vouch made for them.
    pub fn decline_vouch(ctx: Context<DeclineVouch>) -> Result<()> {
        instructions::vouch::decline_vouch(ctx)
    }
}
