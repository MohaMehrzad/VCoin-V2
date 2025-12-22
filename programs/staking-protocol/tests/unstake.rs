//! Integration tests for staking-protocol unstake instruction

mod common;

use common::*;
use solana_sdk::signature::{Keypair, Signer};

#[tokio::test]
async fn test_unstake_instruction_format() {
    let ctx = TestContext::new().await;
    
    let (pool_pda, _bump) = ctx.get_pool_pda();
    let user = Keypair::new();
    let (user_stake_pda, _) = ctx.get_user_stake_pda(&user.pubkey());
    let amount = 500u64;
    
    let ix = create_unstake_ix(
        &ctx.program_id,
        &user.pubkey(),
        &pool_pda,
        &user_stake_pda,
        amount,
    );
    
    assert_eq!(ix.program_id, ctx.program_id);
    assert_eq!(ix.accounts.len(), 4);
    assert!(ix.accounts[0].is_signer);
    assert_eq!(&ix.data[8..16], &amount.to_le_bytes());
}

#[tokio::test]
async fn test_unstake_zero_amount() {
    let ctx = TestContext::new().await;
    
    let (pool_pda, _bump) = ctx.get_pool_pda();
    let user = Keypair::new();
    let (user_stake_pda, _) = ctx.get_user_stake_pda(&user.pubkey());
    
    let ix = create_unstake_ix(
        &ctx.program_id,
        &user.pubkey(),
        &pool_pda,
        &user_stake_pda,
        0,
    );
    
    assert_eq!(&ix.data[8..16], &0u64.to_le_bytes());
}

#[tokio::test]
async fn test_unstake_partial_amount() {
    let ctx = TestContext::new().await;
    
    let (pool_pda, _bump) = ctx.get_pool_pda();
    let user = Keypair::new();
    let (user_stake_pda, _) = ctx.get_user_stake_pda(&user.pubkey());
    let partial_amount = 500 * 1_000_000_000u64;
    
    let ix = create_unstake_ix(
        &ctx.program_id,
        &user.pubkey(),
        &pool_pda,
        &user_stake_pda,
        partial_amount,
    );
    
    assert_eq!(&ix.data[8..16], &partial_amount.to_le_bytes());
}

#[tokio::test]
async fn test_unstake_full_amount() {
    let ctx = TestContext::new().await;
    
    let (pool_pda, _bump) = ctx.get_pool_pda();
    let user = Keypair::new();
    let (user_stake_pda, _) = ctx.get_user_stake_pda(&user.pubkey());
    let full_amount = 1000 * 1_000_000_000u64;
    
    let ix = create_unstake_ix(
        &ctx.program_id,
        &user.pubkey(),
        &pool_pda,
        &user_stake_pda,
        full_amount,
    );
    
    assert_eq!(&ix.data[8..16], &full_amount.to_le_bytes());
}

#[tokio::test]
async fn test_unstake_exceeds_balance() {
    let ctx = TestContext::new().await;
    
    let (pool_pda, _bump) = ctx.get_pool_pda();
    let user = Keypair::new();
    let (user_stake_pda, _) = ctx.get_user_stake_pda(&user.pubkey());
    let excessive_amount = u64::MAX;
    
    let ix = create_unstake_ix(
        &ctx.program_id,
        &user.pubkey(),
        &pool_pda,
        &user_stake_pda,
        excessive_amount,
    );
    
    assert_eq!(&ix.data[8..16], &excessive_amount.to_le_bytes());
}

#[tokio::test]
async fn test_unstake_multiple_times() {
    let ctx = TestContext::new().await;
    
    let (pool_pda, _bump) = ctx.get_pool_pda();
    let user = Keypair::new();
    let (user_stake_pda, _) = ctx.get_user_stake_pda(&user.pubkey());
    
    for i in 1..=3 {
        let amount = i * 100 * 1_000_000_000u64;
        let ix = create_unstake_ix(
            &ctx.program_id,
            &user.pubkey(),
            &pool_pda,
            &user_stake_pda,
            amount,
        );
        
        assert_eq!(&ix.data[8..16], &amount.to_le_bytes());
    }
}

