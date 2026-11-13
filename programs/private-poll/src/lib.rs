//! VCoin private polls (formerly governance-protocol; same program id).
//!
//! Binding governance runs on Realms with staking-protocol as voter-weight plugin. This
//! program adds secret-ballot polls whose result a Realms proposal can require through
//! [`private_poll::assert_poll_passed`]:
//! * ballots are exponential-ElGamal encrypted under a DAO-registered committee key, with a
//!   448-byte CDS proof that exactly one choice carries the voter's weight (audit ZK-01);
//! * the weight is the voter's staking weight at the poll's end time, computed on-chain;
//! * the committee comes from a Pedersen DKG and is Feldman-checked at registration
//!   (ZK-03); polls snapshot it;
//! * members post DLEQ-proven partial decryptions after voting ends; anyone finalizes once
//!   with a tally the program verifies; a poll can be consumed by its named consumer once.
//!
//! All cryptography lives in `zk-poll-core`, shared with the off-chain SDK.

use anchor_lang::prelude::*;

pub mod constants;
pub mod contexts;
pub mod errors;
pub mod events;
pub mod instructions;
pub mod state;

#[cfg(test)]
mod tests;

use constants::{BALLOT_LEN, NUM_CHOICES};
use contexts::*;
use state::PollParams;
use zk_poll_core::{BALLOT_PROOF_LEN, DLEQ_PROOF_LEN};

declare_id!("3R256kBN9iXozjypQFRAmegBhd6HJqXWqdNG7Th78HYe");

#[program]
pub mod private_poll {
    use super::*;

    /// Creates the config pointing at the staking registrar. Upgrade authority only.
    pub fn initialize_config(ctx: Context<InitializeConfig>, params: PollParams) -> Result<()> {
        instructions::config::initialize_config(ctx, params)
    }

    /// Registers the decryption committee (authority = the DAO). Feldman-verified.
    pub fn set_committee(
        ctx: Context<UpdateConfig>,
        threshold: u8,
        commitments: Vec<[u8; 32]>,
        member_pks: Vec<[u8; 32]>,
        member_authorities: Vec<Pubkey>,
    ) -> Result<()> {
        instructions::config::set_committee(
            ctx,
            threshold,
            commitments,
            member_pks,
            member_authorities,
        )
    }

    pub fn set_poll_params(ctx: Context<UpdateConfig>, params: PollParams) -> Result<()> {
        instructions::config::set_poll_params(ctx, params)
    }

    pub fn set_paused(ctx: Context<UpdateConfig>, paused: bool) -> Result<()> {
        instructions::config::set_paused(ctx, paused)
    }

    pub fn propose_authority(ctx: Context<UpdateConfig>, new_authority: Pubkey) -> Result<()> {
        instructions::config::propose_authority(ctx, new_authority)
    }

    pub fn cancel_authority_transfer(ctx: Context<UpdateConfig>) -> Result<()> {
        instructions::config::cancel_authority_transfer(ctx)
    }

    pub fn accept_authority(ctx: Context<AcceptAuthority>) -> Result<()> {
        instructions::config::accept_authority(ctx)
    }

    /// Opens a poll. `consumer` is the only signer that may later consume a pass.
    pub fn create_poll(
        ctx: Context<CreatePoll>,
        description_hash: [u8; 32],
        start_time: i64,
        end_time: i64,
        consumer: Pubkey,
    ) -> Result<()> {
        instructions::poll::create_poll(ctx, description_hash, start_time, end_time, consumer)
    }

    /// Casts an encrypted ballot (`R ‖ C` per choice) with its validity proof.
    pub fn cast_vote(
        ctx: Context<CastVote>,
        ballot: [u8; BALLOT_LEN],
        proof: [u8; BALLOT_PROOF_LEN],
    ) -> Result<()> {
        instructions::poll::cast_vote(ctx, ballot, proof)
    }

    /// Committee member `member_index` (1-based) posts its partial decryption.
    pub fn submit_share(
        ctx: Context<SubmitShare>,
        member_index: u8,
        shares: [[u8; 32]; NUM_CHOICES],
        proof: [u8; DLEQ_PROOF_LEN],
    ) -> Result<()> {
        instructions::poll::submit_share(ctx, member_index, shares, proof)
    }

    /// Settles the poll once with a verified tally.
    pub fn finalize_poll(
        ctx: Context<SettlePoll>,
        tally: [u64; NUM_CHOICES],
        lagrange: Vec<[u8; 32]>,
    ) -> Result<()> {
        instructions::poll::finalize_poll(ctx, tally, lagrange)
    }

    pub fn expire_poll(ctx: Context<SettlePoll>) -> Result<()> {
        instructions::poll::expire_poll(ctx)
    }

    /// Fails unless the poll passed and this is its first use by its consumer.
    pub fn assert_poll_passed(ctx: Context<AssertPollPassed>) -> Result<()> {
        instructions::poll::assert_poll_passed(ctx)
    }

    pub fn close_poll(ctx: Context<ClosePoll>) -> Result<()> {
        instructions::poll::close_poll(ctx)
    }

    pub fn close_ballot(ctx: Context<CloseBallot>) -> Result<()> {
        instructions::poll::close_ballot(ctx)
    }
}
