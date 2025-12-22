//! Integration tests for sscre-protocol claim instructions

mod common;

use common::*;
use solana_sdk::signature::{Keypair, Signer};

#[tokio::test]
async fn test_claim_rewards_instruction_format() {
    let ctx = TestContext::new().await;
    
    let (pool_pda, _bump) = ctx.get_pool_pda();
    let (epoch_pda, _) = ctx.get_epoch_pda(1);
    let user = Keypair::new();
    let (user_claim_pda, _) = ctx.get_user_claim_pda(&user.pubkey());
    let (circuit_breaker_pda, _) = ctx.get_circuit_breaker_pda();
    let amount = 1000 * 1_000_000_000u64;
    let merkle_proof = vec![[1u8; 32], [2u8; 32]];
    
    let ix = create_claim_rewards_ix(
        &ctx.program_id,
        &user.pubkey(),
        &pool_pda,
        &epoch_pda,
        &user_claim_pda,
        &circuit_breaker_pda,
        amount,
        merkle_proof,
    );
    
    assert_eq!(ix.program_id, ctx.program_id);
    assert_eq!(ix.accounts.len(), 7);
}

#[tokio::test]
async fn test_claim_rewards_different_amounts() {
    let ctx = TestContext::new().await;
    
    let (pool_pda, _bump) = ctx.get_pool_pda();
    let (epoch_pda, _) = ctx.get_epoch_pda(1);
    let (circuit_breaker_pda, _) = ctx.get_circuit_breaker_pda();
    
    for amount in [1u64, 1000, 50000, 100000] {
        let user = Keypair::new();
        let (user_claim_pda, _) = ctx.get_user_claim_pda(&user.pubkey());
        let amount_with_decimals = amount * 1_000_000_000;
        let merkle_proof = vec![[1u8; 32]];
        
        let ix = create_claim_rewards_ix(
            &ctx.program_id,
            &user.pubkey(),
            &pool_pda,
            &epoch_pda,
            &user_claim_pda,
            &circuit_breaker_pda,
            amount_with_decimals,
            merkle_proof,
        );
        
        assert_eq!(&ix.data[8..16], &amount_with_decimals.to_le_bytes());
    }
}

#[tokio::test]
async fn test_claim_rewards_empty_proof() {
    let ctx = TestContext::new().await;
    
    let (pool_pda, _bump) = ctx.get_pool_pda();
    let (epoch_pda, _) = ctx.get_epoch_pda(1);
    let user = Keypair::new();
    let (user_claim_pda, _) = ctx.get_user_claim_pda(&user.pubkey());
    let (circuit_breaker_pda, _) = ctx.get_circuit_breaker_pda();
    let merkle_proof: Vec<[u8; 32]> = vec![];
    
    let ix = create_claim_rewards_ix(
        &ctx.program_id,
        &user.pubkey(),
        &pool_pda,
        &epoch_pda,
        &user_claim_pda,
        &circuit_breaker_pda,
        1000,
        merkle_proof,
    );
    
    assert_eq!(&ix.data[16..20], &0u32.to_le_bytes());
}

#[tokio::test]
async fn test_claim_rewards_deep_proof() {
    let ctx = TestContext::new().await;
    
    let (pool_pda, _bump) = ctx.get_pool_pda();
    let (epoch_pda, _) = ctx.get_epoch_pda(1);
    let user = Keypair::new();
    let (user_claim_pda, _) = ctx.get_user_claim_pda(&user.pubkey());
    let (circuit_breaker_pda, _) = ctx.get_circuit_breaker_pda();
    let merkle_proof: Vec<[u8; 32]> = (0..20).map(|i| [i as u8; 32]).collect();
    
    let ix = create_claim_rewards_ix(
        &ctx.program_id,
        &user.pubkey(),
        &pool_pda,
        &epoch_pda,
        &user_claim_pda,
        &circuit_breaker_pda,
        1000,
        merkle_proof.clone(),
    );
    
    assert_eq!(&ix.data[16..20], &(merkle_proof.len() as u32).to_le_bytes());
}

#[tokio::test]
async fn test_claim_rewards_multiple_users() {
    let ctx = TestContext::new().await;
    
    let (pool_pda, _bump) = ctx.get_pool_pda();
    let (epoch_pda, _) = ctx.get_epoch_pda(1);
    let (circuit_breaker_pda, _) = ctx.get_circuit_breaker_pda();
    
    for i in 0..5 {
        let user = Keypair::new();
        let (user_claim_pda, _) = ctx.get_user_claim_pda(&user.pubkey());
        let merkle_proof = vec![[i as u8; 32]];
        
        let ix = create_claim_rewards_ix(
            &ctx.program_id,
            &user.pubkey(),
            &pool_pda,
            &epoch_pda,
            &user_claim_pda,
            &circuit_breaker_pda,
            (i + 1) as u64 * 1000,
            merkle_proof,
        );
        
        assert!(ix.accounts[4].is_signer);
    }
}

