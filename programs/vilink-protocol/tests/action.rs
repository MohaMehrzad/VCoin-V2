//! Integration tests for vilink-protocol action instructions

mod common;

use common::*;
use solana_sdk::signature::{Keypair, Signer};

#[tokio::test]
async fn test_create_action_instruction_format() {
    let ctx = TestContext::new().await;
    
    let (config_pda, _bump) = ctx.get_config_pda();
    let creator = Keypair::new();
    let action = Keypair::new();
    let target = Keypair::new();
    
    let ix = create_action_ix(
        &ctx.program_id,
        &creator.pubkey(),
        &config_pda,
        &action.pubkey(),
        0, // tip
        1_000_000_000,
        &target.pubkey(),
    );
    
    assert_eq!(ix.program_id, ctx.program_id);
    assert_eq!(ix.accounts.len(), 4);
}

#[tokio::test]
async fn test_create_action_tip() {
    let ctx = TestContext::new().await;
    
    let (config_pda, _bump) = ctx.get_config_pda();
    let creator = Keypair::new();
    let action = Keypair::new();
    let target = Keypair::new();
    
    let ix = create_action_ix(
        &ctx.program_id,
        &creator.pubkey(),
        &config_pda,
        &action.pubkey(),
        0, // ACTION_TIP
        1_000_000_000,
        &target.pubkey(),
    );
    
    assert_eq!(ix.data[8], 0);
}

#[tokio::test]
async fn test_create_action_vouch() {
    let ctx = TestContext::new().await;
    
    let (config_pda, _bump) = ctx.get_config_pda();
    let creator = Keypair::new();
    let action = Keypair::new();
    let target = Keypair::new();
    
    let ix = create_action_ix(
        &ctx.program_id,
        &creator.pubkey(),
        &config_pda,
        &action.pubkey(),
        1, // ACTION_VOUCH
        0,
        &target.pubkey(),
    );
    
    assert_eq!(ix.data[8], 1);
}

#[tokio::test]
async fn test_create_action_follow() {
    let ctx = TestContext::new().await;
    
    let (config_pda, _bump) = ctx.get_config_pda();
    let creator = Keypair::new();
    let action = Keypair::new();
    let target = Keypair::new();
    
    let ix = create_action_ix(
        &ctx.program_id,
        &creator.pubkey(),
        &config_pda,
        &action.pubkey(),
        2, // ACTION_FOLLOW
        0,
        &target.pubkey(),
    );
    
    assert_eq!(ix.data[8], 2);
}

#[tokio::test]
async fn test_create_action_different_amounts() {
    let ctx = TestContext::new().await;
    
    let (config_pda, _bump) = ctx.get_config_pda();
    let creator = Keypair::new();
    let target = Keypair::new();
    
    for amount in [100_000_000u64, 1_000_000_000, 10_000_000_000] {
        let action = Keypair::new();
        
        let ix = create_action_ix(
            &ctx.program_id,
            &creator.pubkey(),
            &config_pda,
            &action.pubkey(),
            0, // tip
            amount,
            &target.pubkey(),
        );
        
        assert_eq!(&ix.data[9..17], &amount.to_le_bytes());
    }
}

#[tokio::test]
async fn test_create_action_all_types() {
    let ctx = TestContext::new().await;
    
    let (config_pda, _bump) = ctx.get_config_pda();
    let creator = Keypair::new();
    let target = Keypair::new();
    
    for action_type in 0..=7 {
        let action = Keypair::new();
        
        let ix = create_action_ix(
            &ctx.program_id,
            &creator.pubkey(),
            &config_pda,
            &action.pubkey(),
            action_type,
            if action_type == 0 { 1_000_000_000 } else { 0 },
            &target.pubkey(),
        );
        
        assert_eq!(ix.data[8], action_type);
    }
}

#[tokio::test]
async fn test_create_action_multiple_creators() {
    let ctx = TestContext::new().await;
    
    let (config_pda, _bump) = ctx.get_config_pda();
    let target = Keypair::new();
    
    for _ in 0..5 {
        let creator = Keypair::new();
        let action = Keypair::new();
        
        let ix = create_action_ix(
            &ctx.program_id,
            &creator.pubkey(),
            &config_pda,
            &action.pubkey(),
            0,
            1_000_000_000,
            &target.pubkey(),
        );
        
        assert_eq!(ix.accounts[2].pubkey, creator.pubkey());
    }
}

