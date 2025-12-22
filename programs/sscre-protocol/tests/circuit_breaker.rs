//! Integration tests for sscre-protocol circuit breaker

mod common;

use common::*;
use solana_sdk::signature::{Keypair, Signer};

#[tokio::test]
async fn test_trigger_circuit_breaker_instruction_format() {
    let ctx = TestContext::new().await;
    
    let (pool_pda, _bump) = ctx.get_pool_pda();
    let (circuit_breaker_pda, _) = ctx.get_circuit_breaker_pda();
    
    let ix = create_trigger_circuit_breaker_ix(
        &ctx.program_id,
        &ctx.payer.pubkey(),
        &pool_pda,
        &circuit_breaker_pda,
        1,
    );
    
    assert_eq!(ix.program_id, ctx.program_id);
    assert_eq!(ix.accounts.len(), 3);
}

#[tokio::test]
async fn test_trigger_circuit_breaker_different_reasons() {
    let ctx = TestContext::new().await;
    
    let (pool_pda, _bump) = ctx.get_pool_pda();
    let (circuit_breaker_pda, _) = ctx.get_circuit_breaker_pda();
    
    for reason in 0..=5 {
        let ix = create_trigger_circuit_breaker_ix(
            &ctx.program_id,
            &ctx.payer.pubkey(),
            &pool_pda,
            &circuit_breaker_pda,
            reason,
        );
        
        assert_eq!(ix.data[8], reason);
    }
}

#[tokio::test]
async fn test_trigger_circuit_breaker_unauthorized() {
    let ctx = TestContext::new().await;
    
    let (pool_pda, _bump) = ctx.get_pool_pda();
    let (circuit_breaker_pda, _) = ctx.get_circuit_breaker_pda();
    let unauthorized = Keypair::new();
    
    let ix = create_trigger_circuit_breaker_ix(
        &ctx.program_id,
        &unauthorized.pubkey(),
        &pool_pda,
        &circuit_breaker_pda,
        1,
    );
    
    assert_eq!(ix.accounts[2].pubkey, unauthorized.pubkey());
}

