//! The eight proof-of-concept tests of the 2026-11 audit (`docs/audit/2026-11`, `poc/tests/poc.rs`)
//! replayed against v3. Each originally demonstrated a bug; here it asserts the fixed behaviour.

mod common;

use anchor_lang::prelude::Pubkey;
use common::*;
use governance::{instruction as gov_ix, state::proposal::VoteType};
use integration_tests::solana_sdk::signer::Signer;
use integration_tests::stack::*;
use integration_tests::*;
use rand::rngs::OsRng;
use staking_protocol::{
    constants::{MAX_LOCK_SECS, MIN_LOCK_SECS},
    state::{DepositEntry, Voter, VoterWeightAction, WeightParams},
    weight::{deposit_weight, voter_weight},
};

const T0: i64 = 1_700_000_000;

fn deposit(amount: u64, lock_secs: i64) -> DepositEntry {
    DepositEntry {
        amount,
        lock_start: T0,
        lock_end: T0 + lock_secs,
        is_used: true,
    }
}

/// GOV-02 (Critical): ten 10-VCoin wallets out-voted the whole supply. Weight is now linear in
/// stake, so splitting gains nothing and the supply holder dominates proportionally.
#[test]
fn gov02_weight_is_linear_so_splitting_wallets_gains_nothing() {
    let p = WeightParams::default();
    let small = deposit_weight(&deposit(10 * VCOIN, MAX_LOCK_SECS), &p, T0).unwrap();
    let pooled = deposit_weight(&deposit(100 * VCOIN, MAX_LOCK_SECS), &p, T0).unwrap();
    let whale = deposit_weight(&deposit(1_000_000_000 * VCOIN, MAX_LOCK_SECS), &p, T0).unwrap();
    assert_eq!(
        10 * small,
        pooled,
        "ten wallets weigh exactly what one pooled wallet weighs"
    );
    assert_eq!(
        whale / small,
        100_000_000,
        "the supply outweighs a 10-VCoin wallet 100M to 1"
    );
    assert!(3 * small < whale);
}

/// GOV-09: the proposal threshold was 0.000001 veVCoin. It now lives in Realms, in plugin weight
/// units, and a small staker below it cannot create a proposal.
#[test]
fn gov09_a_staker_below_the_realms_threshold_cannot_propose() {
    let mut d = dao(0);
    let whale = join(&mut d, 2_000 * VCOIN, MAX_LOCK_SECS);
    let governance = governance_over(&mut d, Pubkey::new_unique(), &whale); // threshold: 1 VCoin of weight
    let minnow = join(&mut d, VCOIN / 4, MAX_LOCK_SECS); // 0.5 VCoin of weight
    let seed = Pubkey::new_unique();
    let owner = minnow.st.kp.pubkey();
    let create = gov_ix::create_proposal(
        &GOV_ID,
        &governance,
        &minnow.tor,
        &owner,
        &owner,
        Some(minnow.vwr),
        &d.realm,
        "spam".into(),
        String::new(),
        &d.s.vcoin_mint,
        VoteType::SingleChoice,
        vec!["Yes".into()],
        true,
        &seed,
    );
    let tx = [
        update_vwr_ix(
            &d,
            &minnow,
            VoterWeightAction::CreateProposal,
            Some(governance),
        ),
        create,
    ];
    assert_anchor_error(send(&mut d.s.svm, &tx, &minnow.st.kp, &[]), 520); // NotEnoughTokensToCreateProposal
}

/// ZK-01 (Critical): every honest SDK vote was rejected. Honest ballots from the SDK now verify
/// with zk-poll-core, the exact code private-poll runs.
#[test]
fn zk01_honest_sdk_ballots_verify_with_the_onchain_verifier() {
    use zk_voting_sdk::*;
    let dealers: Vec<Dealer> = (1..=3)
        .map(|i| Dealer::new(i, 3, 3, &mut OsRng).unwrap())
        .collect();
    let dealings: Vec<PublicDealing> = dealers.iter().map(Dealer::public_dealing).collect();
    let shares: Vec<ShareMessage> = dealers.iter().map(|d| d.share_for(1).unwrap()).collect();
    let key = finish(1, 3, &dealings, &shares).unwrap();
    for i in 0..20u8 {
        let ctx = BallotContext {
            program_id: private_poll::ID.to_bytes(),
            poll: [i; 32],
            voter: [i.wrapping_add(1); 32],
            election_pk: key.group.election_pk(),
            weight: 1 + i as u64 * 37,
        };
        let b = encrypt_ballot(&ctx, Choice::try_from(i % 3).unwrap(), &mut OsRng).unwrap();
        verify_ballot(&ctx, &b.ballot, &b.proof).expect("honest ballot verifies");
    }
}

/// SSC-10: claims from epoch 1024 on were impossible (a fixed-size per-user bitmap). Claims are
/// now one PDA per (epoch account, claimant), and leaves verify for any epoch index.
#[test]
fn ssc10_epochs_beyond_1024_are_claimable() {
    use sscre_protocol::{
        constants::*,
        merkle::{builder::Tree, compute_leaf, verify_proof},
    };
    let claimant = Pubkey::new_unique();
    let status = |epoch: u64| {
        let epoch_pda = pda(&[EPOCH_SEED, &epoch.to_le_bytes()], &sscre_protocol::ID);
        pda(
            &[CLAIM_SEED, epoch_pda.as_ref(), claimant.as_ref()],
            &sscre_protocol::ID,
        )
    };
    assert_ne!(status(1023), status(1024));
    assert_ne!(status(1024), status(u64::MAX));
    for epoch in [1023u64, 1024, 1_000_000] {
        let tree = Tree::new(epoch, &[(claimant, 5), (Pubkey::new_unique(), 7)], 12).unwrap();
        assert!(verify_proof(
            &tree.proof(0),
            &tree.root(),
            &compute_leaf(&claimant, 5, epoch)
        ));
    }
}

/// SSC-03: the documented leaf never matched the program's. The documented recipe now is the
/// program's recipe (and the old big-endian, epoch-less one still does not match).
#[test]
fn ssc03_the_documented_leaf_is_the_onchain_leaf() {
    use sha3::{Digest, Keccak256};
    let user = Pubkey::new_unique();
    let amount: u64 = 5 * VCOIN;
    for epoch in [0u64, 1, 12] {
        let onchain = sscre_protocol::merkle::compute_leaf(&user, amount, epoch);
        let mut documented = Keccak256::new();
        documented.update(b"SSCRE_CLAIM_V1");
        documented.update(user.as_ref());
        documented.update(amount.to_le_bytes());
        documented.update(epoch.to_le_bytes());
        assert_eq!(
            onchain,
            <[u8; 32]>::from(documented.finalize()),
            "epoch {epoch}"
        );
        let old: [u8; 32] = Keccak256::new()
            .chain_update(b"SSCRE_CLAIM_V1")
            .chain_update(user.as_ref())
            .chain_update(amount.to_be_bytes())
            .finalize()
            .into();
        assert_ne!(onchain, old);
    }
}

/// STK-03: a top-up with a short lock lowered the recorded ve and orphaned ~139k veVCoin.
/// Receipts are now 1:1 with principal and a top-up keeps the later lock end.
#[test]
fn stk03_a_short_lock_top_up_keeps_receipts_equal_to_principal() {
    let mut s = setup();
    let st = new_staker(&mut s, 100_000 * VCOIN + 1);
    let first = deposit_ix(&s, &st, 0, 100_000 * VCOIN, MAX_LOCK_SECS);
    send(&mut s.svm, &[first], &st.kp, &[]).expect("4-year deposit");
    let lock_end = read_account::<Voter>(&s.svm, &st.voter).deposits[0].lock_end;
    let top_up = deposit_ix(&s, &st, 0, 1, MIN_LOCK_SECS);
    send(&mut s.svm, &[top_up], &st.kp, &[]).expect("1-week top-up");
    let voter: Voter = read_account(&s.svm, &st.voter);
    assert_eq!(voter.deposits[0].amount, 100_000 * VCOIN + 1);
    assert_eq!(
        voter.deposits[0].lock_end, lock_end,
        "the later lock end is kept"
    );
    assert_eq!(
        token::balance(&s.svm, &st.vevcoin),
        100_000 * VCOIN + 1,
        "no orphaned receipts"
    );
    let w = voter_weight(&voter, &WeightParams::default(), now(&s.svm)).unwrap();
    assert!(
        w >= 200_000 * VCOIN - VCOIN,
        "weight reflects the 4-year lock: {w}"
    );
}

/// STK-08: a one-week lock was under-credited by >15% (truncated duration factor). Weight is
/// now exact up to a single division per term.
#[test]
fn stk08_a_one_week_lock_is_credited_exactly() {
    let p = WeightParams::default();
    let amount = 1_000 * VCOIN;
    let w = deposit_weight(&deposit(amount, MIN_LOCK_SECS), &p, T0).unwrap();
    let exact = amount as u128 + amount as u128 * MIN_LOCK_SECS as u128 / MAX_LOCK_SECS as u128;
    assert_eq!(w as u128, exact);
}

/// ID-04: email + phone + social (no documents) was classified as KYC.
#[test]
fn id04_no_documents_is_basic_not_kyc() {
    use identity_protocol::{constants::*, state::level_for_claims};
    assert_eq!(
        level_for_claims(CLAIM_EMAIL | CLAIM_PHONE | CLAIM_SOCIAL),
        LEVEL_BASIC
    );
    assert_eq!(level_for_claims(CLAIM_KYC), LEVEL_KYC);
}
