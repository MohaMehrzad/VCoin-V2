//! Integration tests for gasless-protocol fee deduction instructions

mod common;

use common::*;
use solana_sdk::signature::{Keypair, Signer};

#[tokio::test]
async fn test_deduct_vcoin_fee_instruction_format() {
    let ctx = TestContext::new().await;
    
    let (config_pda, _bump) = ctx.get_config_pda();
    let user = Keypair::new();
    
    let ix = create_deduct_vcoin_fee_ix(
        &ctx.program_id,
        &user.pubkey(),
        &config_pda,
        1_000_000_000,
    );
    
    assert_eq!(ix.program_id, ctx.program_id);
    assert_eq!(ix.accounts.len(), 3);
}

#[tokio::test]
async fn test_deduct_vcoin_fee_different_amounts() {
    let ctx = TestContext::new().await;
    
    let (config_pda, _bump) = ctx.get_config_pda();
    let user = Keypair::new();
    
    for amount in [5_000u64, 10_000, 100_000, 1_000_000] {
        let ix = create_deduct_vcoin_fee_ix(
            &ctx.program_id,
            &user.pubkey(),
            &config_pda,
            amount,
        );
        
        assert_eq!(&ix.data[8..16], &amount.to_le_bytes());
    }
}

#[tokio::test]
async fn test_deduct_vcoin_fee_zero_amount() {
    let ctx = TestContext::new().await;
    
    let (config_pda, _bump) = ctx.get_config_pda();
    let user = Keypair::new();
    
    let ix = create_deduct_vcoin_fee_ix(
        &ctx.program_id,
        &user.pubkey(),
        &config_pda,
        0,
    );
    
    assert_eq!(&ix.data[8..16], &0u64.to_le_bytes());
}

#[tokio::test]
async fn test_deduct_vcoin_fee_multiple_users() {
    let ctx = TestContext::new().await;
    
    let (config_pda, _bump) = ctx.get_config_pda();
    
    for i in 0..5 {
        let user = Keypair::new();
        
        let ix = create_deduct_vcoin_fee_ix(
            &ctx.program_id,
            &user.pubkey(),
            &config_pda,
            (i + 1) as u64 * 1_000_000,
        );
        
        assert!(ix.accounts[1].is_signer);
    }
}

