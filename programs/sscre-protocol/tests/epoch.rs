//! Integration tests for sscre-protocol epoch instructions

mod common;

use common::*;
use solana_sdk::signature::{Keypair, Signer};

#[tokio::test]
async fn test_start_epoch_instruction_format() {
    let ctx = TestContext::new().await;
    
    let (pool_pda, _bump) = ctx.get_pool_pda();
    let (epoch_pda, _) = ctx.get_epoch_pda(1);
    let (circuit_breaker_pda, _) = ctx.get_circuit_breaker_pda();
    let allocation = 1_000_000 * 1_000_000_000u64;
    
    let ix = create_start_epoch_ix(
        &ctx.program_id,
        &ctx.payer.pubkey(),
        &pool_pda,
        &epoch_pda,
        &circuit_breaker_pda,
        allocation,
    );
    
    assert_eq!(ix.program_id, ctx.program_id);
    assert_eq!(ix.accounts.len(), 5);
}

#[tokio::test]
async fn test_start_epoch_different_allocations() {
    let ctx = TestContext::new().await;
    
    let (pool_pda, _bump) = ctx.get_pool_pda();
    let (circuit_breaker_pda, _) = ctx.get_circuit_breaker_pda();
    
    for (i, allocation) in [1_000_000u64, 5_000_000, 10_000_000].iter().enumerate() {
        let (epoch_pda, _) = ctx.get_epoch_pda(i as u64 + 1);
        let allocation_with_decimals = allocation * 1_000_000_000;
        
        let ix = create_start_epoch_ix(
            &ctx.program_id,
            &ctx.payer.pubkey(),
            &pool_pda,
            &epoch_pda,
            &circuit_breaker_pda,
            allocation_with_decimals,
        );
        
        assert_eq!(&ix.data[8..16], &allocation_with_decimals.to_le_bytes());
    }
}

#[tokio::test]
async fn test_start_epoch_zero_allocation() {
    let ctx = TestContext::new().await;
    
    let (pool_pda, _bump) = ctx.get_pool_pda();
    let (epoch_pda, _) = ctx.get_epoch_pda(1);
    let (circuit_breaker_pda, _) = ctx.get_circuit_breaker_pda();
    
    let ix = create_start_epoch_ix(
        &ctx.program_id,
        &ctx.payer.pubkey(),
        &pool_pda,
        &epoch_pda,
        &circuit_breaker_pda,
        0,
    );
    
    assert_eq!(&ix.data[8..16], &0u64.to_le_bytes());
}

#[tokio::test]
async fn test_start_epoch_max_allocation() {
    let ctx = TestContext::new().await;
    
    let (pool_pda, _bump) = ctx.get_pool_pda();
    let (epoch_pda, _) = ctx.get_epoch_pda(1);
    let (circuit_breaker_pda, _) = ctx.get_circuit_breaker_pda();
    let max_allocation = 10_000_000 * 1_000_000_000u64; // Max epoch emission
    
    let ix = create_start_epoch_ix(
        &ctx.program_id,
        &ctx.payer.pubkey(),
        &pool_pda,
        &epoch_pda,
        &circuit_breaker_pda,
        max_allocation,
    );
    
    assert_eq!(&ix.data[8..16], &max_allocation.to_le_bytes());
}

#[tokio::test]
async fn test_start_epoch_sequential() {
    let ctx = TestContext::new().await;
    
    let (pool_pda, _bump) = ctx.get_pool_pda();
    let (circuit_breaker_pda, _) = ctx.get_circuit_breaker_pda();
    
    for epoch_num in 1..=5 {
        let (epoch_pda, _) = ctx.get_epoch_pda(epoch_num);
        
        let ix = create_start_epoch_ix(
            &ctx.program_id,
            &ctx.payer.pubkey(),
            &pool_pda,
            &epoch_pda,
            &circuit_breaker_pda,
            1_000_000 * 1_000_000_000,
        );
        
        assert_eq!(ix.accounts[1].pubkey, epoch_pda);
    }
}

