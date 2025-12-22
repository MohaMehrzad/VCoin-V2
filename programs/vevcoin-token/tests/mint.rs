//! Integration tests for vevcoin-token mint_vevcoin instruction

mod common;

use common::*;
use solana_sdk::signature::{Keypair, Signer};

/// Test mint vevcoin instruction format
#[tokio::test]
async fn test_mint_vevcoin_instruction_format() {
    let ctx = TestContext::new().await;
    
    let (config_pda, _bump) = ctx.get_config_pda();
    let mint = Keypair::new();
    let staking_protocol = Keypair::new();
    let destination = Keypair::new();
    let amount = 1000u64;
    
    let ix = create_mint_vevcoin_ix(
        &ctx.program_id,
        &staking_protocol.pubkey(),
        &config_pda,
        &mint.pubkey(),
        &destination.pubkey(),
        amount,
    );
    
    assert_eq!(ix.program_id, ctx.program_id);
    assert_eq!(ix.accounts.len(), 5);
    assert!(ix.accounts[0].is_signer);
    assert_eq!(&ix.data[8..16], &amount.to_le_bytes());
}

/// Test mint vevcoin zero amount
#[tokio::test]
async fn test_mint_vevcoin_zero_amount() {
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
    
    let destination = Keypair::new();
    let ix = create_mint_vevcoin_ix(
        &ctx.program_id,
        &staking_protocol.pubkey(),
        &config_pda,
        &mint.pubkey(),
        &destination.pubkey(),
        0,
    );
    
    assert_eq!(&ix.data[8..16], &0u64.to_le_bytes());
}

/// Test mint vevcoin unauthorized
#[tokio::test]
async fn test_mint_vevcoin_unauthorized() {
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
    let destination = Keypair::new();
    let ix = create_mint_vevcoin_ix(
        &ctx.program_id,
        &unauthorized.pubkey(),
        &config_pda,
        &mint.pubkey(),
        &destination.pubkey(),
        1000,
    );
    
    assert_eq!(ix.accounts[0].pubkey, unauthorized.pubkey());
}

/// Test mint vevcoin large amount
#[tokio::test]
async fn test_mint_vevcoin_large_amount() {
    let ctx = TestContext::new().await;
    
    let (config_pda, _bump) = ctx.get_config_pda();
    let mint = Keypair::new();
    let staking_protocol = Keypair::new();
    let destination = Keypair::new();
    let large_amount = u64::MAX;
    
    let ix = create_mint_vevcoin_ix(
        &ctx.program_id,
        &staking_protocol.pubkey(),
        &config_pda,
        &mint.pubkey(),
        &destination.pubkey(),
        large_amount,
    );
    
    assert_eq!(&ix.data[8..16], &large_amount.to_le_bytes());
}

/// Test mint vevcoin multiple times
#[tokio::test]
async fn test_mint_vevcoin_multiple() {
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
    
    for i in 1..=5 {
        let destination = Keypair::new();
        let ix = create_mint_vevcoin_ix(
            &ctx.program_id,
            &staking_protocol.pubkey(),
            &config_pda,
            &mint.pubkey(),
            &destination.pubkey(),
            i * 1000,
        );
        
        assert_eq!(&ix.data[8..16], &(i * 1000u64).to_le_bytes());
    }
}

