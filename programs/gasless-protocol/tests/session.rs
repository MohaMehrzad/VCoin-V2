//! Integration tests for gasless-protocol session key instructions

mod common;

use common::*;
use solana_sdk::signature::{Keypair, Signer};

const SESSION_DURATION: i64 = 24 * 60 * 60;

#[tokio::test]
async fn test_create_session_key_instruction_format() {
    let ctx = TestContext::new().await;
    
    let (config_pda, _bump) = ctx.get_config_pda();
    let user = Keypair::new();
    let session_keypair = Keypair::new();
    let (session_pda, _) = ctx.get_session_pda(&user.pubkey(), &session_keypair.pubkey());
    
    let ix = create_session_key_ix(
        &ctx.program_id,
        &user.pubkey(),
        &config_pda,
        &session_pda,
        &session_keypair.pubkey(),
        0xFFFF, // all scopes
        SESSION_DURATION,
        1000,
        100_000_000_000_000,
        0,
    );
    
    assert_eq!(ix.program_id, ctx.program_id);
    assert_eq!(ix.accounts.len(), 4);
}

#[tokio::test]
async fn test_create_session_key_different_scopes() {
    let ctx = TestContext::new().await;
    
    let (config_pda, _bump) = ctx.get_config_pda();
    let user = Keypair::new();
    
    for scope in [0x01u16, 0x03, 0x0F, 0xFF, 0xFFFF] {
        let session_keypair = Keypair::new();
        let (session_pda, _) = ctx.get_session_pda(&user.pubkey(), &session_keypair.pubkey());
        
        let ix = create_session_key_ix(
            &ctx.program_id,
            &user.pubkey(),
            &config_pda,
            &session_pda,
            &session_keypair.pubkey(),
            scope,
            SESSION_DURATION,
            1000,
            100_000_000_000_000,
            0,
        );
        
        assert_eq!(&ix.data[40..42], &scope.to_le_bytes());
    }
}

#[tokio::test]
async fn test_create_session_key_different_durations() {
    let ctx = TestContext::new().await;
    
    let (config_pda, _bump) = ctx.get_config_pda();
    let user = Keypair::new();
    
    for duration in [3600i64, 7200, 86400, 172800] {
        let session_keypair = Keypair::new();
        let (session_pda, _) = ctx.get_session_pda(&user.pubkey(), &session_keypair.pubkey());
        
        let ix = create_session_key_ix(
            &ctx.program_id,
            &user.pubkey(),
            &config_pda,
            &session_pda,
            &session_keypair.pubkey(),
            0xFFFF,
            duration,
            1000,
            100_000_000_000_000,
            0,
        );
        
        assert_eq!(&ix.data[42..50], &duration.to_le_bytes());
    }
}

#[tokio::test]
async fn test_create_session_key_different_fee_methods() {
    let ctx = TestContext::new().await;
    
    let (config_pda, _bump) = ctx.get_config_pda();
    let user = Keypair::new();
    
    for fee_method in 0..=2 {
        let session_keypair = Keypair::new();
        let (session_pda, _) = ctx.get_session_pda(&user.pubkey(), &session_keypair.pubkey());
        
        let ix = create_session_key_ix(
            &ctx.program_id,
            &user.pubkey(),
            &config_pda,
            &session_pda,
            &session_keypair.pubkey(),
            0xFFFF,
            SESSION_DURATION,
            1000,
            100_000_000_000_000,
            fee_method,
        );
        
        assert_eq!(ix.data[62], fee_method);
    }
}

#[tokio::test]
async fn test_execute_session_action_instruction_format() {
    let ctx = TestContext::new().await;
    
    let (config_pda, _bump) = ctx.get_config_pda();
    let user = Keypair::new();
    let session = Keypair::new();
    
    let ix = create_execute_session_action_ix(
        &ctx.program_id,
        &user.pubkey(),
        &config_pda,
        &session.pubkey(),
        1, // tip action
        1_000_000_000,
    );
    
    assert_eq!(ix.program_id, ctx.program_id);
    assert_eq!(ix.accounts.len(), 3);
}

#[tokio::test]
async fn test_execute_session_action_different_types() {
    let ctx = TestContext::new().await;
    
    let (config_pda, _bump) = ctx.get_config_pda();
    let user = Keypair::new();
    let session = Keypair::new();
    
    for action_type in 0..=7 {
        let ix = create_execute_session_action_ix(
            &ctx.program_id,
            &user.pubkey(),
            &config_pda,
            &session.pubkey(),
            action_type,
            1_000_000_000,
        );
        
        assert_eq!(&ix.data[8..10], &action_type.to_le_bytes());
    }
}

#[tokio::test]
async fn test_execute_session_action_different_amounts() {
    let ctx = TestContext::new().await;
    
    let (config_pda, _bump) = ctx.get_config_pda();
    let user = Keypair::new();
    let session = Keypair::new();
    
    for amount in [0u64, 1_000_000_000, 10_000_000_000, 100_000_000_000] {
        let ix = create_execute_session_action_ix(
            &ctx.program_id,
            &user.pubkey(),
            &config_pda,
            &session.pubkey(),
            1,
            amount,
        );
        
        assert_eq!(&ix.data[10..18], &amount.to_le_bytes());
    }
}

