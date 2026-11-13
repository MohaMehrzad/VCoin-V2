//! content-registry in LiteSVM with staking tiers and gasless session keys.
//! Audit CR-01..13.
#![allow(clippy::result_large_err)] // TransactionResult, as in the harness

use anchor_lang::{prelude::Pubkey, system_program};
use anchor_spl::token_2022::spl_token_2022;
use content_registry::{
    constants::{
        CONTENT_RECORD_SEED, EDIT_COST, MAX_URI_LEN, ORACLE_CHANGE_DELAY, REFUND_DEADLINE,
        REFUND_DELAY, REGISTRY_CONFIG_SEED, USER_ENERGY_SEED,
    },
    errors::ContentError,
    state::{ContentRecord, UserEnergy},
};
use gasless_protocol::{
    constants::{FEE_VAULT_SEED, GASLESS_CONFIG_SEED, SCOPE_CONTENT, SCOPE_FOLLOW, SESSION_SEED},
    errors::GaslessError,
};
use integration_tests::solana_sdk::{instruction::Instruction, signature::Keypair, signer::Signer};
use integration_tests::stack::*;
use integration_tests::*;
use litesvm::types::TransactionResult;
use vcoin_common::CommonError;

fn err(e: ContentError) -> u32 {
    6000 + e as u32
}

fn gerr(e: GaslessError) -> u32 {
    6000 + e as u32
}

fn cpda(seeds: &[&[u8]]) -> Pubkey {
    Pubkey::find_program_address(seeds, &content_registry::ID).0
}

fn config_pda() -> Pubkey {
    cpda(&[REGISTRY_CONFIG_SEED])
}

fn energy_pda(user: &Pubkey) -> Pubkey {
    cpda(&[USER_ENERGY_SEED, user.as_ref()])
}

fn content_pda(author: &Pubkey, id: &[u8; 32]) -> Pubkey {
    cpda(&[CONTENT_RECORD_SEED, author.as_ref(), id.as_ref()])
}

fn voter_pda(user: &Pubkey) -> Pubkey {
    pda(
        &[staking_protocol::constants::VOTER_SEED, user.as_ref()],
        &staking_protocol::ID,
    )
}

fn session_pda(signer: &Pubkey, authority: &Pubkey) -> Pubkey {
    pda(
        &[
            SESSION_SEED,
            content_registry::ID.as_ref(),
            signer.as_ref(),
            authority.as_ref(),
        ],
        &gasless_protocol::ID,
    )
}

struct Env {
    s: Stack,
    oracle: Keypair,
}

fn init_ix(authority: Pubkey, oracle: Pubkey) -> Instruction {
    ix(
        content_registry::ID,
        content_registry::accounts::Initialize {
            authority,
            config: config_pda(),
            program: content_registry::ID,
            program_data: programdata_address(&content_registry::ID),
            system_program: system_program::ID,
        },
        content_registry::instruction::Initialize {
            engagement_oracle: oracle,
        },
    )
}

/// Staking stack + gasless (sessions) + content registry.
fn env() -> Env {
    let mut s = setup();
    let admin = s.admin.insecure_clone();
    for (id, name) in [
        (gasless_protocol::ID, "gasless_protocol"),
        (content_registry::ID, "content_registry"),
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
    let oracle = funded_keypair(&mut s.svm, 2);
    send(
        &mut s.svm,
        &[gasless_init, init_ix(admin.pubkey(), oracle.pubkey())],
        &admin,
        &[],
    )
    .expect("init");
    Env { s, oracle }
}

/// A wallet with `locked` VCoin staked for a week (0 = no staking account) and an energy account.
fn user(e: &mut Env, locked: u64) -> Keypair {
    let kp = if locked > 0 {
        let st = new_staker(&mut e.s, locked);
        let dep = deposit_ix(
            &e.s,
            &st,
            0,
            locked,
            staking_protocol::constants::MIN_LOCK_SECS,
        );
        send(&mut e.s.svm, &[dep], &st.kp, &[]).expect("stake");
        st.kp
    } else {
        funded_keypair(&mut e.s.svm, 5)
    };
    let stake = (locked > 0).then(|| voter_pda(&kp.pubkey()));
    let init = ix(
        content_registry::ID,
        content_registry::accounts::InitUserEnergy {
            config: config_pda(),
            user: kp.pubkey(),
            user_energy: energy_pda(&kp.pubkey()),
            stake,
            payer: kp.pubkey(),
            system_program: system_program::ID,
        },
        content_registry::instruction::InitUserEnergy {},
    );
    send(&mut e.s.svm, &[init], &kp, &[]).expect("init energy");
    kp
}

fn energy(e: &Env, user: &Pubkey) -> UserEnergy {
    read_account(&e.s.svm, &energy_pda(user))
}

#[allow(clippy::too_many_arguments)]
fn post_ix(
    author: &Pubkey,
    signer: &Pubkey,
    session: Option<Pubkey>,
    stake: Option<Pubkey>,
    id: [u8; 32],
    uri: &str,
    content_type: u8,
    payer: &Pubkey,
) -> Instruction {
    ix(
        content_registry::ID,
        content_registry::accounts::CreateContent {
            config: config_pda(),
            author: *author,
            signer: *signer,
            session,
            user_energy: energy_pda(author),
            stake,
            content: content_pda(author, &id),
            payer: *payer,
            system_program: system_program::ID,
        },
        content_registry::instruction::CreateContent {
            tracking_id: id,
            content_hash: [id[0]; 32],
            uri: uri.to_string(),
            content_type,
        },
    )
}

fn post(e: &mut Env, author: &Keypair, id: [u8; 32], content_type: u8) -> TransactionResult {
    let stake =
        e.s.svm
            .get_account(&voter_pda(&author.pubkey()))
            .map(|_| voter_pda(&author.pubkey()));
    let i = post_ix(
        &author.pubkey(),
        &author.pubkey(),
        None,
        stake,
        id,
        "ipfs://x",
        content_type,
        &author.pubkey(),
    );
    send(&mut e.s.svm, &[i], author, &[])
}

fn edit_ix(
    author: &Pubkey,
    signer: &Pubkey,
    session: Option<Pubkey>,
    id: [u8; 32],
    hash: [u8; 32],
) -> Instruction {
    ix(
        content_registry::ID,
        content_registry::accounts::EditContent {
            config: config_pda(),
            author: *author,
            signer: *signer,
            session,
            user_energy: energy_pda(author),
            content: content_pda(author, &id),
        },
        content_registry::instruction::EditContent {
            content_hash: hash,
            uri: "ipfs://y".to_string(),
        },
    )
}

#[test]
fn tiers_come_from_locked_stake_and_follow_it_on_every_post() {
    let mut e = env();
    let nobody = user(&mut e, 0);
    assert_eq!(
        (
            energy(&e, &nobody.pubkey()).tier,
            energy(&e, &nobody.pubkey()).max_energy
        ),
        (0, 200)
    );
    let staker = user(&mut e, 5_000 * VCOIN);
    let en = energy(&e, &staker.pubkey());
    assert_eq!((en.tier, en.max_energy, en.current_energy), (2, 800, 800));

    post(&mut e, &staker, [1; 32], 0).expect("post at tier 2");
    assert_eq!(energy(&e, &staker.pubkey()).current_energy, 790);
    // After the lock ends the next post refreshes the tier to 0 (energy capped to 200).
    warp_by(&mut e.s.svm, staking_protocol::constants::MIN_LOCK_SECS);
    post(&mut e, &staker, [2; 32], 0).expect("post after unlock");
    let en = energy(&e, &staker.pubkey());
    assert_eq!((en.tier, en.max_energy, en.current_energy), (0, 200, 190));
}

#[test]
fn ids_are_per_author_costs_and_limits_apply_and_delete_closes() {
    let mut e = env();
    let alice = user(&mut e, 0);
    let bob = user(&mut e, 0);
    let id = [42u8; 32];
    post(&mut e, &alice, id, 0).expect("alice");
    post(&mut e, &bob, id, 0).expect("bob uses the same id (no squatting)");
    assert_eq!(
        read_account::<ContentRecord>(&e.s.svm, &content_pda(&bob.pubkey(), &id)).author,
        bob.pubkey()
    );

    assert_anchor_error(
        post(&mut e, &alice, [3; 32], 5),
        err(ContentError::InvalidContentType),
    );
    let long = "x".repeat(MAX_URI_LEN + 1);
    let i = post_ix(
        &alice.pubkey(),
        &alice.pubkey(),
        None,
        None,
        [4; 32],
        &long,
        0,
        &alice.pubkey(),
    );
    assert_anchor_error(
        send(&mut e.s.svm, &[i], &alice, &[]),
        err(ContentError::UriTooLong),
    );
    // 190 left: three media posts (50 each), then not enough.
    for n in 5..8u8 {
        post(&mut e, &alice, [n; 32], 2).expect("media");
    }
    assert_anchor_error(
        post(&mut e, &alice, [9; 32], 2),
        err(ContentError::InsufficientEnergy),
    );

    // Edits cost energy and bump the version; strangers cannot edit.
    let before = energy(&e, &alice.pubkey()).current_energy;
    send(
        &mut e.s.svm,
        &[edit_ix(&alice.pubkey(), &alice.pubkey(), None, id, [7; 32])],
        &alice,
        &[],
    )
    .expect("edit");
    let rec: ContentRecord = read_account(&e.s.svm, &content_pda(&alice.pubkey(), &id));
    assert_eq!((rec.version, rec.content_hash), (2, [7; 32]));
    assert_eq!(
        energy(&e, &alice.pubkey()).current_energy,
        before - EDIT_COST
    );
    let r = send(
        &mut e.s.svm,
        &[edit_ix(&alice.pubkey(), &bob.pubkey(), None, id, [8; 32])],
        &bob,
        &[],
    );
    assert_anchor_error(r, err(ContentError::Unauthorized));

    let del = ix(
        content_registry::ID,
        content_registry::accounts::DeleteContent {
            content: content_pda(&alice.pubkey(), &id),
            author: alice.pubkey(),
        },
        content_registry::instruction::DeleteContent {},
    );
    send(&mut e.s.svm, &[del], &alice, &[]).expect("delete");
    assert!(
        e.s.svm
            .get_account(&content_pda(&alice.pubkey(), &id))
            .is_none_or(|a| a.lamports == 0),
        "closed"
    );
}

fn session_ix(
    authority: &Pubkey,
    signer: &Pubkey,
    payer: &Pubkey,
    target: Pubkey,
    scope: u16,
    valid_until: i64,
) -> Instruction {
    ix(
        gasless_protocol::ID,
        gasless_protocol::accounts::CreateSession {
            config: pda(&[GASLESS_CONFIG_SEED], &gasless_protocol::ID),
            session: pda(
                &[
                    SESSION_SEED,
                    target.as_ref(),
                    signer.as_ref(),
                    authority.as_ref(),
                ],
                &gasless_protocol::ID,
            ),
            authority: *authority,
            session_signer: *signer,
            payer: *payer,
            system_program: system_program::ID,
        },
        gasless_protocol::instruction::CreateSession {
            target_program: target,
            scope,
            valid_until,
        },
    )
}

#[test]
fn session_keys_post_and_edit_within_scope_only() {
    let mut e = env();
    let alice = user(&mut e, 0);
    let session = Keypair::new();
    let relayer = funded_keypair(&mut e.s.svm, 5);
    let until = now(&e.s.svm) + 3_600;
    let create = session_ix(
        &alice.pubkey(),
        &session.pubkey(),
        &relayer.pubkey(),
        content_registry::ID,
        SCOPE_CONTENT,
        until,
    );
    send(&mut e.s.svm, &[create], &relayer, &[&alice, &session]).expect("session");
    let sess = Some(session_pda(&session.pubkey(), &alice.pubkey()));

    let id = [5u8; 32];
    let p = post_ix(
        &alice.pubkey(),
        &session.pubkey(),
        sess,
        None,
        id,
        "ipfs://s",
        0,
        &relayer.pubkey(),
    );
    send(&mut e.s.svm, &[p], &relayer, &[&session]).expect("session posts for alice");
    let edit = edit_ix(&alice.pubkey(), &session.pubkey(), sess, id, [6; 32]);
    send(&mut e.s.svm, &[edit], &relayer, &[&session]).expect("session edits");

    // No session account at all: refused.
    let p = post_ix(
        &alice.pubkey(),
        &session.pubkey(),
        None,
        None,
        [9; 32],
        "ipfs://s",
        0,
        &relayer.pubkey(),
    );
    assert_anchor_error(
        send(&mut e.s.svm, &[p], &relayer, &[&session]),
        err(ContentError::Unauthorized),
    );
    // A follow-only session is out of scope.
    let other = Keypair::new();
    let create = session_ix(
        &alice.pubkey(),
        &other.pubkey(),
        &relayer.pubkey(),
        content_registry::ID,
        SCOPE_FOLLOW,
        until,
    );
    send(&mut e.s.svm, &[create], &relayer, &[&alice, &other]).expect("follow session");
    let p = post_ix(
        &alice.pubkey(),
        &other.pubkey(),
        Some(session_pda(&other.pubkey(), &alice.pubkey())),
        None,
        [10; 32],
        "u",
        0,
        &relayer.pubkey(),
    );
    assert_anchor_error(
        send(&mut e.s.svm, &[p], &relayer, &[&other]),
        gerr(GaslessError::ActionNotInScope),
    );
    // Deletion needs the wallet itself.
    let del = ix(
        content_registry::ID,
        content_registry::accounts::DeleteContent {
            content: content_pda(&alice.pubkey(), &id),
            author: session.pubkey(),
        },
        content_registry::instruction::DeleteContent {},
    );
    assert!(send(&mut e.s.svm, &[del], &relayer, &[&session]).is_err());
    // Expired session.
    warp_by(&mut e.s.svm, 3_600);
    let p = post_ix(
        &alice.pubkey(),
        &session.pubkey(),
        sess,
        None,
        [11; 32],
        "u",
        0,
        &relayer.pubkey(),
    );
    assert_anchor_error(
        send(&mut e.s.svm, &[p], &relayer, &[&session]),
        gerr(GaslessError::SessionExpired),
    );
}

fn engagement_ix(oracle: &Pubkey, author: &Pubkey, id: [u8; 32], count: u32) -> Instruction {
    ix(
        content_registry::ID,
        content_registry::accounts::UpdateEngagement {
            config: config_pda(),
            oracle: *oracle,
            content: content_pda(author, &id),
        },
        content_registry::instruction::UpdateEngagement {
            engagement_count: count,
        },
    )
}

fn refund_ix(author: &Pubkey, id: [u8; 32]) -> Instruction {
    ix(
        content_registry::ID,
        content_registry::accounts::ClaimRefund {
            config: config_pda(),
            author: *author,
            user_energy: energy_pda(author),
            content: content_pda(author, &id),
        },
        content_registry::instruction::ClaimRefund {},
    )
}

#[test]
fn refunds_open_after_a_day_close_after_a_week_and_never_waste_a_claim() {
    let mut e = env();
    let alice = user(&mut e, 0);
    let id = [1u8; 32];
    post(&mut e, &alice, id, 2).expect("media post (50)");
    let claim = refund_ix(&alice.pubkey(), id);
    assert_anchor_error(
        send(&mut e.s.svm, std::slice::from_ref(&claim), &alice, &[]),
        err(ContentError::RefundTooEarly),
    );
    warp_by(&mut e.s.svm, REFUND_DELAY);
    assert_anchor_error(
        send(&mut e.s.svm, std::slice::from_ref(&claim), &alice, &[]),
        err(ContentError::RefundNotReady),
    );

    let stranger = funded_keypair(&mut e.s.svm, 1);
    let fake = engagement_ix(&stranger.pubkey(), &alice.pubkey(), id, 1_000);
    assert_anchor_error(
        send(&mut e.s.svm, &[fake], &stranger, &[]),
        err(ContentError::NotEngagementOracle),
    );
    let oracle = e.oracle.insecure_clone();
    send(
        &mut e.s.svm,
        &[engagement_ix(&oracle.pubkey(), &alice.pubkey(), id, 60)],
        &oracle,
        &[],
    )
    .expect("engagement");

    // Burn energy so the refund is visible, then claim 50% of 50.
    post(&mut e, &alice, [2; 32], 2).expect("spend");
    let before = energy(&e, &alice.pubkey()).current_energy;
    warp_by(&mut e.s.svm, 1);
    send(&mut e.s.svm, std::slice::from_ref(&claim), &alice, &[]).expect("refund");
    assert_eq!(energy(&e, &alice.pubkey()).current_energy, before + 25);
    warp_by(&mut e.s.svm, 1);
    assert_anchor_error(
        send(&mut e.s.svm, &[claim], &alice, &[]),
        err(ContentError::RefundAlreadyClaimed),
    );

    // A week later the window is closed.
    send(
        &mut e.s.svm,
        &[engagement_ix(
            &oracle.pubkey(),
            &alice.pubkey(),
            [2; 32],
            500,
        )],
        &oracle,
        &[],
    )
    .expect("viral");
    warp_by(&mut e.s.svm, REFUND_DEADLINE);
    let late = refund_ix(&alice.pubkey(), [2; 32]);
    assert_anchor_error(
        send(&mut e.s.svm, &[late], &alice, &[]),
        err(ContentError::RefundWindowClosed),
    );
}

#[test]
fn init_is_gated_oracle_changes_wait_and_pause_stops_posting() {
    let mut s = setup();
    let admin = s.admin.insecure_clone();
    deploy_upgradeable(
        &mut s.svm,
        content_registry::ID,
        &program_so("content_registry"),
        Some(admin.pubkey()),
    );
    let stranger = funded_keypair(&mut s.svm, 2);
    let r = send(
        &mut s.svm,
        &[init_ix(stranger.pubkey(), stranger.pubkey())],
        &stranger,
        &[],
    );
    assert_anchor_error(r, u32::from(CommonError::NotUpgradeAuthority));

    let mut e = env();
    let admin = e.s.admin.insecure_clone();
    let alice = user(&mut e, 0);
    post(&mut e, &alice, [1; 32], 0).expect("post");
    let new_oracle = funded_keypair(&mut e.s.svm, 1);
    let propose = ix(
        content_registry::ID,
        content_registry::accounts::UpdateConfig {
            config: config_pda(),
            authority: admin.pubkey(),
        },
        content_registry::instruction::ProposeOracle {
            oracle: new_oracle.pubkey(),
        },
    );
    send(&mut e.s.svm, &[propose], &admin, &[]).expect("propose");
    let apply = ix(
        content_registry::ID,
        content_registry::accounts::ApplyOracle {
            config: config_pda(),
        },
        content_registry::instruction::ApplyOracle {},
    );
    assert_anchor_error(
        send(&mut e.s.svm, std::slice::from_ref(&apply), &admin, &[]),
        err(ContentError::TimelockActive),
    );
    warp_by(&mut e.s.svm, ORACLE_CHANGE_DELAY);
    send(&mut e.s.svm, &[apply], &admin, &[]).expect("apply");
    let old = e.oracle.insecure_clone();
    let r = send(
        &mut e.s.svm,
        &[engagement_ix(&old.pubkey(), &alice.pubkey(), [1; 32], 5)],
        &old,
        &[],
    );
    assert_anchor_error(r, err(ContentError::NotEngagementOracle));
    send(
        &mut e.s.svm,
        &[engagement_ix(
            &new_oracle.pubkey(),
            &alice.pubkey(),
            [1; 32],
            5,
        )],
        &new_oracle,
        &[],
    )
    .expect("new oracle");

    let pause = ix(
        content_registry::ID,
        content_registry::accounts::UpdateConfig {
            config: config_pda(),
            authority: admin.pubkey(),
        },
        content_registry::instruction::SetPaused { paused: true },
    );
    send(&mut e.s.svm, &[pause], &admin, &[]).expect("pause");
    assert_anchor_error(post(&mut e, &alice, [2; 32], 0), err(ContentError::Paused));
}
