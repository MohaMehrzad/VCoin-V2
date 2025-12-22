//! Integration tests for staking-protocol update_tier instruction

mod common;

use common::*;
use solana_sdk::signature::{Keypair, Signer};

#[tokio::test]
async fn test_update_tier_instruction_format() {
    let ctx = TestContext::new().await;
    
    let (pool_pda, _bump) = ctx.get_pool_pda();
    let user = Keypair::new();
    let (user_stake_pda, _) = ctx.get_user_stake_pda(&user.pubkey());
    
    let ix = create_update_tier_ix(
        &ctx.program_id,
        &user.pubkey(),
        &pool_pda,
        &user_stake_pda,
    );
    
    assert_eq!(ix.program_id, ctx.program_id);
    assert_eq!(ix.accounts.len(), 3);
    assert!(ix.accounts[0].is_signer);
    assert_eq!(ix.data.len(), 8);
}

#[tokio::test]
async fn test_update_tier_for_different_users() {
    let ctx = TestContext::new().await;
    
    let (pool_pda, _bump) = ctx.get_pool_pda();
    
    for _ in 0..5 {
        let user = Keypair::new();
        let (user_stake_pda, _) = ctx.get_user_stake_pda(&user.pubkey());
        
        let ix = create_update_tier_ix(
            &ctx.program_id,
            &user.pubkey(),
            &pool_pda,
            &user_stake_pda,
        );
        
        assert_eq!(ix.accounts[0].pubkey, user.pubkey());
    }
}

