//! Common test utilities for vilink-protocol integration tests

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

pub const CONFIG_SEED: &[u8] = b"vilink-config";
pub const ACTION_SEED: &[u8] = b"action";
pub const DAPP_REGISTRY_SEED: &[u8] = b"dapp";
pub const BATCH_SEED: &[u8] = b"batch";

pub struct TestContext {
    pub banks_client: BanksClient,
    pub payer: Keypair,
    pub recent_blockhash: Hash,
    pub program_id: Pubkey,
}

impl TestContext {
    pub async fn new() -> Self {
        let program_id = vilink_protocol::id();
        let program_test = ProgramTest::new(
            "vilink_protocol",
            program_id,
            processor!(vilink_protocol::entry),
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
        Pubkey::find_program_address(&[CONFIG_SEED], &self.program_id)
    }

    pub fn get_dapp_pda(&self, authority: &Pubkey) -> (Pubkey, u8) {
        Pubkey::find_program_address(&[DAPP_REGISTRY_SEED, authority.as_ref()], &self.program_id)
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
    vcoin_mint: &Pubkey,
    treasury: &Pubkey,
) -> Instruction {
    let mut data = vec![0u8; 8 + 32];
    let discriminator = anchor_lang::solana_program::hash::hash(b"global:initialize").to_bytes();
    data[..8].copy_from_slice(&discriminator[..8]);
    data[8..40].copy_from_slice(treasury.as_ref());

    Instruction {
        program_id: *program_id,
        accounts: vec![
            AccountMeta::new(*config, false),
            AccountMeta::new_readonly(*vcoin_mint, false),
            AccountMeta::new(*authority, true),
            AccountMeta::new_readonly(solana_sdk::system_program::id(), false),
        ],
        data,
    }
}

pub fn create_action_ix(
    program_id: &Pubkey,
    creator: &Pubkey,
    config: &Pubkey,
    action: &Pubkey,
    action_type: u8,
    amount: u64,
    target: &Pubkey,
) -> Instruction {
    let metadata_hash = [0u8; 32];
    let expiry_seconds = 7 * 24 * 60 * 60i64;
    let mut data = vec![0u8; 8 + 1 + 8 + 32 + 32 + 8 + 1 + 4 + 1 + 32];
    let discriminator = anchor_lang::solana_program::hash::hash(b"global:create_action").to_bytes();
    data[..8].copy_from_slice(&discriminator[..8]);
    data[8] = action_type;
    data[9..17].copy_from_slice(&amount.to_le_bytes());
    data[17..49].copy_from_slice(target.as_ref());
    data[49..81].copy_from_slice(&metadata_hash);
    data[81..89].copy_from_slice(&expiry_seconds.to_le_bytes());
    data[89] = 0; // one_time = false
    data[90..94].copy_from_slice(&0u32.to_le_bytes()); // max_executions
    data[94] = 0; // None for content_id

    Instruction {
        program_id: *program_id,
        accounts: vec![
            AccountMeta::new(*config, false),
            AccountMeta::new(*action, false),
            AccountMeta::new(*creator, true),
            AccountMeta::new_readonly(solana_sdk::system_program::id(), false),
        ],
        data,
    }
}

pub fn create_register_dapp_ix(
    program_id: &Pubkey,
    authority: &Pubkey,
    config: &Pubkey,
    dapp: &Pubkey,
    dapp_authority: &Pubkey,
    name: [u8; 32],
    allowed_actions: u8,
    fee_share_bps: u16,
) -> Instruction {
    let webhook_hash = [0u8; 32];
    let mut data = vec![0u8; 8 + 32 + 32 + 1 + 2];
    let discriminator = anchor_lang::solana_program::hash::hash(b"global:register_dapp").to_bytes();
    data[..8].copy_from_slice(&discriminator[..8]);
    data[8..40].copy_from_slice(&name);
    data[40..72].copy_from_slice(&webhook_hash);
    data[72] = allowed_actions;
    data[73..75].copy_from_slice(&fee_share_bps.to_le_bytes());

    Instruction {
        program_id: *program_id,
        accounts: vec![
            AccountMeta::new_readonly(*config, false),
            AccountMeta::new(*dapp, false),
            AccountMeta::new_readonly(*dapp_authority, false),
            AccountMeta::new(*authority, true),
            AccountMeta::new_readonly(solana_sdk::system_program::id(), false),
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
            AccountMeta::new(*config, false),
            AccountMeta::new(*authority, true),
        ],
        data,
    }
}

