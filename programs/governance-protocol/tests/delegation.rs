//! Integration tests for governance-protocol delegation instructions

mod common;

use common::*;
use solana_sdk::signature::{Keypair, Signer};

#[tokio::test]
async fn test_delegate_votes_instruction_format() {
    let ctx = TestContext::new().await;
    
    let (config_pda, _bump) = ctx.get_config_pda();
    let delegator = Keypair::new();
    let delegate = Keypair::new();
    let delegation = Keypair::new();
    
    let ix = create_delegate_votes_ix(
        &ctx.program_id,
        &delegator.pubkey(),
        &delegate.pubkey(),
        &delegation.pubkey(),
        &config_pda,
        0, // full delegation
        0xFF, // all categories
        1000,
        0, // no expiry
        true,
    );
    
    assert_eq!(ix.program_id, ctx.program_id);
    assert_eq!(ix.accounts.len(), 5);
    assert!(ix.accounts[0].is_signer);
}

#[tokio::test]
async fn test_delegate_votes_different_types() {
    let ctx = TestContext::new().await;
    
    let (config_pda, _bump) = ctx.get_config_pda();
    let delegator = Keypair::new();
    let delegate = Keypair::new();
    
    for delegation_type in 0..=2 {
        let delegation = Keypair::new();
        
        let ix = create_delegate_votes_ix(
            &ctx.program_id,
            &delegator.pubkey(),
            &delegate.pubkey(),
            &delegation.pubkey(),
            &config_pda,
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
    
    for categories in [0x01u8, 0x02, 0x04, 0x08, 0xFF] {
        let delegation = Keypair::new();
        
        let ix = create_delegate_votes_ix(
            &ctx.program_id,
            &delegator.pubkey(),
            &delegate.pubkey(),
            &delegation.pubkey(),
            &config_pda,
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
    let delegation = Keypair::new();
    let expires_at = 1735689600i64; // Some future timestamp
    
    let ix = create_delegate_votes_ix(
        &ctx.program_id,
        &delegator.pubkey(),
        &delegate.pubkey(),
        &delegation.pubkey(),
        &config_pda,
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
    let delegation = Keypair::new();
    
    let ix = create_delegate_votes_ix(
        &ctx.program_id,
        &delegator.pubkey(),
        &delegate.pubkey(),
        &delegation.pubkey(),
        &config_pda,
        0,
        0xFF,
        1000,
        0,
        false,
    );
    
    assert_eq!(ix.data[26], 0);
}

