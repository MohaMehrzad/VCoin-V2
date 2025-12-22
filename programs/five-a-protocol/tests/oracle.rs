//! Integration tests for five-a-protocol oracle instructions

mod common;

use common::*;
use solana_sdk::signature::{Keypair, Signer};

#[tokio::test]
async fn test_register_oracle_instruction_format() {
    let ctx = TestContext::new().await;
    
    let (config_pda, _bump) = ctx.get_config_pda();
    let oracle = Keypair::new();
    let (oracle_pda, _) = ctx.get_oracle_pda(&oracle.pubkey());
    
    let ix = create_register_oracle_ix(
        &ctx.program_id,
        &ctx.payer.pubkey(),
        &config_pda,
        &oracle.pubkey(),
        &oracle_pda,
        "TestOracle",
    );
    
    assert_eq!(ix.program_id, ctx.program_id);
    assert_eq!(ix.accounts.len(), 5);
}

#[tokio::test]
async fn test_register_oracle_short_name() {
    let ctx = TestContext::new().await;
    
    let (config_pda, _bump) = ctx.get_config_pda();
    let oracle = Keypair::new();
    let (oracle_pda, _) = ctx.get_oracle_pda(&oracle.pubkey());
    
    let ix = create_register_oracle_ix(
        &ctx.program_id,
        &ctx.payer.pubkey(),
        &config_pda,
        &oracle.pubkey(),
        &oracle_pda,
        "A",
    );
    
    assert!(ix.data.len() > 8);
}

#[tokio::test]
async fn test_register_oracle_long_name() {
    let ctx = TestContext::new().await;
    
    let (config_pda, _bump) = ctx.get_config_pda();
    let oracle = Keypair::new();
    let (oracle_pda, _) = ctx.get_oracle_pda(&oracle.pubkey());
    
    let long_name = "VeryLongOracleNameThatIsQuiteLong";
    let ix = create_register_oracle_ix(
        &ctx.program_id,
        &ctx.payer.pubkey(),
        &config_pda,
        &oracle.pubkey(),
        &oracle_pda,
        long_name,
    );
    
    assert!(ix.data.len() > 8 + 4);
}

#[tokio::test]
async fn test_submit_score_instruction_format() {
    let ctx = TestContext::new().await;
    
    let (config_pda, _bump) = ctx.get_config_pda();
    let oracle = Keypair::new();
    let user = Keypair::new();
    let (user_score_pda, _) = ctx.get_user_score_pda(&user.pubkey());
    
    let ix = create_submit_score_ix(
        &ctx.program_id,
        &oracle.pubkey(),
        &config_pda,
        &user_score_pda,
        8000, // authenticity
        7500, // accuracy
        8500, // agility
        9000, // activity
        7000, // approved
    );
    
    assert_eq!(ix.program_id, ctx.program_id);
    assert_eq!(ix.accounts.len(), 3);
    assert!(ix.accounts[0].is_signer);
}

#[tokio::test]
async fn test_submit_score_max_values() {
    let ctx = TestContext::new().await;
    
    let (config_pda, _bump) = ctx.get_config_pda();
    let oracle = Keypair::new();
    let user = Keypair::new();
    let (user_score_pda, _) = ctx.get_user_score_pda(&user.pubkey());
    
    let ix = create_submit_score_ix(
        &ctx.program_id,
        &oracle.pubkey(),
        &config_pda,
        &user_score_pda,
        10000, // max
        10000,
        10000,
        10000,
        10000,
    );
    
    assert_eq!(&ix.data[8..10], &10000u16.to_le_bytes());
}

#[tokio::test]
async fn test_submit_score_zero_values() {
    let ctx = TestContext::new().await;
    
    let (config_pda, _bump) = ctx.get_config_pda();
    let oracle = Keypair::new();
    let user = Keypair::new();
    let (user_score_pda, _) = ctx.get_user_score_pda(&user.pubkey());
    
    let ix = create_submit_score_ix(
        &ctx.program_id,
        &oracle.pubkey(),
        &config_pda,
        &user_score_pda,
        0, 0, 0, 0, 0,
    );
    
    assert_eq!(&ix.data[8..10], &0u16.to_le_bytes());
}

