//! Integration tests for governance-protocol vote instructions

mod common;

use common::*;
use solana_sdk::signature::{Keypair, Signer};

#[tokio::test]
async fn test_cast_vote_instruction_format() {
    let ctx = TestContext::new().await;
    
    let (config_pda, _bump) = ctx.get_config_pda();
    let voter = Keypair::new();
    let proposal = Keypair::new();
    let (vote_record_pda, _) = ctx.get_vote_record_pda(&proposal.pubkey(), &voter.pubkey());
    
    let ix = create_cast_vote_ix(
        &ctx.program_id,
        &voter.pubkey(),
        &proposal.pubkey(),
        &vote_record_pda,
        &config_pda,
        1, // for
        1000,
        8000,
        1, // bronze
    );
    
    assert_eq!(ix.program_id, ctx.program_id);
    assert_eq!(ix.accounts.len(), 5);
    assert!(ix.accounts[0].is_signer);
}

#[tokio::test]
async fn test_cast_vote_for() {
    let ctx = TestContext::new().await;
    
    let (config_pda, _bump) = ctx.get_config_pda();
    let voter = Keypair::new();
    let proposal = Keypair::new();
    let (vote_record_pda, _) = ctx.get_vote_record_pda(&proposal.pubkey(), &voter.pubkey());
    
    let ix = create_cast_vote_ix(
        &ctx.program_id,
        &voter.pubkey(),
        &proposal.pubkey(),
        &vote_record_pda,
        &config_pda,
        1, // for
        1000,
        8000,
        1,
    );
    
    assert_eq!(ix.data[8], 1);
}

#[tokio::test]
async fn test_cast_vote_against() {
    let ctx = TestContext::new().await;
    
    let (config_pda, _bump) = ctx.get_config_pda();
    let voter = Keypair::new();
    let proposal = Keypair::new();
    let (vote_record_pda, _) = ctx.get_vote_record_pda(&proposal.pubkey(), &voter.pubkey());
    
    let ix = create_cast_vote_ix(
        &ctx.program_id,
        &voter.pubkey(),
        &proposal.pubkey(),
        &vote_record_pda,
        &config_pda,
        2, // against
        1000,
        8000,
        1,
    );
    
    assert_eq!(ix.data[8], 2);
}

#[tokio::test]
async fn test_cast_vote_abstain() {
    let ctx = TestContext::new().await;
    
    let (config_pda, _bump) = ctx.get_config_pda();
    let voter = Keypair::new();
    let proposal = Keypair::new();
    let (vote_record_pda, _) = ctx.get_vote_record_pda(&proposal.pubkey(), &voter.pubkey());
    
    let ix = create_cast_vote_ix(
        &ctx.program_id,
        &voter.pubkey(),
        &proposal.pubkey(),
        &vote_record_pda,
        &config_pda,
        0, // abstain
        1000,
        8000,
        1,
    );
    
    assert_eq!(ix.data[8], 0);
}

#[tokio::test]
async fn test_cast_vote_different_tiers() {
    let ctx = TestContext::new().await;
    
    let (config_pda, _bump) = ctx.get_config_pda();
    let proposal = Keypair::new();
    
    for tier in 0..=4 {
        let voter = Keypair::new();
        let (vote_record_pda, _) = ctx.get_vote_record_pda(&proposal.pubkey(), &voter.pubkey());
        
        let ix = create_cast_vote_ix(
            &ctx.program_id,
            &voter.pubkey(),
            &proposal.pubkey(),
            &vote_record_pda,
            &config_pda,
            1,
            1000,
            8000,
            tier,
        );
        
        assert_eq!(ix.data[19], tier);
    }
}

#[tokio::test]
async fn test_cast_vote_different_scores() {
    let ctx = TestContext::new().await;
    
    let (config_pda, _bump) = ctx.get_config_pda();
    let proposal = Keypair::new();
    
    for score in [0u16, 5000, 8000, 10000] {
        let voter = Keypair::new();
        let (vote_record_pda, _) = ctx.get_vote_record_pda(&proposal.pubkey(), &voter.pubkey());
        
        let ix = create_cast_vote_ix(
            &ctx.program_id,
            &voter.pubkey(),
            &proposal.pubkey(),
            &vote_record_pda,
            &config_pda,
            1,
            1000,
            score,
            1,
        );
        
        assert_eq!(&ix.data[17..19], &score.to_le_bytes());
    }
}

#[tokio::test]
async fn test_cast_vote_different_balances() {
    let ctx = TestContext::new().await;
    
    let (config_pda, _bump) = ctx.get_config_pda();
    let proposal = Keypair::new();
    
    for balance in [1u64, 1000, 100_000, 1_000_000] {
        let voter = Keypair::new();
        let (vote_record_pda, _) = ctx.get_vote_record_pda(&proposal.pubkey(), &voter.pubkey());
        
        let ix = create_cast_vote_ix(
            &ctx.program_id,
            &voter.pubkey(),
            &proposal.pubkey(),
            &vote_record_pda,
            &config_pda,
            1,
            balance,
            8000,
            1,
        );
        
        assert_eq!(&ix.data[9..17], &balance.to_le_bytes());
    }
}

#[tokio::test]
async fn test_cast_vote_multiple_voters() {
    let ctx = TestContext::new().await;
    
    let (config_pda, _bump) = ctx.get_config_pda();
    let proposal = Keypair::new();
    
    for i in 0..10 {
        let voter = Keypair::new();
        let (vote_record_pda, _) = ctx.get_vote_record_pda(&proposal.pubkey(), &voter.pubkey());
        
        let ix = create_cast_vote_ix(
            &ctx.program_id,
            &voter.pubkey(),
            &proposal.pubkey(),
            &vote_record_pda,
            &config_pda,
            (i % 3) as u8,
            (i + 1) * 100,
            8000,
            1,
        );
        
        assert_eq!(ix.accounts[0].pubkey, voter.pubkey());
    }
}

