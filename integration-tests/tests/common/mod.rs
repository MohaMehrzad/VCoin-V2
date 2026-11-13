//! Shared Realms (SPL Governance) fixtures for the LiteSVM suites: a VCoin realm wired to
//! the staking plugin, members, proposals and execution.
#![allow(dead_code)]

use anchor_lang::{prelude::Pubkey, system_program, AccountSerialize};
use anchor_spl::token_2022::spl_token_2022;
use five_a_protocol::{constants::USER_SCORE_SEED, state::UserScore};
use governance::{
    instruction as gov_ix,
    state::{
        enums::{MintMaxVoterWeightSource, ProposalState, VoteThreshold, VoteTipping},
        governance::{get_governance_address, GovernanceConfig},
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
use staking_protocol::{constants::*, errors::StakingError, state::VoterWeightAction};

pub const GOV_ID: Pubkey = Pubkey::from_str_const("GovER5Lthms3bLBqWub97yVrMmEogzX7xNjdXpPPCVZw");
pub const VOTING_BASE_TIME: u32 = 3_600;
pub const HOLD_UP_TIME: u32 = 60;
/// spl-governance error codes (GovernanceError).
pub const ERR_VWR_EXPIRED: u32 = 577;
pub const ERR_VWR_INVALID_ACTION: u32 = 587;
pub const ERR_VWR_INVALID_TARGET: u32 = 588;

pub fn serr(e: StakingError) -> u32 {
    6000 + e as u32
}

pub struct Dao {
    pub s: Stack,
    pub realm: Pubkey,
    pub max_vwr: Pubkey,
}

pub struct Member {
    pub st: Staker,
    pub vwr: Pubkey,
    pub tor: Pubkey,
}

pub fn load_governance(s: &mut Stack) {
    let so = std::fs::read(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/fixtures/spl_governance.so"
    ))
    .unwrap();
    s.svm.add_program(GOV_ID, &so).unwrap();
}

/// Realm for `mint` with this staking program as (max) voter-weight addin, Dormant token type.
pub fn create_realm(s: &mut Stack, name: &str, mint: &Pubkey, token_2022: bool) -> Pubkey {
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

pub fn configure_ix(
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
pub fn dao(min_vote_score: u16) -> Dao {
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

pub fn join(d: &mut Dao, stake: u64, lock: i64) -> Member {
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

pub fn update_vwr_ix(
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

pub fn update_max_ix(d: &Dao) -> Instruction {
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
pub fn governance_over(d: &mut Dao, governed: Pubkey, m: &Member) -> Pubkey {
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
pub fn open_proposal(d: &mut Dao, m: &Member, governance: Pubkey, ixs: Vec<Instruction>) -> Pubkey {
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

pub fn cast_vote_ix(
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
pub fn vote(d: &mut Dao, m: &Member, governance: Pubkey, proposal: Pubkey, authority: &Keypair) {
    let tx = [
        update_vwr_ix(d, m, VoterWeightAction::CastVote, Some(proposal)),
        update_max_ix(d),
        cast_vote_ix(d, m, governance, proposal, &authority.pubkey()),
    ];
    send(&mut d.s.svm, &tx, authority, &[]).expect("cast vote through the plugin");
}

/// Ends voting, finalizes with a fresh MaxVWR and waits out the hold-up.
pub fn finalize(d: &mut Dao, m: &Member, governance: Pubkey, proposal: Pubkey) {
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

pub fn execute_ix(
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

pub fn proposal_state(d: &Dao, p: &Pubkey) -> ProposalV2 {
    let acc = d.s.svm.get_account(p).unwrap();
    borsh::BorshDeserialize::deserialize(&mut &acc.data[..]).unwrap()
}

pub fn set_score(d: &mut Dao, owner: &Pubkey, composite_score: u16) {
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
