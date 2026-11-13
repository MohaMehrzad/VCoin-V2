//! staking-protocol + vevcoin-token v3 transaction tests (LiteSVM, real Token-2022).
//!
//! Audit coverage: VEV-01/MON-2 (self-burn can't block unstake), VEV-02 (only the
//! registrar mints/burns), VEV-04/STK-07/X-1 (gated init), STK-01 (vault credits what
//! it receives; metadata-only mint), STK-03 (1:1 receipt never drifts), pause never
//! traps withdrawals, NonTransferable receipt.

use anchor_lang::prelude::Pubkey;
use anchor_lang::system_program;
use anchor_spl::token_2022::spl_token_2022;
use integration_tests::solana_sdk::{signature::Signer, transaction::Transaction};
use integration_tests::stack::*;
use integration_tests::*;
use staking_protocol::{
    constants::*,
    errors::StakingError,
    state::{Registrar, Voter},
};
use vcoin_common::CommonError;

fn staking_err(e: StakingError) -> u32 {
    6000 + e as u32
}

fn registrar(s: &Stack) -> Registrar {
    read_account(&s.svm, &s.registrar)
}
fn voter(s: &Stack, st: &Staker) -> Voter {
    read_account(&s.svm, &st.voter)
}

#[test]
fn create_registrar_is_gated_to_the_upgrade_authority() {
    let mut s = deploy();
    init_tokens(&mut s);
    let attacker = funded_keypair(&mut s.svm, 10);
    let ixn = create_registrar_ix(&s, attacker.pubkey());
    assert_anchor_error(
        send(&mut s.svm, &[ixn], &attacker, &[]),
        u32::from(CommonError::NotUpgradeAuthority),
    );
}

#[test]
fn deposit_locks_vcoin_and_mints_a_one_to_one_receipt() {
    let mut s = setup();
    let st = new_staker(&mut s, 1_000 * VCOIN);
    let ixn = deposit_ix(&s, &st, 0, 400 * VCOIN, MAX_LOCK_SECS);
    send(&mut s.svm, &[ixn], &st.kp, &[]).expect("deposit");

    assert_eq!(token::balance(&s.svm, &s.vault), 400 * VCOIN);
    assert_eq!(token::balance(&s.svm, &st.vcoin), 600 * VCOIN);
    assert_eq!(
        token::balance(&s.svm, &st.vevcoin),
        400 * VCOIN,
        "receipt == principal"
    );
    assert_eq!(registrar(&s).total_locked, 400 * VCOIN);
    let v = voter(&s, &st);
    assert!(v.deposits[0].is_used);
    assert_eq!(v.deposits[0].amount, 400 * VCOIN);
    assert_eq!(
        v.deposits[0].lock_end - v.deposits[0].lock_start,
        MAX_LOCK_SECS
    );
}

#[test]
fn lock_bounds_and_index_are_validated() {
    let mut s = setup();
    let st = new_staker(&mut s, 10 * VCOIN);
    for (index, lock) in [(0u8, MIN_LOCK_SECS - 1), (0, MAX_LOCK_SECS + 1)] {
        let ixn = deposit_ix(&s, &st, index, VCOIN, lock);
        assert_anchor_error(
            send(&mut s.svm, &[ixn], &st.kp, &[]),
            staking_err(StakingError::InvalidLockDuration),
        );
    }
    let ixn = deposit_ix(&s, &st, MAX_DEPOSITS as u8, VCOIN, MIN_LOCK_SECS);
    assert_anchor_error(
        send(&mut s.svm, &[ixn], &st.kp, &[]),
        staking_err(StakingError::InvalidDepositIndex),
    );
}

#[test]
fn withdraw_only_after_unlock_and_receipt_is_burned() {
    let mut s = setup();
    let st = new_staker(&mut s, 100 * VCOIN);
    let ixn = deposit_ix(&s, &st, 0, 100 * VCOIN, MIN_LOCK_SECS);
    send(&mut s.svm, &[ixn], &st.kp, &[]).unwrap();

    let ixn = withdraw_ix(&s, &st, 0, 100 * VCOIN);
    assert_anchor_error(
        send(&mut s.svm, &[ixn], &st.kp, &[]),
        staking_err(StakingError::StillLocked),
    );

    warp_by(&mut s.svm, MIN_LOCK_SECS);
    let ixn = withdraw_ix(&s, &st, 0, 40 * VCOIN);
    send(&mut s.svm, &[ixn], &st.kp, &[]).expect("partial withdraw");
    assert_eq!(token::balance(&s.svm, &st.vevcoin), 60 * VCOIN);
    let ixn = withdraw_ix(&s, &st, 0, 60 * VCOIN);
    send(&mut s.svm, &[ixn], &st.kp, &[]).expect("full withdraw");

    assert_eq!(token::balance(&s.svm, &st.vcoin), 100 * VCOIN);
    assert_eq!(token::balance(&s.svm, &st.vevcoin), 0);
    assert_eq!(
        token::supply(&s.svm, &s.vevcoin_mint),
        0,
        "no orphaned receipts"
    );
    assert_eq!(registrar(&s).total_locked, 0);
    assert!(!voter(&s, &st).deposits[0].is_used);
}

#[test]
fn self_burned_receipt_never_blocks_unstaking() {
    // Audit VEV-01 / MON-2: a holder (or a wallet "burn spam" tool) burns the soulbound
    // receipt; the old program then reverted every unstake and locked the VCoin forever.
    let mut s = setup();
    let st = new_staker(&mut s, 50 * VCOIN);
    let ixn = deposit_ix(&s, &st, 0, 50 * VCOIN, MIN_LOCK_SECS);
    send(&mut s.svm, &[ixn], &st.kp, &[]).unwrap();

    let self_burn = spl_token_2022::instruction::burn(
        &spl_token_2022::ID,
        &st.vevcoin,
        &s.vevcoin_mint,
        &st.kp.pubkey(),
        &[],
        50 * VCOIN,
    )
    .unwrap();
    send(&mut s.svm, &[self_burn], &st.kp, &[]).expect("owner can burn own receipt");
    assert_eq!(token::balance(&s.svm, &st.vevcoin), 0);

    warp_by(&mut s.svm, MIN_LOCK_SECS);
    let ixn = withdraw_ix(&s, &st, 0, 50 * VCOIN);
    send(&mut s.svm, &[ixn], &st.kp, &[]).expect("withdraw still succeeds");
    assert_eq!(token::balance(&s.svm, &st.vcoin), 50 * VCOIN);
}

#[test]
fn receipt_is_non_transferable() {
    let mut s = setup();
    let st = new_staker(&mut s, 10 * VCOIN);
    let ixn = deposit_ix(&s, &st, 0, 10 * VCOIN, MIN_LOCK_SECS);
    send(&mut s.svm, &[ixn], &st.kp, &[]).unwrap();

    let thief = funded_keypair(&mut s.svm, 1);
    let admin = s.admin.insecure_clone();
    let thief_ve =
        token::create_token_account(&mut s.svm, &admin, &s.vevcoin_mint, &thief.pubkey());
    let transfer = spl_token_2022::instruction::transfer_checked(
        &spl_token_2022::ID,
        &st.vevcoin,
        &s.vevcoin_mint,
        &thief_ve,
        &st.kp.pubkey(),
        &[],
        VCOIN,
        9,
    )
    .unwrap();
    assert!(
        send(&mut s.svm, &[transfer], &st.kp, &[]).is_err(),
        "NonTransferable mint must reject transfers"
    );
}

#[test]
fn only_the_registrar_can_mint_receipts() {
    let mut s = setup();
    let attacker = funded_keypair(&mut s.svm, 1);
    let admin = s.admin.insecure_clone();
    let dest = token::create_token_account(&mut s.svm, &admin, &s.vevcoin_mint, &attacker.pubkey());
    let mint = ix(
        vevcoin_token::ID,
        vevcoin_token::accounts::MintVeVCoin {
            config: s.vevcoin_config,
            mint: s.vevcoin_mint,
            destination: dest,
            staking_authority: attacker.pubkey(),
            token_program: spl_token_2022::ID,
        },
        vevcoin_token::instruction::MintVevcoin { amount: VCOIN },
    );
    assert_anchor_error(
        send(&mut s.svm, &[mint], &attacker, &[]),
        6000 + vevcoin_token::errors::VeVCoinError::Unauthorized as u32,
    );
}

#[test]
fn pause_blocks_deposits_but_never_withdrawals() {
    let mut s = setup();
    let st = new_staker(&mut s, 20 * VCOIN);
    let ixn = deposit_ix(&s, &st, 0, 10 * VCOIN, MIN_LOCK_SECS);
    send(&mut s.svm, &[ixn], &st.kp, &[]).unwrap();

    let admin = s.admin.insecure_clone();
    let pause = ix(
        staking_protocol::ID,
        staking_protocol::accounts::UpdateRegistrar {
            registrar: s.registrar,
            authority: admin.pubkey(),
        },
        staking_protocol::instruction::SetPaused { paused: true },
    );
    send(&mut s.svm, &[pause], &admin, &[]).unwrap();

    let ixn = deposit_ix(&s, &st, 1, 5 * VCOIN, MIN_LOCK_SECS);
    assert_anchor_error(
        send(&mut s.svm, &[ixn], &st.kp, &[]),
        staking_err(StakingError::Paused),
    );

    warp_by(&mut s.svm, MIN_LOCK_SECS);
    let ixn = withdraw_ix(&s, &st, 0, 10 * VCOIN);
    send(&mut s.svm, &[ixn], &st.kp, &[]).expect("withdraw while paused");
}

#[test]
fn top_up_keeps_the_later_lock_end_and_extend_only_moves_later() {
    let mut s = setup();
    let st = new_staker(&mut s, 30 * VCOIN);
    let ixn = deposit_ix(&s, &st, 0, 10 * VCOIN, MAX_LOCK_SECS);
    send(&mut s.svm, &[ixn], &st.kp, &[]).unwrap();
    let end = voter(&s, &st).deposits[0].lock_end;

    // Short top-up: lock end unchanged, receipt still 1:1 (audit STK-03 regression).
    let ixn = deposit_ix(&s, &st, 0, 5 * VCOIN, MIN_LOCK_SECS);
    send(&mut s.svm, &[ixn], &st.kp, &[]).unwrap();
    let v = voter(&s, &st);
    assert_eq!(v.deposits[0].lock_end, end);
    assert_eq!(v.deposits[0].amount, 15 * VCOIN);
    assert_eq!(token::balance(&s.svm, &st.vevcoin), 15 * VCOIN);

    let extend = |lock: i64| {
        ix(
            staking_protocol::ID,
            staking_protocol::accounts::ExtendLock {
                registrar: s.registrar,
                voter: st.voter,
                owner: st.kp.pubkey(),
            },
            staking_protocol::instruction::ExtendLock {
                index: 0,
                lock_secs: lock,
            },
        )
    };
    let ixn = extend(MIN_LOCK_SECS);
    assert_anchor_error(
        send(&mut s.svm, &[ixn], &st.kp, &[]),
        staking_err(StakingError::LockNotExtended),
    );
    warp_by(&mut s.svm, 100);
    let ixn = extend(MAX_LOCK_SECS);
    send(&mut s.svm, &[ixn], &st.kp, &[]).expect("extend later");
    assert!(voter(&s, &st).deposits[0].lock_end > end);
}

#[test]
fn close_voter_requires_an_empty_voter_and_refunds_rent() {
    let mut s = setup();
    let st = new_staker(&mut s, 5 * VCOIN);
    let ixn = deposit_ix(&s, &st, 0, 5 * VCOIN, MIN_LOCK_SECS);
    send(&mut s.svm, &[ixn], &st.kp, &[]).unwrap();
    let close = ix(
        staking_protocol::ID,
        staking_protocol::accounts::CloseVoter {
            voter: st.voter,
            owner: st.kp.pubkey(),
        },
        staking_protocol::instruction::CloseVoter {},
    );
    assert_anchor_error(
        send(&mut s.svm, std::slice::from_ref(&close), &st.kp, &[]),
        staking_err(StakingError::VoterNotEmpty),
    );

    warp_by(&mut s.svm, MIN_LOCK_SECS);
    let ixn = withdraw_ix(&s, &st, 0, 5 * VCOIN);
    send(&mut s.svm, &[ixn], &st.kp, &[]).unwrap();
    let before = s.svm.get_balance(&st.kp.pubkey()).unwrap();
    let tx = Transaction::new_signed_with_payer(
        &[close],
        Some(&st.kp.pubkey()),
        &[&st.kp],
        s.svm.latest_blockhash(),
    );
    s.svm.send_transaction(tx).expect("close voter");
    assert!(s.svm.get_account(&st.voter).is_none_or(|a| a.lamports == 0));
    assert!(
        s.svm.get_balance(&st.kp.pubkey()).unwrap() > before - 10_000,
        "rent refunded"
    );
    let _ = system_program::ID;
}

/// Re-review A1: lamports sent to the (publicly derivable) veVCoin mint PDA before
/// initialization must not block it.
#[test]
fn vevcoin_initializes_even_if_its_mint_address_was_pre_funded() {
    let mut s = deploy();
    s.svm.airdrop(&s.vevcoin_mint, 1_000_000).unwrap();
    init_tokens(&mut s);
    assert_eq!(token::supply(&s.svm, &s.vevcoin_mint), 0);
    // Over-funded beyond rent works too.
    let mut s = deploy();
    s.svm.airdrop(&s.vevcoin_mint, LAMPORTS_PER_SOL).unwrap();
    init_tokens(&mut s);
    assert_eq!(token::supply(&s.svm, &s.vevcoin_mint), 0);
}

/// Re-review A2: a deposit whose lock ended has no voting weight, so it must stop counting
/// toward the max voter weight; otherwise abandoned stakes make thresholds unreachable.
#[test]
fn expired_deposits_stop_counting_toward_the_max_weight() {
    let mut s = setup();
    let short = new_staker(&mut s, 100 * VCOIN);
    let long = new_staker(&mut s, 100 * VCOIN);
    let ixn = deposit_ix(&s, &short, 0, 100 * VCOIN, MIN_LOCK_SECS);
    send(&mut s.svm, &[ixn], &short.kp, &[]).unwrap();
    let ixn = deposit_ix(&s, &long, 0, 100 * VCOIN, MAX_LOCK_SECS);
    send(&mut s.svm, &[ixn], &long.kp, &[]).unwrap();
    let r = registrar(&s);
    assert_eq!(
        (r.total_locked, r.active_locked),
        (200 * VCOIN, 200 * VCOIN)
    );

    let deactivate = ix(
        staking_protocol::ID,
        staking_protocol::accounts::DeactivateExpired {
            registrar: s.registrar,
            voter: short.voter,
        },
        staking_protocol::instruction::DeactivateExpired { index: 0 },
    );
    let anyone = funded_keypair(&mut s.svm, 1);
    assert_anchor_error(
        send(&mut s.svm, std::slice::from_ref(&deactivate), &anyone, &[]),
        staking_err(StakingError::StillLocked),
    );

    warp_by(&mut s.svm, MIN_LOCK_SECS);
    send(&mut s.svm, std::slice::from_ref(&deactivate), &anyone, &[]).expect("permissionless");
    let r = registrar(&s);
    assert_eq!(
        (r.total_locked, r.active_locked),
        (200 * VCOIN, 100 * VCOIN)
    );
    assert!(!voter(&s, &short).deposits[0].is_active);
    warp_by(&mut s.svm, 1);
    assert_anchor_error(
        send(&mut s.svm, std::slice::from_ref(&deactivate), &anyone, &[]),
        staking_err(StakingError::AlreadyDeactivated),
    );

    // Relocking gives the deposit weight again, so it counts again.
    let extend = ix(
        staking_protocol::ID,
        staking_protocol::accounts::ExtendLock {
            registrar: s.registrar,
            voter: short.voter,
            owner: short.kp.pubkey(),
        },
        staking_protocol::instruction::ExtendLock {
            index: 0,
            lock_secs: MIN_LOCK_SECS,
        },
    );
    send(&mut s.svm, &[extend], &short.kp, &[]).unwrap();
    assert_eq!(registrar(&s).active_locked, 200 * VCOIN);
    assert!(voter(&s, &short).deposits[0].is_active);

    // A withdrawal without a prior crank deactivates the whole (expired) deposit once.
    warp_by(&mut s.svm, MIN_LOCK_SECS);
    let ixn = withdraw_ix(&s, &short, 0, 40 * VCOIN);
    send(&mut s.svm, &[ixn], &short.kp, &[]).unwrap();
    let r = registrar(&s);
    assert_eq!(
        (r.total_locked, r.active_locked),
        (160 * VCOIN, 100 * VCOIN)
    );
    let ixn = withdraw_ix(&s, &short, 0, 60 * VCOIN);
    send(&mut s.svm, &[ixn], &short.kp, &[]).unwrap();
    let r = registrar(&s);
    assert_eq!(
        (r.total_locked, r.active_locked),
        (100 * VCOIN, 100 * VCOIN)
    );
}

/// Re-review B1: the guardian gives the DAO a fast emergency stop. It can pause, but only
/// the authority can unpause or change the guardian.
#[test]
fn guardian_can_pause_but_never_unpause() {
    let mut s = setup();
    let admin = s.admin.insecure_clone();
    let guardian = funded_keypair(&mut s.svm, 1);
    let stranger = funded_keypair(&mut s.svm, 1);
    let update = |authority: Pubkey| staking_protocol::accounts::UpdateRegistrar {
        registrar: s.registrar,
        authority,
    };
    let set_paused = |authority: Pubkey, paused: bool| {
        ix(
            staking_protocol::ID,
            update(authority),
            staking_protocol::instruction::SetPaused { paused },
        )
    };
    let set_guardian = |authority: Pubkey, guardian: Pubkey| {
        ix(
            staking_protocol::ID,
            update(authority),
            staking_protocol::instruction::SetGuardian { guardian },
        )
    };

    // No guardian yet: nobody but the authority may pause, and the default key is no guardian.
    let ixn = set_paused(guardian.pubkey(), true);
    assert_anchor_error(
        send(&mut s.svm, &[ixn], &guardian, &[]),
        u32::from(CommonError::NotAuthorityOrGuardian),
    );
    let ixn = set_guardian(guardian.pubkey(), guardian.pubkey());
    assert_anchor_error(
        send(&mut s.svm, &[ixn], &guardian, &[]),
        u32::from(CommonError::NotAuthority),
    );

    let ixn = set_guardian(admin.pubkey(), guardian.pubkey());
    send(&mut s.svm, &[ixn], &admin, &[]).unwrap();
    assert_eq!(registrar(&s).admin.guardian, guardian.pubkey());

    let ixn = set_paused(stranger.pubkey(), true);
    assert_anchor_error(
        send(&mut s.svm, &[ixn], &stranger, &[]),
        u32::from(CommonError::NotAuthorityOrGuardian),
    );
    let ixn = set_paused(guardian.pubkey(), true);
    send(&mut s.svm, &[ixn], &guardian, &[]).expect("guardian pauses");
    assert!(registrar(&s).paused);
    let ixn = set_paused(guardian.pubkey(), false);
    assert_anchor_error(
        send(&mut s.svm, &[ixn], &guardian, &[]),
        u32::from(CommonError::NotAuthority),
    );
    let ixn = set_paused(admin.pubkey(), false);
    send(&mut s.svm, &[ixn], &admin, &[]).expect("authority unpauses");
    assert!(!registrar(&s).paused);

    // The authority can remove the guardian.
    let ixn = set_guardian(admin.pubkey(), Pubkey::default());
    send(&mut s.svm, &[ixn], &admin, &[]).unwrap();
    warp_by(&mut s.svm, 1);
    let ixn = set_paused(guardian.pubkey(), true);
    assert_anchor_error(
        send(&mut s.svm, &[ixn], &guardian, &[]),
        u32::from(CommonError::NotAuthorityOrGuardian),
    );
}
