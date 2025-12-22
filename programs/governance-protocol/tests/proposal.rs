//! Integration tests for governance-protocol proposal instructions

mod common;

use common::*;
use solana_sdk::signature::{Keypair, Signer};

#[tokio::test]
async fn test_create_proposal_instruction_format() {
    let ctx = TestContext::new().await;
    
    let (config_pda, _bump) = ctx.get_config_pda();
    let proposer = Keypair::new();
    let (proposal_pda, _) = ctx.get_proposal_pda(1);
    let title_hash = [1u8; 32];
    
    let ix = create_proposal_ix(
        &ctx.program_id,
        &proposer.pubkey(),
        &proposal_pda,
        &config_pda,
        title_hash,
        "ipfs://QmDescription",
        0,
        false,
    );
    
    assert_eq!(ix.program_id, ctx.program_id);
    assert_eq!(ix.accounts.len(), 4);
    assert!(ix.accounts[0].is_signer);
}

#[tokio::test]
async fn test_create_proposal_different_types() {
    let ctx = TestContext::new().await;
    
    let (config_pda, _bump) = ctx.get_config_pda();
    let proposer = Keypair::new();
    
    for proposal_type in 0..=4 {
        let (proposal_pda, _) = ctx.get_proposal_pda(proposal_type as u64);
        let title_hash = [proposal_type; 32];
        
        let ix = create_proposal_ix(
            &ctx.program_id,
            &proposer.pubkey(),
            &proposal_pda,
            &config_pda,
            title_hash,
            &format!("ipfs://proposal{}", proposal_type),
            proposal_type,
            false,
        );
        
        assert!(ix.data.len() > 8 + 32);
    }
}

#[tokio::test]
async fn test_create_proposal_with_private_voting() {
    let ctx = TestContext::new().await;
    
    let (config_pda, _bump) = ctx.get_config_pda();
    let proposer = Keypair::new();
    let (proposal_pda, _) = ctx.get_proposal_pda(1);
    let title_hash = [2u8; 32];
    
    let ix = create_proposal_ix(
        &ctx.program_id,
        &proposer.pubkey(),
        &proposal_pda,
        &config_pda,
        title_hash,
        "ipfs://QmPrivateProposal",
        0,
        true,
    );
    
    let last_byte = *ix.data.last().unwrap();
    assert_eq!(last_byte, 1); // private voting enabled
}

#[tokio::test]
async fn test_create_proposal_short_uri() {
    let ctx = TestContext::new().await;
    
    let (config_pda, _bump) = ctx.get_config_pda();
    let proposer = Keypair::new();
    let (proposal_pda, _) = ctx.get_proposal_pda(1);
    let title_hash = [3u8; 32];
    
    let ix = create_proposal_ix(
        &ctx.program_id,
        &proposer.pubkey(),
        &proposal_pda,
        &config_pda,
        title_hash,
        "a",
        0,
        false,
    );
    
    assert!(ix.data.len() > 8 + 32);
}

#[tokio::test]
async fn test_create_proposal_long_uri() {
    let ctx = TestContext::new().await;
    
    let (config_pda, _bump) = ctx.get_config_pda();
    let proposer = Keypair::new();
    let (proposal_pda, _) = ctx.get_proposal_pda(1);
    let title_hash = [4u8; 32];
    let long_uri = "ipfs://QmVeryLongDescriptionURIThatIsQuiteVeryLongIndeed";
    
    let ix = create_proposal_ix(
        &ctx.program_id,
        &proposer.pubkey(),
        &proposal_pda,
        &config_pda,
        title_hash,
        long_uri,
        0,
        false,
    );
    
    assert!(ix.data.len() > 8 + 32 + 4 + long_uri.len());
}

#[tokio::test]
async fn test_create_proposal_multiple_proposers() {
    let ctx = TestContext::new().await;
    
    let (config_pda, _bump) = ctx.get_config_pda();
    
    for i in 0..5 {
        let proposer = Keypair::new();
        let (proposal_pda, _) = ctx.get_proposal_pda(i);
        let mut title_hash = [0u8; 32];
        title_hash[0] = i as u8;
        
        let ix = create_proposal_ix(
            &ctx.program_id,
            &proposer.pubkey(),
            &proposal_pda,
            &config_pda,
            title_hash,
            &format!("ipfs://proposal{}", i),
            0,
            false,
        );
        
        assert_eq!(ix.accounts[0].pubkey, proposer.pubkey());
    }
}

