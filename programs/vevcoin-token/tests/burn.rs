//! Integration tests for vevcoin-token burn_vevcoin instruction

mod common;

use common::*;
use solana_sdk::signature::{Keypair, Signer};

/// Test burn vevcoin instruction format
#[tokio::test]
async fn test_burn_vevcoin_instruction_format() {
    let ctx = TestContext::new().await;
    
    let (config_pda, _bump) = ctx.get_config_pda();
    let mint = Keypair::new();
    let staking_protocol = Keypair::new();
    let source = Keypair::new();
    let amount = 500u64;
    
    let ix = create_burn_vevcoin_ix(
        &ctx.program_id,
        &staking_protocol.pubkey(),
        &config_pda,
        &mint.pubkey(),
        &source.pubkey(),
        amount,
    );
    
    assert_eq!(ix.program_id, ctx.program_id);
    assert_eq!(ix.accounts.len(), 5);
    assert!(ix.accounts[0].is_signer);
    assert_eq!(&ix.data[8..16], &amount.to_le_bytes());
}

/// Test burn vevcoin zero amount
#[tokio::test]
async fn test_burn_vevcoin_zero_amount() {
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
    
    let source = Keypair::new();
    let ix = create_burn_vevcoin_ix(
        &ctx.program_id,
        &staking_protocol.pubkey(),
        &config_pda,
        &mint.pubkey(),
        &source.pubkey(),
        0,
    );
    
    assert_eq!(&ix.data[8..16], &0u64.to_le_bytes());
}

/// Test burn vevcoin unauthorized
#[tokio::test]
async fn test_burn_vevcoin_unauthorized() {
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
    let source = Keypair::new();
    let ix = create_burn_vevcoin_ix(
        &ctx.program_id,
        &unauthorized.pubkey(),
        &config_pda,
        &mint.pubkey(),
        &source.pubkey(),
        500,
    );
    
    assert_eq!(ix.accounts[0].pubkey, unauthorized.pubkey());
}

/// Test burn vevcoin exceeds balance
#[tokio::test]
async fn test_burn_vevcoin_exceeds_balance() {
    let ctx = TestContext::new().await;
    
    let (config_pda, _bump) = ctx.get_config_pda();
    let mint = Keypair::new();
    let staking_protocol = Keypair::new();
    let source = Keypair::new();
    let excessive_amount = u64::MAX;
    
    let ix = create_burn_vevcoin_ix(
        &ctx.program_id,
        &staking_protocol.pubkey(),
        &config_pda,
        &mint.pubkey(),
        &source.pubkey(),
        excessive_amount,
    );
    
    assert_eq!(&ix.data[8..16], &excessive_amount.to_le_bytes());
}

/// Test burn vevcoin all balance
#[tokio::test]
async fn test_burn_vevcoin_all_balance() {
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
    
    let config = ctx.get_account(config_pda).await;
    assert!(config.is_some());
}

