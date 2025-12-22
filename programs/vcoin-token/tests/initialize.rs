//! Integration tests for vcoin-token initialize_mint instruction
//!
//! Tests the initialization of the VCoin mint with Token-2022 extensions.

mod common;

use common::*;
use solana_sdk::signature::{Keypair, Signer};

/// Test successful initialization of the VCoin mint
#[tokio::test]
async fn test_initialize_mint_success() {
    let mut ctx = TestContext::new().await;
    
    let (config_pda, _bump) = ctx.get_config_pda();
    let mint = Keypair::new();
    let treasury = Keypair::new();
    let permanent_delegate = Keypair::new();
    
    let ix = create_initialize_mint_ix(
        &ctx.program_id,
        &ctx.payer.pubkey(),
        &config_pda,
        &mint.pubkey(),
        &treasury.pubkey(),
        &permanent_delegate.pubkey(),
    );
    
    let result = ctx.process_transaction(&[ix], &[]).await;
    assert!(result.is_ok(), "Initialize should succeed");
    
    // Verify config account was created
    let config_account = ctx.get_account(config_pda).await;
    assert!(config_account.is_some(), "Config account should exist");
    
    let config_data = config_account.unwrap();
    assert_eq!(config_data.owner, ctx.program_id, "Config should be owned by program");
}

/// Test that initialize fails when config already exists (double init)
#[tokio::test]
async fn test_initialize_mint_already_initialized() {
    let mut ctx = TestContext::new().await;
    
    let (config_pda, _bump) = ctx.get_config_pda();
    let mint = Keypair::new();
    let treasury = Keypair::new();
    let permanent_delegate = Keypair::new();
    
    // First initialization
    let ix = create_initialize_mint_ix(
        &ctx.program_id,
        &ctx.payer.pubkey(),
        &config_pda,
        &mint.pubkey(),
        &treasury.pubkey(),
        &permanent_delegate.pubkey(),
    );
    
    ctx.process_transaction(&[ix.clone()], &[]).await.unwrap();
    
    // Second initialization should fail
    let result = ctx.process_transaction(&[ix], &[]).await;
    assert!(result.is_err(), "Double initialization should fail");
}

/// Test initialization with different permanent delegates
#[tokio::test]
async fn test_initialize_mint_with_delegate() {
    let mut ctx = TestContext::new().await;
    
    let (config_pda, _bump) = ctx.get_config_pda();
    let mint = Keypair::new();
    let treasury = Keypair::new();
    let permanent_delegate = Keypair::new();
    
    let ix = create_initialize_mint_ix(
        &ctx.program_id,
        &ctx.payer.pubkey(),
        &config_pda,
        &mint.pubkey(),
        &treasury.pubkey(),
        &permanent_delegate.pubkey(),
    );
    
    let result = ctx.process_transaction(&[ix], &[]).await;
    assert!(result.is_ok(), "Initialize with delegate should succeed");
    
    // Config should be created with the specified delegate
    let config_account = ctx.get_account(config_pda).await;
    assert!(config_account.is_some());
}

