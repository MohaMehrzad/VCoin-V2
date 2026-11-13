//! Poll lifecycle: create -> vote (start..end) -> decryption shares (end..reveal deadline)
//! -> finalize (one-shot) or expire -> consume (one-shot) -> close.

use anchor_lang::prelude::*;
use solana_sha256_hasher::hash;
use staking_protocol::governance::five_a_score;
use staking_protocol::weight::{max_weight_for, voter_weight};
use zk_poll_core::{
    ballot_from_bytes, point, poll_weight, verify_ballot, verify_dleq, verify_tally, BallotContext,
    DleqContext, BALLOT_PROOF_LEN, DLEQ_PROOF_LEN, IDENTITY,
};

use crate::constants::*;
use crate::contexts::*;
use crate::errors::{zk, PollError};
use crate::events::*;
use crate::state::PollOutcome;

fn now() -> Result<i64> {
    Ok(Clock::get()?.unix_timestamp)
}

pub fn create_poll(
    ctx: Context<CreatePoll>,
    description_hash: [u8; 32],
    start_time: i64,
    end_time: i64,
    consumer: Pubkey,
) -> Result<()> {
    let config = &mut ctx.accounts.config;
    require!(!config.paused, PollError::Paused);
    require!(config.committee.epoch > 0, PollError::CommitteeNotSet);
    require!(consumer != Pubkey::default(), PollError::InvalidConsumer);
    let now = now()?;
    let params = config.params;
    let duration = end_time
        .checked_sub(start_time)
        .ok_or(PollError::Overflow)?;
    require!(
        start_time >= now
            && start_time - now <= MAX_START_DELAY_SECS
            && (params.min_voting_secs..=params.max_voting_secs).contains(&duration),
        PollError::InvalidSchedule
    );

    let registrar = &ctx.accounts.registrar;
    let creator_weight = poll_weight(
        voter_weight(&ctx.accounts.creator_voter, &registrar.weight, now)
            .ok_or(PollError::Overflow)?,
    );
    require!(
        creator_weight >= params.min_creator_weight,
        PollError::CreatorWeightTooLow
    );
    let max_weight = poll_weight(
        max_weight_for(registrar.active_locked, &registrar.weight).ok_or(PollError::Overflow)?,
    );
    let quorum_weight = (max_weight as u128 * params.quorum_bps as u128 / BPS as u128) as u64;

    let poll = &mut ctx.accounts.poll;
    poll.id = config.poll_count;
    poll.creator = ctx.accounts.creator.key();
    poll.consumer = consumer;
    poll.description_hash = description_hash;
    poll.start_time = start_time;
    poll.end_time = end_time;
    poll.reveal_deadline = end_time
        .checked_add(params.reveal_window_secs)
        .ok_or(PollError::Overflow)?;
    poll.pause_accum_at_creation = config.pause_accum;
    poll.weight_params = registrar.weight;
    poll.quorum_weight = quorum_weight;
    poll.pass_bps = params.pass_bps;
    poll.committee = config.committee;
    poll.agg_r = [IDENTITY; NUM_CHOICES];
    poll.agg_c = [IDENTITY; NUM_CHOICES];
    poll.outcome = PollOutcome::Open;
    poll.bump = ctx.bumps.poll;
    config.poll_count = config
        .poll_count
        .checked_add(1)
        .ok_or(PollError::Overflow)?;

    emit!(PollCreated {
        poll: poll.key(),
        id: poll.id,
        creator: poll.creator,
        consumer,
        description_hash,
        start_time,
        end_time,
        reveal_deadline: poll.reveal_deadline,
        committee_epoch: poll.committee.epoch,
        quorum_weight,
    });
    Ok(())
}

/// Casts an encrypted ballot. The weight is the voter's staking weight at the poll's end
/// time (so only stake locked through the poll counts and nobody can vote twice with the
/// same tokens), in whole-VCoin units; the proof must be for exactly that weight.
pub fn cast_vote(
    ctx: Context<CastVote>,
    ballot: [u8; BALLOT_LEN],
    proof: [u8; BALLOT_PROOF_LEN],
) -> Result<()> {
    require!(!ctx.accounts.config.paused, PollError::Paused);
    let now = now()?;
    let owner = ctx.accounts.owner.key();
    let poll_key = ctx.accounts.poll.key();
    let poll = &mut ctx.accounts.poll;
    require!(
        poll.outcome == PollOutcome::Open && now >= poll.start_time && now < poll.end_time,
        PollError::PollNotOpen
    );

    let staking_weight = voter_weight(&ctx.accounts.stake, &poll.weight_params, poll.end_time)
        .ok_or(PollError::Overflow)?;
    let weight = poll_weight(staking_weight);
    require!(weight > 0, PollError::NoVotingWeight);
    let gate = ctx.accounts.registrar.min_vote_score;
    if gate > 0 {
        let score = five_a_score(&owner, &ctx.accounts.user_score.to_account_info())?;
        require!(score >= gate, PollError::ScoreBelowGate);
    }

    let parsed = ballot_from_bytes(&ballot);
    let context = BallotContext {
        program_id: crate::ID.to_bytes(),
        poll: poll_key.to_bytes(),
        voter: owner.to_bytes(),
        election_pk: poll.committee.election_pk,
        weight,
    };
    zk(verify_ballot(&context, &parsed, &proof))?;

    for (k, ct) in parsed.iter().enumerate() {
        poll.agg_r[k] = point::add(&poll.agg_r[k], &ct.r).map_err(PollError::from)?;
        poll.agg_c[k] = point::add(&poll.agg_c[k], &ct.c).map_err(PollError::from)?;
    }
    poll.total_weight = poll
        .total_weight
        .checked_add(weight)
        .ok_or(PollError::Overflow)?;
    poll.voter_count = poll.voter_count.checked_add(1).ok_or(PollError::Overflow)?;

    let record = &mut ctx.accounts.ballot_record;
    record.poll = poll_key;
    record.voter = owner;
    record.weight = weight;
    record.ballot_hash = hash(&ballot).to_bytes();
    record.cast_at = now;
    record.bump = ctx.bumps.ballot_record;

    emit!(BallotCast {
        poll: poll_key,
        voter: owner,
        weight,
        ballot,
        total_weight: poll.total_weight
    });
    Ok(())
}

/// A committee member's verified partial decryption of the aggregate.
pub fn submit_share(
    ctx: Context<SubmitShare>,
    member_index: u8,
    shares: [[u8; 32]; NUM_CHOICES],
    proof: [u8; DLEQ_PROOF_LEN],
) -> Result<()> {
    require!(!ctx.accounts.config.paused, PollError::Paused);
    let now = now()?;
    let poll_key = ctx.accounts.poll.key();
    let poll = &mut ctx.accounts.poll;
    require!(
        poll.outcome == PollOutcome::Open,
        PollError::PollAlreadyFinal
    );
    require!(now >= poll.end_time, PollError::VotingNotEnded);
    require!(
        now < poll.effective_reveal_deadline(&ctx.accounts.config, now),
        PollError::RevealClosed
    );
    let committee = &poll.committee;
    require!(
        member_index >= 1 && member_index <= committee.size,
        PollError::NotCommitteeMember
    );
    let i = (member_index - 1) as usize;
    require_keys_eq!(
        committee.member_authorities[i],
        ctx.accounts.member.key(),
        PollError::NotCommitteeMember
    );
    let bit = 1u8 << i;
    require!(
        poll.shares_bitmap & bit == 0,
        PollError::ShareAlreadySubmitted
    );

    let context = DleqContext {
        program_id: crate::ID.to_bytes(),
        poll: poll_key.to_bytes(),
        member_index,
        member_pk: committee.member_pks[i],
    };
    zk(verify_dleq(&context, &poll.agg_r, &shares, &proof))?;
    poll.partials[i] = shares;
    poll.shares_bitmap |= bit;

    emit!(ShareSubmitted {
        poll: poll_key,
        member_index,
        shares
    });
    Ok(())
}

/// One-shot settlement (audit GOV-01/GOV-08/ZK-05). With ballots, needs at least
/// `threshold` verified shares and `lagrange[j]` for the j-th submitted member in index
/// order; the claimed tally must decrypt the aggregate and sum to the weight cast.
pub fn finalize_poll(
    ctx: Context<SettlePoll>,
    tally: [u64; NUM_CHOICES],
    lagrange: Vec<[u8; 32]>,
) -> Result<()> {
    require!(!ctx.accounts.config.paused, PollError::Paused);
    let now = now()?;
    let poll_key = ctx.accounts.poll.key();
    let poll = &mut ctx.accounts.poll;
    require!(
        poll.outcome == PollOutcome::Open,
        PollError::PollAlreadyFinal
    );
    require!(now >= poll.end_time, PollError::VotingNotEnded);

    if poll.voter_count == 0 {
        require!(
            tally == [0; NUM_CHOICES] && lagrange.is_empty(),
            PollError::TallyMismatch
        );
    } else {
        require!(
            poll.shares_submitted() >= poll.committee.threshold as u32,
            PollError::NotEnoughShares
        );
        require!(
            lagrange.len() == poll.shares_submitted() as usize,
            PollError::InvalidLagrange
        );
        let mut coefficients = lagrange.iter();
        let pairs: Vec<([u8; 32], [[u8; 32]; NUM_CHOICES])> = (0..MAX_COMMITTEE)
            .filter(|i| poll.shares_bitmap & (1 << i) != 0)
            .map(|i| (*coefficients.next().unwrap(), poll.partials[i]))
            .collect();
        zk(verify_tally(&poll.agg_c, &pairs, &tally))?;
        let sum = tally
            .iter()
            .try_fold(0u64, |acc, t| acc.checked_add(*t))
            .ok_or(PollError::Overflow)?;
        require!(sum == poll.total_weight, PollError::TallyMismatch);
    }

    let (yes, no) = (tally[0] as u128, tally[1] as u128);
    let passed = poll.total_weight >= poll.quorum_weight
        && yes * BPS as u128 > (yes + no) * poll.pass_bps as u128;
    poll.outcome = if passed {
        PollOutcome::Passed
    } else {
        PollOutcome::Rejected
    };
    poll.tally = tally;
    poll.finalized_at = now;

    emit!(PollFinalized {
        poll: poll_key,
        outcome: poll.outcome,
        tally,
        total_weight: poll.total_weight,
        quorum_weight: poll.quorum_weight,
    });
    Ok(())
}

/// Anyone may expire a poll whose reveal window closed without enough shares; it can
/// never pass afterwards.
pub fn expire_poll(ctx: Context<SettlePoll>) -> Result<()> {
    // Members cannot post shares while paused, so nothing may expire meanwhile.
    require!(!ctx.accounts.config.paused, PollError::Paused);
    let now = now()?;
    let poll_key = ctx.accounts.poll.key();
    let poll = &mut ctx.accounts.poll;
    require!(
        poll.outcome == PollOutcome::Open,
        PollError::PollAlreadyFinal
    );
    require!(
        now >= poll.effective_reveal_deadline(&ctx.accounts.config, now),
        PollError::RevealStillOpen
    );
    require!(
        poll.voter_count > 0 && poll.shares_submitted() < poll.committee.threshold as u32,
        PollError::FinalizeInstead
    );
    poll.outcome = PollOutcome::Expired;
    poll.finalized_at = now;
    emit!(PollFinalized {
        poll: poll_key,
        outcome: PollOutcome::Expired,
        tally: [0; NUM_CHOICES],
        total_weight: poll.total_weight,
        quorum_weight: poll.quorum_weight,
    });
    Ok(())
}

/// For a Realms proposal transaction: fails unless the poll passed, the signer is the poll's
/// consumer (e.g. the governance PDA) and the pass was not used before.
pub fn assert_poll_passed(ctx: Context<AssertPollPassed>) -> Result<()> {
    require!(!ctx.accounts.config.paused, PollError::Paused);
    let poll_key = ctx.accounts.poll.key();
    let poll = &mut ctx.accounts.poll;
    require!(
        poll.outcome == PollOutcome::Passed,
        PollError::PollNotPassed
    );
    require_keys_eq!(
        poll.consumer,
        ctx.accounts.consumer.key(),
        PollError::WrongConsumer
    );
    require!(!poll.consumed, PollError::AlreadyConsumed);
    poll.consumed = true;
    emit!(PollConsumed {
        poll: poll_key,
        consumer: poll.consumer
    });
    Ok(())
}

pub fn close_poll(ctx: Context<ClosePoll>) -> Result<()> {
    let poll = &ctx.accounts.poll;
    require!(poll.outcome != PollOutcome::Open, PollError::PollNotFinal);
    if poll.outcome == PollOutcome::Passed && !poll.consumed {
        let grace_end = poll
            .finalized_at
            .checked_add(PASSED_CLOSE_GRACE_SECS)
            .ok_or(PollError::Overflow)?;
        require!(now()? >= grace_end, PollError::CloseTooEarly);
    }
    Ok(())
}

pub fn close_ballot(ctx: Context<CloseBallot>) -> Result<()> {
    let info = ctx.accounts.poll.to_account_info();
    if *info.owner == crate::ID && !info.data_is_empty() {
        let poll = crate::state::Poll::try_deserialize(&mut &info.try_borrow_data()?[..])?;
        require!(poll.outcome != PollOutcome::Open, PollError::PollNotFinal);
    }
    Ok(())
}
