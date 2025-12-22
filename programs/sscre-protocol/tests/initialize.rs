//! Integration tests for sscre-protocol initialize instructions

mod common;

use common::*;
use solana_sdk::signature::{Keypair, Signer};

#[tokio::test]
async fn test_initialize_pool_instruction_format() {
    let ctx = TestContext::new().await;
    
    let (pool_pda, _bump) = ctx.get_pool_pda();
    let vcoin_mint = Keypair::new();
    let pool_vault = Keypair::new();
    let fee_recipient = Keypair::new();
    
    let ix = create_initialize_pool_ix(
        &ctx.program_id,
        &ctx.payer.pubkey(),
        &pool_pda,
        &vcoin_mint.pubkey(),
        &pool_vault.pubkey(),
        &fee_recipient.pubkey(),
    );
    
    assert_eq!(ix.program_id, ctx.program_id);
    assert_eq!(ix.accounts.len(), 6);
}

#[tokio::test]
async fn test_initialize_pool_with_fee_recipient() {
    let ctx = TestContext::new().await;
    
    let (pool_pda, _bump) = ctx.get_pool_pda();
    let vcoin_mint = Keypair::new();
    let pool_vault = Keypair::new();
    let fee_recipient = Keypair::new();
    
    let ix = create_initialize_pool_ix(
        &ctx.program_id,
        &ctx.payer.pubkey(),
        &pool_pda,
        &vcoin_mint.pubkey(),
        &pool_vault.pubkey(),
        &fee_recipient.pubkey(),
    );
    
    assert_eq!(&ix.data[8..40], fee_recipient.pubkey().as_ref());
}

#[tokio::test]
async fn test_initialize_pool_different_authorities() {
    let ctx = TestContext::new().await;
    
    let (pool_pda, _bump) = ctx.get_pool_pda();
    let vcoin_mint = Keypair::new();
    let pool_vault = Keypair::new();
    
    for _ in 0..3 {
        let fee_recipient = Keypair::new();
        
        let ix = create_initialize_pool_ix(
            &ctx.program_id,
            &ctx.payer.pubkey(),
            &pool_pda,
            &vcoin_mint.pubkey(),
            &pool_vault.pubkey(),
            &fee_recipient.pubkey(),
        );
        
        assert!(ix.data.len() > 8);
    }
}

