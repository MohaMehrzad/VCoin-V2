//! five-a-protocol in LiteSVM: oracle consensus with competing tallies, oracle management,
//! expiry, and reputation-only vouching. Audit 5A-01..09, 5A-L1..L5.
#![allow(clippy::result_large_err)] // TransactionResult, as in the harness

use anchor_lang::{prelude::Pubkey, system_program};
use five_a_protocol::{
    constants::*,
    errors::FiveAError,
    state::{FiveAConfig, ScoreValues, UserScore, UserVouchStatus, VoucherStats},
};
use integration_tests::solana_sdk::{instruction::Instruction, signature::Keypair, signer::Signer};
use integration_tests::*;
use litesvm::LiteSVM;
use vcoin_common::CommonError;

fn err(e: FiveAError) -> u32 {
    6000 + e as u32
}

fn config_pda() -> Pubkey {
    Pubkey::find_program_address(&[FIVE_A_CONFIG_SEED], &five_a_protocol::ID).0
}

fn pda(seeds: &[&[u8]]) -> Pubkey {
    Pubkey::find_program_address(seeds, &five_a_protocol::ID).0
}

fn score_pda(user: &Pubkey) -> Pubkey {
    pda(&[USER_SCORE_SEED, user.as_ref()])
}

fn pending_pda(user: &Pubkey) -> Pubkey {
    pda(&[PENDING_SCORE_SEED, user.as_ref()])
}

fn flat(v: u16) -> ScoreValues {
    ScoreValues {
        authenticity: v,
        accuracy: v,
        agility: v,
        activity: v,
        approved: v,
    }
}

struct FiveA {
    svm: LiteSVM,
    admin: Keypair,
    oracles: Vec<Keypair>,
}

fn admin_ix<D: anchor_lang::InstructionData>(authority: Pubkey, data: D) -> Instruction {
    ix(
        five_a_protocol::ID,
        five_a_protocol::accounts::UpdateConfig {
            config: config_pda(),
            authority,
        },
        data,
    )
}

fn init_ix(authority: Pubkey) -> Instruction {
    ix(
        five_a_protocol::ID,
        five_a_protocol::accounts::Initialize {
            authority,
            config: config_pda(),
            program: five_a_protocol::ID,
            program_data: programdata_address(&five_a_protocol::ID),
            system_program: system_program::ID,
        },
        five_a_protocol::instruction::Initialize {},
    )
}

/// Deployed and initialized, three oracles active, consensus 2.
fn five_a() -> FiveA {
    let mut svm = LiteSVM::new();
    let admin = funded_keypair(&mut svm, 100);
    deploy_upgradeable(
        &mut svm,
        five_a_protocol::ID,
        &program_so("five_a_protocol"),
        Some(admin.pubkey()),
    );
    send(&mut svm, &[init_ix(admin.pubkey())], &admin, &[]).expect("initialize");
    let oracles: Vec<Keypair> = (0..3).map(|_| funded_keypair(&mut svm, 10)).collect();
    for o in &oracles {
        let reg = admin_ix(
            admin.pubkey(),
            five_a_protocol::instruction::RegisterOracle { oracle: o.pubkey() },
        );
        send(&mut svm, &[reg], &admin, &[]).expect("register oracle");
    }
    warp_by(&mut svm, ORACLE_ACTIVATION_DELAY);
    FiveA {
        svm,
        admin,
        oracles,
    }
}

fn submit_ix(
    oracle: &Pubkey,
    user: &Pubkey,
    values: ScoreValues,
    rent_payer: &Pubkey,
) -> Instruction {
    ix(
        five_a_protocol::ID,
        five_a_protocol::accounts::SubmitScore {
            config: config_pda(),
            oracle: *oracle,
            user: *user,
            pending: pending_pda(user),
            rent_payer: *rent_payer,
            user_score: score_pda(user),
            system_program: system_program::ID,
        },
        five_a_protocol::instruction::SubmitScore { values },
    )
}

fn submit(
    f: &mut FiveA,
    oracle: usize,
    user: &Pubkey,
    values: ScoreValues,
    rent_payer: &Pubkey,
) -> litesvm::types::TransactionResult {
    let o = f.oracles[oracle].insecure_clone();
    send(
        &mut f.svm,
        &[submit_ix(&o.pubkey(), user, values, rent_payer)],
        &o,
        &[],
    )
}

/// Two oracles agree on `flat(v)` for `user` (oracle 0 opens the round).
fn set_score(f: &mut FiveA, user: &Pubkey, v: u16) {
    let opener = f.oracles[0].pubkey();
    submit(f, 0, user, flat(v), &opener).expect("submit 1");
    submit(f, 1, user, flat(v), &opener).expect("submit 2");
    assert_eq!(
        read_account::<UserScore>(&f.svm, &score_pda(user)).composite_score,
        v
    );
}

#[test]
fn a_score_applies_only_when_two_oracles_agree_and_a_liar_cannot_block_it() {
    let mut f = five_a();
    let user = Pubkey::new_unique();
    let opener = f.oracles[0].pubkey();

    submit(&mut f, 0, &user, flat(9_000), &opener).expect("dishonest first submission");
    assert_eq!(
        read_account::<UserScore>(&f.svm, &score_pda(&user)).update_count,
        0,
        "1 of 2: not applied"
    );
    submit(&mut f, 1, &user, flat(4_000), &opener).expect("honest 1");
    assert_eq!(
        read_account::<UserScore>(&f.svm, &score_pda(&user)).update_count,
        0,
        "1 vs 1: not applied"
    );

    let opener_before = f.svm.get_balance(&opener).unwrap();
    submit(&mut f, 2, &user, flat(4_000), &opener).expect("honest 2 reaches consensus");
    let s: UserScore = read_account(&f.svm, &score_pda(&user));
    assert_eq!(
        (s.composite_score, s.update_count, s.user),
        (4_000, 1, user)
    );
    assert!(
        f.svm
            .get_account(&pending_pda(&user))
            .is_none_or(|a| a.lamports == 0),
        "round closed"
    );
    assert!(
        f.svm.get_balance(&opener).unwrap() > opener_before,
        "rent back to the opener"
    );

    // Rate limit, then a new round works; an oracle may change its submission.
    warp_by(&mut f.svm, 60);
    let o1 = f.oracles[1].pubkey();
    let r = submit(&mut f, 1, &user, flat(5_000), &o1);
    assert_anchor_error(r, err(FiveAError::ScoreUpdateTooFrequent));
    warp_by(&mut f.svm, MIN_SCORE_UPDATE_INTERVAL);
    submit(&mut f, 1, &user, flat(5_000), &o1).expect("new round");
    submit(&mut f, 1, &user, flat(5_500), &o1).expect("change own submission");
    submit(&mut f, 2, &user, flat(5_000), &o1).expect("no longer matches oracle 1");
    assert_eq!(
        read_account::<UserScore>(&f.svm, &score_pda(&user)).composite_score,
        4_000
    );
    submit(&mut f, 0, &user, flat(5_500), &o1).expect("matches oracle 1's new values");
    assert_eq!(
        read_account::<UserScore>(&f.svm, &score_pda(&user)).composite_score,
        5_500
    );
}

#[test]
fn removed_or_pending_oracles_do_not_count() {
    let mut f = five_a();
    let admin = f.admin.insecure_clone();
    let user = Pubkey::new_unique();
    let opener = f.oracles[0].pubkey();

    // A newly registered oracle must wait 24h.
    let fresh = funded_keypair(&mut f.svm, 2);
    let reg = admin_ix(
        admin.pubkey(),
        five_a_protocol::instruction::RegisterOracle {
            oracle: fresh.pubkey(),
        },
    );
    send(&mut f.svm, &[reg], &admin, &[]).expect("register");
    let r = send(
        &mut f.svm,
        &[submit_ix(&fresh.pubkey(), &user, flat(1), &fresh.pubkey())],
        &fresh,
        &[],
    );
    assert_anchor_error(r, err(FiveAError::NotActiveOracle));

    // A removed oracle's earlier submission stops counting.
    submit(&mut f, 0, &user, flat(3_000), &opener).expect("oracle 0");
    let remove = admin_ix(
        admin.pubkey(),
        five_a_protocol::instruction::RemoveOracle { oracle: opener },
    );
    send(&mut f.svm, &[remove], &admin, &[]).expect("remove oracle 0");
    submit(&mut f, 1, &user, flat(3_000), &opener).expect("oracle 1");
    assert_eq!(
        read_account::<UserScore>(&f.svm, &score_pda(&user)).update_count,
        0
    );
    submit(&mut f, 2, &user, flat(3_000), &opener).expect("oracle 2");
    assert_eq!(
        read_account::<UserScore>(&f.svm, &score_pda(&user)).update_count,
        1
    );

    // The removed oracle cannot submit at all.
    let r = submit(&mut f, 0, &Pubkey::new_unique(), flat(1), &opener);
    assert_anchor_error(r, err(FiveAError::NotActiveOracle));
}

#[test]
fn expired_rounds_reset_and_return_rent_to_the_opener() {
    let mut f = five_a();
    let user = Pubkey::new_unique();
    let opener = f.oracles[0].pubkey();
    submit(&mut f, 0, &user, flat(2_000), &opener).expect("open");

    let close = ix(
        five_a_protocol::ID,
        five_a_protocol::accounts::CloseExpiredRound {
            pending: pending_pda(&user),
            rent_payer: opener,
        },
        five_a_protocol::instruction::CloseExpiredRound {},
    );
    let admin = f.admin.insecure_clone();
    let r = send(&mut f.svm, std::slice::from_ref(&close), &admin, &[]);
    assert_anchor_error(r, err(FiveAError::RoundNotExpired));
    warp_by(&mut f.svm, SCORE_UPDATE_EXPIRY + 1);

    // Oracle 0's submission has lapsed, so a late matching one no longer pairs with it.
    let o1 = f.oracles[1].pubkey();
    let r = submit(&mut f, 1, &user, flat(2_000), &o1);
    assert_anchor_error(r, err(FiveAError::WrongRentPayer));
    submit(&mut f, 1, &user, flat(2_000), &opener).expect("late submission");
    assert_eq!(
        read_account::<UserScore>(&f.svm, &score_pda(&user)).update_count,
        0,
        "stale vote dropped"
    );

    warp_by(&mut f.svm, SCORE_UPDATE_EXPIRY + 1);
    let before = f.svm.get_balance(&opener).unwrap();
    send(&mut f.svm, &[close], &admin, &[]).expect("close expired round");
    assert!(f.svm.get_balance(&opener).unwrap() > before);
}

#[test]
fn admin_paths_are_bounded_and_pause_stops_scoring() {
    let mut f = five_a();
    let admin = f.admin.insecure_clone();
    let stranger = funded_keypair(&mut f.svm, 2);
    let r = send(&mut f.svm, &[init_ix(stranger.pubkey())], &stranger, &[]);
    assert!(r.is_err(), "already initialized");

    let mut svm = LiteSVM::new();
    let owner = funded_keypair(&mut svm, 10);
    let other = funded_keypair(&mut svm, 10);
    deploy_upgradeable(
        &mut svm,
        five_a_protocol::ID,
        &program_so("five_a_protocol"),
        Some(owner.pubkey()),
    );
    let r = send(&mut svm, &[init_ix(other.pubkey())], &other, &[]);
    assert_anchor_error(r, u32::from(CommonError::NotUpgradeAuthority));

    let run =
        |f: &mut FiveA, signer: &Keypair, data: Instruction| send(&mut f.svm, &[data], signer, &[]);
    let set = |n: u8| {
        admin_ix(
            admin.pubkey(),
            five_a_protocol::instruction::SetRequiredConsensus {
                required_consensus: n,
            },
        )
    };
    assert_anchor_error(
        run(&mut f, &admin, set(1)),
        err(FiveAError::InvalidConsensus),
    );
    assert_anchor_error(
        run(&mut f, &admin, set(4)),
        err(FiveAError::InvalidConsensus),
    );
    let by_stranger = admin_ix(
        stranger.pubkey(),
        five_a_protocol::instruction::SetRequiredConsensus {
            required_consensus: 3,
        },
    );
    assert_anchor_error(
        run(&mut f, &stranger, by_stranger),
        u32::from(CommonError::NotAuthority),
    );
    run(&mut f, &admin, set(3)).expect("3 of 3");
    assert_eq!(
        read_account::<FiveAConfig>(&f.svm, &config_pda()).required_consensus,
        3
    );
    let remove = admin_ix(
        admin.pubkey(),
        five_a_protocol::instruction::RemoveOracle {
            oracle: f.oracles[2].pubkey(),
        },
    );
    assert_anchor_error(
        run(&mut f, &admin, remove),
        err(FiveAError::InvalidConsensus),
    );
    let dup = admin_ix(
        admin.pubkey(),
        five_a_protocol::instruction::RegisterOracle {
            oracle: f.oracles[0].pubkey(),
        },
    );
    assert_anchor_error(run(&mut f, &admin, dup), err(FiveAError::OracleExists));
    let zero = admin_ix(
        admin.pubkey(),
        five_a_protocol::instruction::RegisterOracle {
            oracle: Pubkey::default(),
        },
    );
    assert_anchor_error(run(&mut f, &admin, zero), err(FiveAError::InvalidOracle));

    let user = Pubkey::new_unique();
    let o0 = f.oracles[0].pubkey();
    let bad = ScoreValues {
        approved: MAX_SCORE + 1,
        ..flat(1)
    };
    assert_anchor_error(
        submit(&mut f, 0, &user, bad, &o0),
        err(FiveAError::InvalidScore),
    );
    let pause = admin_ix(
        admin.pubkey(),
        five_a_protocol::instruction::SetPaused { paused: true },
    );
    run(&mut f, &admin, pause).expect("pause");
    assert_anchor_error(
        submit(&mut f, 0, &user, flat(1), &o0),
        err(FiveAError::Paused),
    );
}

fn vouch_ix(voucher: &Pubkey, vouchee: &Pubkey) -> Instruction {
    ix(
        five_a_protocol::ID,
        five_a_protocol::accounts::VouchForUser {
            config: config_pda(),
            voucher: *voucher,
            voucher_score: score_pda(voucher),
            vouchee: *vouchee,
            vouchee_score: score_pda(vouchee),
            vouch_record: pda(&[VOUCH_RECORD_SEED, voucher.as_ref(), vouchee.as_ref()]),
            reverse_vouch: pda(&[VOUCH_RECORD_SEED, vouchee.as_ref(), voucher.as_ref()]),
            voucher_stats: pda(&[VOUCHER_STATS_SEED, voucher.as_ref()]),
            vouchee_status: pda(&[VOUCH_STATUS_SEED, vouchee.as_ref()]),
            system_program: system_program::ID,
        },
        five_a_protocol::instruction::VouchForUser {},
    )
}

fn close_vouch_accounts(
    voucher: &Pubkey,
    vouchee: &Pubkey,
) -> five_a_protocol::accounts::CloseVouch {
    five_a_protocol::accounts::CloseVouch {
        config: config_pda(),
        vouch_record: pda(&[VOUCH_RECORD_SEED, voucher.as_ref(), vouchee.as_ref()]),
        voucher: *voucher,
        voucher_stats: pda(&[VOUCHER_STATS_SEED, voucher.as_ref()]),
        vouchee_status: pda(&[VOUCH_STATUS_SEED, vouchee.as_ref()]),
    }
}

fn evaluate_ix(oracle: &Pubkey, voucher: &Pubkey, vouchee: &Pubkey) -> Instruction {
    ix(
        five_a_protocol::ID,
        five_a_protocol::accounts::EvaluateVouch {
            close: close_vouch_accounts(voucher, vouchee),
            oracle: *oracle,
            vouchee_score: score_pda(vouchee),
        },
        five_a_protocol::instruction::EvaluateVouch {},
    )
}

fn stats(f: &FiveA, voucher: &Pubkey) -> VoucherStats {
    read_account(&f.svm, &pda(&[VOUCHER_STATS_SEED, voucher.as_ref()]))
}

#[test]
fn vouching_is_for_newcomers_evaluated_by_oracles_and_always_frees_its_slot() {
    let mut f = five_a();
    let voucher = funded_keypair(&mut f.svm, 5);
    set_score(&mut f, &voucher.pubkey(), 6_000); // 3 slots
    let send_vouch = |f: &mut FiveA, who: &Keypair, vouchee: &Pubkey| {
        send(&mut f.svm, &[vouch_ix(&who.pubkey(), vouchee)], who, &[])
    };

    let r = send_vouch(&mut f, &voucher, &voucher.pubkey());
    assert_anchor_error(r, err(FiveAError::CannotVouchSelf));
    let established = Pubkey::new_unique();
    warp_by(&mut f.svm, MIN_SCORE_UPDATE_INTERVAL);
    set_score(&mut f, &established, VOUCH_SUCCESS_SCORE);
    let r = send_vouch(&mut f, &voucher, &established);
    assert_anchor_error(r, err(FiveAError::VoucheeAlreadyEstablished));

    let grower = Keypair::new();
    let ghost = Pubkey::new_unique(); // never scored
    let idle = Pubkey::new_unique();
    for v in [grower.pubkey(), ghost, idle] {
        send_vouch(&mut f, &voucher, &v).expect("vouch");
    }
    let r = send_vouch(&mut f, &voucher, &Pubkey::new_unique());
    assert_anchor_error(r, err(FiveAError::MaxVouchesReached));
    assert_eq!(stats(&f, &voucher.pubkey()).vouches_active, 3);

    // The vouchee grows; it cannot vouch back for its voucher.
    set_score(&mut f, &grower.pubkey(), 7_000);
    f.svm.airdrop(&grower.pubkey(), LAMPORTS_PER_SOL).unwrap();
    let r = send(
        &mut f.svm,
        &[vouch_ix(&grower.pubkey(), &voucher.pubkey())],
        &grower,
        &[],
    );
    assert_anchor_error(r, err(FiveAError::MutualVouch));

    // Oracle-only evaluation after 90 days.
    let oracle = f.oracles[0].insecure_clone();
    let eval = evaluate_ix(&oracle.pubkey(), &voucher.pubkey(), &grower.pubkey());
    assert_anchor_error(
        send(&mut f.svm, std::slice::from_ref(&eval), &oracle, &[]),
        err(FiveAError::EvaluationTooEarly),
    );
    warp_by(&mut f.svm, VOUCH_EVALUATION_PERIOD);
    let stranger = funded_keypair(&mut f.svm, 1);
    let by_stranger = evaluate_ix(&stranger.pubkey(), &voucher.pubkey(), &grower.pubkey());
    assert_anchor_error(
        send(&mut f.svm, &[by_stranger], &stranger, &[]),
        err(FiveAError::NotActiveOracle),
    );
    send(&mut f.svm, &[eval], &oracle, &[]).expect("grower succeeded");
    let ghost_eval = evaluate_ix(&oracle.pubkey(), &voucher.pubkey(), &ghost);
    send(&mut f.svm, &[ghost_eval], &oracle, &[])
        .expect("never-scored vouchee fails instead of blocking");
    let s = stats(&f, &voucher.pubkey());
    assert_eq!(
        (s.successful_vouches, s.failed_vouches, s.vouches_active),
        (1, 1, 1)
    );

    // Nobody evaluates the last one: it expires after a year and frees the slot.
    let expire = ix(
        five_a_protocol::ID,
        close_vouch_accounts(&voucher.pubkey(), &idle),
        five_a_protocol::instruction::ExpireVouch {},
    );
    assert_anchor_error(
        send(&mut f.svm, std::slice::from_ref(&expire), &stranger, &[]),
        err(FiveAError::VouchNotExpired),
    );
    warp_by(&mut f.svm, MAX_VOUCH_AGE - VOUCH_EVALUATION_PERIOD);
    let late = evaluate_ix(&oracle.pubkey(), &voucher.pubkey(), &idle);
    assert_anchor_error(
        send(&mut f.svm, &[late], &oracle, &[]),
        err(FiveAError::VouchExpired),
    );
    send(&mut f.svm, &[expire], &stranger, &[]).expect("expire");
    let s = stats(&f, &voucher.pubkey());
    assert_eq!(
        (s.expired_vouches, s.vouches_active, s.total_vouches_given),
        (1, 0, 3)
    );
    send_vouch(&mut f, &voucher, &Pubkey::new_unique()).expect("slot freed");
}

/// Re-review B7: every submission has its own one-hour clock. An oracle that submits early
/// (with other values) no longer decides when the honest submissions are wiped.
#[test]
fn submissions_use_a_sliding_window_not_the_first_submitters_clock() {
    let mut f = five_a();
    let user = Pubkey::new_unique();
    let opener = f.oracles[2].pubkey();
    submit(&mut f, 2, &user, flat(9_999), &opener).expect("early, different values");
    warp_by(&mut f.svm, SCORE_UPDATE_EXPIRY - 600);
    submit(&mut f, 0, &user, flat(4_000), &opener).expect("honest 1");
    warp_by(&mut f.svm, 1_200); // past the opener's hour, 20 minutes after honest 1
    submit(&mut f, 1, &user, flat(4_000), &opener).expect("honest 2");
    assert_eq!(
        read_account::<UserScore>(&f.svm, &score_pda(&user)).composite_score,
        4_000
    );
}

/// Re-review B2: `required_consensus` (default 2) is a floor; with five active oracles a
/// score needs three matching submissions.
#[test]
fn a_score_needs_a_majority_of_the_active_oracles() {
    let mut f = five_a();
    let admin = f.admin.insecure_clone();
    for _ in 0..2 {
        let o = funded_keypair(&mut f.svm, 10);
        let reg = admin_ix(
            admin.pubkey(),
            five_a_protocol::instruction::RegisterOracle { oracle: o.pubkey() },
        );
        send(&mut f.svm, &[reg], &admin, &[]).expect("register oracle");
        f.oracles.push(o);
    }
    warp_by(&mut f.svm, ORACLE_ACTIVATION_DELAY);
    assert_eq!(
        read_account::<FiveAConfig>(&f.svm, &config_pda()).required_consensus,
        DEFAULT_CONSENSUS
    );

    let user = Pubkey::new_unique();
    let opener = f.oracles[0].pubkey();
    submit(&mut f, 0, &user, flat(9_000), &opener).unwrap();
    submit(&mut f, 1, &user, flat(9_000), &opener).unwrap();
    assert_eq!(
        read_account::<UserScore>(&f.svm, &score_pda(&user)).update_count,
        0,
        "2 of 5 sets nothing"
    );
    submit(&mut f, 2, &user, flat(9_000), &opener).unwrap();
    let score: UserScore = read_account(&f.svm, &score_pda(&user));
    assert_eq!((score.update_count, score.composite_score), (1, 9_000));
}

/// Re-review B8: nobody is vouched for against their will, and a closed vouch frees the
/// vouchee's slot as well as the voucher's.
#[test]
fn a_vouchee_can_decline_and_closed_vouches_free_its_slots() {
    let mut f = five_a();
    let vouchers: Vec<Keypair> = (0..4).map(|_| funded_keypair(&mut f.svm, 5)).collect();
    for v in &vouchers {
        set_score(&mut f, &v.pubkey(), 6_000);
    }
    let vouchee = funded_keypair(&mut f.svm, 1);
    let status = |f: &FiveA| -> UserVouchStatus {
        read_account(
            &f.svm,
            &pda(&[VOUCH_STATUS_SEED, vouchee.pubkey().as_ref()]),
        )
    };
    for v in &vouchers[..3] {
        send(
            &mut f.svm,
            &[vouch_ix(&v.pubkey(), &vouchee.pubkey())],
            v,
            &[],
        )
        .expect("vouch");
    }
    assert_eq!(status(&f).vouches_received, MAX_VOUCHES_RECEIVED);
    let fourth = vouch_ix(&vouchers[3].pubkey(), &vouchee.pubkey());
    assert_anchor_error(
        send(&mut f.svm, std::slice::from_ref(&fourth), &vouchers[3], &[]),
        err(FiveAError::VoucheeFullyVouched),
    );

    let decline = |signer: &Pubkey| {
        ix(
            five_a_protocol::ID,
            five_a_protocol::accounts::DeclineVouch {
                close: close_vouch_accounts(&vouchers[0].pubkey(), &vouchee.pubkey()),
                vouchee: *signer,
            },
            five_a_protocol::instruction::DeclineVouch {},
        )
    };
    let stranger = funded_keypair(&mut f.svm, 1);
    assert_anchor_error(
        send(&mut f.svm, &[decline(&stranger.pubkey())], &stranger, &[]),
        err(FiveAError::NotVouchee),
    );
    send(&mut f.svm, &[decline(&vouchee.pubkey())], &vouchee, &[]).expect("decline");
    assert_eq!(status(&f).vouches_received, MAX_VOUCHES_RECEIVED - 1);
    let s = stats(&f, &vouchers[0].pubkey());
    assert_eq!(
        (s.declined_vouches, s.failed_vouches, s.vouches_active),
        (1, 0, 0)
    );

    warp_by(&mut f.svm, 1);
    send(&mut f.svm, &[fourth], &vouchers[3], &[]).expect("slot freed");
    assert_eq!(status(&f).vouches_received, MAX_VOUCHES_RECEIVED);

    // Once the evaluation period starts, the outcome belongs to the oracles: a failing
    // vouch cannot be turned into a harmless "declined".
    warp_by(&mut f.svm, VOUCH_EVALUATION_PERIOD);
    let late = ix(
        five_a_protocol::ID,
        five_a_protocol::accounts::DeclineVouch {
            close: close_vouch_accounts(&vouchers[1].pubkey(), &vouchee.pubkey()),
            vouchee: vouchee.pubkey(),
        },
        five_a_protocol::instruction::DeclineVouch {},
    );
    assert_anchor_error(
        send(&mut f.svm, &[late], &vouchee, &[]),
        err(FiveAError::DeclineWindowClosed),
    );
}
