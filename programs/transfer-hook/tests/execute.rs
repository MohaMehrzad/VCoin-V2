//! Integration tests for transfer-hook execute instruction

mod common;

use common::*;
use solana_sdk::signature::{Keypair, Signer};

#[tokio::test]
async fn test_execute_instruction_format() {
    let ctx = TestContext::new().await;
    
    let (config_pda, _bump) = ctx.get_config_pda();
    let source = Keypair::new();
    let destination = Keypair::new();
    let amount = 1_000_000_000u64;
    
    let ix = create_execute_ix(
        &ctx.program_id,
        &source.pubkey(),
        &destination.pubkey(),
        &config_pda,
        amount,
    );
    
    assert_eq!(ix.program_id, ctx.program_id);
    assert_eq!(ix.accounts.len(), 3);
    assert_eq!(&ix.data[8..16], &amount.to_le_bytes());
}

#[tokio::test]
async fn test_execute_zero_amount() {
    let ctx = TestContext::new().await;
    
    let (config_pda, _bump) = ctx.get_config_pda();
    let source = Keypair::new();
    let destination = Keypair::new();
    
    let ix = create_execute_ix(
        &ctx.program_id,
        &source.pubkey(),
        &destination.pubkey(),
        &config_pda,
        0,
    );
    
    assert_eq!(&ix.data[8..16], &0u64.to_le_bytes());
}

#[tokio::test]
async fn test_execute_large_amount() {
    let ctx = TestContext::new().await;
    
    let (config_pda, _bump) = ctx.get_config_pda();
    let source = Keypair::new();
    let destination = Keypair::new();
    let large_amount = u64::MAX;
    
    let ix = create_execute_ix(
        &ctx.program_id,
        &source.pubkey(),
        &destination.pubkey(),
        &config_pda,
        large_amount,
    );
    
    assert_eq!(&ix.data[8..16], &large_amount.to_le_bytes());
}

#[tokio::test]
async fn test_execute_same_source_dest() {
    let ctx = TestContext::new().await;
    
    let (config_pda, _bump) = ctx.get_config_pda();
    let same_account = Keypair::new();
    
    let ix = create_execute_ix(
        &ctx.program_id,
        &same_account.pubkey(),
        &same_account.pubkey(),
        &config_pda,
        1000,
    );
    
    assert_eq!(ix.accounts[0].pubkey, ix.accounts[1].pubkey);
}

#[tokio::test]
async fn test_execute_multiple_transfers() {
    let mut ctx = TestContext::new().await;
    
    let (config_pda, _bump) = ctx.get_config_pda();
    let five_a_program = Keypair::new();
    
    let init_ix = create_initialize_ix(
        &ctx.program_id,
        &ctx.payer.pubkey(),
        &config_pda,
        &five_a_program.pubkey(),
        1_000_000_000,
    );
    
    ctx.process_transaction(&[init_ix], &[]).await.unwrap();
    
    for i in 1..=5 {
        let source = Keypair::new();
        let destination = Keypair::new();
        
        let ix = create_execute_ix(
            &ctx.program_id,
            &source.pubkey(),
            &destination.pubkey(),
            &config_pda,
            i * 1_000_000_000,
        );
        
        assert_eq!(&ix.data[8..16], &(i * 1_000_000_000u64).to_le_bytes());
    }
}

