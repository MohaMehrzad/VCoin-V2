//! vilink-protocol in LiteSVM with real Token-2022 VCoin and gasless session keys.
//! Audit VL-1..16.
#![allow(clippy::result_large_err)] // TransactionResult, as in the harness

use anchor_lang::{prelude::Pubkey, system_program};
use anchor_spl::token_2022::spl_token_2022;
use gasless_protocol::{
    constants::{FEE_VAULT_SEED, GASLESS_CONFIG_SEED, SCOPE_VOUCH, SESSION_SEED},
    errors::GaslessError,
};
use integration_tests::solana_sdk::{instruction::Instruction, signature::Keypair, signer::Signer};
use integration_tests::stack::*;
use integration_tests::*;
use litesvm::types::TransactionResult;
use vcoin_common::CommonError;
use vilink_protocol::{
    constants::{
        ACTION_FOLLOW, ACTION_SEED, ACTION_TIP, ACTION_VOUCH, CONFIG_CHANGE_DELAY, CONFIG_SEED,
        DAY, MAX_ACTION_EXPIRY, RECEIPT_SEED, USER_STATS_SEED,
    },
    errors::ViLinkError,
    state::{UserActionStats, ViLinkAction, ViLinkConfig},
};

fn err(e: ViLinkError) -> u32 {
    6000 + e as u32
}

fn vpda(seeds: &[&[u8]]) -> Pubkey {
    Pubkey::find_program_address(seeds, &vilink_protocol::ID).0
}

fn config_pda() -> Pubkey {
    vpda(&[CONFIG_SEED])
}

fn action_pda(creator: &Pubkey, nonce: u64) -> Pubkey {
    vpda(&[ACTION_SEED, creator.as_ref(), &nonce.to_le_bytes()])
}

fn stats_pda(user: &Pubkey) -> Pubkey {
    vpda(&[USER_STATS_SEED, user.as_ref()])
}

fn receipt_pda(action: &Pubkey, executor: &Pubkey) -> Pubkey {
    vpda(&[RECEIPT_SEED, action.as_ref(), executor.as_ref()])
}

fn session_pda(signer: &Pubkey, authority: &Pubkey) -> Pubkey {
    pda(
        &[
            SESSION_SEED,
            vilink_protocol::ID.as_ref(),
            signer.as_ref(),
            authority.as_ref(),
        ],
        &gasless_protocol::ID,
    )
}

struct Env {
    s: Stack,
    treasury: Pubkey,
}

fn init_ix(vcoin_mint: Pubkey, authority: Pubkey, treasury: Pubkey, fee_bps: u16) -> Instruction {
    ix(
        vilink_protocol::ID,
        vilink_protocol::accounts::Initialize {
            authority,
            config: config_pda(),
            vcoin_mint,
            treasury,
            program: vilink_protocol::ID,
            program_data: programdata_address(&vilink_protocol::ID),
            token_program: spl_token_2022::ID,
            system_program: system_program::ID,
        },
        vilink_protocol::instruction::Initialize { fee_bps },
    )
}

fn admin_ix<D: anchor_lang::InstructionData>(authority: Pubkey, data: D) -> Instruction {
    ix(
        vilink_protocol::ID,
        vilink_protocol::accounts::UpdateConfig {
            config: config_pda(),
            authority,
        },
        data,
    )
}

/// Staking stack (VCoin) + gasless (sessions) + vilink at a 2.5% fee.
fn env() -> Env {
    let mut s = setup();
    let admin = s.admin.insecure_clone();
    for (id, name) in [
        (gasless_protocol::ID, "gasless_protocol"),
        (vilink_protocol::ID, "vilink_protocol"),
    ] {
        deploy_upgradeable(&mut s.svm, id, &program_so(name), Some(admin.pubkey()));
    }
    let vcoin_mint = s.vcoin_mint;
    let treasury = token::create_token_account(&mut s.svm, &admin, &vcoin_mint, &admin.pubkey());
    let gasless_init = ix(
        gasless_protocol::ID,
        gasless_protocol::accounts::Initialize {
            authority: admin.pubkey(),
            config: pda(&[GASLESS_CONFIG_SEED], &gasless_protocol::ID),
            vcoin_mint,
            fee_vault: pda(&[FEE_VAULT_SEED], &gasless_protocol::ID),
            treasury,
            program: gasless_protocol::ID,
            program_data: programdata_address(&gasless_protocol::ID),
            token_program: spl_token_2022::ID,
            system_program: system_program::ID,
        },
        gasless_protocol::instruction::Initialize {
            vcoin_fee_per_tx: VCOIN,
        },
    );
    let init = init_ix(vcoin_mint, admin.pubkey(), treasury, 250);
    send(&mut s.svm, &[gasless_init, init], &admin, &[]).expect("init");
    Env { s, treasury }
}

fn create_ix(
    creator: &Pubkey,
    nonce: u64,
    action_type: u8,
    amount: u64,
    target: &Pubkey,
    expiry: i64,
    max: u32,
) -> Instruction {
    ix(
        vilink_protocol::ID,
        vilink_protocol::accounts::CreateAction {
            config: config_pda(),
            creator: *creator,
            creator_stats: stats_pda(creator),
            action: action_pda(creator, nonce),
            system_program: system_program::ID,
        },
        vilink_protocol::instruction::CreateAction {
            action_type,
            amount,
            target: *target,
            metadata_hash: [0; 32],
            expiry_secs: expiry,
            max_executions: max,
            nonce,
        },
    )
}

/// Creates a 7-day link with the creator's next nonce.
fn create(
    e: &mut Env,
    creator: &Keypair,
    action_type: u8,
    amount: u64,
    target: &Pubkey,
    max: u32,
) -> Pubkey {
    let stats = stats_pda(&creator.pubkey());
    let nonce = e.s.svm.get_account(&stats).map_or(0, |_| {
        read_account::<UserActionStats>(&e.s.svm, &stats).actions_created
    });
    let i = create_ix(
        &creator.pubkey(),
        nonce,
        action_type,
        amount,
        target,
        MAX_ACTION_EXPIRY,
        max,
    );
    send(&mut e.s.svm, &[i], creator, &[]).expect("create action");
    action_pda(&creator.pubkey(), nonce)
}

struct Wallet {
    kp: Keypair,
    vcoin: Pubkey,
}

fn wallet(e: &mut Env, vcoin: u64) -> Wallet {
    let kp = funded_keypair(&mut e.s.svm, 5);
    let vcoin_account = fund_vcoin(&mut e.s, &kp.pubkey(), vcoin);
    Wallet {
        kp,
        vcoin: vcoin_account,
    }
}

fn tip(e: &mut Env, action: &Pubkey, executor: &Wallet, target_token: Pubkey) -> TransactionResult {
    let target = read_account::<ViLinkAction>(&e.s.svm, action).target;
    let treasury = read_account::<ViLinkConfig>(&e.s.svm, &config_pda()).treasury;
    let i = ix(
        vilink_protocol::ID,
        vilink_protocol::accounts::ExecuteTip {
            config: config_pda(),
            action: *action,
            executor: executor.kp.pubkey(),
            executor_token: executor.vcoin,
            target_token,
            treasury,
            vcoin_mint: e.s.vcoin_mint,
            receipt: receipt_pda(action, &executor.kp.pubkey()),
            executor_stats: stats_pda(&executor.kp.pubkey()),
            target_stats: stats_pda(&target),
            token_program: spl_token_2022::ID,
            system_program: system_program::ID,
        },
        vilink_protocol::instruction::ExecuteTip {},
    );
    send(&mut e.s.svm, &[i], &executor.kp, &[])
}

/// Vouch (`vouch = true`) or follow on `action` (whose target is `target`).
fn social_ix(
    action: &Pubkey,
    target: &Pubkey,
    executor: &Pubkey,
    signer: &Pubkey,
    session: Option<Pubkey>,
    payer: &Pubkey,
    vouch: bool,
) -> Instruction {
    let accounts = vilink_protocol::accounts::ExecuteSocial {
        config: config_pda(),
        action: *action,
        executor: *executor,
        signer: *signer,
        session,
        receipt: receipt_pda(action, executor),
        executor_stats: stats_pda(executor),
        target_stats: stats_pda(target),
        payer: *payer,
        system_program: system_program::ID,
    };
    if vouch {
        ix(
            vilink_protocol::ID,
            accounts,
            vilink_protocol::instruction::ExecuteVouch {},
        )
    } else {
        ix(
            vilink_protocol::ID,
            accounts,
            vilink_protocol::instruction::ExecuteFollow {},
        )
    }
}

#[test]
fn tips_pay_the_target_net_of_the_links_own_fee_once_per_executor() {
    let mut e = env();
    let creator = wallet(&mut e, 20 * VCOIN);
    let target = wallet(&mut e, 0);
    let link = create(
        &mut e,
        &creator.kp,
        ACTION_TIP,
        10 * VCOIN,
        &target.kp.pubkey(),
        2,
    );

    // The fee rises to 10% (after 24h); the existing link keeps its 2.5% (VL-16).
    let admin = e.s.admin.insecure_clone();
    let propose = ix(
        vilink_protocol::ID,
        vilink_protocol::accounts::ProposeConfig {
            config: config_pda(),
            authority: admin.pubkey(),
            vcoin_mint: e.s.vcoin_mint,
            treasury: e.treasury,
            token_program: spl_token_2022::ID,
        },
        vilink_protocol::instruction::ProposeConfig { fee_bps: 1_000 },
    );
    send(&mut e.s.svm, &[propose], &admin, &[]).expect("propose fee");
    let apply = ix(
        vilink_protocol::ID,
        vilink_protocol::accounts::ApplyConfig {
            config: config_pda(),
        },
        vilink_protocol::instruction::ApplyConfig {},
    );
    assert_anchor_error(
        send(&mut e.s.svm, std::slice::from_ref(&apply), &admin, &[]),
        err(ViLinkError::TimelockActive),
    );
    warp_by(&mut e.s.svm, CONFIG_CHANGE_DELAY);
    send(&mut e.s.svm, &[apply], &admin, &[]).expect("apply");

    // An executor with a token delegate set can still tip (VL-10).
    let a = wallet(&mut e, 100 * VCOIN);
    let approve = spl_token_2022::instruction::approve(
        &spl_token_2022::ID,
        &a.vcoin,
        &Pubkey::new_unique(),
        &a.kp.pubkey(),
        &[],
        1,
    )
    .unwrap();
    send(&mut e.s.svm, &[approve], &a.kp, &[]).expect("approve delegate");
    tip(&mut e, &link, &a, target.vcoin).expect("tip");
    let fee = (10 * VCOIN) / 40; // 2.5%
    assert_eq!(token::balance(&e.s.svm, &target.vcoin), 10 * VCOIN - fee);
    assert_eq!(token::balance(&e.s.svm, &e.treasury), fee);
    let newer = create(
        &mut e,
        &creator.kp,
        ACTION_TIP,
        VCOIN,
        &target.kp.pubkey(),
        1,
    );
    assert_eq!(
        read_account::<ViLinkAction>(&e.s.svm, &newer).fee_bps,
        1_000
    );

    // Once per executor; creator and target cannot execute.
    warp_by(&mut e.s.svm, 1);
    assert!(
        tip(&mut e, &link, &a, target.vcoin).is_err(),
        "receipt exists"
    );
    assert_anchor_error(
        tip(&mut e, &link, &creator, target.vcoin),
        err(ViLinkError::InvalidExecutor),
    );
    let target_as_executor = Wallet {
        kp: target.kp.insecure_clone(),
        vcoin: target.vcoin,
    };
    assert_anchor_error(
        tip(&mut e, &link, &target_as_executor, target.vcoin),
        err(ViLinkError::InvalidExecutor),
    );

    // max_executions = 2.
    let b = wallet(&mut e, 100 * VCOIN);
    tip(&mut e, &link, &b, target.vcoin).expect("second executor");
    let c = wallet(&mut e, 100 * VCOIN);
    assert_anchor_error(
        tip(&mut e, &link, &c, target.vcoin),
        err(ViLinkError::ActionClosed),
    );
    let stats: UserActionStats = read_account(&e.s.svm, &stats_pda(&target.kp.pubkey()));
    assert_eq!(stats.tips_received, 2 * (10 * VCOIN - fee));

    // Tips go only to the target's own account.
    let other = wallet(&mut e, 0);
    assert_anchor_error(
        tip(&mut e, &newer, &c, other.vcoin),
        err(ViLinkError::InvalidTarget),
    );
}

#[test]
fn creation_is_validated_and_disabling_a_type_stops_existing_links() {
    let mut e = env();
    let creator = funded_keypair(&mut e.s.svm, 5);
    let target = Pubkey::new_unique();
    let c = creator.pubkey();
    let cases = [
        (
            create_ix(&c, 0, 3, 0, &target, DAY, 1),
            err(ViLinkError::ActionDisabled),
        ),
        (
            create_ix(&c, 0, ACTION_TIP, 1, &target, DAY, 1),
            err(ViLinkError::InvalidAmount),
        ),
        (
            create_ix(&c, 0, ACTION_VOUCH, 5, &target, DAY, 1),
            err(ViLinkError::InvalidAmount),
        ),
        (
            create_ix(&c, 0, ACTION_VOUCH, 0, &c, DAY, 1),
            err(ViLinkError::InvalidTarget),
        ),
        (
            create_ix(&c, 0, ACTION_VOUCH, 0, &target, 8 * DAY, 1),
            err(ViLinkError::InvalidExpiry),
        ),
        (
            create_ix(&c, 0, ACTION_VOUCH, 0, &target, DAY, 0),
            err(ViLinkError::InvalidMaxExecutions),
        ),
        (
            create_ix(&c, 7, ACTION_VOUCH, 0, &target, DAY, 1),
            err(ViLinkError::InvalidNonce),
        ),
    ];
    for (i, code) in cases {
        assert_anchor_error(send(&mut e.s.svm, &[i], &creator, &[]), code);
    }
    let link = create(&mut e, &creator, ACTION_VOUCH, 0, &target, 5);

    let admin = e.s.admin.insecure_clone();
    let disable = admin_ix(
        admin.pubkey(),
        vilink_protocol::instruction::SetEnabledActions {
            enabled_actions: 1 << ACTION_TIP,
        },
    );
    send(&mut e.s.svm, &[disable], &admin, &[]).expect("disable vouch");
    let executor = funded_keypair(&mut e.s.svm, 2);
    let x = executor.pubkey();
    let v = social_ix(&link, &target, &x, &x, None, &x, true);
    assert_anchor_error(
        send(&mut e.s.svm, &[v], &executor, &[]),
        err(ViLinkError::ActionDisabled),
    );

    let pause = admin_ix(
        admin.pubkey(),
        vilink_protocol::instruction::SetPaused { paused: true },
    );
    send(&mut e.s.svm, &[pause], &admin, &[]).expect("pause");
    let r = send(
        &mut e.s.svm,
        &[create_ix(&c, 1, ACTION_TIP, VCOIN, &target, DAY, 1)],
        &creator,
        &[],
    );
    assert_anchor_error(r, err(ViLinkError::Paused));
}

#[test]
fn vouch_and_follow_accept_scoped_session_keys() {
    let mut e = env();
    let creator = funded_keypair(&mut e.s.svm, 5);
    let target = Pubkey::new_unique();
    let vouch = create(&mut e, &creator, ACTION_VOUCH, 0, &target, 10);
    let follow = create(&mut e, &creator, ACTION_FOLLOW, 0, &target, 10);

    let alice = funded_keypair(&mut e.s.svm, 2);
    let session = Keypair::new();
    let relayer = funded_keypair(&mut e.s.svm, 5);
    let create_session = ix(
        gasless_protocol::ID,
        gasless_protocol::accounts::CreateSession {
            config: pda(&[GASLESS_CONFIG_SEED], &gasless_protocol::ID),
            session: session_pda(&session.pubkey(), &alice.pubkey()),
            authority: alice.pubkey(),
            session_signer: session.pubkey(),
            payer: relayer.pubkey(),
            system_program: system_program::ID,
        },
        gasless_protocol::instruction::CreateSession {
            target_program: vilink_protocol::ID,
            scope: SCOPE_VOUCH,
            valid_until: now(&e.s.svm) + 3_600,
        },
    );
    send(
        &mut e.s.svm,
        &[create_session],
        &relayer,
        &[&alice, &session],
    )
    .expect("session");
    let sess = Some(session_pda(&session.pubkey(), &alice.pubkey()));
    let (a, s, r) = (alice.pubkey(), session.pubkey(), relayer.pubkey());

    send(
        &mut e.s.svm,
        &[social_ix(&vouch, &target, &a, &s, sess, &r, true)],
        &relayer,
        &[&session],
    )
    .expect("session vouches");
    assert_eq!(
        read_account::<UserActionStats>(&e.s.svm, &stats_pda(&a)).vouches_given,
        1
    );
    assert_eq!(
        read_account::<UserActionStats>(&e.s.svm, &stats_pda(&target)).vouches_received,
        1
    );

    // The vouch scope does not cover follow; the wrong instruction for a type is refused.
    let f = social_ix(&follow, &target, &a, &s, sess, &r, false);
    assert_anchor_error(
        send(&mut e.s.svm, &[f], &relayer, &[&session]),
        6000 + GaslessError::ActionNotInScope as u32,
    );
    let wrong = social_ix(&vouch, &target, &r, &r, None, &r, false);
    assert_anchor_error(
        send(&mut e.s.svm, &[wrong], &relayer, &[]),
        err(ViLinkError::WrongActionType),
    );
    // Without a session, another key cannot act for alice; alice herself can.
    let f = social_ix(&follow, &target, &a, &r, None, &r, false);
    assert_anchor_error(
        send(&mut e.s.svm, &[f], &relayer, &[]),
        err(ViLinkError::Unauthorized),
    );
    send(
        &mut e.s.svm,
        &[social_ix(&follow, &target, &a, &a, None, &a, false)],
        &alice,
        &[],
    )
    .expect("follow");
}

#[test]
fn links_and_receipts_close_once_done() {
    let mut e = env();
    let creator = funded_keypair(&mut e.s.svm, 5);
    let target = Pubkey::new_unique();
    let link = create(&mut e, &creator, ACTION_VOUCH, 0, &target, 1);
    let close = ix(
        vilink_protocol::ID,
        vilink_protocol::accounts::CloseAction {
            action: link,
            creator: creator.pubkey(),
        },
        vilink_protocol::instruction::CloseAction {},
    );
    assert_anchor_error(
        send(&mut e.s.svm, std::slice::from_ref(&close), &creator, &[]),
        err(ViLinkError::ActionStillOpen),
    );
    let executor = funded_keypair(&mut e.s.svm, 2);
    let x = executor.pubkey();
    send(
        &mut e.s.svm,
        &[social_ix(&link, &target, &x, &x, None, &x, true)],
        &executor,
        &[],
    )
    .expect("vouch (exhausts the link)");
    send(&mut e.s.svm, &[close], &creator, &[]).expect("close exhausted link");

    let close_receipt = ix(
        vilink_protocol::ID,
        vilink_protocol::accounts::CloseReceipt {
            receipt: receipt_pda(&link, &x),
            action: link,
            rent_payer: x,
        },
        vilink_protocol::instruction::CloseReceipt {},
    );
    let before = e.s.svm.get_balance(&x).unwrap();
    send(&mut e.s.svm, &[close_receipt], &creator, &[]).expect("close receipt");
    assert!(e.s.svm.get_balance(&x).unwrap() > before);
}

#[test]
fn initialization_is_gated_and_validated() {
    let mut s = setup();
    let admin = s.admin.insecure_clone();
    deploy_upgradeable(
        &mut s.svm,
        vilink_protocol::ID,
        &program_so("vilink_protocol"),
        Some(admin.pubkey()),
    );
    let vcoin_mint = s.vcoin_mint;
    let treasury = token::create_token_account(&mut s.svm, &admin, &vcoin_mint, &admin.pubkey());
    let stranger = funded_keypair(&mut s.svm, 2);
    let r = send(
        &mut s.svm,
        &[init_ix(vcoin_mint, stranger.pubkey(), treasury, 250)],
        &stranger,
        &[],
    );
    assert_anchor_error(r, u32::from(CommonError::NotUpgradeAuthority));
    let r = send(
        &mut s.svm,
        &[init_ix(vcoin_mint, admin.pubkey(), treasury, 5)],
        &admin,
        &[],
    );
    assert_anchor_error(r, err(ViLinkError::InvalidFee));
    send(
        &mut s.svm,
        &[init_ix(vcoin_mint, admin.pubkey(), treasury, 250)],
        &admin,
        &[],
    )
    .expect("init");
    let cfg: ViLinkConfig = read_account(&s.svm, &config_pda());
    assert_eq!(
        (cfg.fee_bps, cfg.enabled_actions, cfg.treasury),
        (250, 0b111, treasury)
    );
}
