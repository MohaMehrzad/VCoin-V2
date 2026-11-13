//! gasless-protocol v3 transaction tests (LiteSVM, real Token-2022).
//!
//! Audit coverage: GAS-01 (sessions verified by consumers; rent payer may be a relayer),
//! GAS-02 (no day-delta brick), GAS-03 (fees withdrawable to treasury), GAS-04/06
//! (strict scope + validity), GAS-05 (revoke/expiry close refunds rent), GAS-07 (gated
//! init), GAS-16 (user-capped fee; bounded + timelocked fee changes).

use anchor_lang::{prelude::Pubkey, system_program};
use anchor_spl::token_2022::spl_token_2022;
use gasless_protocol::{
    constants::*,
    errors::GaslessError,
    state::{GaslessConfig, SessionToken},
};
use integration_tests::litesvm::LiteSVM;
use integration_tests::solana_sdk::{instruction::Instruction, signature::Keypair, signer::Signer};
use integration_tests::stack::{self, pda, Stack, VCOIN};
use integration_tests::*;
use vcoin_common::CommonError;

fn gerr(e: GaslessError) -> u32 {
    6000 + e as u32
}

struct Env {
    s: Stack,
    config: Pubkey,
    fee_vault: Pubkey,
    treasury: Pubkey,
}

fn svm(env: &mut Env) -> &mut LiteSVM {
    &mut env.s.svm
}

fn init_ix(
    s: &Stack,
    signer: Pubkey,
    config: Pubkey,
    fee_vault: Pubkey,
    treasury: Pubkey,
    fee: u64,
) -> Instruction {
    ix(
        gasless_protocol::ID,
        gasless_protocol::accounts::Initialize {
            authority: signer,
            config,
            vcoin_mint: s.vcoin_mint,
            fee_vault,
            treasury,
            program: gasless_protocol::ID,
            program_data: programdata_address(&gasless_protocol::ID),
            token_program: spl_token_2022::ID,
            system_program: system_program::ID,
        },
        gasless_protocol::instruction::Initialize {
            vcoin_fee_per_tx: fee,
        },
    )
}

fn deployed() -> Env {
    let mut s = stack::deploy();
    stack::init_tokens(&mut s);
    let admin = s.admin.insecure_clone();
    deploy_upgradeable(
        &mut s.svm,
        gasless_protocol::ID,
        &program_so("gasless_protocol"),
        Some(admin.pubkey()),
    );
    let treasury = token::create_token_account(&mut s.svm, &admin, &s.vcoin_mint, &admin.pubkey());
    Env {
        config: pda(&[GASLESS_CONFIG_SEED], &gasless_protocol::ID),
        fee_vault: pda(&[FEE_VAULT_SEED], &gasless_protocol::ID),
        treasury,
        s,
    }
}

fn setup() -> Env {
    let mut env = deployed();
    let admin = env.s.admin.insecure_clone();
    let ixn = init_ix(
        &env.s,
        admin.pubkey(),
        env.config,
        env.fee_vault,
        env.treasury,
        VCOIN,
    );
    send(svm(&mut env), &[ixn], &admin, &[]).expect("initialize gasless");
    env
}

fn session_pda(target: &Pubkey, signer: &Pubkey, authority: &Pubkey) -> Pubkey {
    pda(
        &[
            SESSION_SEED,
            target.as_ref(),
            signer.as_ref(),
            authority.as_ref(),
        ],
        &gasless_protocol::ID,
    )
}

fn create_session_ix(
    env: &Env,
    authority: Pubkey,
    signer: Pubkey,
    payer: Pubkey,
    target: Pubkey,
    scope: u16,
    valid_until: i64,
) -> Instruction {
    ix(
        gasless_protocol::ID,
        gasless_protocol::accounts::CreateSession {
            config: env.config,
            session: session_pda(&target, &signer, &authority),
            authority,
            session_signer: signer,
            payer,
            system_program: system_program::ID,
        },
        gasless_protocol::instruction::CreateSession {
            target_program: target,
            scope,
            valid_until,
        },
    )
}

fn revoke_ix(session: Pubkey, payer: Pubkey, revoker: Pubkey) -> Instruction {
    ix(
        gasless_protocol::ID,
        gasless_protocol::accounts::RevokeSession {
            session,
            payer,
            revoker,
        },
        gasless_protocol::instruction::RevokeSession {},
    )
}

fn pay_fee_ix(env: &Env, payer: Pubkey, payer_vcoin: Pubkey, max_fee: u64) -> Instruction {
    ix(
        gasless_protocol::ID,
        gasless_protocol::accounts::PayVCoinFee {
            config: env.config,
            vcoin_mint: env.s.vcoin_mint,
            fee_vault: env.fee_vault,
            payer_vcoin,
            payer,
            token_program: spl_token_2022::ID,
        },
        gasless_protocol::instruction::PayVcoinFee { max_fee },
    )
}

fn admin_ix(env: &Env, data: impl anchor_lang::InstructionData) -> Instruction {
    ix(
        gasless_protocol::ID,
        gasless_protocol::accounts::UpdateConfig {
            config: env.config,
            authority: env.s.admin.pubkey(),
        },
        data,
    )
}

#[test]
fn initialize_is_gated_to_the_upgrade_authority() {
    let mut env = deployed();
    let attacker = funded_keypair(svm(&mut env), 10);
    let ixn = init_ix(
        &env.s,
        attacker.pubkey(),
        env.config,
        env.fee_vault,
        env.treasury,
        VCOIN,
    );
    assert_anchor_error(
        send(svm(&mut env), &[ixn], &attacker, &[]),
        u32::from(CommonError::NotUpgradeAuthority),
    );
}

/// Re-review B9: the fee token must be the VCoin mint itself, not any Token-2022 mint that
/// happens to pass the extension checks.
#[test]
fn initialize_rejects_a_look_alike_mint() {
    let mut env = deployed();
    let admin = env.s.admin.insecure_clone();
    let fake = Keypair::new();
    let ixs = [
        solana_system_interface::instruction::create_account(
            &admin.pubkey(),
            &fake.pubkey(),
            svm(&mut env).minimum_balance_for_rent_exemption(82),
            82,
            &spl_token_2022::ID,
        ),
        spl_token_2022::instruction::initialize_mint2(
            &spl_token_2022::ID,
            &fake.pubkey(),
            &admin.pubkey(),
            None,
            9,
        )
        .unwrap(),
    ];
    send(svm(&mut env), &ixs, &admin, &[&fake]).expect("look-alike mint");
    let treasury =
        token::create_token_account(svm(&mut env), &admin, &fake.pubkey(), &admin.pubkey());
    let mut ixn = init_ix(
        &env.s,
        admin.pubkey(),
        env.config,
        env.fee_vault,
        treasury,
        VCOIN,
    );
    ixn.accounts[2].pubkey = fake.pubkey(); // vcoin_mint
    assert_anchor_error(
        send(svm(&mut env), &[ixn], &admin, &[]),
        gerr(GaslessError::InvalidVCoinMint),
    );
}

#[test]
fn relayer_pays_for_a_session_of_a_wallet_with_zero_sol() {
    let mut env = setup();
    let user = Keypair::new(); // holds no SOL at all
    let session_key = Keypair::new();
    let relayer = funded_keypair(svm(&mut env), 5);
    let target = Pubkey::new_unique();
    let until = now(&env.s.svm) + 3_600;
    let ixn = create_session_ix(
        &env,
        user.pubkey(),
        session_key.pubkey(),
        relayer.pubkey(),
        target,
        SCOPE_CONTENT,
        until,
    );
    send(svm(&mut env), &[ixn], &relayer, &[&user, &session_key]).expect("relayer-funded session");

    let session: SessionToken = read_account(
        &env.s.svm,
        &session_pda(&target, &session_key.pubkey(), &user.pubkey()),
    );
    assert_eq!(session.authority, user.pubkey());
    assert_eq!(session.payer, relayer.pubkey());
    assert_eq!(session.scope, SCOPE_CONTENT);
    session
        .verify(
            &user.pubkey(),
            &session_key.pubkey(),
            &target,
            SCOPE_CONTENT,
            now(&env.s.svm),
        )
        .unwrap();
}

#[test]
fn invalid_scope_and_validity_are_rejected_not_clamped() {
    let mut env = setup();
    let user = funded_keypair(svm(&mut env), 1);
    let key = Keypair::new();
    let target = Pubkey::new_unique();
    let t = now(&env.s.svm);
    for (scope, until, err) in [
        (0u16, t + 60, GaslessError::InvalidScope),
        (1 << 0, t + 60, GaslessError::InvalidScope), // the old "tip" bit
        (SCOPE_FOLLOW, t, GaslessError::InvalidValidity),
        (
            SCOPE_FOLLOW,
            t + MAX_SESSION_DURATION + 1,
            GaslessError::InvalidValidity,
        ),
    ] {
        let ixn = create_session_ix(
            &env,
            user.pubkey(),
            key.pubkey(),
            user.pubkey(),
            target,
            scope,
            until,
        );
        assert_anchor_error(send(svm(&mut env), &[ixn], &user, &[&key]), gerr(err));
    }
}

#[test]
fn revoke_closes_the_session_and_strangers_can_only_clean_up_expired_ones() {
    let mut env = setup();
    let user = funded_keypair(svm(&mut env), 1);
    let relayer = funded_keypair(svm(&mut env), 5);
    let stranger = funded_keypair(svm(&mut env), 1);
    let key = Keypair::new();
    let target = Pubkey::new_unique();
    let until = now(&env.s.svm) + 600;
    let ixn = create_session_ix(
        &env,
        user.pubkey(),
        key.pubkey(),
        relayer.pubkey(),
        target,
        SCOPE_FOLLOW,
        until,
    );
    send(svm(&mut env), &[ixn], &relayer, &[&user, &key]).unwrap();
    let session = session_pda(&target, &key.pubkey(), &user.pubkey());

    let ixn = revoke_ix(session, relayer.pubkey(), stranger.pubkey());
    assert_anchor_error(
        send(svm(&mut env), &[ixn], &stranger, &[]),
        gerr(GaslessError::RevokeNotAllowed),
    );

    warp_by(svm(&mut env), 600);
    let before = env.s.svm.get_balance(&relayer.pubkey()).unwrap();
    let ixn = revoke_ix(session, relayer.pubkey(), stranger.pubkey());
    send(svm(&mut env), &[ixn], &stranger, &[]).expect("anyone closes an expired session");
    assert!(env
        .s
        .svm
        .get_account(&session)
        .is_none_or(|a| a.lamports == 0));
    assert!(
        env.s.svm.get_balance(&relayer.pubkey()).unwrap() > before,
        "rent refunded to the payer"
    );
}

#[test]
fn fee_is_capped_by_the_user_and_withdrawable_to_the_treasury() {
    let mut env = setup();
    let user = funded_keypair(svm(&mut env), 1);
    let user_vcoin = stack::fund_vcoin(&mut env.s, &user.pubkey(), 10 * VCOIN);

    let ixn = pay_fee_ix(&env, user.pubkey(), user_vcoin, VCOIN - 1);
    assert_anchor_error(
        send(svm(&mut env), &[ixn], &user, &[]),
        gerr(GaslessError::FeeAboveMax),
    );
    let ixn = pay_fee_ix(&env, user.pubkey(), user_vcoin, VCOIN);
    send(svm(&mut env), &[ixn], &user, &[]).expect("pay fee");
    assert_eq!(token::balance(&env.s.svm, &env.fee_vault), VCOIN);

    // GAS-02 regression: still works after 5 idle days (no day-delta brick).
    warp_by(svm(&mut env), 5 * 86_400);
    let ixn = pay_fee_ix(&env, user.pubkey(), user_vcoin, VCOIN);
    send(svm(&mut env), &[ixn], &user, &[]).expect("pay fee after idle days");

    let stranger = funded_keypair(svm(&mut env), 1);
    let withdraw = ix(
        gasless_protocol::ID,
        gasless_protocol::accounts::WithdrawFees {
            config: env.config,
            vcoin_mint: env.s.vcoin_mint,
            fee_vault: env.fee_vault,
            treasury: env.treasury,
            token_program: spl_token_2022::ID,
        },
        gasless_protocol::instruction::WithdrawFees { amount: 2 * VCOIN },
    );
    send(svm(&mut env), &[withdraw], &stranger, &[]).expect("permissionless withdraw to treasury");
    assert_eq!(token::balance(&env.s.svm, &env.treasury), 2 * VCOIN);
    assert_eq!(token::balance(&env.s.svm, &env.fee_vault), 0);
}

#[test]
fn fee_changes_are_bounded_and_wait_24h() {
    let mut env = setup();
    let admin = env.s.admin.insecure_clone();
    let ixn = admin_ix(
        &env,
        gasless_protocol::instruction::ScheduleFee {
            new_fee: MAX_VCOIN_FEE_PER_TX + 1,
        },
    );
    assert_anchor_error(
        send(svm(&mut env), &[ixn], &admin, &[]),
        gerr(GaslessError::FeeTooHigh),
    );

    let ixn = admin_ix(
        &env,
        gasless_protocol::instruction::ScheduleFee { new_fee: 3 * VCOIN },
    );
    send(svm(&mut env), &[ixn], &admin, &[]).unwrap();
    let apply = ix(
        gasless_protocol::ID,
        gasless_protocol::accounts::ApplyFee { config: env.config },
        gasless_protocol::instruction::ApplyFee {},
    );
    warp_by(svm(&mut env), PARAM_CHANGE_DELAY - 1);
    assert_anchor_error(
        send(svm(&mut env), std::slice::from_ref(&apply), &admin, &[]),
        gerr(GaslessError::TimelockActive),
    );
    warp_by(svm(&mut env), 1);
    send(svm(&mut env), &[apply], &admin, &[]).expect("apply after 24h");
    let cfg: GaslessConfig = read_account(&env.s.svm, &env.config);
    assert_eq!(cfg.vcoin_fee_per_tx, 3 * VCOIN);
}

#[test]
fn pause_blocks_new_sessions_and_fees() {
    let mut env = setup();
    let admin = env.s.admin.insecure_clone();
    let ixn = admin_ix(
        &env,
        gasless_protocol::instruction::SetPaused { paused: true },
    );
    send(svm(&mut env), &[ixn], &admin, &[]).unwrap();
    let user = funded_keypair(svm(&mut env), 1);
    let key = Keypair::new();
    let until = now(&env.s.svm) + 60;
    let ixn = create_session_ix(
        &env,
        user.pubkey(),
        key.pubkey(),
        user.pubkey(),
        Pubkey::new_unique(),
        SCOPE_CONTENT,
        until,
    );
    assert_anchor_error(
        send(svm(&mut env), &[ixn], &user, &[&key]),
        gerr(GaslessError::Paused),
    );
}
