//! Integration tests for staking-protocol stake instruction

mod common;

use common::*;
use solana_sdk::signature::{Keypair, Signer};

const MIN_LOCK_DURATION: i64 = 7 * 24 * 60 * 60;  // 1 week
const MAX_LOCK_DURATION: i64 = 4 * 365 * 24 * 60 * 60;  // 4 years

#[tokio::test]
async fn test_stake_instruction_format() {
    let ctx = TestContext::new().await;
    
    let (pool_pda, _bump) = ctx.get_pool_pda();
    let user = Keypair::new();
    let (user_stake_pda, _) = ctx.get_user_stake_pda(&user.pubkey());
    let amount = 1000 * 1_000_000_000u64;
    let lock_duration = MIN_LOCK_DURATION;
    
    let ix = create_stake_ix(
        &ctx.program_id,
        &user.pubkey(),
        &pool_pda,
        &user_stake_pda,
        amount,
        lock_duration,
    );
    
    assert_eq!(ix.program_id, ctx.program_id);
    assert_eq!(ix.accounts.len(), 5);
    assert!(ix.accounts[0].is_signer);
}

#[tokio::test]
async fn test_stake_zero_amount() {
    let ctx = TestContext::new().await;
    
    let (pool_pda, _bump) = ctx.get_pool_pda();
    let user = Keypair::new();
    let (user_stake_pda, _) = ctx.get_user_stake_pda(&user.pubkey());
    
    let ix = create_stake_ix(
        &ctx.program_id,
        &user.pubkey(),
        &pool_pda,
        &user_stake_pda,
        0,
        MIN_LOCK_DURATION,
    );
    
    assert_eq!(&ix.data[8..16], &0u64.to_le_bytes());
}

#[tokio::test]
async fn test_stake_min_lock_duration() {
    let ctx = TestContext::new().await;
    
    let (pool_pda, _bump) = ctx.get_pool_pda();
    let user = Keypair::new();
    let (user_stake_pda, _) = ctx.get_user_stake_pda(&user.pubkey());
    
    let ix = create_stake_ix(
        &ctx.program_id,
        &user.pubkey(),
        &pool_pda,
        &user_stake_pda,
        1000,
        MIN_LOCK_DURATION,
    );
    
    assert_eq!(&ix.data[16..24], &MIN_LOCK_DURATION.to_le_bytes());
}

#[tokio::test]
async fn test_stake_max_lock_duration() {
    let ctx = TestContext::new().await;
    
    let (pool_pda, _bump) = ctx.get_pool_pda();
    let user = Keypair::new();
    let (user_stake_pda, _) = ctx.get_user_stake_pda(&user.pubkey());
    
    let ix = create_stake_ix(
        &ctx.program_id,
        &user.pubkey(),
        &pool_pda,
        &user_stake_pda,
        1000,
        MAX_LOCK_DURATION,
    );
    
    assert_eq!(&ix.data[16..24], &MAX_LOCK_DURATION.to_le_bytes());
}

#[tokio::test]
async fn test_stake_lock_duration_too_short() {
    let ctx = TestContext::new().await;
    
    let (pool_pda, _bump) = ctx.get_pool_pda();
    let user = Keypair::new();
    let (user_stake_pda, _) = ctx.get_user_stake_pda(&user.pubkey());
    let too_short = MIN_LOCK_DURATION - 1;
    
    let ix = create_stake_ix(
        &ctx.program_id,
        &user.pubkey(),
        &pool_pda,
        &user_stake_pda,
        1000,
        too_short,
    );
    
    assert_eq!(&ix.data[16..24], &too_short.to_le_bytes());
}

#[tokio::test]
async fn test_stake_lock_duration_too_long() {
    let ctx = TestContext::new().await;
    
    let (pool_pda, _bump) = ctx.get_pool_pda();
    let user = Keypair::new();
    let (user_stake_pda, _) = ctx.get_user_stake_pda(&user.pubkey());
    let too_long = MAX_LOCK_DURATION + 1;
    
    let ix = create_stake_ix(
        &ctx.program_id,
        &user.pubkey(),
        &pool_pda,
        &user_stake_pda,
        1000,
        too_long,
    );
    
    assert_eq!(&ix.data[16..24], &too_long.to_le_bytes());
}

#[tokio::test]
async fn test_stake_bronze_tier_amount() {
    let ctx = TestContext::new().await;
    
    let (pool_pda, _bump) = ctx.get_pool_pda();
    let user = Keypair::new();
    let (user_stake_pda, _) = ctx.get_user_stake_pda(&user.pubkey());
    let bronze_amount = 1000 * 1_000_000_000u64;
    
    let ix = create_stake_ix(
        &ctx.program_id,
        &user.pubkey(),
        &pool_pda,
        &user_stake_pda,
        bronze_amount,
        MIN_LOCK_DURATION,
    );
    
    assert_eq!(&ix.data[8..16], &bronze_amount.to_le_bytes());
}

#[tokio::test]
async fn test_stake_platinum_tier_amount() {
    let ctx = TestContext::new().await;
    
    let (pool_pda, _bump) = ctx.get_pool_pda();
    let user = Keypair::new();
    let (user_stake_pda, _) = ctx.get_user_stake_pda(&user.pubkey());
    let platinum_amount = 100_000 * 1_000_000_000u64;
    
    let ix = create_stake_ix(
        &ctx.program_id,
        &user.pubkey(),
        &pool_pda,
        &user_stake_pda,
        platinum_amount,
        MIN_LOCK_DURATION,
    );
    
    assert_eq!(&ix.data[8..16], &platinum_amount.to_le_bytes());
}

