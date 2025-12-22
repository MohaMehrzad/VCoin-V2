//! Integration tests for five-a-protocol initialize instruction

mod common;

use common::*;
use solana_sdk::signature::{Keypair, Signer};

#[tokio::test]
async fn test_initialize_success() {
    let mut ctx = TestContext::new().await;
    
    let (config_pda, _bump) = ctx.get_config_pda();
    let identity_program = Keypair::new();
    let vcoin_mint = Keypair::new();
    
    let ix = create_initialize_ix(
        &ctx.program_id,
        &ctx.payer.pubkey(),
        &config_pda,
        &identity_program.pubkey(),
        &vcoin_mint.pubkey(),
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
    let identity_program = Keypair::new();
    let vcoin_mint = Keypair::new();
    
    let ix = create_initialize_ix(
        &ctx.program_id,
        &ctx.payer.pubkey(),
        &config_pda,
        &identity_program.pubkey(),
        &vcoin_mint.pubkey(),
    );
    
    ctx.process_transaction(&[ix.clone()], &[]).await.unwrap();
    
    let result = ctx.process_transaction(&[ix], &[]).await;
    assert!(result.is_err());
}

#[tokio::test]
async fn test_initialize_with_programs() {
    let mut ctx = TestContext::new().await;
    
    let (config_pda, _bump) = ctx.get_config_pda();
    let identity_program = Keypair::new();
    let vcoin_mint = Keypair::new();
    
    let ix = create_initialize_ix(
        &ctx.program_id,
        &ctx.payer.pubkey(),
        &config_pda,
        &identity_program.pubkey(),
        &vcoin_mint.pubkey(),
    );
    
    let result = ctx.process_transaction(&[ix], &[]).await;
    assert!(result.is_ok());
}

