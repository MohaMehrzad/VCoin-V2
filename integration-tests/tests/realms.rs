//! staking-protocol as an SPL Governance (Realms) voter-weight plugin, end to end against the
//! deployed mainnet governance binary (`integration-tests/fixtures/spl_governance.so`).
//!
//! Audit coverage: GOV-01 (one-shot finalize), GOV-02 (linear Sybil-neutral weight),
//! GOV-03 (votes lock withdrawals until relinquished), GOV-04/06 (thresholds, hold-up and
//! real execution by the governance PDA), GOV-05 (Realms delegation), GOV-10/11 (DAO-only
//! parameters), X-5 (foreign account reads), 5A participation gate.

use anchor_lang::{prelude::Pubkey, system_program, AccountSerialize};
use anchor_spl::token::spl_token;
use anchor_spl::token_2022::spl_token_2022;
use five_a_protocol::{constants::USER_SCORE_SEED, state::UserScore};
use governance::{
    instruction as gov_ix,
    state::{
        enums::{MintMaxVoterWeightSource, ProposalState, VoteThreshold, VoteTipping},
        governance::{get_governance_address, GovernanceConfig},
        native_treasury::get_native_treasury_address,
        proposal::{get_proposal_address, ProposalV2, VoteType},
        proposal_transaction::{get_proposal_transaction_address, InstructionData},
        realm::{get_realm_address, GoverningTokenConfigAccountArgs},
        realm_config::GoverningTokenType,
        token_owner_record::get_token_owner_record_address,
        vote_record::{Vote, VoteChoice},
    },
};
use integration_tests::solana_sdk::{
    account::Account,
    instruction::{AccountMeta, Instruction},
    signature::Keypair,
    signer::Signer,
};
use integration_tests::stack::*;
use integration_tests::*;
use staking_protocol::{
    constants::*,
    errors::StakingError,
    state::{MaxVoterWeightRecord, Registrar, VoterWeightAction, VoterWeightRecord},
};
use vcoin_common::{CommonError, AUTHORITY_TRANSFER_DELAY};

const GOV_ID: Pubkey = Pubkey::from_str_const("GovER5Lthms3bLBqWub97yVrMmEogzX7xNjdXpPPCVZw");
const VOTING_BASE_TIME: u32 = 3_600;
const HOLD_UP_TIME: u32 = 60;
/// spl-governance error codes (GovernanceError).
const ERR_VWR_EXPIRED: u32 = 577;
const ERR_VWR_INVALID_ACTION: u32 = 587;
const ERR_VWR_INVALID_TARGET: u32 = 588;

fn serr(e: StakingError) -> u32 {
    6000 + e as u32
}

struct Dao {
    s: Stack,
    realm: Pubkey,
    max_vwr: Pubkey,
}

struct Member {
    st: Staker,
    vwr: Pubkey,
    tor: Pubkey,
}

fn load_governance(s: &mut Stack) {
    let so = std::fs::read(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/fixtures/spl_governance.so"
    ))
    .unwrap();
    s.svm.add_program(GOV_ID, &so).unwrap();
}

/// Realm for `mint` with this staking program as (max) voter-weight addin, Dormant token type.
fn create_realm(s: &mut Stack, name: &str, mint: &Pubkey, token_2022: bool) -> Pubkey {
    let admin = s.admin.insecure_clone();
    let create = gov_ix::create_realm(
        &GOV_ID,
        &admin.pubkey(),
        mint,
        &admin.pubkey(),
        None,
        Some(GoverningTokenConfigAccountArgs {
            voter_weight_addin: Some(staking_protocol::ID),
            max_voter_weight_addin: Some(staking_protocol::ID),
            token_type: GoverningTokenType::Dormant,
        }),
        None,
        name.to_string(),
        1,
        MintMaxVoterWeightSource::FULL_SUPPLY_FRACTION,
        token_2022,
        false,
    );
    send(&mut s.svm, &[create], &admin, &[]).expect("create realm");
    get_realm_address(&GOV_ID, name)
}

fn configure_ix(
    (registrar, vcoin_mint): (Pubkey, Pubkey),
    authority: Pubkey,
    realm: Pubkey,
    slash_destination: Pubkey,
    min_vote_score: u16,
) -> Instruction {
    ix(
        staking_protocol::ID,
        staking_protocol::accounts::ConfigureGovernance {
            registrar,
            authority,
            realm,
            vcoin_mint,
            slash_destination,
            max_voter_weight_record: pda(&[MAX_VOTER_WEIGHT_RECORD_SEED], &staking_protocol::ID),
            payer: authority,
            token_program: spl_token_2022::ID,
            system_program: system_program::ID,
        },
        staking_protocol::instruction::ConfigureGovernance {
            governance_program_id: GOV_ID,
            min_vote_score,
            min_propose_score: 0,
        },
    )
}

/// VCoin realm wired to the registrar.
fn dao(min_vote_score: u16) -> Dao {
    let mut s = setup();
    load_governance(&mut s);
    let admin = s.admin.insecure_clone();
    let vcoin_mint = s.vcoin_mint;
    let realm = create_realm(&mut s, "VCoin DAO", &vcoin_mint, true);
    let slash_destination =
        token::create_token_account(&mut s.svm, &admin, &vcoin_mint, &admin.pubkey());
    let configure = configure_ix(
        (s.registrar, s.vcoin_mint),
        admin.pubkey(),
        realm,
        slash_destination,
        min_vote_score,
    );
    send(&mut s.svm, &[configure], &admin, &[]).expect("configure governance");
    let max_vwr = pda(&[MAX_VOTER_WEIGHT_RECORD_SEED], &staking_protocol::ID);
    Dao { s, realm, max_vwr }
}

fn join(d: &mut Dao, stake: u64, lock: i64) -> Member {
    let st = new_staker(&mut d.s, stake);
    let dep = deposit_ix(&d.s, &st, 0, stake, lock);
    send(&mut d.s.svm, &[dep], &st.kp, &[]).expect("deposit");
    let vwr = pda(
        &[VOTER_WEIGHT_RECORD_SEED, st.kp.pubkey().as_ref()],
        &staking_protocol::ID,
    );
    let create_vwr = ix(
        staking_protocol::ID,
        staking_protocol::accounts::CreateVoterWeightRecord {
            registrar: d.s.registrar,
            voter_weight_record: vwr,
            owner: st.kp.pubkey(),
            payer: st.kp.pubkey(),
            system_program: system_program::ID,
        },
        staking_protocol::instruction::CreateVoterWeightRecord {},
    );
    let create_tor = gov_ix::create_token_owner_record(
        &GOV_ID,
        &d.realm,
        &st.kp.pubkey(),
        &d.s.vcoin_mint,
        &st.kp.pubkey(),
    );
    send(&mut d.s.svm, &[create_vwr, create_tor], &st.kp, &[]).expect("vwr + token owner record");
    let tor = get_token_owner_record_address(&GOV_ID, &d.realm, &d.s.vcoin_mint, &st.kp.pubkey());
    Member { st, vwr, tor }
}

fn update_vwr_ix(
    d: &Dao,
    m: &Member,
    action: VoterWeightAction,
    target: Option<Pubkey>,
) -> Instruction {
    let user_score = pda(
        &[USER_SCORE_SEED, m.st.kp.pubkey().as_ref()],
        &five_a_protocol::ID,
    );
    ix(
        staking_protocol::ID,
        staking_protocol::accounts::UpdateVoterWeightRecord {
            registrar: d.s.registrar,
            voter: m.st.voter,
            voter_weight_record: m.vwr,
            user_score,
        },
        staking_protocol::instruction::UpdateVoterWeightRecord { action, target },
    )
}

fn update_max_ix(d: &Dao) -> Instruction {
    ix(
        staking_protocol::ID,
        staking_protocol::accounts::UpdateMaxVoterWeightRecord {
            registrar: d.s.registrar,
            max_voter_weight_record: d.max_vwr,
        },
        staking_protocol::instruction::UpdateMaxVoterWeightRecord {},
    )
}

/// Account governance over `governed`, created by the realm authority.
fn governance_over(d: &mut Dao, governed: Pubkey, m: &Member) -> Pubkey {
    let admin = d.s.admin.insecure_clone();
    let config = GovernanceConfig {
        community_vote_threshold: VoteThreshold::YesVotePercentage(60),
        min_community_weight_to_create_proposal: VCOIN,
        min_transaction_hold_up_time: HOLD_UP_TIME,
        voting_base_time: VOTING_BASE_TIME,
        community_vote_tipping: VoteTipping::Disabled,
        council_vote_threshold: VoteThreshold::Disabled,
        council_veto_vote_threshold: VoteThreshold::Disabled,
        min_council_weight_to_create_proposal: u64::MAX,
        council_vote_tipping: VoteTipping::Disabled,
        community_veto_vote_threshold: VoteThreshold::Disabled,
        voting_cool_off_time: 0,
        deposit_exempt_proposal_count: 10,
    };
    let create = gov_ix::create_governance(
        &GOV_ID,
        &d.realm,
        Some(&governed),
        &m.tor,
        &admin.pubkey(),
        &admin.pubkey(),
        None,
        config,
    );
    send(&mut d.s.svm, &[create], &admin, &[]).expect("create governance");
    get_governance_address(&GOV_ID, &d.realm, &governed)
}

/// Creates a proposal through the plugin, attaches `ixs` as one transaction (if any) and
/// signs it off into Voting.
fn open_proposal(d: &mut Dao, m: &Member, governance: Pubkey, ixs: Vec<Instruction>) -> Pubkey {
    let owner = m.st.kp.pubkey();
    let seed = Pubkey::new_unique();
    let proposal = get_proposal_address(&GOV_ID, &governance, &d.s.vcoin_mint, &seed);
    let create = gov_ix::create_proposal(
        &GOV_ID,
        &governance,
        &m.tor,
        &owner,
        &owner,
        Some(m.vwr),
        &d.realm,
        "Proposal".into(),
        String::new(),
        &d.s.vcoin_mint,
        VoteType::SingleChoice,
        vec!["Approve".into()],
        true,
        &seed,
    );
    let tx = [
        update_vwr_ix(d, m, VoterWeightAction::CreateProposal, Some(governance)),
        create,
    ];
    send(&mut d.s.svm, &tx, &m.st.kp, &[]).expect("create proposal through the plugin");
    let mut tx = Vec::new();
    if !ixs.is_empty() {
        let data = ixs.into_iter().map(InstructionData::from).collect();
        tx.push(gov_ix::insert_transaction(
            &GOV_ID,
            &governance,
            &proposal,
            &m.tor,
            &owner,
            &owner,
            0,
            0,
            HOLD_UP_TIME,
            data,
        ));
    }
    tx.push(gov_ix::sign_off_proposal(
        &GOV_ID,
        &d.realm,
        &governance,
        &proposal,
        &owner,
        Some(&m.tor),
    ));
    send(&mut d.s.svm, &tx, &m.st.kp, &[]).expect("insert + sign off");
    proposal
}

fn cast_vote_ix(
    d: &Dao,
    m: &Member,
    governance: Pubkey,
    proposal: Pubkey,
    authority: &Pubkey,
) -> Instruction {
    gov_ix::cast_vote(
        &GOV_ID,
        &d.realm,
        &governance,
        &proposal,
        &m.tor,
        &m.tor,
        authority,
        &d.s.vcoin_mint,
        authority,
        Some(m.vwr),
        Some(d.max_vwr),
        Vote::Approve(vec![VoteChoice {
            rank: 0,
            weight_percentage: 100,
        }]),
    )
}

/// update VWR + update MaxVWR + CastVote, signed by `authority` (owner or delegate).
fn vote(d: &mut Dao, m: &Member, governance: Pubkey, proposal: Pubkey, authority: &Keypair) {
    let tx = [
        update_vwr_ix(d, m, VoterWeightAction::CastVote, Some(proposal)),
        update_max_ix(d),
        cast_vote_ix(d, m, governance, proposal, &authority.pubkey()),
    ];
    send(&mut d.s.svm, &tx, authority, &[]).expect("cast vote through the plugin");
}

/// Ends voting, finalizes with a fresh MaxVWR and waits out the hold-up.
fn finalize(d: &mut Dao, m: &Member, governance: Pubkey, proposal: Pubkey) {
    warp_by(&mut d.s.svm, VOTING_BASE_TIME as i64 + 1);
    let fin = gov_ix::finalize_vote(
        &GOV_ID,
        &d.realm,
        &governance,
        &proposal,
        &m.tor,
        &d.s.vcoin_mint,
        Some(d.max_vwr),
    );
    let tx = [update_max_ix(d), fin];
    let admin = d.s.admin.insecure_clone();
    send(&mut d.s.svm, &tx, &admin, &[]).expect("finalize");
    assert_eq!(proposal_state(d, &proposal).state, ProposalState::Succeeded);
    warp_by(&mut d.s.svm, HOLD_UP_TIME as i64 + 1);
}

fn execute_ix(
    governance: Pubkey,
    proposal: Pubkey,
    program: Pubkey,
    accounts: Vec<AccountMeta>,
) -> Instruction {
    let ptx = get_proposal_transaction_address(
        &GOV_ID,
        &proposal,
        &0u8.to_le_bytes(),
        &0u16.to_le_bytes(),
    );
    gov_ix::execute_transaction(&GOV_ID, &governance, &proposal, &ptx, &program, &accounts)
}

fn proposal_state(d: &Dao, p: &Pubkey) -> ProposalV2 {
    let acc = d.s.svm.get_account(p).unwrap();
    borsh::BorshDeserialize::deserialize(&mut &acc.data[..]).unwrap()
}

fn set_score(d: &mut Dao, owner: &Pubkey, composite_score: u16) {
    let address = pda(&[USER_SCORE_SEED, owner.as_ref()], &five_a_protocol::ID);
    let mut data = Vec::new();
    UserScore {
        user: *owner,
        composite_score,
        ..Default::default()
    }
    .try_serialize(&mut data)
    .unwrap();
    let lamports = d.s.svm.minimum_balance_for_rent_exemption(data.len());
    d.s.svm
        .set_account(
            address,
            Account {
                lamports,
                data,
                owner: five_a_protocol::ID,
                executable: false,
                rent_epoch: 0,
            },
        )
        .unwrap();
}

#[test]
fn full_dao_lifecycle_with_the_staking_plugin() {
    let mut d = dao(0);
    let admin = d.s.admin.insecure_clone();
    // A 4-year lock gives the maximum weight (2.0x), so this voter holds 100% of max weight.
    let m = join(&mut d, 1_000 * VCOIN, MAX_LOCK_SECS);

    let governed = Pubkey::new_unique();
    let governance = governance_over(&mut d, governed, &m);
    let treasury = get_native_treasury_address(&GOV_ID, &governance);
    let create_treasury = gov_ix::create_native_treasury(&GOV_ID, &governance, &admin.pubkey());
    let fund = solana_system_interface::instruction::transfer(
        &admin.pubkey(),
        &treasury,
        2 * LAMPORTS_PER_SOL,
    );
    send(&mut d.s.svm, &[create_treasury, fund], &admin, &[]).expect("treasury");

    let recipient = Pubkey::new_unique();
    let payment =
        solana_system_interface::instruction::transfer(&treasury, &recipient, LAMPORTS_PER_SOL);
    let proposal = open_proposal(&mut d, &m, governance, vec![payment]);
    let kp = m.st.kp.insecure_clone();
    vote(&mut d, &m, governance, proposal, &kp);

    let vwr: VoterWeightRecord = read_account(&d.s.svm, &m.vwr);
    assert_eq!(
        vwr.voter_weight,
        2_000 * VCOIN,
        "4-year lock = 2.0x, linear in amount"
    );
    assert_eq!(vwr.weight_action, Some(VoterWeightAction::CastVote));
    assert_eq!(vwr.weight_action_target, Some(proposal));
    let max: MaxVoterWeightRecord = read_account(&d.s.svm, &d.max_vwr);
    assert_eq!(
        max.max_voter_weight,
        2_000 * VCOIN,
        "max = total locked at the full lock bonus"
    );
    assert_eq!(
        proposal_state(&d, &proposal).options[0].vote_weight,
        2_000 * VCOIN
    );

    // GOV-03: after the lock ends, the tokens still cannot leave while the vote is unrelinquished.
    warp_by(&mut d.s.svm, MAX_LOCK_SECS);
    let wd = withdraw_ix_with_record(&d.s, &m.st, 0, 1_000 * VCOIN, m.tor);
    assert_anchor_error(
        send(&mut d.s.svm, std::slice::from_ref(&wd), &m.st.kp, &[]),
        serr(StakingError::VotesNotRelinquished),
    );

    finalize(&mut d, &m, governance, proposal);
    let exec = execute_ix(
        governance,
        proposal,
        solana_system_interface::program::ID,
        vec![
            AccountMeta::new(treasury, false),
            AccountMeta::new(recipient, false),
        ],
    );
    send(&mut d.s.svm, &[exec], &admin, &[]).expect("execute: governance PDA signs the payment");
    assert_eq!(d.s.svm.get_balance(&recipient).unwrap(), LAMPORTS_PER_SOL);
    assert_eq!(
        proposal_state(&d, &proposal).state,
        ProposalState::Completed
    );

    // Relinquish (permissionless after finalize) → withdrawal unblocked.
    let relinquish = gov_ix::relinquish_vote(
        &GOV_ID,
        &d.realm,
        &governance,
        &proposal,
        &m.tor,
        &d.s.vcoin_mint,
        None,
        None,
    );
    send(&mut d.s.svm, &[relinquish], &admin, &[]).expect("relinquish");
    send(&mut d.s.svm, &[wd], &m.st.kp, &[]).expect("withdraw after relinquish");
    assert_eq!(token::balance(&d.s.svm, &m.st.vcoin), 1_000 * VCOIN);
}

#[test]
fn dao_takes_over_the_registrar_and_changes_parameters() {
    let mut d = dao(0);
    let admin = d.s.admin.insecure_clone();
    let m = join(&mut d, 1_000 * VCOIN, MAX_LOCK_SECS);
    let registrar = d.s.registrar;
    let governance = governance_over(&mut d, registrar, &m);

    let propose = ix(
        staking_protocol::ID,
        staking_protocol::accounts::UpdateRegistrar {
            registrar,
            authority: admin.pubkey(),
        },
        staking_protocol::instruction::ProposeAuthority {
            new_authority: governance,
        },
    );
    send(&mut d.s.svm, &[propose], &admin, &[]).expect("hand authority to the governance PDA");

    let accept = ix(
        staking_protocol::ID,
        staking_protocol::accounts::AcceptAuthority {
            registrar,
            new_authority: governance,
        },
        staking_protocol::instruction::AcceptAuthority {},
    );
    let set_gates = ix(
        staking_protocol::ID,
        staking_protocol::accounts::UpdateRegistrar {
            registrar,
            authority: governance,
        },
        staking_protocol::instruction::SetGateParams {
            min_vote_score: 1_234,
            min_propose_score: 2_000,
        },
    );
    let proposal = open_proposal(&mut d, &m, governance, vec![accept, set_gates]);
    let kp = m.st.kp.insecure_clone();
    vote(&mut d, &m, governance, proposal, &kp);
    finalize(&mut d, &m, governance, proposal);
    warp_by(&mut d.s.svm, AUTHORITY_TRANSFER_DELAY);

    let exec = execute_ix(
        governance,
        proposal,
        staking_protocol::ID,
        vec![
            AccountMeta::new(registrar, false),
            AccountMeta::new_readonly(governance, false),
        ],
    );
    send(&mut d.s.svm, &[exec], &admin, &[])
        .expect("governance PDA accepts authority and sets gates");
    let reg: Registrar = read_account(&d.s.svm, &registrar);
    assert_eq!(reg.admin.authority, governance);
    assert_eq!((reg.min_vote_score, reg.min_propose_score), (1_234, 2_000));

    // The bootstrap key is powerless now.
    let rogue = ix(
        staking_protocol::ID,
        staking_protocol::accounts::UpdateRegistrar {
            registrar,
            authority: admin.pubkey(),
        },
        staking_protocol::instruction::SetGateParams {
            min_vote_score: 0,
            min_propose_score: 0,
        },
    );
    assert_anchor_error(
        send(&mut d.s.svm, &[rogue], &admin, &[]),
        u32::from(CommonError::NotAuthority),
    );
}

#[test]
fn realms_rejects_stale_or_mistargeted_weight_records() {
    let mut d = dao(0);
    let m = join(&mut d, 100 * VCOIN, MAX_LOCK_SECS);
    let governance = governance_over(&mut d, Pubkey::new_unique(), &m);
    let proposal = open_proposal(&mut d, &m, governance, vec![]);
    let owner = m.st.kp.pubkey();

    // Record written in an earlier slot.
    let update = update_vwr_ix(&d, &m, VoterWeightAction::CastVote, Some(proposal));
    send(&mut d.s.svm, &[update], &m.st.kp, &[]).expect("update");
    warp_by(&mut d.s.svm, 1);
    let tx = [
        update_max_ix(&d),
        cast_vote_ix(&d, &m, governance, proposal, &owner),
    ];
    assert_anchor_error(send(&mut d.s.svm, &tx, &m.st.kp, &[]), ERR_VWR_EXPIRED);

    // Record written for another action.
    let tx = [
        update_vwr_ix(&d, &m, VoterWeightAction::CreateProposal, Some(proposal)),
        update_max_ix(&d),
        cast_vote_ix(&d, &m, governance, proposal, &owner),
    ];
    assert_anchor_error(
        send(&mut d.s.svm, &tx, &m.st.kp, &[]),
        ERR_VWR_INVALID_ACTION,
    );

    // Record written for another proposal.
    let tx = [
        update_vwr_ix(
            &d,
            &m,
            VoterWeightAction::CastVote,
            Some(Pubkey::new_unique()),
        ),
        update_max_ix(&d),
        cast_vote_ix(&d, &m, governance, proposal, &owner),
    ];
    assert_anchor_error(
        send(&mut d.s.svm, &tx, &m.st.kp, &[]),
        ERR_VWR_INVALID_TARGET,
    );

    let kp = m.st.kp.insecure_clone();
    vote(&mut d, &m, governance, proposal, &kp);
}

#[test]
fn a_governance_delegate_votes_with_the_owners_weight_and_locks_the_owners_stake() {
    let mut d = dao(0);
    let m = join(&mut d, 500 * VCOIN, MAX_LOCK_SECS);
    let governance = governance_over(&mut d, Pubkey::new_unique(), &m);
    let proposal = open_proposal(&mut d, &m, governance, vec![]);

    let delegate = funded_keypair(&mut d.s.svm, 2);
    let owner = m.st.kp.pubkey();
    let set_delegate = gov_ix::set_governance_delegate(
        &GOV_ID,
        &owner,
        &d.realm,
        &d.s.vcoin_mint,
        &owner,
        &Some(delegate.pubkey()),
    );
    send(&mut d.s.svm, &[set_delegate], &m.st.kp, &[]).expect("set delegate");

    vote(&mut d, &m, governance, proposal, &delegate);
    assert_eq!(
        proposal_state(&d, &proposal).options[0].vote_weight,
        1_000 * VCOIN
    );

    warp_by(&mut d.s.svm, MAX_LOCK_SECS);
    let wd = withdraw_ix_with_record(&d.s, &m.st, 0, 500 * VCOIN, m.tor);
    assert_anchor_error(
        send(&mut d.s.svm, &[wd], &m.st.kp, &[]),
        serr(StakingError::VotesNotRelinquished),
    );
}

#[test]
fn reputation_gate_blocks_voting_below_the_minimum_score() {
    let mut d = dao(6_000);
    let m = join(&mut d, 10 * VCOIN, MAX_LOCK_SECS);
    let owner = m.st.kp.pubkey();
    let target = Some(Pubkey::new_unique());

    // No 5A score account → score 0.
    let update = update_vwr_ix(&d, &m, VoterWeightAction::CastVote, target);
    assert_anchor_error(
        send(&mut d.s.svm, &[update], &m.st.kp, &[]),
        serr(StakingError::ScoreBelowGate),
    );
    // Proposal creation uses the separate (zero) propose gate.
    let update = update_vwr_ix(&d, &m, VoterWeightAction::CreateProposal, target);
    send(&mut d.s.svm, &[update], &m.st.kp, &[]).expect("propose gate is 0");

    set_score(&mut d, &owner, 5_999);
    warp_by(&mut d.s.svm, 1);
    let update = update_vwr_ix(&d, &m, VoterWeightAction::CastVote, target);
    assert_anchor_error(
        send(&mut d.s.svm, std::slice::from_ref(&update), &m.st.kp, &[]),
        serr(StakingError::ScoreBelowGate),
    );

    set_score(&mut d, &owner, 6_000);
    send(&mut d.s.svm, &[update], &m.st.kp, &[]).expect("score at the gate may vote");
    let vwr: VoterWeightRecord = read_account(&d.s.svm, &m.vwr);
    // Same as without a gate: 1x baseline + lock bonus decayed by the one second warped above.
    let stake = 10 * VCOIN as u128;
    let expected = stake + stake * (MAX_LOCK_SECS as u128 - 1) / MAX_LOCK_SECS as u128;
    assert_eq!(
        vwr.voter_weight as u128, expected,
        "reputation gates, never multiplies"
    );

    // A score account at the wrong address is rejected.
    let mut bad = update_vwr_ix(&d, &m, VoterWeightAction::CastVote, target);
    bad.accounts[3].pubkey = pda(
        &[USER_SCORE_SEED, Pubkey::new_unique().as_ref()],
        &five_a_protocol::ID,
    );
    assert_anchor_error(
        send(&mut d.s.svm, &[bad], &m.st.kp, &[]),
        serr(StakingError::InvalidScoreAccount),
    );
}

#[test]
fn configure_governance_validates_the_realm_and_runs_once() {
    let mut s = setup();
    load_governance(&mut s);
    let admin = s.admin.insecure_clone();
    let vcoin_mint = s.vcoin_mint;
    let slash_destination =
        token::create_token_account(&mut s.svm, &admin, &vcoin_mint, &admin.pubkey());
    let realm = create_realm(&mut s, "VCoin DAO", &vcoin_mint, true);

    // Not the registrar authority.
    let stranger = funded_keypair(&mut s.svm, 2);
    let r = send(
        &mut s.svm,
        &[configure_ix(
            (s.registrar, s.vcoin_mint),
            stranger.pubkey(),
            realm,
            slash_destination,
            0,
        )],
        &stranger,
        &[],
    );
    assert_anchor_error(r, u32::from(CommonError::NotAuthority));

    // Gate above the 5A maximum.
    let r = send(
        &mut s.svm,
        &[configure_ix(
            (s.registrar, s.vcoin_mint),
            admin.pubkey(),
            realm,
            slash_destination,
            10_001,
        )],
        &admin,
        &[],
    );
    assert_anchor_error(r, serr(StakingError::InvalidGateParams));

    // Realm whose community mint is not VCoin.
    let other = Keypair::new();
    let mint_ixs = [
        solana_system_interface::instruction::create_account(
            &admin.pubkey(),
            &other.pubkey(),
            s.svm.minimum_balance_for_rent_exemption(82),
            82,
            &spl_token::ID,
        ),
        spl_token::instruction::initialize_mint2(
            &spl_token::ID,
            &other.pubkey(),
            &admin.pubkey(),
            None,
            9,
        )
        .unwrap(),
    ];
    send(&mut s.svm, &mint_ixs, &admin, &[&other]).expect("other mint");
    let other_realm = create_realm(&mut s, "Other DAO", &other.pubkey(), false);
    let r = send(
        &mut s.svm,
        &[configure_ix(
            (s.registrar, s.vcoin_mint),
            admin.pubkey(),
            other_realm,
            slash_destination,
            0,
        )],
        &admin,
        &[],
    );
    assert_anchor_error(r, serr(StakingError::InvalidRealm));

    // Look-alike realm bytes owned by another program, and a governance account of the wrong type.
    let mut data = vec![REALM_V2_ACCOUNT_TYPE];
    data.extend_from_slice(vcoin_mint.as_ref());
    for (owner, tag) in [
        (Pubkey::new_unique(), REALM_V2_ACCOUNT_TYPE),
        (GOV_ID, TOKEN_OWNER_RECORD_V2_ACCOUNT_TYPE),
    ] {
        let fake = Pubkey::new_unique();
        let mut bytes = data.clone();
        bytes[0] = tag;
        s.svm
            .set_account(
                fake,
                Account {
                    lamports: LAMPORTS_PER_SOL,
                    data: bytes,
                    owner,
                    executable: false,
                    rent_epoch: 0,
                },
            )
            .unwrap();
        let r = send(
            &mut s.svm,
            &[configure_ix(
                (s.registrar, s.vcoin_mint),
                admin.pubkey(),
                fake,
                slash_destination,
                0,
            )],
            &admin,
            &[],
        );
        assert_anchor_error(r, serr(StakingError::InvalidRealm));
    }

    send(
        &mut s.svm,
        &[configure_ix(
            (s.registrar, s.vcoin_mint),
            admin.pubkey(),
            realm,
            slash_destination,
            0,
        )],
        &admin,
        &[],
    )
    .expect("configure");
    let reg: Registrar = read_account(&s.svm, &s.registrar);
    assert_eq!(
        (reg.realm, reg.governance_program_id, reg.slash_destination),
        (realm, GOV_ID, slash_destination)
    );
    warp_by(&mut s.svm, 1);
    let r = send(
        &mut s.svm,
        &[configure_ix(
            (s.registrar, s.vcoin_mint),
            admin.pubkey(),
            realm,
            slash_destination,
            0,
        )],
        &admin,
        &[],
    );
    assert!(r.is_err(), "one-shot");
}

#[test]
fn withdraw_requires_the_owners_real_token_owner_record() {
    let mut d = dao(0);
    let m = join(&mut d, 10 * VCOIN, MIN_LOCK_SECS);
    warp_by(&mut d.s.svm, MIN_LOCK_SECS);

    // Any other account in place of the record (e.g. an empty one) is refused.
    let forged = withdraw_ix_with_record(&d.s, &m.st, 0, 10 * VCOIN, Pubkey::new_unique());
    assert_anchor_error(
        send(&mut d.s.svm, &[forged], &m.st.kp, &[]),
        serr(StakingError::InvalidTokenOwnerRecord),
    );

    let wd = withdraw_ix_with_record(&d.s, &m.st, 0, 10 * VCOIN, m.tor);
    send(&mut d.s.svm, &[wd], &m.st.kp, &[]).expect("no open votes");
}
