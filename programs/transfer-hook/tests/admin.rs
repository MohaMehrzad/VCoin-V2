//! Integration tests for transfer-hook admin instructions

mod common;

use common::*;
use solana_sdk::signature::{Keypair, Signer};

#[tokio::test]
async fn test_set_paused_true() {
    let mut ctx = TestContext::new().await;
    
    let (config_pda, _bump) = ctx.get_config_pda();
    let five_a_program = Keypair::new();
    
    let init_ix = create_initialize_ix(
        &ctx.program_id,
        &ctx.payer.pubkey(),
        &config_pda,
        &five_a_program.pubkey(),
        1_000_000_000,
    );
    
    ctx.process_transaction(&[init_ix], &[]).await.unwrap();
    
    let pause_ix = create_set_paused_ix(
        &ctx.program_id,
        &ctx.payer.pubkey(),
        &config_pda,
        true,
    );
    
    let result = ctx.process_transaction(&[pause_ix], &[]).await;
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_set_paused_false() {
    let mut ctx = TestContext::new().await;
    
    let (config_pda, _bump) = ctx.get_config_pda();
    let five_a_program = Keypair::new();
    
    let init_ix = create_initialize_ix(
        &ctx.program_id,
        &ctx.payer.pubkey(),
        &config_pda,
        &five_a_program.pubkey(),
        1_000_000_000,
    );
    
    ctx.process_transaction(&[init_ix], &[]).await.unwrap();
    
    let pause_ix = create_set_paused_ix(&ctx.program_id, &ctx.payer.pubkey(), &config_pda, true);
    ctx.process_transaction(&[pause_ix], &[]).await.unwrap();
    
    let unpause_ix = create_set_paused_ix(&ctx.program_id, &ctx.payer.pubkey(), &config_pda, false);
    let result = ctx.process_transaction(&[unpause_ix], &[]).await;
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_set_paused_unauthorized() {
    let mut ctx = TestContext::new().await;
    
    let (config_pda, _bump) = ctx.get_config_pda();
    let five_a_program = Keypair::new();
    
    let init_ix = create_initialize_ix(
        &ctx.program_id,
        &ctx.payer.pubkey(),
        &config_pda,
        &five_a_program.pubkey(),
        1_000_000_000,
    );
    
    ctx.process_transaction(&[init_ix], &[]).await.unwrap();
    
    let unauthorized = Keypair::new();
    let pause_ix = create_set_paused_ix(
        &ctx.program_id,
        &unauthorized.pubkey(),
        &config_pda,
        true,
    );
    
    assert_eq!(pause_ix.accounts[0].pubkey, unauthorized.pubkey());
}

#[tokio::test]
async fn test_update_authority_success() {
    let mut ctx = TestContext::new().await;
    
    let (config_pda, _bump) = ctx.get_config_pda();
    let five_a_program = Keypair::new();
    
    let init_ix = create_initialize_ix(
        &ctx.program_id,
        &ctx.payer.pubkey(),
        &config_pda,
        &five_a_program.pubkey(),
        1_000_000_000,
    );
    
    ctx.process_transaction(&[init_ix], &[]).await.unwrap();
    
    let new_authority = Keypair::new();
    let update_ix = create_update_authority_ix(
        &ctx.program_id,
        &ctx.payer.pubkey(),
        &config_pda,
        &new_authority.pubkey(),
    );
    
    let result = ctx.process_transaction(&[update_ix], &[]).await;
    assert!(result.is_ok());
}

