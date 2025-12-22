//! Integration tests for content-registry content instructions

mod common;

use common::*;
use solana_sdk::signature::{Keypair, Signer};

#[tokio::test]
async fn test_create_content_instruction_format() {
    let ctx = TestContext::new().await;
    
    let (config_pda, _bump) = ctx.get_config_pda();
    let creator = Keypair::new();
    let tracking_id = [1u8; 32];
    let (content_pda, _) = ctx.get_content_pda(&tracking_id);
    let content_hash = [2u8; 32];
    
    let ix = create_content_ix(
        &ctx.program_id,
        &creator.pubkey(),
        &content_pda,
        &config_pda,
        tracking_id,
        content_hash,
        "ipfs://Qm...",
        0, // post
    );
    
    assert_eq!(ix.program_id, ctx.program_id);
    assert_eq!(ix.accounts.len(), 4);
    assert!(ix.accounts[0].is_signer);
}

#[tokio::test]
async fn test_create_content_different_types() {
    let ctx = TestContext::new().await;
    
    let (config_pda, _bump) = ctx.get_config_pda();
    let creator = Keypair::new();
    
    for content_type in 0..=4 {
        let mut tracking_id = [0u8; 32];
        tracking_id[0] = content_type;
        let (content_pda, _) = ctx.get_content_pda(&tracking_id);
        let content_hash = [content_type; 32];
        
        let ix = create_content_ix(
            &ctx.program_id,
            &creator.pubkey(),
            &content_pda,
            &config_pda,
            tracking_id,
            content_hash,
            &format!("ipfs://content{}", content_type),
            content_type,
        );
        
        let last_byte = ix.data.last().unwrap();
        assert_eq!(*last_byte, content_type);
    }
}

#[tokio::test]
async fn test_create_content_short_uri() {
    let ctx = TestContext::new().await;
    
    let (config_pda, _bump) = ctx.get_config_pda();
    let creator = Keypair::new();
    let tracking_id = [3u8; 32];
    let (content_pda, _) = ctx.get_content_pda(&tracking_id);
    let content_hash = [4u8; 32];
    
    let ix = create_content_ix(
        &ctx.program_id,
        &creator.pubkey(),
        &content_pda,
        &config_pda,
        tracking_id,
        content_hash,
        "a",
        0,
    );
    
    assert!(ix.data.len() > 8 + 64);
}

#[tokio::test]
async fn test_create_content_long_uri() {
    let ctx = TestContext::new().await;
    
    let (config_pda, _bump) = ctx.get_config_pda();
    let creator = Keypair::new();
    let tracking_id = [5u8; 32];
    let (content_pda, _) = ctx.get_content_pda(&tracking_id);
    let content_hash = [6u8; 32];
    let long_uri = "ipfs://QmVeryLongContentHashThatIsQuiteVeryLongIndeed";
    
    let ix = create_content_ix(
        &ctx.program_id,
        &creator.pubkey(),
        &content_pda,
        &config_pda,
        tracking_id,
        content_hash,
        long_uri,
        0,
    );
    
    assert!(ix.data.len() > 8 + 64 + 4);
}

#[tokio::test]
async fn test_create_content_multiple_creators() {
    let ctx = TestContext::new().await;
    
    let (config_pda, _bump) = ctx.get_config_pda();
    
    for i in 0..5 {
        let creator = Keypair::new();
        let mut tracking_id = [0u8; 32];
        tracking_id[0] = i;
        let (content_pda, _) = ctx.get_content_pda(&tracking_id);
        let content_hash = [i; 32];
        
        let ix = create_content_ix(
            &ctx.program_id,
            &creator.pubkey(),
            &content_pda,
            &config_pda,
            tracking_id,
            content_hash,
            &format!("ipfs://content{}", i),
            0,
        );
        
        assert_eq!(ix.accounts[0].pubkey, creator.pubkey());
    }
}

#[tokio::test]
async fn test_delete_content_instruction_format() {
    let ctx = TestContext::new().await;
    
    let (config_pda, _bump) = ctx.get_config_pda();
    let creator = Keypair::new();
    let tracking_id = [7u8; 32];
    let (content_pda, _) = ctx.get_content_pda(&tracking_id);
    
    let ix = create_delete_content_ix(
        &ctx.program_id,
        &creator.pubkey(),
        &content_pda,
        &config_pda,
    );
    
    assert_eq!(ix.program_id, ctx.program_id);
    assert_eq!(ix.accounts.len(), 3);
    assert!(ix.accounts[0].is_signer);
    assert_eq!(ix.data.len(), 8);
}

#[tokio::test]
async fn test_delete_content_different_creators() {
    let ctx = TestContext::new().await;
    
    let (config_pda, _bump) = ctx.get_config_pda();
    
    for i in 0..3 {
        let creator = Keypair::new();
        let mut tracking_id = [0u8; 32];
        tracking_id[0] = i;
        let (content_pda, _) = ctx.get_content_pda(&tracking_id);
        
        let ix = create_delete_content_ix(
            &ctx.program_id,
            &creator.pubkey(),
            &content_pda,
            &config_pda,
        );
        
        assert_eq!(ix.accounts[0].pubkey, creator.pubkey());
    }
}

