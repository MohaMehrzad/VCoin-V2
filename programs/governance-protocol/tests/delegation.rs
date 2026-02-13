//! Integration tests for governance-protocol delegation instructions
//! H-NEW-03: Updated to include balance validation accounts

mod common;

use common::*;
use solana_sdk::pubkey::Pubkey;
use solana_sdk::signature::{Keypair, Signer};

/// Helper to derive test PDAs for delegation
fn derive_delegation_pdas(
    program_id: &Pubkey,
    delegator: &Pubkey,
    delegate: &Pubkey,
    staking_program: &Pubkey,
) -> (Pubkey, Pubkey, Pubkey) {
    let (delegation_pda, _) = Pubkey::find_program_address(
        &[DELEGATION_SEED, delegator.as_ref()],
        program_id,
    );
    let (delegate_stats_pda, _) = Pubkey::find_program_address(
        &[DELEGATE_STATS_SEED, delegate.as_ref()],
        program_id,
    );
    let (user_stake_pda, _) = Pubkey::find_program_address(
        &[USER_STAKE_SEED, delegator.as_ref()],
        staking_program,
    );
    (delegation_pda, delegate_stats_pda, user_stake_pda)
}

#[tokio::test]
async fn test_delegate_votes_instruction_format() {
    let ctx = TestContext::new().await;

    let (config_pda, _bump) = ctx.get_config_pda();
    let delegator = Keypair::new();
    let delegate = Keypair::new();
    let staking_program = Keypair::new();

    let (delegation_pda, delegate_stats_pda, user_stake_pda) = derive_delegation_pdas(
        &ctx.program_id,
        &delegator.pubkey(),
        &delegate.pubkey(),
        &staking_program.pubkey(),
    );

    let ix = create_delegate_votes_ix(
        &ctx.program_id,
        &delegator.pubkey(),
        &delegate.pubkey(),
        &delegation_pda,
        &delegate_stats_pda,
        &config_pda,
        &user_stake_pda,
        0, // full delegation
        0xFF, // all categories
        1000,
        0, // no expiry
        true,
    );

    assert_eq!(ix.program_id, ctx.program_id);
    // H-NEW-03: 7 accounts = delegation, delegate_stats, delegator, delegate, config, user_stake, system_program
    assert_eq!(ix.accounts.len(), 7);
    // delegator is at index 2 (after delegation and delegate_stats) and is signer
    assert!(ix.accounts[2].is_signer);
}

#[tokio::test]
async fn test_delegate_votes_different_types() {
    let ctx = TestContext::new().await;

    let (config_pda, _bump) = ctx.get_config_pda();
    let delegator = Keypair::new();
    let delegate = Keypair::new();
    let staking_program = Keypair::new();

    for delegation_type in 0..=2 {
        let (delegation_pda, delegate_stats_pda, user_stake_pda) = derive_delegation_pdas(
            &ctx.program_id,
            &delegator.pubkey(),
            &delegate.pubkey(),
            &staking_program.pubkey(),
        );

        let ix = create_delegate_votes_ix(
            &ctx.program_id,
            &delegator.pubkey(),
            &delegate.pubkey(),
            &delegation_pda,
            &delegate_stats_pda,
            &config_pda,
            &user_stake_pda,
            delegation_type,
            0xFF,
            1000,
            0,
            true,
        );

        assert_eq!(ix.data[8], delegation_type);
    }
}

#[tokio::test]
async fn test_delegate_votes_different_categories() {
    let ctx = TestContext::new().await;

    let (config_pda, _bump) = ctx.get_config_pda();
    let delegator = Keypair::new();
    let delegate = Keypair::new();
    let staking_program = Keypair::new();

    for categories in [0x01u8, 0x02, 0x04, 0x08, 0xFF] {
        let (delegation_pda, delegate_stats_pda, user_stake_pda) = derive_delegation_pdas(
            &ctx.program_id,
            &delegator.pubkey(),
            &delegate.pubkey(),
            &staking_program.pubkey(),
        );

        let ix = create_delegate_votes_ix(
            &ctx.program_id,
            &delegator.pubkey(),
            &delegate.pubkey(),
            &delegation_pda,
            &delegate_stats_pda,
            &config_pda,
            &user_stake_pda,
            0,
            categories,
            1000,
            0,
            true,
        );

        assert_eq!(ix.data[9], categories);
    }
}

#[tokio::test]
async fn test_delegate_votes_with_expiry() {
    let ctx = TestContext::new().await;

    let (config_pda, _bump) = ctx.get_config_pda();
    let delegator = Keypair::new();
    let delegate = Keypair::new();
    let staking_program = Keypair::new();
    let expires_at = 1735689600i64; // Some future timestamp

    let (delegation_pda, delegate_stats_pda, user_stake_pda) = derive_delegation_pdas(
        &ctx.program_id,
        &delegator.pubkey(),
        &delegate.pubkey(),
        &staking_program.pubkey(),
    );

    let ix = create_delegate_votes_ix(
        &ctx.program_id,
        &delegator.pubkey(),
        &delegate.pubkey(),
        &delegation_pda,
        &delegate_stats_pda,
        &config_pda,
        &user_stake_pda,
        0,
        0xFF,
        1000,
        expires_at,
        true,
    );

    assert_eq!(&ix.data[18..26], &expires_at.to_le_bytes());
}

#[tokio::test]
async fn test_delegate_votes_non_revocable() {
    let ctx = TestContext::new().await;

    let (config_pda, _bump) = ctx.get_config_pda();
    let delegator = Keypair::new();
    let delegate = Keypair::new();
    let staking_program = Keypair::new();

    let (delegation_pda, delegate_stats_pda, user_stake_pda) = derive_delegation_pdas(
        &ctx.program_id,
        &delegator.pubkey(),
        &delegate.pubkey(),
        &staking_program.pubkey(),
    );

    let ix = create_delegate_votes_ix(
        &ctx.program_id,
        &delegator.pubkey(),
        &delegate.pubkey(),
        &delegation_pda,
        &delegate_stats_pda,
        &config_pda,
        &user_stake_pda,
        0,
        0xFF,
        1000,
        0,
        false,
    );

    assert_eq!(ix.data[26], 0);
}
