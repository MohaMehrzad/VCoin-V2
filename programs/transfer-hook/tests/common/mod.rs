//! Common test utilities for transfer-hook integration tests

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

pub const HOOK_CONFIG_SEED: &[u8] = b"hook-config";

pub struct TestContext {
    pub banks_client: BanksClient,
    pub payer: Keypair,
    pub recent_blockhash: Hash,
    pub program_id: Pubkey,
}

impl TestContext {
    pub async fn new() -> Self {
        let program_id = transfer_hook::id();
        let program_test = ProgramTest::new(
            "transfer_hook",
            program_id,
            processor!(transfer_hook::entry),
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
        Pubkey::find_program_address(&[HOOK_CONFIG_SEED], &self.program_id)
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
    five_a_program: &Pubkey,
    min_activity_amount: u64,
) -> Instruction {
    let mut data = vec![0u8; 8 + 32 + 8];
    let discriminator = anchor_lang::solana_program::hash::hash(b"global:initialize").to_bytes();
    data[..8].copy_from_slice(&discriminator[..8]);
    data[8..40].copy_from_slice(five_a_program.as_ref());
    data[40..48].copy_from_slice(&min_activity_amount.to_le_bytes());

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

pub fn create_execute_ix(
    program_id: &Pubkey,
    source: &Pubkey,
    destination: &Pubkey,
    config: &Pubkey,
    amount: u64,
) -> Instruction {
    let mut data = vec![0u8; 8 + 8];
    let discriminator = anchor_lang::solana_program::hash::hash(b"global:execute").to_bytes();
    data[..8].copy_from_slice(&discriminator[..8]);
    data[8..16].copy_from_slice(&amount.to_le_bytes());

    Instruction {
        program_id: *program_id,
        accounts: vec![
            AccountMeta::new_readonly(*source, false),
            AccountMeta::new_readonly(*destination, false),
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

