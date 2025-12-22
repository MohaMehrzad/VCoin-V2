//! Integration tests for gasless-protocol initialize instruction

mod common;

use common::*;
use solana_sdk::signature::{Keypair, Signer};

#[tokio::test]
async fn test_initialize_success() {
    let ctx = TestContext::new().await;
    
    let (config_pda, _bump) = ctx.get_config_pda();
    let fee_payer = Keypair::new();
    let daily_budget = 10_000_000_000u64;
    
    let ix = create_initialize_ix(
        &ctx.program_id,
        &ctx.payer.pubkey(),
        &config_pda,
        &fee_payer.pubkey(),
        daily_budget,
    );
    
    assert_eq!(ix.program_id, ctx.program_id);
    assert_eq!(ix.accounts.len(), 3);
}

#[tokio::test]
async fn test_initialize_with_fee_payer() {
    let ctx = TestContext::new().await;
    
    let (config_pda, _bump) = ctx.get_config_pda();
    let fee_payer = Keypair::new();
    
    let ix = create_initialize_ix(
        &ctx.program_id,
        &ctx.payer.pubkey(),
        &config_pda,
        &fee_payer.pubkey(),
        10_000_000_000,
    );
    
    assert_eq!(&ix.data[8..40], fee_payer.pubkey().as_ref());
}

#[tokio::test]
async fn test_initialize_different_budgets() {
    let ctx = TestContext::new().await;
    
    let (config_pda, _bump) = ctx.get_config_pda();
    let fee_payer = Keypair::new();
    
    for budget in [1_000_000_000u64, 10_000_000_000, 100_000_000_000] {
        let ix = create_initialize_ix(
            &ctx.program_id,
            &ctx.payer.pubkey(),
            &config_pda,
            &fee_payer.pubkey(),
            budget,
        );
        
        assert_eq!(&ix.data[40..48], &budget.to_le_bytes());
    }
}

#[tokio::test]
async fn test_initialize_zero_budget() {
    let ctx = TestContext::new().await;
    
    let (config_pda, _bump) = ctx.get_config_pda();
    let fee_payer = Keypair::new();
    
    let ix = create_initialize_ix(
        &ctx.program_id,
        &ctx.payer.pubkey(),
        &config_pda,
        &fee_payer.pubkey(),
        0,
    );
    
    assert_eq!(&ix.data[40..48], &0u64.to_le_bytes());
}

