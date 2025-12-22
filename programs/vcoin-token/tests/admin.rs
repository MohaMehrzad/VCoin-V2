//! Integration tests for vcoin-token admin instructions
//!
//! Tests set_paused, update_authority, update_permanent_delegate

mod common;

use common::*;
use solana_sdk::signature::{Keypair, Signer};

// ============================================================================
// set_paused tests
// ============================================================================

/// Test set_paused to true
#[tokio::test]
async fn test_set_paused_true() {
    let mut ctx = TestContext::new().await;
    
    let (config_pda, _bump) = ctx.get_config_pda();
    let mint = Keypair::new();
    let treasury = Keypair::new();
    let permanent_delegate = Keypair::new();
    
    // Initialize
    let init_ix = create_initialize_mint_ix(
        &ctx.program_id,
        &ctx.payer.pubkey(),
        &config_pda,
        &mint.pubkey(),
        &treasury.pubkey(),
        &permanent_delegate.pubkey(),
    );
    
    ctx.process_transaction(&[init_ix], &[]).await.unwrap();
    
    // Pause
    let pause_ix = create_set_paused_ix(
        &ctx.program_id,
        &ctx.payer.pubkey(),
        &config_pda,
        true,
    );
    
    let result = ctx.process_transaction(&[pause_ix], &[]).await;
    assert!(result.is_ok(), "Setting paused should succeed");
}

/// Test set_paused to false (unpause)
#[tokio::test]
async fn test_set_paused_false() {
    let mut ctx = TestContext::new().await;
    
    let (config_pda, _bump) = ctx.get_config_pda();
    let mint = Keypair::new();
    let treasury = Keypair::new();
    let permanent_delegate = Keypair::new();
    
    // Initialize
    let init_ix = create_initialize_mint_ix(
        &ctx.program_id,
        &ctx.payer.pubkey(),
        &config_pda,
        &mint.pubkey(),
        &treasury.pubkey(),
        &permanent_delegate.pubkey(),
    );
    
    ctx.process_transaction(&[init_ix], &[]).await.unwrap();
    
    // Pause first
    let pause_ix = create_set_paused_ix(
        &ctx.program_id,
        &ctx.payer.pubkey(),
        &config_pda,
        true,
    );
    ctx.process_transaction(&[pause_ix], &[]).await.unwrap();
    
    // Unpause
    let unpause_ix = create_set_paused_ix(
        &ctx.program_id,
        &ctx.payer.pubkey(),
        &config_pda,
        false,
    );
    
    let result = ctx.process_transaction(&[unpause_ix], &[]).await;
    assert!(result.is_ok(), "Setting unpaused should succeed");
}

/// Test set_paused unauthorized
#[tokio::test]
async fn test_set_paused_unauthorized() {
    let mut ctx = TestContext::new().await;
    
    let (config_pda, _bump) = ctx.get_config_pda();
    let mint = Keypair::new();
    let treasury = Keypair::new();
    let permanent_delegate = Keypair::new();
    
    // Initialize
    let init_ix = create_initialize_mint_ix(
        &ctx.program_id,
        &ctx.payer.pubkey(),
        &config_pda,
        &mint.pubkey(),
        &treasury.pubkey(),
        &permanent_delegate.pubkey(),
    );
    
    ctx.process_transaction(&[init_ix], &[]).await.unwrap();
    
    // Try pausing with unauthorized signer
    let unauthorized = Keypair::new();
    let pause_ix = create_set_paused_ix(
        &ctx.program_id,
        &unauthorized.pubkey(),
        &config_pda,
        true,
    );
    
    // Verify instruction uses unauthorized signer
    assert_eq!(pause_ix.accounts[0].pubkey, unauthorized.pubkey());
}

// ============================================================================
// update_authority tests
// ============================================================================

/// Test update_authority success
#[tokio::test]
async fn test_update_authority_success() {
    let mut ctx = TestContext::new().await;
    
    let (config_pda, _bump) = ctx.get_config_pda();
    let mint = Keypair::new();
    let treasury = Keypair::new();
    let permanent_delegate = Keypair::new();
    
    // Initialize
    let init_ix = create_initialize_mint_ix(
        &ctx.program_id,
        &ctx.payer.pubkey(),
        &config_pda,
        &mint.pubkey(),
        &treasury.pubkey(),
        &permanent_delegate.pubkey(),
    );
    
    ctx.process_transaction(&[init_ix], &[]).await.unwrap();
    
    // Update authority
    let new_authority = Keypair::new();
    let update_ix = create_update_authority_ix(
        &ctx.program_id,
        &ctx.payer.pubkey(),
        &config_pda,
        &new_authority.pubkey(),
    );
    
    let result = ctx.process_transaction(&[update_ix], &[]).await;
    assert!(result.is_ok(), "Update authority should succeed");
}

/// Test update_authority unauthorized
#[tokio::test]
async fn test_update_authority_unauthorized() {
    let mut ctx = TestContext::new().await;
    
    let (config_pda, _bump) = ctx.get_config_pda();
    let mint = Keypair::new();
    let treasury = Keypair::new();
    let permanent_delegate = Keypair::new();
    
    // Initialize
    let init_ix = create_initialize_mint_ix(
        &ctx.program_id,
        &ctx.payer.pubkey(),
        &config_pda,
        &mint.pubkey(),
        &treasury.pubkey(),
        &permanent_delegate.pubkey(),
    );
    
    ctx.process_transaction(&[init_ix], &[]).await.unwrap();
    
    // Try updating with wrong authority
    let unauthorized = Keypair::new();
    let new_authority = Keypair::new();
    let update_ix = create_update_authority_ix(
        &ctx.program_id,
        &unauthorized.pubkey(),
        &config_pda,
        &new_authority.pubkey(),
    );
    
    // Verify instruction uses unauthorized signer
    assert_eq!(update_ix.accounts[0].pubkey, unauthorized.pubkey());
}

/// Test update_authority to self
#[tokio::test]
async fn test_update_authority_to_self() {
    let mut ctx = TestContext::new().await;
    
    let (config_pda, _bump) = ctx.get_config_pda();
    let mint = Keypair::new();
    let treasury = Keypair::new();
    let permanent_delegate = Keypair::new();
    
    // Initialize
    let init_ix = create_initialize_mint_ix(
        &ctx.program_id,
        &ctx.payer.pubkey(),
        &config_pda,
        &mint.pubkey(),
        &treasury.pubkey(),
        &permanent_delegate.pubkey(),
    );
    
    ctx.process_transaction(&[init_ix], &[]).await.unwrap();
    
    // Update authority to same value
    let update_ix = create_update_authority_ix(
        &ctx.program_id,
        &ctx.payer.pubkey(),
        &config_pda,
        &ctx.payer.pubkey(), // Same as current
    );
    
    let result = ctx.process_transaction(&[update_ix], &[]).await;
    assert!(result.is_ok(), "Update authority to self should succeed");
}

// ============================================================================
// update_permanent_delegate tests
// ============================================================================

/// Test update_permanent_delegate success
#[tokio::test]
async fn test_update_delegate_success() {
    let mut ctx = TestContext::new().await;
    
    let (config_pda, _bump) = ctx.get_config_pda();
    let mint = Keypair::new();
    let treasury = Keypair::new();
    let permanent_delegate = Keypair::new();
    
    // Initialize
    let init_ix = create_initialize_mint_ix(
        &ctx.program_id,
        &ctx.payer.pubkey(),
        &config_pda,
        &mint.pubkey(),
        &treasury.pubkey(),
        &permanent_delegate.pubkey(),
    );
    
    ctx.process_transaction(&[init_ix], &[]).await.unwrap();
    
    // Update delegate
    let new_delegate = Keypair::new();
    let update_ix = create_update_delegate_ix(
        &ctx.program_id,
        &ctx.payer.pubkey(),
        &config_pda,
        &new_delegate.pubkey(),
    );
    
    let result = ctx.process_transaction(&[update_ix], &[]).await;
    assert!(result.is_ok(), "Update delegate should succeed");
}

/// Test update_permanent_delegate unauthorized
#[tokio::test]
async fn test_update_delegate_unauthorized() {
    let mut ctx = TestContext::new().await;
    
    let (config_pda, _bump) = ctx.get_config_pda();
    let mint = Keypair::new();
    let treasury = Keypair::new();
    let permanent_delegate = Keypair::new();
    
    // Initialize
    let init_ix = create_initialize_mint_ix(
        &ctx.program_id,
        &ctx.payer.pubkey(),
        &config_pda,
        &mint.pubkey(),
        &treasury.pubkey(),
        &permanent_delegate.pubkey(),
    );
    
    ctx.process_transaction(&[init_ix], &[]).await.unwrap();
    
    // Try updating with wrong authority
    let unauthorized = Keypair::new();
    let new_delegate = Keypair::new();
    let update_ix = create_update_delegate_ix(
        &ctx.program_id,
        &unauthorized.pubkey(),
        &config_pda,
        &new_delegate.pubkey(),
    );
    
    // Verify instruction uses unauthorized signer
    assert_eq!(update_ix.accounts[0].pubkey, unauthorized.pubkey());
}

/// Test update_permanent_delegate multiple times
#[tokio::test]
async fn test_update_delegate_multiple_times() {
    let mut ctx = TestContext::new().await;
    
    let (config_pda, _bump) = ctx.get_config_pda();
    let mint = Keypair::new();
    let treasury = Keypair::new();
    let permanent_delegate = Keypair::new();
    
    // Initialize
    let init_ix = create_initialize_mint_ix(
        &ctx.program_id,
        &ctx.payer.pubkey(),
        &config_pda,
        &mint.pubkey(),
        &treasury.pubkey(),
        &permanent_delegate.pubkey(),
    );
    
    ctx.process_transaction(&[init_ix], &[]).await.unwrap();
    
    // Update delegate multiple times
    for i in 0..3 {
        let new_delegate = Keypair::new();
        let update_ix = create_update_delegate_ix(
            &ctx.program_id,
            &ctx.payer.pubkey(),
            &config_pda,
            &new_delegate.pubkey(),
        );
        
        let result = ctx.process_transaction(&[update_ix], &[]).await;
        assert!(result.is_ok(), "Update delegate #{} should succeed", i + 1);
    }
}

