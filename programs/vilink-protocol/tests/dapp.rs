//! Integration tests for vilink-protocol dapp instructions

mod common;

use common::*;
use solana_sdk::signature::{Keypair, Signer};

#[tokio::test]
async fn test_register_dapp_instruction_format() {
    let ctx = TestContext::new().await;
    
    let (config_pda, _bump) = ctx.get_config_pda();
    let dapp_authority = Keypair::new();
    let (dapp_pda, _) = ctx.get_dapp_pda(&dapp_authority.pubkey());
    let name = [b'T', b'e', b's', b't', 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
    
    let ix = create_register_dapp_ix(
        &ctx.program_id,
        &ctx.payer.pubkey(),
        &config_pda,
        &dapp_pda,
        &dapp_authority.pubkey(),
        name,
        0xFF,
        250,
    );
    
    assert_eq!(ix.program_id, ctx.program_id);
    assert_eq!(ix.accounts.len(), 5);
}

#[tokio::test]
async fn test_register_dapp_different_allowed_actions() {
    let ctx = TestContext::new().await;
    
    let (config_pda, _bump) = ctx.get_config_pda();
    
    for allowed_actions in [0x01u8, 0x03, 0x0F, 0xFF] {
        let dapp_authority = Keypair::new();
        let (dapp_pda, _) = ctx.get_dapp_pda(&dapp_authority.pubkey());
        let name = [0u8; 32];
        
        let ix = create_register_dapp_ix(
            &ctx.program_id,
            &ctx.payer.pubkey(),
            &config_pda,
            &dapp_pda,
            &dapp_authority.pubkey(),
            name,
            allowed_actions,
            250,
        );
        
        assert_eq!(ix.data[72], allowed_actions);
    }
}

#[tokio::test]
async fn test_register_dapp_different_fee_shares() {
    let ctx = TestContext::new().await;
    
    let (config_pda, _bump) = ctx.get_config_pda();
    
    for fee_share in [0u16, 100, 250, 500, 1000] {
        let dapp_authority = Keypair::new();
        let (dapp_pda, _) = ctx.get_dapp_pda(&dapp_authority.pubkey());
        let name = [0u8; 32];
        
        let ix = create_register_dapp_ix(
            &ctx.program_id,
            &ctx.payer.pubkey(),
            &config_pda,
            &dapp_pda,
            &dapp_authority.pubkey(),
            name,
            0xFF,
            fee_share,
        );
        
        assert_eq!(&ix.data[73..75], &fee_share.to_le_bytes());
    }
}

#[tokio::test]
async fn test_register_dapp_multiple_dapps() {
    let ctx = TestContext::new().await;
    
    let (config_pda, _bump) = ctx.get_config_pda();
    
    for i in 0..5 {
        let dapp_authority = Keypair::new();
        let (dapp_pda, _) = ctx.get_dapp_pda(&dapp_authority.pubkey());
        let mut name = [0u8; 32];
        name[0] = b'D';
        name[1] = b'a';
        name[2] = b'p';
        name[3] = b'p';
        name[4] = b'0' + i;
        
        let ix = create_register_dapp_ix(
            &ctx.program_id,
            &ctx.payer.pubkey(),
            &config_pda,
            &dapp_pda,
            &dapp_authority.pubkey(),
            name,
            0xFF,
            250,
        );
        
        assert_eq!(ix.accounts[2].pubkey, dapp_authority.pubkey());
    }
}

