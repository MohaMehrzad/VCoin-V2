//! vcoin-token v3 transaction tests (LiteSVM, real Token-2022 program).
//!
//! Covers audit TOK-04 / X-1 (upgrade-authority gated init), TOK-01 / TH-* (metadata-only
//! mint: no permanent delegate, hook or freeze authority), MON-8 (cap vs real supply),
//! X-2 (24h authority timelock) and pause coverage.

use anchor_lang::{prelude::Pubkey, system_program, AccountDeserialize};
use anchor_spl::token_2022::spl_token_2022::{
    self,
    extension::{BaseStateWithExtensions, ExtensionType, StateWithExtensions},
    state::Mint as MintState,
};
use anchor_spl::token_interface::spl_token_metadata_interface::state::TokenMetadata;
use integration_tests::litesvm::LiteSVM;
use integration_tests::solana_sdk::{instruction::Instruction, signature::Keypair, signer::Signer};
use integration_tests::*;
use vcoin_common::{CommonError, AUTHORITY_TRANSFER_DELAY};
use vcoin_token::{
    constants::*,
    errors::VCoinError,
    state::{MetadataField, VCoinConfig},
};

fn code(e: VCoinError) -> u32 {
    6000 + e as u32
}
fn common(e: CommonError) -> u32 {
    u32::from(e)
}

struct Env {
    svm: LiteSVM,
    upgrade_auth: Keypair,
    config: Pubkey,
    mint: Pubkey,
    program_data: Pubkey,
}

fn setup() -> Env {
    let mut svm = LiteSVM::new();
    let upgrade_auth = funded_keypair(&mut svm, 10);
    let program_data = deploy_upgradeable(
        &mut svm,
        vcoin_token::ID,
        &program_so("vcoin_token"),
        Some(upgrade_auth.pubkey()),
    );
    let config = Pubkey::find_program_address(&[VCOIN_CONFIG_SEED], &vcoin_token::ID).0;
    let mint = Pubkey::find_program_address(&[VCOIN_MINT_SEED], &vcoin_token::ID).0;
    Env {
        svm,
        upgrade_auth,
        config,
        mint,
        program_data,
    }
}

fn initialize_ix(env: &Env, signer: Pubkey) -> Instruction {
    ix(
        vcoin_token::ID,
        vcoin_token::accounts::Initialize {
            authority: signer,
            config: env.config,
            mint: env.mint,
            program: vcoin_token::ID,
            program_data: env.program_data,
            token_program: spl_token_2022::ID,
            system_program: system_program::ID,
        },
        vcoin_token::instruction::Initialize {
            name: "VCoin".into(),
            symbol: "VIWO".into(),
            uri: "https://viwoapp.com/vcoin.json".into(),
        },
    )
}

fn initialized() -> Env {
    let mut env = setup();
    let ixn = initialize_ix(&env, env.upgrade_auth.pubkey());
    let payer = env.upgrade_auth.insecure_clone();
    send(&mut env.svm, &[ixn], &payer, &[]).expect("initialize");
    env
}

fn mint_ix(env: &Env, authority: Pubkey, destination: Pubkey, amount: u64) -> Instruction {
    ix(
        vcoin_token::ID,
        vcoin_token::accounts::MintTokens {
            config: env.config,
            mint: env.mint,
            destination,
            authority,
            token_program: spl_token_2022::ID,
        },
        vcoin_token::instruction::MintTokens { amount },
    )
}

fn update_config_ix(
    env: &Env,
    authority: Pubkey,
    data: impl anchor_lang::InstructionData,
) -> Instruction {
    ix(
        vcoin_token::ID,
        vcoin_token::accounts::UpdateConfig {
            config: env.config,
            authority,
        },
        data,
    )
}

fn config(env: &Env) -> VCoinConfig {
    read_account(&env.svm, &env.config)
}

#[test]
fn initialize_rejects_anyone_but_the_upgrade_authority() {
    let mut env = setup();
    let attacker = funded_keypair(&mut env.svm, 10);
    let ixn = initialize_ix(&env, attacker.pubkey());
    assert_anchor_error(
        send(&mut env.svm, &[ixn], &attacker, &[]),
        common(CommonError::NotUpgradeAuthority),
    );
}

#[test]
fn initialize_creates_a_metadata_only_mint_controlled_by_the_config_pda() {
    let mut env = initialized();
    let cfg = config(&env);
    assert_eq!(cfg.admin.authority, env.upgrade_auth.pubkey());
    assert_eq!(cfg.mint, env.mint);
    assert!(!cfg.paused);

    let mint_acc = env.svm.get_account(&env.mint).unwrap();
    assert_eq!(mint_acc.owner, spl_token_2022::ID);
    let state = StateWithExtensions::<MintState>::unpack(&mint_acc.data).unwrap();
    assert_eq!(state.base.decimals, 9);
    assert_eq!(state.base.mint_authority, Some(env.config).into());
    assert!(state.base.freeze_authority.is_none(), "no freeze authority");
    let mut exts = state.get_extension_types().unwrap();
    exts.sort_by_key(|e| *e as u16);
    assert_eq!(
        exts,
        vec![ExtensionType::MetadataPointer, ExtensionType::TokenMetadata]
    );
    let md = state.get_variable_len_extension::<TokenMetadata>().unwrap();
    assert_eq!((md.name.as_str(), md.symbol.as_str()), ("VCoin", "VIWO"));

    // The shared validator that other programs use accepts this mint.
    let mut data = mint_acc.data.clone();
    let mut lamports = mint_acc.lamports;
    let ai = anchor_lang::prelude::AccountInfo::new(
        &env.mint,
        false,
        false,
        &mut lamports,
        &mut data,
        &mint_acc.owner,
        false,
        0,
    );
    vcoin_common::validate_vcoin_mint(&ai).expect("mint passes validate_vcoin_mint");

    // A second initialize cannot overwrite the config.
    let ixn = initialize_ix(&env, env.upgrade_auth.pubkey());
    let payer = env.upgrade_auth.insecure_clone();
    assert!(send(&mut env.svm, &[ixn], &payer, &[]).is_err());
}

#[test]
fn supply_cap_is_enforced_against_the_real_mint_supply() {
    let mut env = initialized();
    let auth = env.upgrade_auth.insecure_clone();
    let dest = token::create_token_account(&mut env.svm, &auth, &env.mint, &auth.pubkey());

    let ixn = mint_ix(&env, auth.pubkey(), dest, TOTAL_SUPPLY);
    send(&mut env.svm, &[ixn], &auth, &[]).expect("mint the full cap");
    assert_eq!(token::supply(&env.svm, &env.mint), TOTAL_SUPPLY);
    assert_eq!(token::balance(&env.svm, &dest), TOTAL_SUPPLY);

    let ixn = mint_ix(&env, auth.pubkey(), dest, 1);
    assert_anchor_error(
        send(&mut env.svm, &[ixn], &auth, &[]),
        code(VCoinError::ExceedsMaxSupply),
    );
}

#[test]
fn only_the_authority_can_mint_and_zero_is_rejected() {
    let mut env = initialized();
    let auth = env.upgrade_auth.insecure_clone();
    let attacker = funded_keypair(&mut env.svm, 1);
    let dest = token::create_token_account(&mut env.svm, &auth, &env.mint, &attacker.pubkey());

    let ixn = mint_ix(&env, attacker.pubkey(), dest, 1);
    assert_anchor_error(
        send(&mut env.svm, &[ixn], &attacker, &[]),
        common(CommonError::NotAuthority),
    );
    let ixn = mint_ix(&env, auth.pubkey(), dest, 0);
    assert_anchor_error(
        send(&mut env.svm, &[ixn], &auth, &[]),
        code(VCoinError::ZeroAmount),
    );
}

#[test]
fn pause_blocks_minting_and_only_the_authority_can_toggle_it() {
    let mut env = initialized();
    let auth = env.upgrade_auth.insecure_clone();
    let attacker = funded_keypair(&mut env.svm, 1);
    let dest = token::create_token_account(&mut env.svm, &auth, &env.mint, &auth.pubkey());

    let ixn = update_config_ix(
        &env,
        attacker.pubkey(),
        vcoin_token::instruction::SetPaused { paused: true },
    );
    assert_anchor_error(
        send(&mut env.svm, &[ixn], &attacker, &[]),
        common(CommonError::NotAuthorityOrGuardian),
    );

    let ixn = update_config_ix(
        &env,
        auth.pubkey(),
        vcoin_token::instruction::SetPaused { paused: true },
    );
    send(&mut env.svm, &[ixn], &auth, &[]).unwrap();
    let ixn = mint_ix(&env, auth.pubkey(), dest, 5);
    assert_anchor_error(
        send(&mut env.svm, &[ixn], &auth, &[]),
        code(VCoinError::TokenPaused),
    );

    let ixn = update_config_ix(
        &env,
        auth.pubkey(),
        vcoin_token::instruction::SetPaused { paused: false },
    );
    send(&mut env.svm, &[ixn], &auth, &[]).unwrap();
    let ixn = mint_ix(&env, auth.pubkey(), dest, 5);
    send(&mut env.svm, &[ixn], &auth, &[]).unwrap();
    assert_eq!(token::balance(&env.svm, &dest), 5);
}

#[test]
fn authority_transfer_requires_the_24h_delay() {
    let mut env = initialized();
    let old = env.upgrade_auth.insecure_clone();
    let new = funded_keypair(&mut env.svm, 1);
    let dest = token::create_token_account(&mut env.svm, &old, &env.mint, &old.pubkey());

    let ixn = update_config_ix(
        &env,
        old.pubkey(),
        vcoin_token::instruction::ProposeAuthority {
            new_authority: new.pubkey(),
        },
    );
    send(&mut env.svm, &[ixn], &old, &[]).unwrap();

    let accept = ix(
        vcoin_token::ID,
        vcoin_token::accounts::AcceptAuthority {
            config: env.config,
            new_authority: new.pubkey(),
        },
        vcoin_token::instruction::AcceptAuthority {},
    );
    warp_by(&mut env.svm, AUTHORITY_TRANSFER_DELAY - 1);
    assert_anchor_error(
        send(&mut env.svm, std::slice::from_ref(&accept), &new, &[]),
        common(CommonError::AuthorityTimelockActive),
    );
    warp_by(&mut env.svm, 1);
    send(&mut env.svm, &[accept], &new, &[]).expect("accept after 24h");
    assert_eq!(config(&env).admin.authority, new.pubkey());

    let ixn = mint_ix(&env, old.pubkey(), dest, 1);
    assert_anchor_error(
        send(&mut env.svm, &[ixn], &old, &[]),
        common(CommonError::NotAuthority),
    );
    let ixn = mint_ix(&env, new.pubkey(), dest, 1);
    send(&mut env.svm, &[ixn], &new, &[]).expect("new authority mints");
}

#[test]
fn authority_can_update_metadata_and_pays_the_rent() {
    let mut env = initialized();
    let auth = env.upgrade_auth.insecure_clone();
    let long_uri = format!("https://viwoapp.com/{}", "x".repeat(150));
    let ixn = ix(
        vcoin_token::ID,
        vcoin_token::accounts::UpdateMetadata {
            config: env.config,
            mint: env.mint,
            authority: auth.pubkey(),
            token_program: spl_token_2022::ID,
            system_program: system_program::ID,
        },
        vcoin_token::instruction::UpdateMetadata {
            field: MetadataField::Uri,
            value: long_uri.clone(),
        },
    );
    send(&mut env.svm, &[ixn], &auth, &[]).expect("update uri");
    let data = env.svm.get_account(&env.mint).unwrap().data;
    let state = StateWithExtensions::<MintState>::unpack(&data).unwrap();
    assert_eq!(
        state
            .get_variable_len_extension::<TokenMetadata>()
            .unwrap()
            .uri,
        long_uri
    );

    // Sanity: config still deserializes after the metadata realloc of the mint.
    let raw = env.svm.get_account(&env.config).unwrap().data;
    VCoinConfig::try_deserialize(&mut raw.as_slice()).unwrap();
}
