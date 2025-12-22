//! Common test utilities for identity-protocol integration tests

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

pub const IDENTITY_CONFIG_SEED: &[u8] = b"identity-config";
pub const IDENTITY_SEED: &[u8] = b"identity";

pub struct TestContext {
    pub banks_client: BanksClient,
    pub payer: Keypair,
    pub recent_blockhash: Hash,
    pub program_id: Pubkey,
}

impl TestContext {
    pub async fn new() -> Self {
        let program_id = identity_protocol::id();
        let program_test = ProgramTest::new(
            "identity_protocol",
            program_id,
            processor!(identity_protocol::entry),
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
        Pubkey::find_program_address(&[IDENTITY_CONFIG_SEED], &self.program_id)
    }

    pub fn get_identity_pda(&self, user: &Pubkey) -> (Pubkey, u8) {
        Pubkey::find_program_address(&[IDENTITY_SEED, user.as_ref()], &self.program_id)
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
    sas_program: &Pubkey,
    usdc_mint: &Pubkey,
) -> Instruction {
    let mut data = vec![0u8; 8 + 32 + 32];
    let discriminator = anchor_lang::solana_program::hash::hash(b"global:initialize").to_bytes();
    data[..8].copy_from_slice(&discriminator[..8]);
    data[8..40].copy_from_slice(sas_program.as_ref());
    data[40..72].copy_from_slice(usdc_mint.as_ref());

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

pub fn create_identity_ix(
    program_id: &Pubkey,
    user: &Pubkey,
    identity: &Pubkey,
    config: &Pubkey,
    did_hash: [u8; 32],
    username: &str,
) -> Instruction {
    let username_bytes = username.as_bytes();
    let mut data = vec![0u8; 8 + 32 + 4 + username_bytes.len()];
    let discriminator = anchor_lang::solana_program::hash::hash(b"global:create_identity").to_bytes();
    data[..8].copy_from_slice(&discriminator[..8]);
    data[8..40].copy_from_slice(&did_hash);
    data[40..44].copy_from_slice(&(username_bytes.len() as u32).to_le_bytes());
    data[44..].copy_from_slice(username_bytes);

    Instruction {
        program_id: *program_id,
        accounts: vec![
            AccountMeta::new(*user, true),
            AccountMeta::new(*identity, false),
            AccountMeta::new_readonly(*config, false),
            AccountMeta::new_readonly(solana_sdk::system_program::id(), false),
        ],
        data,
    }
}

pub fn create_subscribe_ix(
    program_id: &Pubkey,
    user: &Pubkey,
    identity: &Pubkey,
    config: &Pubkey,
    tier: u8,
) -> Instruction {
    let mut data = vec![0u8; 8 + 1];
    let discriminator = anchor_lang::solana_program::hash::hash(b"global:subscribe").to_bytes();
    data[..8].copy_from_slice(&discriminator[..8]);
    data[8] = tier;

    Instruction {
        program_id: *program_id,
        accounts: vec![
            AccountMeta::new(*user, true),
            AccountMeta::new(*identity, false),
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

