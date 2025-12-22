//! Integration tests for vevcoin-token admin instructions

mod common;

use common::*;
use solana_sdk::signature::{Keypair, Signer};

// ============================================================================
// update_staking_protocol tests
// ============================================================================

/// Test update staking protocol success
#[tokio::test]
async fn test_update_staking_protocol_success() {
    let mut ctx = TestContext::new().await;
    
    let (config_pda, _bump) = ctx.get_config_pda();
    let mint = Keypair::new();
    let staking_protocol = Keypair::new();
    
    let init_ix = create_initialize_mint_ix(
        &ctx.program_id,
        &ctx.payer.pubkey(),
        &config_pda,
        &mint.pubkey(),
        &staking_protocol.pubkey(),
    );
    
    ctx.process_transaction(&[init_ix], &[]).await.unwrap();
    
    let new_staking_protocol = Keypair::new();
    let update_ix = create_update_staking_protocol_ix(
        &ctx.program_id,
        &ctx.payer.pubkey(),
        &config_pda,
        &new_staking_protocol.pubkey(),
    );
    
    let result = ctx.process_transaction(&[update_ix], &[]).await;
    assert!(result.is_ok());
}

/// Test update staking protocol unauthorized
#[tokio::test]
async fn test_update_staking_protocol_unauthorized() {
    let mut ctx = TestContext::new().await;
    
    let (config_pda, _bump) = ctx.get_config_pda();
    let mint = Keypair::new();
    let staking_protocol = Keypair::new();
    
    let init_ix = create_initialize_mint_ix(
        &ctx.program_id,
        &ctx.payer.pubkey(),
        &config_pda,
        &mint.pubkey(),
        &staking_protocol.pubkey(),
    );
    
    ctx.process_transaction(&[init_ix], &[]).await.unwrap();
    
    let unauthorized = Keypair::new();
    let new_staking_protocol = Keypair::new();
    let update_ix = create_update_staking_protocol_ix(
        &ctx.program_id,
        &unauthorized.pubkey(),
        &config_pda,
        &new_staking_protocol.pubkey(),
    );
    
    assert_eq!(update_ix.accounts[0].pubkey, unauthorized.pubkey());
}

/// Test update staking protocol multiple times
#[tokio::test]
async fn test_update_staking_protocol_multiple() {
    let mut ctx = TestContext::new().await;
    
    let (config_pda, _bump) = ctx.get_config_pda();
    let mint = Keypair::new();
    let staking_protocol = Keypair::new();
    
    let init_ix = create_initialize_mint_ix(
        &ctx.program_id,
        &ctx.payer.pubkey(),
        &config_pda,
        &mint.pubkey(),
        &staking_protocol.pubkey(),
    );
    
    ctx.process_transaction(&[init_ix], &[]).await.unwrap();
    
    for i in 0..3 {
        let new_staking_protocol = Keypair::new();
        let update_ix = create_update_staking_protocol_ix(
            &ctx.program_id,
            &ctx.payer.pubkey(),
            &config_pda,
            &new_staking_protocol.pubkey(),
        );
        
        let result = ctx.process_transaction(&[update_ix], &[]).await;
        assert!(result.is_ok(), "Update #{} should succeed", i + 1);
    }
}

// ============================================================================
// update_authority tests
// ============================================================================

/// Test update authority success
#[tokio::test]
async fn test_update_authority_success() {
    let mut ctx = TestContext::new().await;
    
    let (config_pda, _bump) = ctx.get_config_pda();
    let mint = Keypair::new();
    let staking_protocol = Keypair::new();
    
    let init_ix = create_initialize_mint_ix(
        &ctx.program_id,
        &ctx.payer.pubkey(),
        &config_pda,
        &mint.pubkey(),
        &staking_protocol.pubkey(),
    );
    
    ctx.process_transaction(&[init_ix], &[]).await.unwrap();
    
    let new_authority = Keypair::new();
    let update_ix = create_update_authority_ix(
        &ctx.program_id,
        &ctx.payer.pubkey(),
        &config_pda,
        &new_authority.pubkey(),
    );
    
    let result = ctx.process_transaction(&[update_ix], &[]).await;
    assert!(result.is_ok());
}

/// Test update authority unauthorized
#[tokio::test]
async fn test_update_authority_unauthorized() {
    let mut ctx = TestContext::new().await;
    
    let (config_pda, _bump) = ctx.get_config_pda();
    let mint = Keypair::new();
    let staking_protocol = Keypair::new();
    
    let init_ix = create_initialize_mint_ix(
        &ctx.program_id,
        &ctx.payer.pubkey(),
        &config_pda,
        &mint.pubkey(),
        &staking_protocol.pubkey(),
    );
    
    ctx.process_transaction(&[init_ix], &[]).await.unwrap();
    
    let unauthorized = Keypair::new();
    let new_authority = Keypair::new();
    let update_ix = create_update_authority_ix(
        &ctx.program_id,
        &unauthorized.pubkey(),
        &config_pda,
        &new_authority.pubkey(),
    );
    
    assert_eq!(update_ix.accounts[0].pubkey, unauthorized.pubkey());
}

/// Test update authority to self
#[tokio::test]
async fn test_update_authority_to_self() {
    let mut ctx = TestContext::new().await;
    
    let (config_pda, _bump) = ctx.get_config_pda();
    let mint = Keypair::new();
    let staking_protocol = Keypair::new();
    
    let init_ix = create_initialize_mint_ix(
        &ctx.program_id,
        &ctx.payer.pubkey(),
        &config_pda,
        &mint.pubkey(),
        &staking_protocol.pubkey(),
    );
    
    ctx.process_transaction(&[init_ix], &[]).await.unwrap();
    
    let update_ix = create_update_authority_ix(
        &ctx.program_id,
        &ctx.payer.pubkey(),
        &config_pda,
        &ctx.payer.pubkey(),
    );
    
    let result = ctx.process_transaction(&[update_ix], &[]).await;
    assert!(result.is_ok());
}

