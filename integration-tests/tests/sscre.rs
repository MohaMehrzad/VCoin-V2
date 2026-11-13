//! sscre-protocol (per-epoch merkle distributor) in LiteSVM with real Token-2022 VCoin.
//! Audit: SSC-01..13, T-SSC-*.
#![allow(clippy::result_large_err)] // TransactionResult, as in the harness

use anchor_lang::{prelude::Pubkey, system_program};
use anchor_spl::token_2022::spl_token_2022;
use integration_tests::solana_sdk::{instruction::Instruction, signature::Keypair, signer::Signer};
use integration_tests::stack::*;
use integration_tests::*;
use sscre_protocol::{
    constants::*,
    errors::SscreError,
    merkle::builder::Tree,
    state::{ClaimStatus, Epoch, EpochState, PoolConfig, PoolParams},
};
use vcoin_common::CommonError;

const BUDGET: u64 = 10_000_000 * VCOIN;

fn err(e: SscreError) -> u32 {
    6000 + e as u32
}

struct Pool {
    s: Stack,
    config: Pubkey,
    reserve: Pubkey,
    fee_destination: Pubkey,
    oracles: Vec<Keypair>,
}

fn fee_vault() -> Pubkey {
    pda(&[FEE_VAULT_SEED], &sscre_protocol::ID)
}

fn withdraw_fees_ix(p: &Pool) -> Instruction {
    ix(
        sscre_protocol::ID,
        sscre_protocol::accounts::WithdrawFees {
            config: p.config,
            fee_vault: fee_vault(),
            fee_destination: p.fee_destination,
            vcoin_mint: p.s.vcoin_mint,
            token_program: spl_token_2022::ID,
        },
        sscre_protocol::instruction::WithdrawFees {},
    )
}

fn epoch_pda(i: u64) -> Pubkey {
    pda(&[EPOCH_SEED, &i.to_le_bytes()], &sscre_protocol::ID)
}

fn epoch_vault(i: u64) -> Pubkey {
    pda(&[EPOCH_VAULT_SEED, &i.to_le_bytes()], &sscre_protocol::ID)
}

fn status_pda(i: u64, claimant: &Pubkey) -> Pubkey {
    pda(
        &[CLAIM_SEED, epoch_pda(i).as_ref(), claimant.as_ref()],
        &sscre_protocol::ID,
    )
}

fn init_ix(
    vcoin_mint: Pubkey,
    authority: Pubkey,
    fee_destination: Pubkey,
    fee_bps: u16,
) -> Instruction {
    ix(
        sscre_protocol::ID,
        sscre_protocol::accounts::InitializePool {
            authority,
            config: pda(&[POOL_CONFIG_SEED], &sscre_protocol::ID),
            vcoin_mint,
            reserve_vault: pda(&[RESERVE_VAULT_SEED], &sscre_protocol::ID),
            fee_vault: fee_vault(),
            fee_destination,
            program: sscre_protocol::ID,
            program_data: programdata(&sscre_protocol::ID),
            token_program: spl_token_2022::ID,
            system_program: system_program::ID,
        },
        sscre_protocol::instruction::InitializePool {
            params: PoolParams::default(),
            first_epoch_budget: BUDGET,
            fee_bps,
        },
    )
}

fn admin_ix<D: anchor_lang::InstructionData>(
    config: Pubkey,
    authority: Pubkey,
    data: D,
) -> Instruction {
    ix(
        sscre_protocol::ID,
        sscre_protocol::accounts::UpdateConfig { config, authority },
        data,
    )
}

fn transfer_vcoin(s: &mut Stack, to: &Pubkey, amount: u64) {
    let admin = s.admin.insecure_clone();
    let from = fund_vcoin(s, &admin.pubkey(), amount);
    let t = spl_token_2022::instruction::transfer_checked(
        &spl_token_2022::ID,
        &from,
        &s.vcoin_mint,
        to,
        &admin.pubkey(),
        &[],
        amount,
        9,
    )
    .unwrap();
    send(&mut s.svm, &[t], &admin, &[]).expect("transfer VCoin");
}

/// Pool with a funded reserve and three active oracles (quorum 2).
fn pool(reserve_amount: u64) -> Pool {
    let mut p = pool_without_oracles(reserve_amount);
    add_oracles(&mut p, 3);
    p
}

fn add_oracles(p: &mut Pool, n: usize) {
    let admin = p.s.admin.insecure_clone();
    for _ in 0..n {
        let o = funded_keypair(&mut p.s.svm, 2);
        let reg = admin_ix(
            p.config,
            admin.pubkey(),
            sscre_protocol::instruction::RegisterOracle { oracle: o.pubkey() },
        );
        send(&mut p.s.svm, &[reg], &admin, &[]).expect("register oracle");
        p.oracles.push(o);
    }
    warp_by(&mut p.s.svm, ORACLE_ACTIVATION_DELAY);
}

fn pool_without_oracles(reserve_amount: u64) -> Pool {
    let mut s = setup();
    let admin = s.admin.insecure_clone();
    deploy_upgradeable(
        &mut s.svm,
        sscre_protocol::ID,
        &program_so("sscre_protocol"),
        Some(admin.pubkey()),
    );
    let vcoin_mint = s.vcoin_mint;
    let fee_destination =
        token::create_token_account(&mut s.svm, &admin, &vcoin_mint, &admin.pubkey());
    let init = init_ix(
        s.vcoin_mint,
        admin.pubkey(),
        fee_destination,
        DEFAULT_FEE_BPS,
    );
    send(&mut s.svm, &[init], &admin, &[]).expect("initialize pool");
    let config = pda(&[POOL_CONFIG_SEED], &sscre_protocol::ID);
    let reserve = pda(&[RESERVE_VAULT_SEED], &sscre_protocol::ID);
    if reserve_amount > 0 {
        transfer_vcoin(&mut s, &reserve, reserve_amount);
    }
    Pool {
        s,
        config,
        reserve,
        fee_destination,
        oracles: Vec::new(),
    }
}

fn start_ix(p: &Pool, index: u64, payer: Pubkey) -> Instruction {
    start_ix_as(p, index, payer, None)
}

/// `authority` signs only when starting an epoch the reserve cannot fund in full.
fn start_ix_as(p: &Pool, index: u64, payer: Pubkey, authority: Option<Pubkey>) -> Instruction {
    ix(
        sscre_protocol::ID,
        sscre_protocol::accounts::StartEpoch {
            config: p.config,
            reserve_vault: p.reserve,
            vcoin_mint: p.s.vcoin_mint,
            epoch: epoch_pda(index),
            epoch_vault: epoch_vault(index),
            payer,
            authority,
            token_program: spl_token_2022::ID,
            system_program: system_program::ID,
        },
        sscre_protocol::instruction::StartEpoch {},
    )
}

fn start(p: &mut Pool, index: u64) {
    let admin = p.s.admin.insecure_clone();
    let instr = start_ix(p, index, admin.pubkey());
    send(&mut p.s.svm, &[instr], &admin, &[]).expect("start epoch");
}

fn vote_ix(p: &Pool, oracle: &Pubkey, index: u64, root: [u8; 32], max_total: u64) -> Instruction {
    ix(
        sscre_protocol::ID,
        sscre_protocol::accounts::VoteRoot {
            config: p.config,
            epoch: epoch_pda(index),
            oracle: *oracle,
        },
        sscre_protocol::instruction::VoteRoot {
            root,
            max_total_claim: max_total,
        },
    )
}

fn vote(
    p: &mut Pool,
    oracle: usize,
    index: u64,
    root: [u8; 32],
    max_total: u64,
) -> TransactionResultAlias {
    let o = p.oracles[oracle].insecure_clone();
    let instr = vote_ix(p, &o.pubkey(), index, root, max_total);
    send(&mut p.s.svm, &[instr], &o, &[])
}

type TransactionResultAlias = litesvm::types::TransactionResult;

fn activate_ix(p: &Pool, index: u64) -> Instruction {
    ix(
        sscre_protocol::ID,
        sscre_protocol::accounts::ActivateRoot {
            config: p.config,
            epoch: epoch_pda(index),
        },
        sscre_protocol::instruction::ActivateRoot {},
    )
}

struct Claimant {
    kp: Keypair,
    token: Pubkey,
}

fn claimant(p: &mut Pool) -> Claimant {
    let kp = Keypair::new(); // no SOL: a relayer pays
    let admin = p.s.admin.insecure_clone();
    let vcoin_mint = p.s.vcoin_mint;
    let token = token::create_token_account(&mut p.s.svm, &admin, &vcoin_mint, &kp.pubkey());
    Claimant { kp, token }
}

fn claim_ix(
    p: &Pool,
    index: u64,
    c: &Claimant,
    amount: u64,
    proof: Vec<[u8; 32]>,
    payer: Pubkey,
) -> Instruction {
    ix(
        sscre_protocol::ID,
        sscre_protocol::accounts::Claim {
            config: p.config,
            epoch: epoch_pda(index),
            vault: epoch_vault(index),
            vcoin_mint: p.s.vcoin_mint,
            claimant: c.kp.pubkey(),
            claimant_token: c.token,
            fee_vault: fee_vault(),
            claim_status: status_pda(index, &c.kp.pubkey()),
            payer,
            token_program: spl_token_2022::ID,
            system_program: system_program::ID,
        },
        sscre_protocol::instruction::Claim { amount, proof },
    )
}

fn clawback_ix(p: &Pool, index: u64, rent_payer: Pubkey) -> Instruction {
    ix(
        sscre_protocol::ID,
        sscre_protocol::accounts::Clawback {
            config: p.config,
            epoch: epoch_pda(index),
            vault: epoch_vault(index),
            reserve_vault: p.reserve,
            vcoin_mint: p.s.vcoin_mint,
            rent_payer,
            token_program: spl_token_2022::ID,
        },
        sscre_protocol::instruction::Clawback {},
    )
}

/// Ends the epoch, gets two matching votes, waits the dispute delay and activates.
fn root_and_activate(p: &mut Pool, index: u64, root: [u8; 32], max_total: u64) {
    let end = read_account::<Epoch>(&p.s.svm, &epoch_pda(index)).end_ts;
    if now(&p.s.svm) < end {
        warp_to(&mut p.s.svm, end);
    }
    vote(p, 0, index, root, max_total).expect("vote 1");
    vote(p, 1, index, root, max_total).expect("vote 2");
    warp_by(&mut p.s.svm, DEFAULT_DISPUTE_DELAY);
    let admin = p.s.admin.insecure_clone();
    let instr = activate_ix(p, index);
    send(&mut p.s.svm, &[instr], &admin, &[]).expect("activate");
}

#[test]
fn a_full_epoch_pays_relayed_claims_and_returns_the_rest() {
    let mut p = pool(50_000_000 * VCOIN);
    start(&mut p, 0);
    assert_eq!(token::balance(&p.s.svm, &p.reserve), 40_000_000 * VCOIN);
    assert_eq!(token::balance(&p.s.svm, &epoch_vault(0)), BUDGET);
    assert_eq!(
        read_account::<PoolConfig>(&p.s.svm, &p.config).next_epoch_budget,
        BUDGET / 100 * 98
    );

    let claimants: Vec<Claimant> = (0..5).map(|_| claimant(&mut p)).collect();
    let amounts = [1_000 * VCOIN, 2_500 * VCOIN, 333 * VCOIN + 7, 10 * VCOIN, 1];
    let leaves: Vec<(Pubkey, u64)> = claimants
        .iter()
        .zip(amounts)
        .map(|(c, a)| (c.kp.pubkey(), a))
        .collect();
    let tree = Tree::new(0, &leaves, BUDGET).unwrap();

    // Before the epoch ends nobody can vote; a pending oracle cannot vote at all.
    let r = vote(&mut p, 0, 0, tree.root(), tree.total);
    assert_anchor_error(r, err(SscreError::EpochNotEnded));
    warp_by(&mut p.s.svm, DEFAULT_EPOCH_DURATION);
    let late = Keypair::new();
    let admin = p.s.admin.insecure_clone();
    let reg = admin_ix(
        p.config,
        admin.pubkey(),
        sscre_protocol::instruction::RegisterOracle {
            oracle: late.pubkey(),
        },
    );
    send(&mut p.s.svm, &[reg], &admin, &[]).expect("register late oracle");
    let instr = vote_ix(&p, &late.pubkey(), 0, tree.root(), tree.total);
    assert_anchor_error(
        send(&mut p.s.svm, &[instr], &admin, &[&late]),
        err(SscreError::NotActiveOracle),
    );

    // One vote proposes nothing; the second matching vote starts the dispute delay.
    vote(&mut p, 0, 0, tree.root(), tree.total).expect("vote 1");
    let act = activate_ix(&p, 0);
    assert_anchor_error(
        send(&mut p.s.svm, std::slice::from_ref(&act), &admin, &[]),
        err(SscreError::NoProposedRoot),
    );
    vote(&mut p, 1, 0, tree.root(), tree.total).expect("vote 2");
    assert_anchor_error(
        send(&mut p.s.svm, std::slice::from_ref(&act), &admin, &[]),
        err(SscreError::DisputeDelayActive),
    );
    warp_by(&mut p.s.svm, DEFAULT_DISPUTE_DELAY);

    // Re-review B2: the late oracle is active by now, so 2 of 4 is no longer a majority.
    // The third vote restores the quorum, and the dispute delay starts again (B3).
    assert_anchor_error(
        send(&mut p.s.svm, std::slice::from_ref(&act), &admin, &[]),
        err(SscreError::QuorumLost),
    );
    vote(&mut p, 2, 0, tree.root(), tree.total).expect("vote 3");
    assert_anchor_error(
        send(&mut p.s.svm, std::slice::from_ref(&act), &admin, &[]),
        err(SscreError::DisputeDelayActive),
    );
    warp_by(&mut p.s.svm, DEFAULT_DISPUTE_DELAY);
    send(&mut p.s.svm, &[act], &admin, &[]).expect("activate");

    // A relayer pays rent and fees; claimants hold no SOL (SSC-07).
    let relayer = funded_keypair(&mut p.s.svm, 5);
    let mut fees = 0;
    for (i, c) in claimants.iter().enumerate() {
        let instr = claim_ix(&p, 0, c, amounts[i], tree.proof(i), relayer.pubkey());
        send(&mut p.s.svm, &[instr], &relayer, &[&c.kp]).expect("claim");
        let fee = (amounts[i] as u128 * 100).div_ceil(10_000) as u64;
        fees += fee;
        assert_eq!(
            token::balance(&p.s.svm, &c.token),
            amounts[i] - fee,
            "1% fee rounded up"
        );
    }
    // Re-review B6: fees collect in the program's vault; anyone sweeps them to the treasury.
    assert_eq!(token::balance(&p.s.svm, &fee_vault()), fees);
    assert_eq!(token::balance(&p.s.svm, &p.fee_destination), 0);
    let sweep = withdraw_fees_ix(&p);
    send(&mut p.s.svm, std::slice::from_ref(&sweep), &relayer, &[]).expect("withdraw fees");
    assert_eq!(token::balance(&p.s.svm, &p.fee_destination), fees);
    warp_by(&mut p.s.svm, 1);
    assert_anchor_error(
        send(&mut p.s.svm, &[sweep], &relayer, &[]),
        err(SscreError::NothingToWithdraw),
    );
    let e: Epoch = read_account(&p.s.svm, &epoch_pda(0));
    assert_eq!(
        (e.total_claimed, e.num_claims),
        (tree.total, 5),
        "gross accounting"
    );

    // No double claim, no inflated amount.
    warp_by(&mut p.s.svm, 1);
    let again = claim_ix(
        &p,
        0,
        &claimants[0],
        amounts[0],
        tree.proof(0),
        relayer.pubkey(),
    );
    assert!(send(&mut p.s.svm, &[again], &relayer, &[&claimants[0].kp]).is_err());
    let extra = claimant(&mut p);
    let forged = claim_ix(&p, 0, &extra, amounts[0], tree.proof(0), relayer.pubkey());
    assert_anchor_error(
        send(&mut p.s.svm, &[forged], &relayer, &[&extra.kp]),
        err(SscreError::InvalidProof),
    );

    // Clawback only after the window; it returns exactly the unclaimed budget.
    let cb = clawback_ix(&p, 0, admin.pubkey());
    assert_anchor_error(
        send(&mut p.s.svm, std::slice::from_ref(&cb), &admin, &[]),
        err(SscreError::ClaimWindowOpen),
    );
    let status = ix(
        sscre_protocol::ID,
        sscre_protocol::accounts::CloseClaimStatus {
            epoch: epoch_pda(0),
            claim_status: status_pda(0, &claimants[0].kp.pubkey()),
            rent_payer: relayer.pubkey(),
        },
        sscre_protocol::instruction::CloseClaimStatus {},
    );
    assert_anchor_error(
        send(&mut p.s.svm, std::slice::from_ref(&status), &admin, &[]),
        err(SscreError::EpochNotClosed),
    );
    warp_by(&mut p.s.svm, DEFAULT_CLAIM_WINDOW);
    let late_claim = claim_ix(&p, 0, &extra, 1, vec![], relayer.pubkey());
    assert_anchor_error(
        send(&mut p.s.svm, &[late_claim], &relayer, &[&extra.kp]),
        err(SscreError::ClaimWindowClosed),
    );
    send(&mut p.s.svm, &[cb], &admin, &[]).expect("clawback");
    assert_eq!(
        token::balance(&p.s.svm, &p.reserve),
        40_000_000 * VCOIN + BUDGET - tree.total
    );
    assert!(
        p.s.svm
            .get_account(&epoch_vault(0))
            .is_none_or(|a| a.lamports == 0),
        "vault closed"
    );
    assert_eq!(
        read_account::<Epoch>(&p.s.svm, &epoch_pda(0)).state,
        EpochState::Closed
    );
    let relayer_before = p.s.svm.get_balance(&relayer.pubkey()).unwrap();
    send(&mut p.s.svm, &[status], &admin, &[]).expect("close status");
    let rent = p.s.svm.minimum_balance_for_rent_exemption(ClaimStatus::LEN);
    assert_eq!(
        p.s.svm.get_balance(&relayer.pubkey()).unwrap(),
        relayer_before + rent
    );
}

#[test]
fn an_epoch_never_pays_beyond_its_approved_total_or_budget() {
    let mut p = pool(BUDGET);
    start(&mut p, 0);
    warp_by(&mut p.s.svm, DEFAULT_EPOCH_DURATION);
    let r = vote(&mut p, 0, 0, [1; 32], BUDGET + 1);
    assert_anchor_error(r, err(SscreError::InvalidRoot));
    let r = vote(&mut p, 0, 0, [0; 32], 1);
    assert_anchor_error(r, err(SscreError::InvalidRoot));

    // A tree whose leaves exceed the voted max_total: claims stop at the cap (SSC-02).
    let cs: Vec<Claimant> = (0..3).map(|_| claimant(&mut p)).collect();
    let leaves: Vec<(Pubkey, u64)> = cs.iter().map(|c| (c.kp.pubkey(), 400 * VCOIN)).collect();
    let tree = Tree::new(0, &leaves, u64::MAX).unwrap();
    root_and_activate(&mut p, 0, tree.root(), 1_000 * VCOIN);
    let relayer = funded_keypair(&mut p.s.svm, 5);
    for (i, c) in cs.iter().take(2).enumerate() {
        let instr = claim_ix(&p, 0, c, 400 * VCOIN, tree.proof(i), relayer.pubkey());
        send(&mut p.s.svm, &[instr], &relayer, &[&c.kp]).expect("within cap");
    }
    let instr = claim_ix(&p, 0, &cs[2], 400 * VCOIN, tree.proof(2), relayer.pubkey());
    assert_anchor_error(
        send(&mut p.s.svm, &[instr], &relayer, &[&cs[2].kp]),
        err(SscreError::ExceedsMaxTotalClaim),
    );
    // Proof length is bounded before hashing.
    let instr = claim_ix(
        &p,
        0,
        &cs[2],
        1,
        vec![[0; 32]; MAX_PROOF_LEN + 1],
        relayer.pubkey(),
    );
    assert_anchor_error(
        send(&mut p.s.svm, &[instr], &relayer, &[&cs[2].kp]),
        err(SscreError::ProofTooLong),
    );
}

#[test]
fn veto_quorum_loss_and_oracle_management() {
    let mut p = pool(BUDGET);
    let admin = p.s.admin.insecure_clone();
    start(&mut p, 0);
    warp_by(&mut p.s.svm, DEFAULT_EPOCH_DURATION);
    vote(&mut p, 0, 0, [1; 32], 100).unwrap();
    vote(&mut p, 1, 0, [1; 32], 100).unwrap();

    // DAO veto wipes the candidate and every vote.
    let stranger = funded_keypair(&mut p.s.svm, 1);
    let config = p.config;
    let veto = move |authority: Pubkey| {
        ix(
            sscre_protocol::ID,
            sscre_protocol::accounts::VetoRoot {
                config,
                epoch: epoch_pda(0),
                authority,
            },
            sscre_protocol::instruction::VetoRoot {},
        )
    };
    let r = send(&mut p.s.svm, &[veto(stranger.pubkey())], &stranger, &[]);
    assert_anchor_error(r, u32::from(CommonError::NotAuthorityOrGuardian));
    send(&mut p.s.svm, &[veto(admin.pubkey())], &admin, &[]).expect("veto");
    let e: Epoch = read_account(&p.s.svm, &epoch_pda(0));
    assert_eq!(e.proposed_at, 0);
    assert!(e.votes.iter().all(|v| v.oracle == Pubkey::default()));

    // Re-review B1: a DAO vote is slower than the dispute delay, so the guardian can veto
    // (and pause) at once; it cannot do anything else.
    let guardian = funded_keypair(&mut p.s.svm, 1);
    let set_guardian = admin_ix(
        p.config,
        admin.pubkey(),
        sscre_protocol::instruction::SetGuardian {
            guardian: guardian.pubkey(),
        },
    );
    send(&mut p.s.svm, &[set_guardian], &admin, &[]).expect("set guardian");
    vote(&mut p, 0, 0, [9; 32], 100).unwrap();
    vote(&mut p, 1, 0, [9; 32], 100).unwrap();
    send(&mut p.s.svm, &[veto(guardian.pubkey())], &guardian, &[]).expect("guardian veto");
    assert_eq!(
        read_account::<Epoch>(&p.s.svm, &epoch_pda(0)).proposed_at,
        0
    );
    let r = send(
        &mut p.s.svm,
        &[admin_ix(
            p.config,
            guardian.pubkey(),
            sscre_protocol::instruction::SetRootQuorum { quorum: 3 },
        )],
        &guardian,
        &[],
    );
    assert_anchor_error(r, u32::from(CommonError::NotAuthority));

    // Changing a vote away from the candidate drops the proposal.
    vote(&mut p, 0, 0, [2; 32], 100).unwrap();
    vote(&mut p, 1, 0, [2; 32], 100).unwrap();
    assert_ne!(
        read_account::<Epoch>(&p.s.svm, &epoch_pda(0)).proposed_at,
        0
    );
    vote(&mut p, 1, 0, [3; 32], 100).unwrap();
    assert_eq!(
        read_account::<Epoch>(&p.s.svm, &epoch_pda(0)).proposed_at,
        0
    );

    // Removing a supporting oracle makes the candidate lose quorum at activation.
    vote(&mut p, 1, 0, [2; 32], 100).unwrap();
    let remove = admin_ix(
        p.config,
        admin.pubkey(),
        sscre_protocol::instruction::RemoveOracle {
            oracle: p.oracles[0].pubkey(),
        },
    );
    send(&mut p.s.svm, &[remove], &admin, &[]).expect("remove oracle");
    warp_by(&mut p.s.svm, DEFAULT_DISPUTE_DELAY);
    let act = activate_ix(&p, 0);
    assert_anchor_error(
        send(&mut p.s.svm, &[act], &admin, &[]),
        err(SscreError::QuorumLost),
    );

    // Bounds.
    for (data, code) in [
        (
            sscre_protocol::instruction::SetRootQuorum { quorum: 1 },
            err(SscreError::InvalidQuorum),
        ),
        (
            sscre_protocol::instruction::SetRootQuorum { quorum: 3 },
            err(SscreError::InvalidQuorum),
        ),
    ] {
        let r = send(
            &mut p.s.svm,
            &[admin_ix(p.config, admin.pubkey(), data)],
            &admin,
            &[],
        );
        assert_anchor_error(r, code);
    }
    let r = send(
        &mut p.s.svm,
        &[admin_ix(
            p.config,
            admin.pubkey(),
            sscre_protocol::instruction::RemoveOracle {
                oracle: p.oracles[1].pubkey(),
            },
        )],
        &admin,
        &[],
    );
    assert_anchor_error(r, err(SscreError::InvalidQuorum));
    let r = send(
        &mut p.s.svm,
        &[admin_ix(
            p.config,
            admin.pubkey(),
            sscre_protocol::instruction::RegisterOracle {
                oracle: p.oracles[1].pubkey(),
            },
        )],
        &admin,
        &[],
    );
    assert_anchor_error(r, err(SscreError::OracleExists));
    let r = send(
        &mut p.s.svm,
        &[admin_ix(
            p.config,
            admin.pubkey(),
            sscre_protocol::instruction::RegisterOracle {
                oracle: Pubkey::default(),
            },
        )],
        &admin,
        &[],
    );
    assert_anchor_error(r, err(SscreError::InvalidOracle));
}

#[test]
fn epochs_are_rate_limited_decay_and_stop_at_an_empty_reserve() {
    let mut p = pool(BUDGET + BUDGET / 2);
    let admin = p.s.admin.insecure_clone();
    start(&mut p, 0);
    let again = start_ix(&p, 1, admin.pubkey());
    assert_anchor_error(
        send(&mut p.s.svm, &[again], &admin, &[]),
        err(SscreError::EpochTooSoon),
    );
    warp_by(&mut p.s.svm, DEFAULT_EPOCH_DURATION);
    // Re-review B5: only the authority may start an epoch the reserve cannot fund in full.
    let stranger = funded_keypair(&mut p.s.svm, 2);
    let partial = start_ix(&p, 1, stranger.pubkey());
    assert_anchor_error(
        send(&mut p.s.svm, &[partial], &stranger, &[]),
        err(SscreError::ReserveBelowBudget),
    );
    // Being the payer is not enough, and a stranger signing as "authority" is refused.
    let as_payer_only = start_ix(&p, 1, admin.pubkey());
    assert_anchor_error(
        send(&mut p.s.svm, &[as_payer_only], &admin, &[]),
        err(SscreError::ReserveBelowBudget),
    );
    let fake = start_ix_as(&p, 1, stranger.pubkey(), Some(stranger.pubkey()));
    assert_anchor_error(
        send(&mut p.s.svm, &[fake], &stranger, &[]),
        err(SscreError::ReserveBelowBudget),
    );
    // The authority signs; someone else pays the rent (after launch the authority is a
    // governance PDA, which can sign through a proposal but cannot pay).
    let sponsored = start_ix_as(&p, 1, stranger.pubkey(), Some(admin.pubkey()));
    send(&mut p.s.svm, &[sponsored], &stranger, &[&admin])
        .expect("authority-approved partial epoch");
    assert_eq!(
        read_account::<Epoch>(&p.s.svm, &epoch_pda(1)).budget,
        BUDGET / 2,
        "min(9.8M, reserve 5M)"
    );
    warp_by(&mut p.s.svm, DEFAULT_EPOCH_DURATION);
    let empty = start_ix(&p, 2, admin.pubkey());
    assert_anchor_error(
        send(&mut p.s.svm, &[empty], &admin, &[]),
        err(SscreError::ReserveEmpty),
    );

    // An epoch that never gets a root returns its whole budget after the root deadline.
    let cb = clawback_ix(&p, 1, stranger.pubkey()); // epoch 1's rent was paid by `stranger`
    assert_anchor_error(
        send(&mut p.s.svm, std::slice::from_ref(&cb), &admin, &[]),
        err(SscreError::ClaimWindowOpen),
    );
    warp_by(&mut p.s.svm, DEFAULT_CLAIM_WINDOW);
    let r = vote(&mut p, 0, 1, [1; 32], 1);
    assert_anchor_error(r, err(SscreError::RootDeadlinePassed));
    send(&mut p.s.svm, &[cb], &admin, &[]).expect("clawback unrooted epoch");
    assert_eq!(token::balance(&p.s.svm, &p.reserve), BUDGET / 2);

    // The DAO cannot raise the next budget above the initial ceiling.
    let raise = admin_ix(
        p.config,
        admin.pubkey(),
        sscre_protocol::instruction::SetNextEpochBudget { budget: BUDGET + 1 },
    );
    assert_anchor_error(
        send(&mut p.s.svm, &[raise], &admin, &[]),
        err(SscreError::BudgetTooHigh),
    );
}

#[test]
fn a_pause_blocks_claims_but_extends_the_window() {
    let mut p = pool(BUDGET);
    let admin = p.s.admin.insecure_clone();
    start(&mut p, 0);
    let c = claimant(&mut p);
    let tree = Tree::new(0, &[(c.kp.pubkey(), 50 * VCOIN)], BUDGET).unwrap();
    root_and_activate(&mut p, 0, tree.root(), tree.total);

    warp_by(&mut p.s.svm, DEFAULT_CLAIM_WINDOW - 10 * DAY);
    let pause = |paused: bool| {
        admin_ix(
            p.config,
            admin.pubkey(),
            sscre_protocol::instruction::SetPaused { paused },
        )
    };
    send(&mut p.s.svm, &[pause(true)], &admin, &[]).expect("pause");
    warp_by(&mut p.s.svm, 30 * DAY);
    let relayer = funded_keypair(&mut p.s.svm, 2);
    let claim = claim_ix(&p, 0, &c, 50 * VCOIN, tree.proof(0), relayer.pubkey());
    assert_anchor_error(
        send(
            &mut p.s.svm,
            std::slice::from_ref(&claim),
            &relayer,
            &[&c.kp],
        ),
        err(SscreError::Paused),
    );
    send(&mut p.s.svm, &[pause(false)], &admin, &[]).expect("unpause");
    // 20 days past the original deadline, but 30 paused days were added back.
    let cb = clawback_ix(&p, 0, admin.pubkey());
    assert_anchor_error(
        send(&mut p.s.svm, &[cb], &admin, &[]),
        err(SscreError::ClaimWindowOpen),
    );
    send(&mut p.s.svm, &[claim], &relayer, &[&c.kp]).expect("claim after the pause");
}

#[test]
fn initialization_is_gated_and_fee_changes_are_timelocked_and_snapshotted() {
    let mut s = setup();
    let admin = s.admin.insecure_clone();
    deploy_upgradeable(
        &mut s.svm,
        sscre_protocol::ID,
        &program_so("sscre_protocol"),
        Some(admin.pubkey()),
    );
    let vcoin_mint = s.vcoin_mint;
    let fee_dest = token::create_token_account(&mut s.svm, &admin, &vcoin_mint, &admin.pubkey());
    let stranger = funded_keypair(&mut s.svm, 2);
    let r = send(
        &mut s.svm,
        &[init_ix(s.vcoin_mint, stranger.pubkey(), fee_dest, 100)],
        &stranger,
        &[],
    );
    assert_anchor_error(r, u32::from(CommonError::NotUpgradeAuthority));
    let r = send(
        &mut s.svm,
        &[init_ix(s.vcoin_mint, admin.pubkey(), fee_dest, 101)],
        &admin,
        &[],
    );
    assert_anchor_error(r, err(SscreError::FeeTooHigh));

    let mut p = pool(BUDGET);
    let admin = p.s.admin.insecure_clone();
    start(&mut p, 0);
    let c = claimant(&mut p);
    let tree = Tree::new(0, &[(c.kp.pubkey(), 1_000 * VCOIN)], BUDGET).unwrap();
    root_and_activate(&mut p, 0, tree.root(), tree.total);

    let vcoin_mint = p.s.vcoin_mint;
    let new_dest = token::create_token_account(&mut p.s.svm, &admin, &vcoin_mint, &admin.pubkey());
    let propose = |fee_bps: u16| {
        ix(
            sscre_protocol::ID,
            sscre_protocol::accounts::ProposeFee {
                config: p.config,
                authority: admin.pubkey(),
                vcoin_mint,
                fee_destination: new_dest,
                token_program: spl_token_2022::ID,
            },
            sscre_protocol::instruction::ProposeFee { fee_bps },
        )
    };
    assert_anchor_error(
        send(&mut p.s.svm, &[propose(101)], &admin, &[]),
        err(SscreError::FeeTooHigh),
    );
    send(&mut p.s.svm, &[propose(0)], &admin, &[]).expect("propose 0%");
    let apply = ix(
        sscre_protocol::ID,
        sscre_protocol::accounts::ApplyFee { config: p.config },
        sscre_protocol::instruction::ApplyFee {},
    );
    assert_anchor_error(
        send(&mut p.s.svm, std::slice::from_ref(&apply), &admin, &[]),
        err(SscreError::FeeTimelockActive),
    );
    warp_by(&mut p.s.svm, FEE_CHANGE_DELAY);
    send(&mut p.s.svm, &[apply], &admin, &[]).expect("apply");
    let cfg: PoolConfig = read_account(&p.s.svm, &p.config);
    assert_eq!((cfg.fee_bps, cfg.fee_destination), (0, new_dest));

    // The activated epoch keeps the 1% it was activated with; the sweep pays the new account
    // and refuses the old one.
    let relayer = funded_keypair(&mut p.s.svm, 2);
    let claim = claim_ix(&p, 0, &c, 1_000 * VCOIN, tree.proof(0), relayer.pubkey());
    send(&mut p.s.svm, &[claim], &relayer, &[&c.kp]).expect("claim");
    assert_eq!(token::balance(&p.s.svm, &c.token), 990 * VCOIN);
    assert_eq!(token::balance(&p.s.svm, &fee_vault()), 10 * VCOIN);
    let old = withdraw_fees_ix(&p);
    assert!(send(&mut p.s.svm, &[old], &relayer, &[]).is_err());
    p.fee_destination = new_dest;
    let sweep = withdraw_fees_ix(&p);
    send(&mut p.s.svm, &[sweep], &relayer, &[]).expect("withdraw fees");
    assert_eq!(token::balance(&p.s.svm, &new_dest), 10 * VCOIN);
}

/// Re-review B2: the configured quorum (default 2) is a floor. A root always needs a strict
/// majority of the active oracles, so a minority of a larger set cannot settle one.
#[test]
fn a_root_needs_a_majority_of_the_active_oracles() {
    let mut p = pool(BUDGET);
    add_oracles(&mut p, 2); // 5 active, configured quorum still 2
    assert_eq!(
        read_account::<PoolConfig>(&p.s.svm, &p.config).root_quorum,
        DEFAULT_ROOT_QUORUM
    );
    start(&mut p, 0);
    warp_by(&mut p.s.svm, DEFAULT_EPOCH_DURATION);

    vote(&mut p, 0, 0, [1; 32], 100).unwrap();
    vote(&mut p, 1, 0, [1; 32], 100).unwrap();
    assert_eq!(
        read_account::<Epoch>(&p.s.svm, &epoch_pda(0)).proposed_at,
        0,
        "2 of 5 proposes nothing"
    );
    // A competing pair cannot propose either.
    vote(&mut p, 3, 0, [2; 32], 100).unwrap();
    vote(&mut p, 4, 0, [2; 32], 100).unwrap();
    assert_eq!(
        read_account::<Epoch>(&p.s.svm, &epoch_pda(0)).proposed_at,
        0
    );
    vote(&mut p, 2, 0, [1; 32], 100).unwrap();
    let e: Epoch = read_account(&p.s.svm, &epoch_pda(0));
    assert_ne!(e.proposed_at, 0, "3 of 5 is a majority");
    assert_eq!(e.proposed_root, [1; 32]);
}

/// Re-review B3: a proposal that lost its quorum (oracle removed, quorum raised) must not
/// keep its old timestamp, and a re-registered oracle's old vote must not count.
#[test]
fn a_proposal_that_lost_quorum_restarts_its_dispute_delay() {
    let mut p = pool(BUDGET);
    let admin = p.s.admin.insecure_clone();
    start(&mut p, 0);
    warp_by(&mut p.s.svm, DEFAULT_EPOCH_DURATION);
    vote(&mut p, 0, 0, [1; 32], 100).unwrap();
    vote(&mut p, 1, 0, [1; 32], 100).unwrap();
    let first = read_account::<Epoch>(&p.s.svm, &epoch_pda(0)).proposed_at;
    assert_ne!(first, 0);

    // The DAO removes oracle 0, adds it back and raises the quorum to all three.
    let oracle0 = p.oracles[0].pubkey();
    for data_ix in [
        admin_ix(
            p.config,
            admin.pubkey(),
            sscre_protocol::instruction::RemoveOracle { oracle: oracle0 },
        ),
        admin_ix(
            p.config,
            admin.pubkey(),
            sscre_protocol::instruction::RegisterOracle { oracle: oracle0 },
        ),
        admin_ix(
            p.config,
            admin.pubkey(),
            sscre_protocol::instruction::SetRootQuorum { quorum: 3 },
        ),
    ] {
        send(&mut p.s.svm, &[data_ix], &admin, &[]).expect("oracle management");
    }
    // The old dispute delay has run out and oracle 0 is active again.
    warp_by(
        &mut p.s.svm,
        DEFAULT_DISPUTE_DELAY.max(ORACLE_ACTIVATION_DELAY),
    );
    let act = activate_ix(&p, 0);
    assert_anchor_error(
        send(&mut p.s.svm, std::slice::from_ref(&act), &admin, &[]),
        err(SscreError::QuorumLost),
    );

    // Oracle 2 agrees: still only 2 live votes, because oracle 0 voted under its old
    // registration.
    vote(&mut p, 2, 0, [1; 32], 100).unwrap();
    assert_eq!(
        read_account::<Epoch>(&p.s.svm, &epoch_pda(0)).proposed_at,
        0
    );
    // Oracle 0 votes again: proposed anew, with a fresh dispute delay.
    vote(&mut p, 0, 0, [1; 32], 100).unwrap();
    let second = read_account::<Epoch>(&p.s.svm, &epoch_pda(0)).proposed_at;
    assert!(second > first);
    assert_anchor_error(
        send(&mut p.s.svm, std::slice::from_ref(&act), &admin, &[]),
        err(SscreError::DisputeDelayActive),
    );
    warp_by(&mut p.s.svm, DEFAULT_DISPUTE_DELAY);
    send(&mut p.s.svm, &[act], &admin, &[]).expect("activate");
}

/// Re-review B4/B5: an epoch keeps the windows it started with, and cannot start before
/// enough oracles exist to ever settle its root.
#[test]
fn epochs_snapshot_their_windows_and_need_oracles_to_start() {
    let mut p = pool_without_oracles(BUDGET);
    let admin = p.s.admin.insecure_clone();
    let early = start_ix(&p, 0, admin.pubkey());
    assert_anchor_error(
        send(&mut p.s.svm, &[early], &admin, &[]),
        err(SscreError::NotEnoughOracles),
    );
    add_oracles(&mut p, 3);
    start(&mut p, 0);

    // The DAO stretches both windows afterwards; epoch 0 is unaffected.
    let params = PoolParams {
        claim_window: MAX_CLAIM_WINDOW,
        dispute_delay: MAX_DISPUTE_DELAY,
        ..PoolParams::default()
    };
    let set = admin_ix(
        p.config,
        admin.pubkey(),
        sscre_protocol::instruction::SetParams { params },
    );
    send(&mut p.s.svm, &[set], &admin, &[]).expect("set params");
    let e: Epoch = read_account(&p.s.svm, &epoch_pda(0));
    assert_eq!(
        (e.claim_window, e.dispute_delay),
        (DEFAULT_CLAIM_WINDOW, DEFAULT_DISPUTE_DELAY)
    );

    warp_by(&mut p.s.svm, DEFAULT_EPOCH_DURATION);
    vote(&mut p, 0, 0, [1; 32], 100).unwrap();
    vote(&mut p, 1, 0, [1; 32], 100).unwrap();
    warp_by(&mut p.s.svm, DEFAULT_DISPUTE_DELAY);
    let act = activate_ix(&p, 0);
    send(&mut p.s.svm, &[act], &admin, &[]).expect("activates after the original delay");
    let e: Epoch = read_account(&p.s.svm, &epoch_pda(0));
    assert_eq!(e.claim_deadline - e.activated_at, DEFAULT_CLAIM_WINDOW);
}
