//! Common test utilities for content-registry integration tests

use anchor_lang::prelude::*;
use solana_program_test::*;
use solana_sdk::{
    account::Account,
    hash::Hash,
    instruction::{AccountMeta, Instruction},
    pubkey::Pubkey,
    signature::{Keypair, Signer},
    transaction::Transaction,
};

pub const REGISTRY_CONFIG_SEED: &[u8] = b"registry-config";
pub const CONTENT_RECORD_SEED: &[u8] = b"content-record";
pub const USER_ENERGY_SEED: &[u8] = b"user-energy";
pub const ENERGY_CONFIG_SEED: &[u8] = b"energy-config";

pub struct TestContext {
    pub banks_client: BanksClient,
    pub payer: Keypair,
    pub recent_blockhash: Hash,
    pub program_id: Pubkey,
}

impl TestContext {
    pub async fn new() -> Self {
        let program_id = content_registry::id();
        let program_test = ProgramTest::new(
            "content_registry",
            program_id,
            processor!(content_registry::entry),
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

    pub fn get_config_pda(&self) -> (Pubkey, u8) {
        Pubkey::find_program_address(&[REGISTRY_CONFIG_SEED], &self.program_id)
    }

    pub fn get_content_pda(&self, tracking_id: &[u8; 32]) -> (Pubkey, u8) {
        Pubkey::find_program_address(&[CONTENT_RECORD_SEED, tracking_id], &self.program_id)
    }

    pub fn get_user_energy_pda(&self, user: &Pubkey) -> (Pubkey, u8) {
        Pubkey::find_program_address(&[USER_ENERGY_SEED, user.as_ref()], &self.program_id)
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

pub fn create_initialize_ix(
    program_id: &Pubkey,
    authority: &Pubkey,
    config: &Pubkey,
    identity_program: &Pubkey,
    staking_program: &Pubkey,
) -> Instruction {
    let mut data = vec![0u8; 8 + 32 + 32];
    let discriminator = anchor_lang::solana_program::hash::hash(b"global:initialize").to_bytes();
    data[..8].copy_from_slice(&discriminator[..8]);
    data[8..40].copy_from_slice(identity_program.as_ref());
    data[40..72].copy_from_slice(staking_program.as_ref());

    Instruction {
        program_id: *program_id,
        accounts: vec![
            AccountMeta::new(*authority, true),
            AccountMeta::new(*config, false),
            AccountMeta::new_readonly(solana_sdk::system_program::id(), false),
        ],
        data,
    }
}

pub fn create_content_ix(
    program_id: &Pubkey,
    creator: &Pubkey,
    content: &Pubkey,
    config: &Pubkey,
    tracking_id: [u8; 32],
    content_hash: [u8; 32],
    content_uri: &str,
    content_type: u8,
) -> Instruction {
    let uri_bytes = content_uri.as_bytes();
    let mut data = vec![0u8; 8 + 32 + 32 + 4 + uri_bytes.len() + 1];
    let discriminator = anchor_lang::solana_program::hash::hash(b"global:create_content").to_bytes();
    data[..8].copy_from_slice(&discriminator[..8]);
    data[8..40].copy_from_slice(&tracking_id);
    data[40..72].copy_from_slice(&content_hash);
    data[72..76].copy_from_slice(&(uri_bytes.len() as u32).to_le_bytes());
    data[76..76 + uri_bytes.len()].copy_from_slice(uri_bytes);
    data[76 + uri_bytes.len()] = content_type;

    Instruction {
        program_id: *program_id,
        accounts: vec![
            AccountMeta::new(*creator, true),
            AccountMeta::new(*content, false),
            AccountMeta::new_readonly(*config, false),
            AccountMeta::new_readonly(solana_sdk::system_program::id(), false),
        ],
        data,
    }
}

pub fn create_delete_content_ix(
    program_id: &Pubkey,
    creator: &Pubkey,
    content: &Pubkey,
    config: &Pubkey,
) -> Instruction {
    let mut data = vec![0u8; 8];
    let discriminator = anchor_lang::solana_program::hash::hash(b"global:delete_content").to_bytes();
    data[..8].copy_from_slice(&discriminator[..8]);

    Instruction {
        program_id: *program_id,
        accounts: vec![
            AccountMeta::new(*creator, true),
            AccountMeta::new(*content, false),
            AccountMeta::new_readonly(*config, false),
        ],
        data,
    }
}

pub fn create_set_paused_ix(
    program_id: &Pubkey,
    authority: &Pubkey,
    config: &Pubkey,
    paused: bool,
) -> Instruction {
    let mut data = vec![0u8; 8 + 1];
    let discriminator = anchor_lang::solana_program::hash::hash(b"global:set_paused").to_bytes();
    data[..8].copy_from_slice(&discriminator[..8]);
    data[8] = if paused { 1 } else { 0 };

    Instruction {
        program_id: *program_id,
        accounts: vec![
            AccountMeta::new(*authority, true),
            AccountMeta::new(*config, false),
        ],
        data,
    }
}

