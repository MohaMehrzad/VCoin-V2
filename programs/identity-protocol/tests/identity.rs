//! Integration tests for identity-protocol identity instructions

mod common;

use common::*;
use solana_sdk::signature::{Keypair, Signer};

#[tokio::test]
async fn test_create_identity_instruction_format() {
    let ctx = TestContext::new().await;
    
    let (config_pda, _bump) = ctx.get_config_pda();
    let user = Keypair::new();
    let (identity_pda, _) = ctx.get_identity_pda(&user.pubkey());
    let did_hash = [1u8; 32];
    
    let ix = create_identity_ix(
        &ctx.program_id,
        &user.pubkey(),
        &identity_pda,
        &config_pda,
        did_hash,
        "testuser",
    );
    
    assert_eq!(ix.program_id, ctx.program_id);
    assert_eq!(ix.accounts.len(), 4);
    assert!(ix.accounts[0].is_signer);
}

#[tokio::test]
async fn test_create_identity_short_username() {
    let ctx = TestContext::new().await;
    
    let (config_pda, _bump) = ctx.get_config_pda();
    let user = Keypair::new();
    let (identity_pda, _) = ctx.get_identity_pda(&user.pubkey());
    let did_hash = [2u8; 32];
    
    let ix = create_identity_ix(
        &ctx.program_id,
        &user.pubkey(),
        &identity_pda,
        &config_pda,
        did_hash,
        "a",
    );
    
    assert!(ix.data.len() > 8 + 32);
}

#[tokio::test]
async fn test_create_identity_long_username() {
    let ctx = TestContext::new().await;
    
    let (config_pda, _bump) = ctx.get_config_pda();
    let user = Keypair::new();
    let (identity_pda, _) = ctx.get_identity_pda(&user.pubkey());
    let did_hash = [3u8; 32];
    let long_username = "verylongusernamethatisquitelong";
    
    let ix = create_identity_ix(
        &ctx.program_id,
        &user.pubkey(),
        &identity_pda,
        &config_pda,
        did_hash,
        long_username,
    );
    
    assert!(ix.data.len() > 8 + 32 + 4);
}

#[tokio::test]
async fn test_create_identity_unique_did_hash() {
    let ctx = TestContext::new().await;
    
    let (config_pda, _bump) = ctx.get_config_pda();
    
    for i in 0..5 {
        let user = Keypair::new();
        let (identity_pda, _) = ctx.get_identity_pda(&user.pubkey());
        let mut did_hash = [0u8; 32];
        did_hash[0] = i;
        
        let ix = create_identity_ix(
            &ctx.program_id,
            &user.pubkey(),
            &identity_pda,
            &config_pda,
            did_hash,
            &format!("user{}", i),
        );
        
        assert_eq!(&ix.data[8..40], &did_hash);
    }
}

#[tokio::test]
async fn test_create_identity_multiple_users() {
    let ctx = TestContext::new().await;
    
    let (config_pda, _bump) = ctx.get_config_pda();
    
    for i in 0..3 {
        let user = Keypair::new();
        let (identity_pda, _) = ctx.get_identity_pda(&user.pubkey());
        let did_hash = [i as u8; 32];
        
        let ix = create_identity_ix(
            &ctx.program_id,
            &user.pubkey(),
            &identity_pda,
            &config_pda,
            did_hash,
            &format!("user{}", i),
        );
        
        assert_eq!(ix.accounts[0].pubkey, user.pubkey());
    }
}

#[tokio::test]
async fn test_subscribe_instruction_format() {
    let ctx = TestContext::new().await;
    
    let (config_pda, _bump) = ctx.get_config_pda();
    let user = Keypair::new();
    let (identity_pda, _) = ctx.get_identity_pda(&user.pubkey());
    
    let ix = create_subscribe_ix(
        &ctx.program_id,
        &user.pubkey(),
        &identity_pda,
        &config_pda,
        1, // tier
    );
    
    assert_eq!(ix.program_id, ctx.program_id);
    assert_eq!(ix.accounts.len(), 3);
    assert_eq!(ix.data[8], 1);
}

#[tokio::test]
async fn test_subscribe_different_tiers() {
    let ctx = TestContext::new().await;
    
    let (config_pda, _bump) = ctx.get_config_pda();
    let user = Keypair::new();
    let (identity_pda, _) = ctx.get_identity_pda(&user.pubkey());
    
    for tier in 0..=3 {
        let ix = create_subscribe_ix(
            &ctx.program_id,
            &user.pubkey(),
            &identity_pda,
            &config_pda,
            tier,
        );
        
        assert_eq!(ix.data[8], tier);
    }
}

