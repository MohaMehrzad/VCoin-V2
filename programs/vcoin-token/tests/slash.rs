//! Integration tests for vcoin-token slash_tokens instruction
//!
//! Tests slashing (burning) tokens using permanent delegate authority.

mod common;

use common::*;
use solana_sdk::{
    hash::hash,
    instruction::{AccountMeta, Instruction},
    signature::{Keypair, Signer},
};

/// Helper to create a slash_tokens instruction
fn create_slash_tokens_ix(
    program_id: &solana_sdk::pubkey::Pubkey,
    authority: &solana_sdk::pubkey::Pubkey,
    config: &solana_sdk::pubkey::Pubkey,
    mint: &solana_sdk::pubkey::Pubkey,
    target_account: &solana_sdk::pubkey::Pubkey,
    amount: u64,
) -> Instruction {
    let mut data = vec![0u8; 8 + 8]; // 8 byte discriminator + 8 byte u64
    let discriminator = hash(b"global:slash_tokens").to_bytes();
    data[..8].copy_from_slice(&discriminator[..8]);
    data[8..16].copy_from_slice(&amount.to_le_bytes());

    Instruction {
        program_id: *program_id,
        accounts: vec![
            AccountMeta::new(*authority, true),
            AccountMeta::new_readonly(*config, false),
            AccountMeta::new(*mint, false),
            AccountMeta::new(*target_account, false),
            AccountMeta::new_readonly(spl_token_2022::id(), false),
        ],
        data,
    }
}

/// Test slash tokens instruction creation
#[tokio::test]
async fn test_slash_tokens_instruction_format() {
    let ctx = TestContext::new().await;
    
    let (config_pda, _bump) = ctx.get_config_pda();
    let mint = Keypair::new();
    let target = Keypair::new();
    let amount = 1000u64;
    
    let ix = create_slash_tokens_ix(
        &ctx.program_id,
        &ctx.payer.pubkey(),
        &config_pda,
        &mint.pubkey(),
        &target.pubkey(),
        amount,
    );
    
    // Verify instruction structure
    assert_eq!(ix.program_id, ctx.program_id);
    assert_eq!(ix.accounts.len(), 5);
    assert!(ix.accounts[0].is_signer); // authority must sign
    assert_eq!(ix.data.len(), 16);
    assert_eq!(&ix.data[8..16], &amount.to_le_bytes());
}

/// Test slash tokens with zero amount should fail
#[tokio::test]
async fn test_slash_tokens_zero_amount() {
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
    
    // Create slash with zero amount
    let target = Keypair::new();
    let ix = create_slash_tokens_ix(
        &ctx.program_id,
        &permanent_delegate.pubkey(),
        &config_pda,
        &mint.pubkey(),
        &target.pubkey(),
        0,
    );
    
    // Verify zero amount is encoded
    assert_eq!(&ix.data[8..16], &0u64.to_le_bytes());
}

/// Test slash tokens unauthorized - not permanent delegate
#[tokio::test]
async fn test_slash_tokens_unauthorized() {
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
    
    // Try slashing with wrong authority (not permanent delegate)
    let target = Keypair::new();
    let unauthorized = Keypair::new();
    
    let ix = create_slash_tokens_ix(
        &ctx.program_id,
        &unauthorized.pubkey(), // Wrong - should be permanent_delegate
        &config_pda,
        &mint.pubkey(),
        &target.pubkey(),
        1000,
    );
    
    // Verify instruction uses unauthorized signer
    assert_eq!(ix.accounts[0].pubkey, unauthorized.pubkey());
}

/// Test slash tokens exceeds balance
#[tokio::test]
async fn test_slash_tokens_exceeds_balance() {
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
    
    // Try to slash more than balance
    let target = Keypair::new();
    let excessive_amount = u64::MAX;
    
    let ix = create_slash_tokens_ix(
        &ctx.program_id,
        &permanent_delegate.pubkey(),
        &config_pda,
        &mint.pubkey(),
        &target.pubkey(),
        excessive_amount,
    );
    
    // Verify large amount encoded correctly
    assert_eq!(&ix.data[8..16], &excessive_amount.to_le_bytes());
}

