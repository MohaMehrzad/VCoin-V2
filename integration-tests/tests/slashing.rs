//! Slashing of staked deposits (audit TOK-02/03, MON-6/7): DAO-proposed, frozen withdrawals,
//! 48h appeal window, permissionless execution of `min(amount, deposits)` to the DAO
//! treasury, cancel / expiry, and receipts that cannot be used to dodge a slash.

mod common;

use anchor_lang::{prelude::Pubkey, system_program};
use anchor_spl::token_2022::spl_token_2022;
use common::*;
use governance::{instruction as gov_ix, state::native_treasury::get_native_treasury_address};
use integration_tests::solana_sdk::{
    instruction::{AccountMeta, Instruction},
    signer::Signer,
};
use integration_tests::stack::*;
use integration_tests::*;
use staking_protocol::{
    constants::*,
    errors::StakingError,
    state::{Registrar, SlashRequest, Voter},
};
use vcoin_common::{CommonError, AUTHORITY_TRANSFER_DELAY};

const REASON: [u8; 32] = [7u8; 32];

fn slash_pda(voter: &Pubkey, nonce: u64) -> Pubkey {
    pda(
        &[SLASH_SEED, voter.as_ref(), &nonce.to_le_bytes()],
        &staking_protocol::ID,
    )
}

fn propose_ix(
    registrar: Pubkey,
    authority: Pubkey,
    payer: Pubkey,
    voter: Pubkey,
    nonce: u64,
    amount: u64,
) -> Instruction {
    ix(
        staking_protocol::ID,
        staking_protocol::accounts::ProposeSlash {
            registrar,
            authority,
            voter,
            slash_request: slash_pda(&voter, nonce),
            payer,
            system_program: system_program::ID,
        },
        staking_protocol::instruction::ProposeSlash {
            amount,
            reason_hash: REASON,
        },
    )
}

fn close_accounts(
    registrar: Pubkey,
    voter: Pubkey,
    nonce: u64,
    rent_payer: Pubkey,
) -> staking_protocol::accounts::CloseSlash {
    staking_protocol::accounts::CloseSlash {
        registrar,
        voter,
        slash_request: slash_pda(&voter, nonce),
        rent_payer,
    }
}

fn cancel_ix(
    registrar: Pubkey,
    authority: Pubkey,
    voter: Pubkey,
    nonce: u64,
    rent_payer: Pubkey,
) -> Instruction {
    ix(
        staking_protocol::ID,
        staking_protocol::accounts::CancelSlash {
            close: close_accounts(registrar, voter, nonce, rent_payer),
            authority,
        },
        staking_protocol::instruction::CancelSlash {},
    )
}

fn expire_ix(registrar: Pubkey, voter: Pubkey, nonce: u64, rent_payer: Pubkey) -> Instruction {
    ix(
        staking_protocol::ID,
        close_accounts(registrar, voter, nonce, rent_payer),
        staking_protocol::instruction::ExpireSlash {},
    )
}

fn execute_slash_ix(s: &Stack, owner: Pubkey, nonce: u64, rent_payer: Pubkey) -> Instruction {
    let voter = pda(&[VOTER_SEED, owner.as_ref()], &staking_protocol::ID);
    let reg: Registrar = read_account(&s.svm, &s.registrar);
    ix(
        staking_protocol::ID,
        staking_protocol::accounts::ExecuteSlash {
            registrar: s.registrar,
            voter,
            slash_request: slash_pda(&voter, nonce),
            rent_payer,
            vault: s.vault,
            vcoin_mint: s.vcoin_mint,
            slash_destination: reg.slash_destination,
            vevcoin_mint: s.vevcoin_mint,
            owner,
            owner_vevcoin: receipt_ata(s, &owner),
            vevcoin_config: s.vevcoin_config,
            vevcoin_program: vevcoin_token::ID,
            token_program: spl_token_2022::ID,
        },
        staking_protocol::instruction::ExecuteSlash {},
    )
}

fn pending(s: &Stack, voter: &Pubkey) -> u8 {
    read_account::<Voter>(&s.svm, voter).pending_slashes
}

#[test]
fn slash_freezes_withdrawals_waits_out_the_appeal_window_and_pays_the_treasury() {
    let mut d = dao(0);
    let admin = d.s.admin.insecure_clone();
    let m = join(&mut d, 100 * VCOIN, MIN_LOCK_SECS);
    let destination = read_account::<Registrar>(&d.s.svm, &d.s.registrar).slash_destination;

    let propose = propose_ix(
        d.s.registrar,
        admin.pubkey(),
        admin.pubkey(),
        m.st.voter,
        0,
        30 * VCOIN,
    );
    send(&mut d.s.svm, &[propose], &admin, &[]).expect("propose");
    let req: SlashRequest = read_account(&d.s.svm, &slash_pda(&m.st.voter, 0));
    assert_eq!(
        (req.owner, req.amount, req.reason_hash),
        (m.st.kp.pubkey(), 30 * VCOIN, REASON)
    );
    assert_eq!(req.executable_at - req.proposed_at, SLASH_DELAY);
    assert_eq!(req.expires_at - req.executable_at, SLASH_EXECUTION_WINDOW);
    assert_eq!(pending(&d.s, &m.st.voter), 1);

    // Inside the appeal window: not executable.
    let cranker = funded_keypair(&mut d.s.svm, 2);
    let exec = execute_slash_ix(&d.s, m.st.kp.pubkey(), 0, admin.pubkey());
    let r = send(&mut d.s.svm, std::slice::from_ref(&exec), &cranker, &[]);
    assert_anchor_error(r, serr(StakingError::SlashNotReady));

    // The lock has ended, but withdrawals stay frozen while the slash is pending.
    warp_by(&mut d.s.svm, MIN_LOCK_SECS);
    let wd = withdraw_ix_with_record(&d.s, &m.st, 0, 100 * VCOIN, m.tor);
    let r = send(&mut d.s.svm, std::slice::from_ref(&wd), &m.st.kp, &[]);
    assert_anchor_error(r, serr(StakingError::SlashPending));

    // Anyone executes; request rent goes back to whoever paid it.
    let admin_before = d.s.svm.get_balance(&admin.pubkey()).unwrap();
    send(&mut d.s.svm, &[exec], &cranker, &[]).expect("execute");
    assert_eq!(token::balance(&d.s.svm, &destination), 30 * VCOIN);
    assert_eq!(token::balance(&d.s.svm, &d.s.vault), 70 * VCOIN);
    assert_eq!(
        token::balance(&d.s.svm, &m.st.vevcoin),
        70 * VCOIN,
        "receipt stays 1:1 with principal"
    );
    let voter: Voter = read_account(&d.s.svm, &m.st.voter);
    assert_eq!(
        (voter.deposits[0].amount, voter.pending_slashes),
        (70 * VCOIN, 0)
    );
    assert_eq!(
        read_account::<Registrar>(&d.s.svm, &d.s.registrar).total_locked,
        70 * VCOIN
    );
    assert!(d
        .s
        .svm
        .get_account(&slash_pda(&m.st.voter, 0))
        .is_none_or(|a| a.lamports == 0));
    let rent =
        d.s.svm
            .minimum_balance_for_rent_exemption(SlashRequest::LEN);
    assert_eq!(
        d.s.svm.get_balance(&admin.pubkey()).unwrap(),
        admin_before + rent
    );

    let wd = withdraw_ix_with_record(&d.s, &m.st, 0, 70 * VCOIN, m.tor);
    send(&mut d.s.svm, &[wd], &m.st.kp, &[]).expect("withdraw the remainder");
    assert_eq!(token::balance(&d.s.svm, &m.st.vcoin), 70 * VCOIN);
}

#[test]
fn slash_takes_at_most_what_is_staked_across_slots() {
    let mut d = dao(0);
    let admin = d.s.admin.insecure_clone();
    let m = join(&mut d, 10 * VCOIN, MIN_LOCK_SECS);
    let top_up = 20 * VCOIN;
    let extra = fund_vcoin(&mut d.s, &m.st.kp.pubkey(), top_up);
    let mut dep = deposit_ix(&d.s, &m.st, 1, top_up, MAX_LOCK_SECS);
    let source = dep
        .accounts
        .iter()
        .position(|a| a.pubkey == m.st.vcoin)
        .unwrap();
    dep.accounts[source].pubkey = extra;
    send(&mut d.s.svm, &[dep], &m.st.kp, &[]).expect("second slot");

    let propose = propose_ix(
        d.s.registrar,
        admin.pubkey(),
        admin.pubkey(),
        m.st.voter,
        0,
        1_000 * VCOIN,
    );
    send(&mut d.s.svm, &[propose], &admin, &[]).expect("propose");
    warp_by(&mut d.s.svm, SLASH_DELAY);
    let exec = execute_slash_ix(&d.s, m.st.kp.pubkey(), 0, admin.pubkey());
    send(&mut d.s.svm, &[exec], &admin, &[]).expect("execute");

    let destination = read_account::<Registrar>(&d.s.svm, &d.s.registrar).slash_destination;
    assert_eq!(
        token::balance(&d.s.svm, &destination),
        30 * VCOIN,
        "min(requested, staked)"
    );
    let voter: Voter = read_account(&d.s.svm, &m.st.voter);
    assert!(voter.is_empty(), "both slots consumed (locked or not)");
    assert_eq!(token::balance(&d.s.svm, &m.st.vevcoin), 0);
    assert_eq!(
        read_account::<Registrar>(&d.s.svm, &d.s.registrar).total_locked,
        0
    );
}

#[test]
fn cancel_and_expiry_release_the_freeze() {
    let mut d = dao(0);
    let admin = d.s.admin.insecure_clone();
    let m = join(&mut d, 50 * VCOIN, MIN_LOCK_SECS);
    let stranger = funded_keypair(&mut d.s.svm, 2);

    // Cancel: authority only.
    let propose = propose_ix(
        d.s.registrar,
        admin.pubkey(),
        admin.pubkey(),
        m.st.voter,
        0,
        VCOIN,
    );
    send(&mut d.s.svm, &[propose], &admin, &[]).expect("propose #0");
    let r = send(
        &mut d.s.svm,
        &[cancel_ix(
            d.s.registrar,
            stranger.pubkey(),
            m.st.voter,
            0,
            admin.pubkey(),
        )],
        &stranger,
        &[],
    );
    assert_anchor_error(r, u32::from(CommonError::NotAuthority));
    send(
        &mut d.s.svm,
        &[cancel_ix(
            d.s.registrar,
            admin.pubkey(),
            m.st.voter,
            0,
            admin.pubkey(),
        )],
        &admin,
        &[],
    )
    .expect("cancel");
    assert_eq!(pending(&d.s, &m.st.voter), 0);

    // Expiry: nobody executed within the window.
    let propose = propose_ix(
        d.s.registrar,
        admin.pubkey(),
        stranger.pubkey(),
        m.st.voter,
        1,
        VCOIN,
    );
    send(&mut d.s.svm, &[propose], &admin, &[&stranger]).expect("propose #1 (stranger pays rent)");
    let r = send(
        &mut d.s.svm,
        &[expire_ix(d.s.registrar, m.st.voter, 1, stranger.pubkey())],
        &stranger,
        &[],
    );
    assert_anchor_error(r, serr(StakingError::SlashNotExpired));
    warp_by(&mut d.s.svm, SLASH_DELAY + SLASH_EXECUTION_WINDOW);
    let exec = execute_slash_ix(&d.s, m.st.kp.pubkey(), 1, stranger.pubkey());
    assert_anchor_error(
        send(&mut d.s.svm, &[exec], &admin, &[]),
        serr(StakingError::SlashWindowClosed),
    );
    // Rent must go back to the recorded payer, not to whoever closes it.
    let r = send(
        &mut d.s.svm,
        &[expire_ix(d.s.registrar, m.st.voter, 1, admin.pubkey())],
        &admin,
        &[],
    );
    assert!(r.is_err(), "has_one = rent_payer");
    send(
        &mut d.s.svm,
        &[expire_ix(d.s.registrar, m.st.voter, 1, stranger.pubkey())],
        &admin,
        &[],
    )
    .expect("expire");
    assert_eq!(pending(&d.s, &m.st.voter), 0);

    let wd = withdraw_ix_with_record(&d.s, &m.st, 0, 50 * VCOIN, m.tor);
    send(&mut d.s.svm, &[wd], &m.st.kp, &[]).expect("unfrozen");
}

#[test]
fn only_the_authority_slashes_and_only_once_governance_is_wired() {
    let mut s = setup();
    let admin = s.admin.insecure_clone();
    let st = new_staker(&mut s, 10 * VCOIN);
    let dep = deposit_ix(&s, &st, 0, 10 * VCOIN, MIN_LOCK_SECS);
    send(&mut s.svm, &[dep], &st.kp, &[]).expect("deposit");

    // No slash destination yet.
    let r = send(
        &mut s.svm,
        &[propose_ix(
            s.registrar,
            admin.pubkey(),
            admin.pubkey(),
            st.voter,
            0,
            VCOIN,
        )],
        &admin,
        &[],
    );
    assert_anchor_error(r, serr(StakingError::GovernanceNotConfigured));

    let mut d = dao(0);
    let admin = d.s.admin.insecure_clone();
    let m = join(&mut d, 10 * VCOIN, MIN_LOCK_SECS);
    let r = send(
        &mut d.s.svm,
        &[propose_ix(
            d.s.registrar,
            m.st.kp.pubkey(),
            m.st.kp.pubkey(),
            m.st.voter,
            0,
            VCOIN,
        )],
        &m.st.kp,
        &[],
    );
    assert_anchor_error(r, u32::from(CommonError::NotAuthority));
    let r = send(
        &mut d.s.svm,
        &[propose_ix(
            d.s.registrar,
            admin.pubkey(),
            admin.pubkey(),
            m.st.voter,
            0,
            0,
        )],
        &admin,
        &[],
    );
    assert_anchor_error(r, serr(StakingError::ZeroAmount));
}

#[test]
fn burning_and_closing_the_receipt_cannot_dodge_a_slash() {
    let mut d = dao(0);
    let admin = d.s.admin.insecure_clone();
    let m = join(&mut d, 40 * VCOIN, MIN_LOCK_SECS);
    let owner = m.st.kp.pubkey();
    let propose = propose_ix(
        d.s.registrar,
        admin.pubkey(),
        admin.pubkey(),
        m.st.voter,
        0,
        25 * VCOIN,
    );
    send(&mut d.s.svm, &[propose], &admin, &[]).expect("propose");

    // The owner burns every receipt and closes the ATA.
    let burn = spl_token_2022::instruction::burn(
        &spl_token_2022::ID,
        &m.st.vevcoin,
        &d.s.vevcoin_mint,
        &owner,
        &[],
        40 * VCOIN,
    )
    .unwrap();
    let close = spl_token_2022::instruction::close_account(
        &spl_token_2022::ID,
        &m.st.vevcoin,
        &owner,
        &owner,
        &[],
    )
    .unwrap();
    send(&mut d.s.svm, &[burn, close], &m.st.kp, &[]).expect("self-burn + close");
    assert!(d
        .s
        .svm
        .get_account(&m.st.vevcoin)
        .is_none_or(|a| a.lamports == 0));

    // Anyone recreates the canonical ATA and executes.
    warp_by(&mut d.s.svm, SLASH_DELAY);
    create_receipt_ata(&mut d.s, &owner);
    let exec = execute_slash_ix(&d.s, owner, 0, admin.pubkey());
    send(&mut d.s.svm, &[exec], &admin, &[]).expect("execute");
    let destination = read_account::<Registrar>(&d.s.svm, &d.s.registrar).slash_destination;
    assert_eq!(token::balance(&d.s.svm, &destination), 25 * VCOIN);
    assert_eq!(
        read_account::<Voter>(&d.s.svm, &m.st.voter).deposits[0].amount,
        15 * VCOIN
    );
}

#[test]
fn deposits_only_mint_receipts_into_the_owners_ata() {
    let mut s = setup();
    let st = new_staker(&mut s, 10 * VCOIN);
    let admin = s.admin.insecure_clone();
    let side = token::create_token_account(&mut s.svm, &admin, &s.vevcoin_mint, &st.kp.pubkey());
    let mut dep = deposit_ix(&s, &st, 0, 10 * VCOIN, MIN_LOCK_SECS);
    let receipt_index = dep
        .accounts
        .iter()
        .position(|a| a.pubkey == st.vevcoin)
        .unwrap();
    dep.accounts[receipt_index].pubkey = side;
    let r = send(&mut s.svm, &[dep], &st.kp, &[]);
    assert_anchor_error(
        r,
        anchor_lang::error::ErrorCode::ConstraintAssociated as u32,
    );
}

#[test]
fn the_dao_slashes_through_a_realms_proposal() {
    let mut d = dao(0);
    let admin = d.s.admin.insecure_clone();
    let whale = join(&mut d, 1_000 * VCOIN, MAX_LOCK_SECS);
    let offender = join(&mut d, 10 * VCOIN, MIN_LOCK_SECS);
    let registrar = d.s.registrar;
    let governance = governance_over(&mut d, registrar, &whale);

    // The native treasury pays the request's rent and gets it back on close.
    let treasury = get_native_treasury_address(&GOV_ID, &governance);
    let create_treasury = gov_ix::create_native_treasury(&GOV_ID, &governance, &admin.pubkey());
    let fund = solana_system_interface::instruction::transfer(
        &admin.pubkey(),
        &treasury,
        LAMPORTS_PER_SOL,
    );
    send(&mut d.s.svm, &[create_treasury, fund], &admin, &[]).expect("treasury");

    let hand_over = ix(
        staking_protocol::ID,
        staking_protocol::accounts::UpdateRegistrar {
            registrar,
            authority: admin.pubkey(),
        },
        staking_protocol::instruction::ProposeAuthority {
            new_authority: governance,
        },
    );
    send(&mut d.s.svm, &[hand_over], &admin, &[]).expect("propose handover");
    let accept = ix(
        staking_protocol::ID,
        staking_protocol::accounts::AcceptAuthority {
            registrar,
            new_authority: governance,
        },
        staking_protocol::instruction::AcceptAuthority {},
    );
    let slash = propose_ix(
        d.s.registrar,
        governance,
        treasury,
        offender.st.voter,
        0,
        4 * VCOIN,
    );
    let proposal = open_proposal(&mut d, &whale, governance, vec![accept, slash.clone()]);
    let voter_kp = whale.st.kp.insecure_clone();
    vote(&mut d, &whale, governance, proposal, &voter_kp);
    finalize(&mut d, &whale, governance, proposal);
    warp_by(&mut d.s.svm, AUTHORITY_TRANSFER_DELAY);

    let mut metas = vec![
        AccountMeta::new(registrar, false),
        AccountMeta::new_readonly(governance, false),
    ];
    metas.extend(slash.accounts.iter().skip(2).map(|a| AccountMeta {
        is_signer: false,
        ..a.clone()
    }));
    let exec = execute_ix(governance, proposal, staking_protocol::ID, metas);
    send(&mut d.s.svm, &[exec], &admin, &[]).expect("DAO accepts authority and proposes the slash");
    assert_eq!(pending(&d.s, &offender.st.voter), 1);

    warp_by(&mut d.s.svm, SLASH_DELAY);
    let cranker = funded_keypair(&mut d.s.svm, 2);
    let treasury_before = d.s.svm.get_balance(&treasury).unwrap();
    let exec_slash = execute_slash_ix(&d.s, offender.st.kp.pubkey(), 0, treasury);
    send(&mut d.s.svm, &[exec_slash], &cranker, &[]).expect("permissionless execution");
    let destination = read_account::<Registrar>(&d.s.svm, &registrar).slash_destination;
    assert_eq!(token::balance(&d.s.svm, &destination), 4 * VCOIN);
    assert!(
        d.s.svm.get_balance(&treasury).unwrap() > treasury_before,
        "rent back to the treasury"
    );
}

#[test]
fn the_authority_can_repoint_the_slash_destination() {
    let mut d = dao(0);
    let admin = d.s.admin.insecure_clone();
    let vcoin_mint = d.s.vcoin_mint;
    let stranger = funded_keypair(&mut d.s.svm, 2);
    let new_destination =
        token::create_token_account(&mut d.s.svm, &admin, &vcoin_mint, &stranger.pubkey());
    let set = |authority: Pubkey, destination: Pubkey, d: &Dao| {
        ix(
            staking_protocol::ID,
            staking_protocol::accounts::SetSlashDestination {
                registrar: d.s.registrar,
                authority,
                vcoin_mint: d.s.vcoin_mint,
                slash_destination: destination,
                token_program: spl_token_2022::ID,
            },
            staking_protocol::instruction::SetSlashDestination {},
        )
    };
    let by_stranger = set(stranger.pubkey(), new_destination, &d);
    assert_anchor_error(
        send(&mut d.s.svm, &[by_stranger], &stranger, &[]),
        u32::from(CommonError::NotAuthority),
    );

    // Must be a VCoin account.
    let receipt = receipt_ata(&d.s, &admin.pubkey());
    create_receipt_ata(&mut d.s, &admin.pubkey());
    let wrong_mint = set(admin.pubkey(), receipt, &d);
    assert_anchor_error(
        send(&mut d.s.svm, &[wrong_mint], &admin, &[]),
        anchor_lang::error::ErrorCode::ConstraintTokenMint as u32,
    );

    let ok = set(admin.pubkey(), new_destination, &d);
    send(&mut d.s.svm, &[ok], &admin, &[]).expect("repoint");
    assert_eq!(
        read_account::<Registrar>(&d.s.svm, &d.s.registrar).slash_destination,
        new_destination
    );
}
