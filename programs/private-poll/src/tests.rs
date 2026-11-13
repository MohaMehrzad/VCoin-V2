//! Unit tests for state and parameters. Instruction behaviour lives in
//! `integration-tests/tests/private_poll.rs`; cryptography in `zk-poll-core` and
//! `zk-voting-sdk`.

use crate::constants::*;
use crate::errors::PollError;
use crate::state::*;
use anchor_lang::prelude::*;
use zk_poll_core::ZkError;

/// `init` allocates through a system-program CPI, capped at 10 KiB.
const _: () = assert!(Poll::LEN <= 10_240);

#[test]
fn account_lens_match_borsh_size() {
    let mut buf = Vec::new();
    PollConfig {
        admin: Default::default(),
        registrar: Pubkey::default(),
        committee: Committee::default(),
        params: PollParams::default(),
        poll_count: 0,
        paused: false,
        paused_since: 0,
        pause_accum: 0,
        pause_seq: 0,
        pause_history: Default::default(),
        bump: 0,
    }
    .serialize(&mut buf)
    .unwrap();
    assert_eq!(PollConfig::LEN, 8 + buf.len());
}

#[test]
fn default_params_are_valid_and_bounds_hold() {
    assert!(PollParams::default().is_valid());
    for bad in [
        PollParams {
            quorum_bps: 10_001,
            ..Default::default()
        },
        PollParams {
            pass_bps: 4_999,
            ..Default::default()
        },
        PollParams {
            pass_bps: 10_000,
            ..Default::default()
        },
        PollParams {
            min_voting_secs: DAY - 1,
            ..Default::default()
        },
        PollParams {
            min_voting_secs: 10 * DAY,
            max_voting_secs: 9 * DAY,
            ..Default::default()
        },
        PollParams {
            max_voting_secs: MAX_VOTING_SECS_BOUND + 1,
            ..Default::default()
        },
        PollParams {
            reveal_window_secs: DAY - 1,
            ..Default::default()
        },
        PollParams {
            reveal_window_secs: MAX_REVEAL_WINDOW_BOUND + 1,
            ..Default::default()
        },
    ] {
        assert!(!bad.is_valid(), "{bad:?}");
    }
}

#[test]
fn every_verifier_error_has_a_program_error() {
    for (zk, poll) in [
        (ZkError::InvalidPoint, PollError::InvalidPoint),
        (ZkError::NonCanonicalScalar, PollError::NonCanonicalScalar),
        (ZkError::InvalidBallotProof, PollError::InvalidBallotProof),
        (ZkError::InvalidDleqProof, PollError::InvalidDleqProof),
        (ZkError::FeldmanMismatch, PollError::FeldmanMismatch),
        (ZkError::InvalidCommittee, PollError::InvalidCommittee),
        (ZkError::TallyMismatch, PollError::TallyMismatch),
    ] {
        assert_eq!(PollError::from(zk) as u32, poll as u32);
    }
}
