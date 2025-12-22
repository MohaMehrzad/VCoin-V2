//! Integration tests for vevcoin-token initialize_mint instruction

mod common;

use common::*;
use solana_sdk::signature::{Keypair, Signer};

/// Test successful initialization
#[tokio::test]
async fn test_initialize_mint_success() {
    let mut ctx = TestContext::new().await;
    
    let (config_pda, _bump) = ctx.get_config_pda();
    let mint = Keypair::new();
    let staking_protocol = Keypair::new();
    
    let ix = create_initialize_mint_ix(
        &ctx.program_id,
        &ctx.payer.pubkey(),
        &config_pda,
        &mint.pubkey(),
        &staking_protocol.pubkey(),
    );
    
    let result = ctx.process_transaction(&[ix], &[]).await;
    assert!(result.is_ok(), "Initialize should succeed");
    
    let config_account = ctx.get_account(config_pda).await;
    assert!(config_account.is_some(), "Config account should exist");
}

/// Test double initialization fails
#[tokio::test]
async fn test_initialize_mint_already_initialized() {
    let mut ctx = TestContext::new().await;
    
    let (config_pda, _bump) = ctx.get_config_pda();
    let mint = Keypair::new();
    let staking_protocol = Keypair::new();
    
    let ix = create_initialize_mint_ix(
        &ctx.program_id,
        &ctx.payer.pubkey(),
        &config_pda,
        &mint.pubkey(),
        &staking_protocol.pubkey(),
    );
    
    ctx.process_transaction(&[ix.clone()], &[]).await.unwrap();
    
    let result = ctx.process_transaction(&[ix], &[]).await;
    assert!(result.is_err(), "Double initialization should fail");
}

/// Test initialization with valid staking protocol
#[tokio::test]
async fn test_initialize_with_staking_protocol() {
    let mut ctx = TestContext::new().await;
    
    let (config_pda, _bump) = ctx.get_config_pda();
    let mint = Keypair::new();
    let staking_protocol = Keypair::new();
    
    let ix = create_initialize_mint_ix(
        &ctx.program_id,
        &ctx.payer.pubkey(),
        &config_pda,
        &mint.pubkey(),
        &staking_protocol.pubkey(),
    );
    
    let result = ctx.process_transaction(&[ix], &[]).await;
    assert!(result.is_ok());
    
    let config = ctx.get_account(config_pda).await;
    assert!(config.is_some());
}

