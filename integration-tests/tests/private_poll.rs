//! private-poll end to end in LiteSVM: Pedersen DKG committee (zk-voting-sdk), stake-weighted
//! encrypted ballots verified on-chain, DLEQ shares, one-shot finalize, and a Realms
//! proposal gated by `assert_poll_passed`.
//!
//! Audit coverage: ZK-01..06, ZK-09, GOV-07/08 (private path), X-1 for the new program.

mod common;

use anchor_lang::{prelude::Pubkey, system_program};
use common::*;
use governance::{instruction as gov_ix, state::native_treasury::get_native_treasury_address};
use integration_tests::solana_sdk::{
    instruction::{AccountMeta, Instruction},
    signature::Keypair,
    signer::Signer,
};
use integration_tests::stack::*;
use integration_tests::*;
use private_poll::{
    constants::{BALLOT_SEED, CONFIG_SEED, POLL_SEED},
    errors::PollError,
    state::{BallotRecord, Poll, PollConfig, PollOutcome, PollParams},
};
use rand::rngs::OsRng;
use staking_protocol::state::Voter;
use vcoin_common::CommonError;
use zk_voting_sdk::{
    combine, encrypt_ballot, finish, partial_decrypt, poll_weight, BallotContext, Choice, Dealer,
    KeyShare, PartialDecryption, PublicDealing, ShareMessage,
};

const DAY: i64 = 24 * 60 * 60;
const VOTING: i64 = 3 * DAY;
const THRESHOLD: u8 = 3;
const SIZE: u8 = 5;

fn perr(e: PollError) -> u32 {
    6000 + e as u32
}

struct Committee {
    keys: Vec<KeyShare>,
    members: Vec<Keypair>,
}

fn config_pda() -> Pubkey {
    pda(&[CONFIG_SEED], &private_poll::ID)
}

fn poll_pda(id: u64) -> Pubkey {
    pda(&[POLL_SEED, &id.to_le_bytes()], &private_poll::ID)
}

fn ballot_pda(poll: &Pubkey, owner: &Pubkey) -> Pubkey {
    pda(
        &[BALLOT_SEED, poll.as_ref(), owner.as_ref()],
        &private_poll::ID,
    )
}

fn dkg() -> Vec<KeyShare> {
    let dealers: Vec<Dealer> = (1..=SIZE)
        .map(|i| Dealer::new(i, THRESHOLD, SIZE, &mut OsRng).unwrap())
        .collect();
    let dealings: Vec<PublicDealing> = dealers.iter().map(Dealer::public_dealing).collect();
    (1..=SIZE)
        .map(|i| {
            let shares: Vec<ShareMessage> =
                dealers.iter().map(|d| d.share_for(i).unwrap()).collect();
            finish(i, THRESHOLD, &dealings, &shares).unwrap()
        })
        .collect()
}

fn init_config_ix(registrar: Pubkey, authority: Pubkey) -> Instruction {
    ix(
        private_poll::ID,
        private_poll::accounts::InitializeConfig {
            authority,
            config: config_pda(),
            registrar,
            program: private_poll::ID,
            program_data: programdata(&private_poll::ID),
            system_program: system_program::ID,
        },
        private_poll::instruction::InitializeConfig {
            params: PollParams::default(),
        },
    )
}

fn set_committee_ix(
    authority: Pubkey,
    threshold: u8,
    commitments: Vec<[u8; 32]>,
    member_pks: Vec<[u8; 32]>,
    member_authorities: Vec<Pubkey>,
) -> Instruction {
    ix(
        private_poll::ID,
        private_poll::accounts::UpdateConfig {
            config: config_pda(),
            authority,
        },
        private_poll::instruction::SetCommittee {
            threshold,
            commitments,
            member_pks,
            member_authorities,
        },
    )
}

/// Deploys private-poll, initializes the config and registers a fresh 3-of-5 committee.
fn install(s: &mut Stack) -> Committee {
    let admin = s.admin.insecure_clone();
    deploy_upgradeable(
        &mut s.svm,
        private_poll::ID,
        &program_so("private_poll"),
        Some(admin.pubkey()),
    );
    let init = init_config_ix(s.registrar, admin.pubkey());
    send(&mut s.svm, &[init], &admin, &[]).expect("initialize config");
    let keys = dkg();
    let members: Vec<Keypair> = (0..SIZE).map(|_| funded_keypair(&mut s.svm, 2)).collect();
    let g = &keys[0].group;
    let set = set_committee_ix(
        admin.pubkey(),
        THRESHOLD,
        g.commitments.clone(),
        g.member_pks.clone(),
        members.iter().map(|m| m.pubkey()).collect(),
    );
    send(&mut s.svm, &[set], &admin, &[]).expect("set committee");
    Committee { keys, members }
}

fn staker(s: &mut Stack, amount: u64) -> Staker {
    let st = new_staker(s, amount);
    let dep = deposit_ix(
        s,
        &st,
        0,
        amount,
        staking_protocol::constants::MAX_LOCK_SECS,
    );
    send(&mut s.svm, &[dep], &st.kp, &[]).expect("deposit");
    st
}

fn create_poll_ix(
    registrar: Pubkey,
    creator: &Staker,
    id: u64,
    start: i64,
    end: i64,
    consumer: Pubkey,
) -> Instruction {
    ix(
        private_poll::ID,
        private_poll::accounts::CreatePoll {
            config: config_pda(),
            registrar,
            creator_voter: creator.voter,
            creator: creator.kp.pubkey(),
            poll: poll_pda(id),
            system_program: system_program::ID,
        },
        private_poll::instruction::CreatePoll {
            description_hash: [9; 32],
            start_time: start,
            end_time: end,
            consumer,
        },
    )
}

/// Opens poll `id` starting in a minute, voting for [`VOTING`], and warps to its start.
fn open_poll(s: &mut Stack, creator: &Staker, consumer: Pubkey) -> Pubkey {
    let id = read_account::<PollConfig>(&s.svm, &config_pda()).poll_count;
    let start = now(&s.svm) + 60;
    let create = create_poll_ix(s.registrar, creator, id, start, start + VOTING, consumer);
    send(&mut s.svm, &[create], &creator.kp, &[]).expect("create poll");
    warp_to(&mut s.svm, start);
    poll_pda(id)
}

fn weight_of(s: &Stack, st: &Staker, poll: &Pubkey) -> u64 {
    let p: Poll = read_account(&s.svm, poll);
    let voter: Voter = read_account(&s.svm, &st.voter);
    poll_weight(
        staking_protocol::weight::voter_weight(&voter, &p.weight_params, p.end_time).unwrap(),
    )
}

fn ballot_ctx(s: &Stack, poll: &Pubkey, voter: &Pubkey, weight: u64) -> BallotContext {
    let p: Poll = read_account(&s.svm, poll);
    BallotContext {
        program_id: private_poll::ID.to_bytes(),
        poll: poll.to_bytes(),
        voter: voter.to_bytes(),
        election_pk: p.committee.election_pk,
        weight,
    }
}

fn cast_ix(
    registrar: Pubkey,
    poll: Pubkey,
    st: &Staker,
    ballot: [u8; 192],
    proof: [u8; 448],
) -> Instruction {
    let owner = st.kp.pubkey();
    ix(
        private_poll::ID,
        private_poll::accounts::CastVote {
            config: config_pda(),
            poll,
            registrar,
            stake: st.voter,
            owner,
            user_score: pda(
                &[five_a_protocol::constants::USER_SCORE_SEED, owner.as_ref()],
                &five_a_protocol::ID,
            ),
            ballot_record: ballot_pda(&poll, &owner),
            system_program: system_program::ID,
        },
        private_poll::instruction::CastVote { ballot, proof },
    )
}

/// Encrypts and casts `choice` with the voter's exact on-chain weight; returns (weight, CU).
fn vote_in(s: &mut Stack, poll: &Pubkey, st: &Staker, choice: Choice) -> (u64, u64) {
    let weight = weight_of(s, st, poll);
    let b = encrypt_ballot(
        &ballot_ctx(s, poll, &st.kp.pubkey(), weight),
        choice,
        &mut OsRng,
    )
    .unwrap();
    let cast = cast_ix(s.registrar, *poll, st, b.ballot_bytes(), b.proof);
    let meta = send(&mut s.svm, &[cast], &st.kp, &[]).expect("cast vote");
    (weight, meta.compute_units_consumed)
}

fn submit_share_ix(poll: Pubkey, member: Pubkey, p: &PartialDecryption) -> Instruction {
    ix(
        private_poll::ID,
        private_poll::accounts::SubmitShare {
            config: config_pda(),
            poll,
            member,
        },
        private_poll::instruction::SubmitShare {
            member_index: p.member_index,
            shares: p.shares,
            proof: p.proof,
        },
    )
}

fn partial(s: &Stack, c: &Committee, poll: &Pubkey, index: u8) -> PartialDecryption {
    let p: Poll = read_account(&s.svm, poll);
    partial_decrypt(
        &c.keys[index as usize - 1],
        private_poll::ID.to_bytes(),
        poll.to_bytes(),
        &p.agg_r,
        &mut OsRng,
    )
    .unwrap()
}

fn submit_shares(s: &mut Stack, c: &Committee, poll: &Pubkey, indices: &[u8]) {
    for &i in indices {
        let part = partial(s, c, poll, i);
        let member = c.members[i as usize - 1].insecure_clone();
        send(
            &mut s.svm,
            &[submit_share_ix(*poll, member.pubkey(), &part)],
            &member,
            &[],
        )
        .expect("share");
    }
}

fn finalize_ix(poll: Pubkey, tally: [u64; 3], lagrange: Vec<[u8; 32]>) -> Instruction {
    ix(
        private_poll::ID,
        private_poll::accounts::SettlePoll {
            config: config_pda(),
            poll,
        },
        private_poll::instruction::FinalizePoll { tally, lagrange },
    )
}

/// Off-chain tally from the on-chain shares (what any observer can compute).
fn tally_of(s: &Stack, poll: &Pubkey) -> ([u64; 3], Vec<[u8; 32]>) {
    let p: Poll = read_account(&s.svm, poll);
    let partials: Vec<PartialDecryption> = (0..SIZE as usize)
        .filter(|i| p.shares_bitmap & (1 << i) != 0)
        .map(|i| PartialDecryption {
            member_index: i as u8 + 1,
            shares: p.partials[i],
            proof: [0; 64],
        })
        .collect();
    let r = combine(p.committee.threshold, &p.agg_c, &partials, p.total_weight).unwrap();
    (r.tally, r.lagrange)
}

fn settle(s: &mut Stack, poll: &Pubkey) -> [u64; 3] {
    let (tally, lagrange) = tally_of(s, poll);
    let payer = s.admin.insecure_clone();
    send(
        &mut s.svm,
        &[finalize_ix(*poll, tally, lagrange)],
        &payer,
        &[],
    )
    .expect("finalize");
    tally
}

fn assert_ix(poll: Pubkey, consumer: Pubkey) -> Instruction {
    ix(
        private_poll::ID,
        private_poll::accounts::AssertPollPassed {
            config: config_pda(),
            poll,
            consumer,
        },
        private_poll::instruction::AssertPollPassed {},
    )
}

#[test]
fn two_hundred_private_ballots_tally_exactly_and_pass_once() {
    let mut s = setup();
    let c = install(&mut s);
    let creator = staker(&mut s, 1_000 * VCOIN);
    let consumer = Keypair::new();
    let poll = open_poll(&mut s, &creator, consumer.pubkey());

    let mut expected = [0u64; 3];
    let mut max_cu = 0;
    for i in 0..200u64 {
        let st = staker(&mut s, (10 + (i * 37) % 490) * VCOIN);
        let choice = [
            Choice::For,
            Choice::Against,
            Choice::For,
            Choice::Abstain,
            Choice::For,
        ][i as usize % 5];
        let (weight, cu) = vote_in(&mut s, &poll, &st, choice);
        expected[choice as usize] += weight;
        max_cu = max_cu.max(cu);
    }
    println!("cast_vote compute units (max of 200): {max_cu}");
    assert!(
        max_cu < 200_000,
        "fits the default compute budget: {max_cu}"
    );

    let p: Poll = read_account(&s.svm, &poll);
    assert_eq!(
        (p.voter_count, p.total_weight),
        (200, expected.iter().sum())
    );

    warp_by(&mut s.svm, VOTING);
    submit_shares(&mut s, &c, &poll, &[2, 4, 5]);
    let tally = settle(&mut s, &poll);
    assert_eq!(tally, expected);
    let p: Poll = read_account(&s.svm, &poll);
    assert_eq!((p.outcome, p.tally), (PollOutcome::Passed, expected));

    // Consumable once, by the named consumer only.
    let stranger = funded_keypair(&mut s.svm, 1);
    let r = send(
        &mut s.svm,
        &[assert_ix(poll, stranger.pubkey())],
        &stranger,
        &[],
    );
    assert_anchor_error(r, perr(PollError::WrongConsumer));
    let payer = s.admin.insecure_clone();
    send(
        &mut s.svm,
        &[assert_ix(poll, consumer.pubkey())],
        &payer,
        &[&consumer],
    )
    .expect("consume");
    warp_by(&mut s.svm, 1);
    let r = send(
        &mut s.svm,
        &[assert_ix(poll, consumer.pubkey())],
        &payer,
        &[&consumer],
    );
    assert_anchor_error(r, perr(PollError::AlreadyConsumed));
}

#[test]
fn ballots_bound_to_another_voter_weight_or_poll_are_rejected() {
    let mut s = setup();
    let _c = install(&mut s);
    let creator = staker(&mut s, 1_000 * VCOIN);
    // A 1-week lock made now ends < 4 days after a 3-day poll opened after it.
    let short = new_staker(&mut s, 50 * VCOIN);
    let dep = deposit_ix(
        &s,
        &short,
        0,
        50 * VCOIN,
        staking_protocol::constants::MIN_LOCK_SECS,
    );
    send(&mut s.svm, &[dep], &short.kp, &[]).expect("short lock");
    let poll = open_poll(&mut s, &creator, Pubkey::new_unique());
    let other_poll = open_poll(&mut s, &creator, Pubkey::new_unique());
    let alice = staker(&mut s, 100 * VCOIN);
    let bob = staker(&mut s, 100 * VCOIN);
    let w = weight_of(&s, &alice, &poll);
    assert_eq!(
        w,
        weight_of(&s, &bob, &poll),
        "same weight, so only the voter binding differs"
    );

    let alice_ballot = encrypt_ballot(
        &ballot_ctx(&s, &poll, &alice.kp.pubkey(), w),
        Choice::For,
        &mut OsRng,
    )
    .unwrap();
    // Bob replays Alice's ballot (ZK-06).
    let replay = cast_ix(
        s.registrar,
        poll,
        &bob,
        alice_ballot.ballot_bytes(),
        alice_ballot.proof,
    );
    assert_anchor_error(
        send(&mut s.svm, &[replay], &bob.kp, &[]),
        perr(PollError::InvalidBallotProof),
    );
    // Wrong weight.
    let heavy = encrypt_ballot(
        &ballot_ctx(&s, &poll, &bob.kp.pubkey(), w + 1),
        Choice::For,
        &mut OsRng,
    )
    .unwrap();
    let cast = cast_ix(s.registrar, poll, &bob, heavy.ballot_bytes(), heavy.proof);
    assert_anchor_error(
        send(&mut s.svm, &[cast], &bob.kp, &[]),
        perr(PollError::InvalidBallotProof),
    );
    // Ballot made for another poll.
    let elsewhere = encrypt_ballot(
        &ballot_ctx(&s, &other_poll, &bob.kp.pubkey(), w),
        Choice::For,
        &mut OsRng,
    )
    .unwrap();
    let cast = cast_ix(
        s.registrar,
        poll,
        &bob,
        elsewhere.ballot_bytes(),
        elsewhere.proof,
    );
    assert_anchor_error(
        send(&mut s.svm, &[cast], &bob.kp, &[]),
        perr(PollError::InvalidBallotProof),
    );
    // Non-canonical scalar (top byte 0xff).
    let mut bad = alice_ballot.proof;
    bad[95] = 0xff;
    let cast = cast_ix(s.registrar, poll, &alice, alice_ballot.ballot_bytes(), bad);
    assert_anchor_error(
        send(&mut s.svm, &[cast], &alice.kp, &[]),
        perr(PollError::NonCanonicalScalar),
    );

    // The honest ballot lands once; a second ballot from the same voter cannot.
    let cast = cast_ix(
        s.registrar,
        poll,
        &alice,
        alice_ballot.ballot_bytes(),
        alice_ballot.proof,
    );
    send(&mut s.svm, &[cast], &alice.kp, &[]).expect("honest ballot");
    let record: BallotRecord = read_account(&s.svm, &ballot_pda(&poll, &alice.kp.pubkey()));
    assert_eq!((record.voter, record.weight), (alice.kp.pubkey(), w));
    warp_by(&mut s.svm, 1);
    let again = encrypt_ballot(
        &ballot_ctx(&s, &poll, &alice.kp.pubkey(), w),
        Choice::Against,
        &mut OsRng,
    )
    .unwrap();
    let cast = cast_ix(s.registrar, poll, &alice, again.ballot_bytes(), again.proof);
    assert!(
        send(&mut s.svm, &[cast], &alice.kp, &[]).is_err(),
        "one ballot per voter"
    );

    // Stake locked when the poll opened but unlocking within min_lock_remaining_to_vote of
    // its end carries no weight (it could be withdrawn and re-staked elsewhere).
    assert_eq!(weight_of(&s, &short, &poll), 0);
    let b = encrypt_ballot(
        &ballot_ctx(&s, &poll, &short.kp.pubkey(), 1),
        Choice::For,
        &mut OsRng,
    )
    .unwrap();
    let cast = cast_ix(s.registrar, poll, &short, b.ballot_bytes(), b.proof);
    assert_anchor_error(
        send(&mut s.svm, &[cast], &short.kp, &[]),
        perr(PollError::NoVotingWeight),
    );

    // After the end: closed.
    warp_by(&mut s.svm, VOTING);
    let b = encrypt_ballot(
        &ballot_ctx(&s, &poll, &bob.kp.pubkey(), w),
        Choice::For,
        &mut OsRng,
    )
    .unwrap();
    let cast = cast_ix(s.registrar, poll, &bob, b.ballot_bytes(), b.proof);
    assert_anchor_error(
        send(&mut s.svm, &[cast], &bob.kp, &[]),
        perr(PollError::PollNotOpen),
    );
}

#[test]
fn shares_are_verified_and_finalize_runs_once() {
    let mut s = setup();
    let c = install(&mut s);
    let creator = staker(&mut s, 1_000 * VCOIN);
    let poll = open_poll(&mut s, &creator, Pubkey::new_unique());
    for i in 0..4u64 {
        let st = staker(&mut s, 200 * VCOIN);
        vote_in(
            &mut s,
            &poll,
            &st,
            if i == 0 { Choice::For } else { Choice::Against },
        );
    }

    // Too early.
    let part = partial(&s, &c, &poll, 1);
    let m1 = c.members[0].insecure_clone();
    let r = send(
        &mut s.svm,
        &[submit_share_ix(poll, m1.pubkey(), &part)],
        &m1,
        &[],
    );
    assert_anchor_error(r, perr(PollError::VotingNotEnded));
    warp_by(&mut s.svm, VOTING);

    // Wrong signer, forged share, then the real one twice.
    let m2 = c.members[1].insecure_clone();
    let r = send(
        &mut s.svm,
        &[submit_share_ix(poll, m2.pubkey(), &part)],
        &m2,
        &[],
    );
    assert_anchor_error(r, perr(PollError::NotCommitteeMember));
    let mut forged = part.clone();
    forged.shares[0] = forged.shares[1];
    let r = send(
        &mut s.svm,
        &[submit_share_ix(poll, m1.pubkey(), &forged)],
        &m1,
        &[],
    );
    assert_anchor_error(r, perr(PollError::InvalidDleqProof));
    send(
        &mut s.svm,
        &[submit_share_ix(poll, m1.pubkey(), &part)],
        &m1,
        &[],
    )
    .expect("share 1");
    warp_by(&mut s.svm, 1);
    let r = send(
        &mut s.svm,
        &[submit_share_ix(poll, m1.pubkey(), &part)],
        &m1,
        &[],
    );
    assert_anchor_error(r, perr(PollError::ShareAlreadySubmitted));

    // Below threshold.
    submit_shares(&mut s, &c, &poll, &[3]);
    let payer = s.admin.insecure_clone();
    let r = send(
        &mut s.svm,
        &[finalize_ix(poll, [0; 3], vec![[0; 32]; 2])],
        &payer,
        &[],
    );
    assert_anchor_error(r, perr(PollError::NotEnoughShares));

    submit_shares(&mut s, &c, &poll, &[5]);
    let (tally, lagrange) = tally_of(&s, &poll);
    let mut wrong = tally;
    wrong.swap(0, 1);
    let r = send(
        &mut s.svm,
        &[finalize_ix(poll, wrong, lagrange.clone())],
        &payer,
        &[],
    );
    assert_anchor_error(r, perr(PollError::TallyMismatch));
    let r = send(
        &mut s.svm,
        &[finalize_ix(poll, tally, lagrange[..2].to_vec())],
        &payer,
        &[],
    );
    assert_anchor_error(r, perr(PollError::InvalidLagrange));

    send(
        &mut s.svm,
        &[finalize_ix(poll, tally, lagrange.clone())],
        &payer,
        &[],
    )
    .expect("finalize");
    assert_eq!(
        read_account::<Poll>(&s.svm, &poll).outcome,
        PollOutcome::Rejected,
        "1 For vs 3 Against"
    );
    warp_by(&mut s.svm, 1);
    let r = send(
        &mut s.svm,
        &[finalize_ix(poll, tally, lagrange)],
        &payer,
        &[],
    );
    assert_anchor_error(r, perr(PollError::PollAlreadyFinal));
    let r = send(&mut s.svm, &[assert_ix(poll, payer.pubkey())], &payer, &[]);
    assert_anchor_error(r, perr(PollError::PollNotPassed));
}

#[test]
fn without_enough_shares_a_poll_expires_and_an_empty_poll_is_rejected() {
    let mut s = setup();
    let c = install(&mut s);
    let creator = staker(&mut s, 1_000 * VCOIN);
    let poll = open_poll(&mut s, &creator, Pubkey::new_unique());
    let st = staker(&mut s, 300 * VCOIN);
    vote_in(&mut s, &poll, &st, Choice::For);
    warp_by(&mut s.svm, VOTING);
    submit_shares(&mut s, &c, &poll, &[1, 2]);

    let expire = ix(
        private_poll::ID,
        private_poll::accounts::SettlePoll {
            config: config_pda(),
            poll,
        },
        private_poll::instruction::ExpirePoll {},
    );
    let payer = s.admin.insecure_clone();
    let r = send(&mut s.svm, std::slice::from_ref(&expire), &payer, &[]);
    assert_anchor_error(r, perr(PollError::RevealStillOpen));
    warp_by(&mut s.svm, PollParams::default().reveal_window_secs);
    let late = partial(&s, &c, &poll, 3);
    let m3 = c.members[2].insecure_clone();
    let r = send(
        &mut s.svm,
        &[submit_share_ix(poll, m3.pubkey(), &late)],
        &m3,
        &[],
    );
    assert_anchor_error(r, perr(PollError::RevealClosed));
    send(&mut s.svm, &[expire], &payer, &[]).expect("expire");
    assert_eq!(
        read_account::<Poll>(&s.svm, &poll).outcome,
        PollOutcome::Expired
    );

    // A poll nobody voted in settles with a zero tally and no shares.
    let empty = open_poll(&mut s, &creator, Pubkey::new_unique());
    warp_by(&mut s.svm, VOTING);
    let r = send(
        &mut s.svm,
        &[finalize_ix(empty, [1, 0, 0], vec![])],
        &payer,
        &[],
    );
    assert_anchor_error(r, perr(PollError::TallyMismatch));
    send(
        &mut s.svm,
        &[finalize_ix(empty, [0; 3], vec![])],
        &payer,
        &[],
    )
    .expect("finalize empty");
    assert_eq!(
        read_account::<Poll>(&s.svm, &empty).outcome,
        PollOutcome::Rejected
    );
}

#[test]
fn config_and_committee_registration_are_guarded() {
    let mut s = setup();
    let admin = s.admin.insecure_clone();
    deploy_upgradeable(
        &mut s.svm,
        private_poll::ID,
        &program_so("private_poll"),
        Some(admin.pubkey()),
    );
    let stranger = funded_keypair(&mut s.svm, 2);
    let r = send(
        &mut s.svm,
        &[init_config_ix(s.registrar, stranger.pubkey())],
        &stranger,
        &[],
    );
    assert_anchor_error(r, u32::from(CommonError::NotUpgradeAuthority));
    send(
        &mut s.svm,
        &[init_config_ix(s.registrar, admin.pubkey())],
        &admin,
        &[],
    )
    .expect("init");

    // No committee yet.
    let creator = staker(&mut s, 1_000 * VCOIN);
    let start = now(&s.svm) + 60;
    let create = create_poll_ix(
        s.registrar,
        &creator,
        0,
        start,
        start + VOTING,
        Pubkey::new_unique(),
    );
    assert_anchor_error(
        send(&mut s.svm, &[create], &creator.kp, &[]),
        perr(PollError::CommitteeNotSet),
    );

    let keys = dkg();
    let g = keys[0].group.clone();
    let auths: Vec<Pubkey> = (0..SIZE).map(|_| Pubkey::new_unique()).collect();
    let attempt = |s: &mut Stack,
                   signer: &Keypair,
                   t: u8,
                   cm: Vec<[u8; 32]>,
                   pks: Vec<[u8; 32]>,
                   a: Vec<Pubkey>| {
        send(
            &mut s.svm,
            &[set_committee_ix(signer.pubkey(), t, cm, pks, a)],
            signer,
            &[],
        )
    };

    let r = attempt(
        &mut s,
        &stranger,
        THRESHOLD,
        g.commitments.clone(),
        g.member_pks.clone(),
        auths.clone(),
    );
    assert_anchor_error(r, u32::from(CommonError::NotAuthority));
    let mut wrong_pk = g.member_pks.clone();
    wrong_pk[4] = g.member_pks[0];
    let r = attempt(
        &mut s,
        &admin,
        THRESHOLD,
        g.commitments.clone(),
        wrong_pk,
        auths.clone(),
    );
    assert_anchor_error(r, perr(PollError::InvalidCommittee));
    let mut swapped = g.member_pks.clone();
    swapped.swap(0, 1);
    let r = attempt(
        &mut s,
        &admin,
        THRESHOLD,
        g.commitments.clone(),
        swapped,
        auths.clone(),
    );
    assert_anchor_error(r, perr(PollError::FeldmanMismatch));
    let mut identity = g.commitments.clone();
    identity[0] = [0; 32];
    let r = attempt(
        &mut s,
        &admin,
        THRESHOLD,
        identity,
        g.member_pks.clone(),
        auths.clone(),
    );
    assert!(r.is_err(), "identity election key");
    let r = attempt(
        &mut s,
        &admin,
        2,
        g.commitments[..2].to_vec(),
        g.member_pks.clone(),
        auths.clone(),
    );
    assert_anchor_error(r, perr(PollError::InvalidCommittee));
    let mut dup_auth = auths.clone();
    dup_auth[1] = dup_auth[0];
    let r = attempt(
        &mut s,
        &admin,
        THRESHOLD,
        g.commitments.clone(),
        g.member_pks.clone(),
        dup_auth,
    );
    assert_anchor_error(r, perr(PollError::InvalidCommittee));

    attempt(
        &mut s,
        &admin,
        THRESHOLD,
        g.commitments.clone(),
        g.member_pks.clone(),
        auths,
    )
    .expect("valid committee");
    let cfg: PollConfig = read_account(&s.svm, &config_pda());
    assert_eq!(
        (
            cfg.committee.epoch,
            cfg.committee.threshold,
            cfg.committee.size
        ),
        (1, THRESHOLD, SIZE)
    );
    assert_eq!(cfg.committee.election_pk, g.election_pk());

    // Schedule and creator checks.
    let poor = staker(&mut s, 10 * VCOIN);
    let start = now(&s.svm) + 60;
    let create = create_poll_ix(
        s.registrar,
        &poor,
        0,
        start,
        start + VOTING,
        Pubkey::new_unique(),
    );
    assert_anchor_error(
        send(&mut s.svm, &[create], &poor.kp, &[]),
        perr(PollError::CreatorWeightTooLow),
    );
    for (st, en) in [
        (start - 120, start + VOTING),
        (start, start + DAY),
        (start + 31 * DAY, start + 34 * DAY),
    ] {
        let create = create_poll_ix(s.registrar, &creator, 0, st, en, Pubkey::new_unique());
        assert_anchor_error(
            send(&mut s.svm, &[create], &creator.kp, &[]),
            perr(PollError::InvalidSchedule),
        );
    }
    let create = create_poll_ix(
        s.registrar,
        &creator,
        0,
        start,
        start + VOTING,
        Pubkey::default(),
    );
    assert_anchor_error(
        send(&mut s.svm, &[create], &creator.kp, &[]),
        perr(PollError::InvalidConsumer),
    );

    // Pause blocks new polls.
    let pause = ix(
        private_poll::ID,
        private_poll::accounts::UpdateConfig {
            config: config_pda(),
            authority: admin.pubkey(),
        },
        private_poll::instruction::SetPaused { paused: true },
    );
    send(&mut s.svm, &[pause], &admin, &[]).expect("pause");
    let create = create_poll_ix(
        s.registrar,
        &creator,
        0,
        start,
        start + VOTING,
        Pubkey::new_unique(),
    );
    assert_anchor_error(
        send(&mut s.svm, &[create], &creator.kp, &[]),
        perr(PollError::Paused),
    );
}

#[test]
fn ballots_and_polls_close_only_when_settled() {
    let mut s = setup();
    let c = install(&mut s);
    let creator = staker(&mut s, 1_000 * VCOIN);
    let consumer = Keypair::new();
    let poll = open_poll(&mut s, &creator, consumer.pubkey());
    let st = staker(&mut s, 500 * VCOIN);
    vote_in(&mut s, &poll, &st, Choice::For);

    let close_ballot = ix(
        private_poll::ID,
        private_poll::accounts::CloseBallot {
            ballot_record: ballot_pda(&poll, &st.kp.pubkey()),
            poll,
            voter: st.kp.pubkey(),
        },
        private_poll::instruction::CloseBallot {},
    );
    let close_poll = ix(
        private_poll::ID,
        private_poll::accounts::ClosePoll {
            poll,
            creator: creator.kp.pubkey(),
        },
        private_poll::instruction::ClosePoll {},
    );
    let r = send(&mut s.svm, std::slice::from_ref(&close_ballot), &st.kp, &[]);
    assert_anchor_error(r, perr(PollError::PollNotFinal));
    let r = send(
        &mut s.svm,
        std::slice::from_ref(&close_poll),
        &creator.kp,
        &[],
    );
    assert_anchor_error(r, perr(PollError::PollNotFinal));

    warp_by(&mut s.svm, VOTING);
    submit_shares(&mut s, &c, &poll, &[1, 2, 3]);
    settle(&mut s, &poll);
    assert_eq!(
        read_account::<Poll>(&s.svm, &poll).outcome,
        PollOutcome::Passed
    );

    // A passed, unconsumed poll stays until consumed (or the grace period ends).
    let r = send(
        &mut s.svm,
        std::slice::from_ref(&close_poll),
        &creator.kp,
        &[],
    );
    assert_anchor_error(r, perr(PollError::CloseTooEarly));
    send(
        &mut s.svm,
        &[assert_ix(poll, consumer.pubkey())],
        &creator.kp,
        &[&consumer],
    )
    .expect("consume");
    send(&mut s.svm, &[close_ballot], &st.kp, &[]).expect("close ballot");
    send(&mut s.svm, &[close_poll], &creator.kp, &[]).expect("close poll");
    assert!(s.svm.get_account(&poll).is_none_or(|a| a.lamports == 0));
}

#[test]
fn a_realms_proposal_executes_only_if_its_private_poll_passed() {
    let mut d = dao(0);
    let admin = d.s.admin.insecure_clone();
    let c = install(&mut d.s);
    // Holds > 60% of Realms max voter weight even after the poll voters stake.
    let whale = join(
        &mut d,
        10_000 * VCOIN,
        staking_protocol::constants::MAX_LOCK_SECS,
    );
    let governance = governance_over(&mut d, Pubkey::new_unique(), &whale);
    let treasury = get_native_treasury_address(&GOV_ID, &governance);
    let create_treasury = gov_ix::create_native_treasury(&GOV_ID, &governance, &admin.pubkey());
    let fund = solana_system_interface::instruction::transfer(
        &admin.pubkey(),
        &treasury,
        3 * LAMPORTS_PER_SOL,
    );
    send(&mut d.s.svm, &[create_treasury, fund], &admin, &[]).expect("treasury");

    // Two polls consumable by the governance PDA: one passes, one is rejected.
    let passing = open_poll(&mut d.s, &whale.st, governance);
    let failing = open_poll(&mut d.s, &whale.st, governance);
    for i in 0..6u64 {
        let st = staker(&mut d.s, 300 * VCOIN);
        vote_in(
            &mut d.s,
            &passing,
            &st,
            if i < 5 { Choice::For } else { Choice::Against },
        );
        vote_in(
            &mut d.s,
            &failing,
            &st,
            if i < 5 { Choice::Against } else { Choice::For },
        );
    }
    warp_by(&mut d.s.svm, VOTING);
    for poll in [passing, failing] {
        submit_shares(&mut d.s, &c, &poll, &[1, 3, 4]);
        settle(&mut d.s, &poll);
    }
    assert_eq!(
        read_account::<Poll>(&d.s.svm, &passing).outcome,
        PollOutcome::Passed
    );
    assert_eq!(
        read_account::<Poll>(&d.s.svm, &failing).outcome,
        PollOutcome::Rejected
    );

    let recipient = Pubkey::new_unique();
    let run = |d: &mut Dao, poll: Pubkey| {
        let gate = assert_ix(poll, governance);
        let pay =
            solana_system_interface::instruction::transfer(&treasury, &recipient, LAMPORTS_PER_SOL);
        let proposal = open_proposal(d, &whale, governance, vec![gate, pay]);
        let kp = whale.st.kp.insecure_clone();
        vote(d, &whale, governance, proposal, &kp);
        finalize(d, &whale, governance, proposal);
        let metas = vec![
            AccountMeta::new_readonly(config_pda(), false),
            AccountMeta::new(poll, false),
            AccountMeta::new_readonly(governance, false),
            AccountMeta::new_readonly(solana_system_interface::program::ID, false),
            AccountMeta::new(treasury, false),
            AccountMeta::new(recipient, false),
        ];
        let exec = execute_ix(governance, proposal, private_poll::ID, metas);
        let payer = d.s.admin.insecure_clone();
        send(&mut d.s.svm, &[exec], &payer, &[])
    };

    run(&mut d, passing).expect("passed poll unlocks the payment");
    assert_eq!(d.s.svm.get_balance(&recipient).unwrap(), LAMPORTS_PER_SOL);
    assert!(read_account::<Poll>(&d.s.svm, &passing).consumed);
    assert_anchor_error(run(&mut d, failing), perr(PollError::PollNotPassed));
    assert_anchor_error(run(&mut d, passing), perr(PollError::AlreadyConsumed));
    assert_eq!(
        d.s.svm.get_balance(&recipient).unwrap(),
        LAMPORTS_PER_SOL,
        "paid exactly once"
    );
}
