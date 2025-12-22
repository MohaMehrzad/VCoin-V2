//! Integration tests for transfer-hook initialize instruction

mod common;

use common::*;
use solana_sdk::signature::{Keypair, Signer};

#[tokio::test]
async fn test_initialize_success() {
    let mut ctx = TestContext::new().await;
    
    let (config_pda, _bump) = ctx.get_config_pda();
    let five_a_program = Keypair::new();
    let min_activity_amount = 1_000_000_000u64;
    
    let ix = create_initialize_ix(
        &ctx.program_id,
        &ctx.payer.pubkey(),
        &config_pda,
        &five_a_program.pubkey(),
        min_activity_amount,
    );
    
    let result = ctx.process_transaction(&[ix], &[]).await;
    assert!(result.is_ok());
    
    let config = ctx.get_account(config_pda).await;
    assert!(config.is_some());
}

#[tokio::test]
async fn test_initialize_already_initialized() {
    let mut ctx = TestContext::new().await;
    
    let (config_pda, _bump) = ctx.get_config_pda();
    let five_a_program = Keypair::new();
    
    let ix = create_initialize_ix(
        &ctx.program_id,
        &ctx.payer.pubkey(),
        &config_pda,
        &five_a_program.pubkey(),
        1_000_000_000,
    );
    
    ctx.process_transaction(&[ix.clone()], &[]).await.unwrap();
    
    let result = ctx.process_transaction(&[ix], &[]).await;
    assert!(result.is_err());
}

#[tokio::test]
async fn test_initialize_with_different_min_amounts() {
    let mut ctx = TestContext::new().await;
    
    let (config_pda, _bump) = ctx.get_config_pda();
    let five_a_program = Keypair::new();
    
    // Test with very low amount
    let ix = create_initialize_ix(
        &ctx.program_id,
        &ctx.payer.pubkey(),
        &config_pda,
        &five_a_program.pubkey(),
        1, // Very low
    );
    
    let result = ctx.process_transaction(&[ix], &[]).await;
    assert!(result.is_ok());
}

