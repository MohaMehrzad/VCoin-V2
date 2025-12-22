//! Integration tests for sscre-protocol admin instructions

mod common;

use common::*;
use solana_sdk::signature::{Keypair, Signer};

#[tokio::test]
async fn test_set_paused_true() {
    let ctx = TestContext::new().await;
    
    let (pool_pda, _bump) = ctx.get_pool_pda();
    
    let ix = create_set_paused_ix(
        &ctx.program_id,
        &ctx.payer.pubkey(),
        &pool_pda,
        true,
    );
    
    assert_eq!(ix.data[8], 1);
}

#[tokio::test]
async fn test_set_paused_false() {
    let ctx = TestContext::new().await;
    
    let (pool_pda, _bump) = ctx.get_pool_pda();
    
    let ix = create_set_paused_ix(
        &ctx.program_id,
        &ctx.payer.pubkey(),
        &pool_pda,
        false,
    );
    
    assert_eq!(ix.data[8], 0);
}

#[tokio::test]
async fn test_set_paused_unauthorized() {
    let ctx = TestContext::new().await;
    
    let (pool_pda, _bump) = ctx.get_pool_pda();
    let unauthorized = Keypair::new();
    
    let ix = create_set_paused_ix(
        &ctx.program_id,
        &unauthorized.pubkey(),
        &pool_pda,
        true,
    );
    
    assert_eq!(ix.accounts[1].pubkey, unauthorized.pubkey());
}

#[tokio::test]
async fn test_set_paused_toggle() {
    let ctx = TestContext::new().await;
    
    let (pool_pda, _bump) = ctx.get_pool_pda();
    
    for paused in [true, false, true, false] {
        let ix = create_set_paused_ix(
            &ctx.program_id,
            &ctx.payer.pubkey(),
            &pool_pda,
            paused,
        );
        
        assert_eq!(ix.data[8], if paused { 1 } else { 0 });
    }
}

