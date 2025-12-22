//! Integration tests for five-a-protocol vouch instructions

mod common;

use common::*;
use solana_sdk::signature::{Keypair, Signer};

#[tokio::test]
async fn test_vouch_for_user_instruction_format() {
    let ctx = TestContext::new().await;
    
    let (config_pda, _bump) = ctx.get_config_pda();
    let voucher = Keypair::new();
    let vouchee = Keypair::new();
    
    let ix = create_vouch_for_user_ix(
        &ctx.program_id,
        &voucher.pubkey(),
        &vouchee.pubkey(),
        &config_pda,
    );
    
    assert_eq!(ix.program_id, ctx.program_id);
    assert_eq!(ix.accounts.len(), 4);
    assert!(ix.accounts[0].is_signer);
}

#[tokio::test]
async fn test_vouch_different_voucher_vouchee() {
    let ctx = TestContext::new().await;
    
    let (config_pda, _bump) = ctx.get_config_pda();
    let voucher = Keypair::new();
    let vouchee = Keypair::new();
    
    let ix = create_vouch_for_user_ix(
        &ctx.program_id,
        &voucher.pubkey(),
        &vouchee.pubkey(),
        &config_pda,
    );
    
    assert_ne!(ix.accounts[0].pubkey, ix.accounts[1].pubkey);
}

#[tokio::test]
async fn test_vouch_same_voucher_vouchee() {
    let ctx = TestContext::new().await;
    
    let (config_pda, _bump) = ctx.get_config_pda();
    let same_user = Keypair::new();
    
    let ix = create_vouch_for_user_ix(
        &ctx.program_id,
        &same_user.pubkey(),
        &same_user.pubkey(),
        &config_pda,
    );
    
    assert_eq!(ix.accounts[0].pubkey, ix.accounts[1].pubkey);
}

#[tokio::test]
async fn test_vouch_multiple_vouchers() {
    let ctx = TestContext::new().await;
    
    let (config_pda, _bump) = ctx.get_config_pda();
    let vouchee = Keypair::new();
    
    for _ in 0..3 {
        let voucher = Keypair::new();
        let ix = create_vouch_for_user_ix(
            &ctx.program_id,
            &voucher.pubkey(),
            &vouchee.pubkey(),
            &config_pda,
        );
        
        assert_eq!(ix.accounts[1].pubkey, vouchee.pubkey());
    }
}

#[tokio::test]
async fn test_vouch_one_voucher_multiple_vouchees() {
    let ctx = TestContext::new().await;
    
    let (config_pda, _bump) = ctx.get_config_pda();
    let voucher = Keypair::new();
    
    for _ in 0..5 {
        let vouchee = Keypair::new();
        let ix = create_vouch_for_user_ix(
            &ctx.program_id,
            &voucher.pubkey(),
            &vouchee.pubkey(),
            &config_pda,
        );
        
        assert_eq!(ix.accounts[0].pubkey, voucher.pubkey());
    }
}

