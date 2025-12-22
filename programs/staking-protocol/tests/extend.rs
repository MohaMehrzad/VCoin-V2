//! Integration tests for staking-protocol extend_lock instruction

mod common;

use common::*;
use solana_sdk::signature::{Keypair, Signer};

const MIN_LOCK_DURATION: i64 = 7 * 24 * 60 * 60;
const MAX_LOCK_DURATION: i64 = 4 * 365 * 24 * 60 * 60;

#[tokio::test]
async fn test_extend_lock_instruction_format() {
    let ctx = TestContext::new().await;
    
    let (pool_pda, _bump) = ctx.get_pool_pda();
    let user = Keypair::new();
    let (user_stake_pda, _) = ctx.get_user_stake_pda(&user.pubkey());
    let new_lock_duration = 30 * 24 * 60 * 60i64; // 30 days
    
    let ix = create_extend_lock_ix(
        &ctx.program_id,
        &user.pubkey(),
        &pool_pda,
        &user_stake_pda,
        new_lock_duration,
    );
    
    assert_eq!(ix.program_id, ctx.program_id);
    assert_eq!(ix.accounts.len(), 3);
    assert!(ix.accounts[0].is_signer);
    assert_eq!(&ix.data[8..16], &new_lock_duration.to_le_bytes());
}

#[tokio::test]
async fn test_extend_lock_to_max() {
    let ctx = TestContext::new().await;
    
    let (pool_pda, _bump) = ctx.get_pool_pda();
    let user = Keypair::new();
    let (user_stake_pda, _) = ctx.get_user_stake_pda(&user.pubkey());
    
    let ix = create_extend_lock_ix(
        &ctx.program_id,
        &user.pubkey(),
        &pool_pda,
        &user_stake_pda,
        MAX_LOCK_DURATION,
    );
    
    assert_eq!(&ix.data[8..16], &MAX_LOCK_DURATION.to_le_bytes());
}

#[tokio::test]
async fn test_extend_lock_exceeds_max() {
    let ctx = TestContext::new().await;
    
    let (pool_pda, _bump) = ctx.get_pool_pda();
    let user = Keypair::new();
    let (user_stake_pda, _) = ctx.get_user_stake_pda(&user.pubkey());
    let too_long = MAX_LOCK_DURATION + 1;
    
    let ix = create_extend_lock_ix(
        &ctx.program_id,
        &user.pubkey(),
        &pool_pda,
        &user_stake_pda,
        too_long,
    );
    
    assert_eq!(&ix.data[8..16], &too_long.to_le_bytes());
}

#[tokio::test]
async fn test_extend_lock_double_duration() {
    let ctx = TestContext::new().await;
    
    let (pool_pda, _bump) = ctx.get_pool_pda();
    let user = Keypair::new();
    let (user_stake_pda, _) = ctx.get_user_stake_pda(&user.pubkey());
    let double = MIN_LOCK_DURATION * 2;
    
    let ix = create_extend_lock_ix(
        &ctx.program_id,
        &user.pubkey(),
        &pool_pda,
        &user_stake_pda,
        double,
    );
    
    assert_eq!(&ix.data[8..16], &double.to_le_bytes());
}

