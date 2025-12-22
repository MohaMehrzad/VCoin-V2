//! Integration tests for staking-protocol initialize_pool instruction

mod common;

use common::*;
use solana_sdk::signature::{Keypair, Signer};

#[tokio::test]
async fn test_initialize_pool_success() {
    let mut ctx = TestContext::new().await;
    
    let (pool_pda, _bump) = ctx.get_pool_pda();
    let vevcoin_program = Keypair::new();
    
    let ix = create_initialize_pool_ix(
        &ctx.program_id,
        &ctx.payer.pubkey(),
        &pool_pda,
        &vevcoin_program.pubkey(),
    );
    
    let result = ctx.process_transaction(&[ix], &[]).await;
    assert!(result.is_ok());
    
    let pool = ctx.get_account(pool_pda).await;
    assert!(pool.is_some());
}

#[tokio::test]
async fn test_initialize_pool_already_initialized() {
    let mut ctx = TestContext::new().await;
    
    let (pool_pda, _bump) = ctx.get_pool_pda();
    let vevcoin_program = Keypair::new();
    
    let ix = create_initialize_pool_ix(
        &ctx.program_id,
        &ctx.payer.pubkey(),
        &pool_pda,
        &vevcoin_program.pubkey(),
    );
    
    ctx.process_transaction(&[ix.clone()], &[]).await.unwrap();
    
    let result = ctx.process_transaction(&[ix], &[]).await;
    assert!(result.is_err());
}

#[tokio::test]
async fn test_initialize_pool_with_vevcoin_program() {
    let mut ctx = TestContext::new().await;
    
    let (pool_pda, _bump) = ctx.get_pool_pda();
    let vevcoin_program = Keypair::new();
    
    let ix = create_initialize_pool_ix(
        &ctx.program_id,
        &ctx.payer.pubkey(),
        &pool_pda,
        &vevcoin_program.pubkey(),
    );
    
    let result = ctx.process_transaction(&[ix], &[]).await;
    assert!(result.is_ok());
}

