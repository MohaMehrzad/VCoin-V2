//! Common test utilities for vevcoin-token integration tests

use anchor_lang::prelude::*;
use solana_program_test::*;
use solana_sdk::{
    account::Account,
    hash::Hash,
    instruction::{AccountMeta, Instruction},
    pubkey::Pubkey,
    signature::{Keypair, Signer},
    system_instruction,
    transaction::Transaction,
};

/// veVCoin configuration seed
pub const VEVCOIN_CONFIG_SEED: &[u8] = b"vevcoin-config";

/// Test context wrapper for vevcoin-token tests
pub struct TestContext {
    pub banks_client: BanksClient,
    pub payer: Keypair,
    pub recent_blockhash: Hash,
    pub program_id: Pubkey,
}

impl TestContext {
    pub async fn new() -> Self {
        let program_id = vevcoin_token::id();
        let program_test = ProgramTest::new(
            "vevcoin_token",
            program_id,
            processor!(vevcoin_token::entry),
        );

        let (banks_client, payer, recent_blockhash) = program_test.start().await;

        Self {
            banks_client,
            payer,
            recent_blockhash,
            program_id,
        }
    }

    pub async fn refresh_blockhash(&mut self) {
        self.recent_blockhash = self.banks_client.get_latest_blockhash().await.unwrap();
    }

    pub async fn get_account(&mut self, pubkey: Pubkey) -> Option<Account> {
        self.banks_client.get_account(pubkey).await.unwrap()
    }

    pub async fn airdrop(&mut self, pubkey: &Pubkey, lamports: u64) {
        let ix = system_instruction::transfer(&self.payer.pubkey(), pubkey, lamports);
        let tx = Transaction::new_signed_with_payer(
            &[ix],
            Some(&self.payer.pubkey()),
            &[&self.payer],
            self.recent_blockhash,
        );
        self.banks_client.process_transaction(tx).await.unwrap();
        self.refresh_blockhash().await;
    }

    pub fn get_config_pda(&self) -> (Pubkey, u8) {
        Pubkey::find_program_address(&[VEVCOIN_CONFIG_SEED], &self.program_id)
    }

    pub async fn process_transaction(
        &mut self,
        instructions: &[Instruction],
        signers: &[&Keypair],
    ) -> Result<(), BanksClientError> {
        let mut all_signers = vec![&self.payer];
        all_signers.extend(signers);
        
        let tx = Transaction::new_signed_with_payer(
            instructions,
            Some(&self.payer.pubkey()),
            &all_signers,
            self.recent_blockhash,
        );
        
        let result = self.banks_client.process_transaction(tx).await;
        self.refresh_blockhash().await;
        result
    }
}

/// Helper to create an initialize_mint instruction
pub fn create_initialize_mint_ix(
    program_id: &Pubkey,
    authority: &Pubkey,
    config: &Pubkey,
    mint: &Pubkey,
    staking_protocol: &Pubkey,
) -> Instruction {
    let mut data = vec![0u8; 8 + 32];
    let discriminator = anchor_lang::solana_program::hash::hash(b"global:initialize_mint").to_bytes();
    data[..8].copy_from_slice(&discriminator[..8]);
    data[8..40].copy_from_slice(staking_protocol.as_ref());

    Instruction {
        program_id: *program_id,
        accounts: vec![
            AccountMeta::new(*authority, true),
            AccountMeta::new(*config, false),
            AccountMeta::new(*mint, false),
            AccountMeta::new_readonly(spl_token_2022::id(), false),
            AccountMeta::new_readonly(solana_sdk::system_program::id(), false),
            AccountMeta::new_readonly(solana_sdk::sysvar::rent::id(), false),
        ],
        data,
    }
}

/// Helper to create a mint_vevcoin instruction
pub fn create_mint_vevcoin_ix(
    program_id: &Pubkey,
    staking_protocol: &Pubkey,
    config: &Pubkey,
    mint: &Pubkey,
    destination: &Pubkey,
    amount: u64,
) -> Instruction {
    let mut data = vec![0u8; 8 + 8];
    let discriminator = anchor_lang::solana_program::hash::hash(b"global:mint_vevcoin").to_bytes();
    data[..8].copy_from_slice(&discriminator[..8]);
    data[8..16].copy_from_slice(&amount.to_le_bytes());

    Instruction {
        program_id: *program_id,
        accounts: vec![
            AccountMeta::new(*staking_protocol, true),
            AccountMeta::new(*config, false),
            AccountMeta::new(*mint, false),
            AccountMeta::new(*destination, false),
            AccountMeta::new_readonly(spl_token_2022::id(), false),
        ],
        data,
    }
}

/// Helper to create a burn_vevcoin instruction
pub fn create_burn_vevcoin_ix(
    program_id: &Pubkey,
    staking_protocol: &Pubkey,
    config: &Pubkey,
    mint: &Pubkey,
    source: &Pubkey,
    amount: u64,
) -> Instruction {
    let mut data = vec![0u8; 8 + 8];
    let discriminator = anchor_lang::solana_program::hash::hash(b"global:burn_vevcoin").to_bytes();
    data[..8].copy_from_slice(&discriminator[..8]);
    data[8..16].copy_from_slice(&amount.to_le_bytes());

    Instruction {
        program_id: *program_id,
        accounts: vec![
            AccountMeta::new(*staking_protocol, true),
            AccountMeta::new(*config, false),
            AccountMeta::new(*mint, false),
            AccountMeta::new(*source, false),
            AccountMeta::new_readonly(spl_token_2022::id(), false),
        ],
        data,
    }
}

/// Helper to create an update_staking_protocol instruction
pub fn create_update_staking_protocol_ix(
    program_id: &Pubkey,
    authority: &Pubkey,
    config: &Pubkey,
    new_staking_protocol: &Pubkey,
) -> Instruction {
    let mut data = vec![0u8; 8 + 32];
    let discriminator = anchor_lang::solana_program::hash::hash(b"global:update_staking_protocol").to_bytes();
    data[..8].copy_from_slice(&discriminator[..8]);
    data[8..40].copy_from_slice(new_staking_protocol.as_ref());

    Instruction {
        program_id: *program_id,
        accounts: vec![
            AccountMeta::new(*authority, true),
            AccountMeta::new(*config, false),
        ],
        data,
    }
}

/// Helper to create an update_authority instruction
pub fn create_update_authority_ix(
    program_id: &Pubkey,
    authority: &Pubkey,
    config: &Pubkey,
    new_authority: &Pubkey,
) -> Instruction {
    let mut data = vec![0u8; 8 + 32];
    let discriminator = anchor_lang::solana_program::hash::hash(b"global:update_authority").to_bytes();
    data[..8].copy_from_slice(&discriminator[..8]);
    data[8..40].copy_from_slice(new_authority.as_ref());

    Instruction {
        program_id: *program_id,
        accounts: vec![
            AccountMeta::new(*authority, true),
            AccountMeta::new(*config, false),
        ],
        data,
    }
}

