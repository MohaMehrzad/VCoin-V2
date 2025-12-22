//! Integration tests for vilink-protocol initialize instruction

mod common;

use common::*;
use solana_sdk::signature::{Keypair, Signer};

#[tokio::test]
async fn test_initialize_success() {
    let ctx = TestContext::new().await;
    
    let (config_pda, _bump) = ctx.get_config_pda();
    let vcoin_mint = Keypair::new();
    let treasury = Keypair::new();
    
    let ix = create_initialize_ix(
        &ctx.program_id,
        &ctx.payer.pubkey(),
        &config_pda,
        &vcoin_mint.pubkey(),
        &treasury.pubkey(),
    );
    
    assert_eq!(ix.program_id, ctx.program_id);
    assert_eq!(ix.accounts.len(), 4);
}

#[tokio::test]
async fn test_initialize_with_treasury() {
    let ctx = TestContext::new().await;
    
    let (config_pda, _bump) = ctx.get_config_pda();
    let vcoin_mint = Keypair::new();
    let treasury = Keypair::new();
    
    let ix = create_initialize_ix(
        &ctx.program_id,
        &ctx.payer.pubkey(),
        &config_pda,
        &vcoin_mint.pubkey(),
        &treasury.pubkey(),
    );
    
    assert_eq!(&ix.data[8..40], treasury.pubkey().as_ref());
}

#[tokio::test]
async fn test_initialize_different_mints() {
    let ctx = TestContext::new().await;
    
    let (config_pda, _bump) = ctx.get_config_pda();
    let treasury = Keypair::new();
    
    for _ in 0..3 {
        let vcoin_mint = Keypair::new();
        
        let ix = create_initialize_ix(
            &ctx.program_id,
            &ctx.payer.pubkey(),
            &config_pda,
            &vcoin_mint.pubkey(),
            &treasury.pubkey(),
        );
        
        assert!(ix.data.len() > 8);
    }
}

