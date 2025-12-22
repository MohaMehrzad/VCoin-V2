//! Common test utilities for gasless-protocol integration tests

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

pub const GASLESS_CONFIG_SEED: &[u8] = b"gasless-config";
pub const SESSION_KEY_SEED: &[u8] = b"session-key";
pub const USER_GASLESS_SEED: &[u8] = b"user-gasless";

pub struct TestContext {
    pub banks_client: BanksClient,
    pub payer: Keypair,
    pub recent_blockhash: Hash,
    pub program_id: Pubkey,
}

impl TestContext {
    pub async fn new() -> Self {
        let program_id = gasless_protocol::id();
        let program_test = ProgramTest::new(
            "gasless_protocol",
            program_id,
            processor!(gasless_protocol::entry),
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
        Pubkey::find_program_address(&[GASLESS_CONFIG_SEED], &self.program_id)
    }

    pub fn get_session_pda(&self, user: &Pubkey, session_pubkey: &Pubkey) -> (Pubkey, u8) {
        Pubkey::find_program_address(
            &[SESSION_KEY_SEED, user.as_ref(), session_pubkey.as_ref()],
            &self.program_id,
        )
    }

    pub fn get_user_gasless_pda(&self, user: &Pubkey) -> (Pubkey, u8) {
        Pubkey::find_program_address(&[USER_GASLESS_SEED, user.as_ref()], &self.program_id)
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
    fee_payer: &Pubkey,
    daily_budget: u64,
) -> Instruction {
    let mut data = vec![0u8; 8 + 32 + 8];
    let discriminator = anchor_lang::solana_program::hash::hash(b"global:initialize").to_bytes();
    data[..8].copy_from_slice(&discriminator[..8]);
    data[8..40].copy_from_slice(fee_payer.as_ref());
    data[40..48].copy_from_slice(&daily_budget.to_le_bytes());

    Instruction {
        program_id: *program_id,
        accounts: vec![
            AccountMeta::new(*config, false),
            AccountMeta::new(*authority, true),
            AccountMeta::new_readonly(solana_sdk::system_program::id(), false),
        ],
        data,
    }
}

pub fn create_session_key_ix(
    program_id: &Pubkey,
    user: &Pubkey,
    config: &Pubkey,
    session: &Pubkey,
    session_pubkey: &Pubkey,
    scope: u16,
    duration_seconds: i64,
    max_actions: u32,
    max_spend: u64,
    fee_method: u8,
) -> Instruction {
    let mut data = vec![0u8; 8 + 32 + 2 + 8 + 4 + 8 + 1];
    let discriminator = anchor_lang::solana_program::hash::hash(b"global:create_session_key").to_bytes();
    data[..8].copy_from_slice(&discriminator[..8]);
    data[8..40].copy_from_slice(session_pubkey.as_ref());
    data[40..42].copy_from_slice(&scope.to_le_bytes());
    data[42..50].copy_from_slice(&duration_seconds.to_le_bytes());
    data[50..54].copy_from_slice(&max_actions.to_le_bytes());
    data[54..62].copy_from_slice(&max_spend.to_le_bytes());
    data[62] = fee_method;

    Instruction {
        program_id: *program_id,
        accounts: vec![
            AccountMeta::new(*config, false),
            AccountMeta::new(*session, false),
            AccountMeta::new(*user, true),
            AccountMeta::new_readonly(solana_sdk::system_program::id(), false),
        ],
        data,
    }
}

pub fn create_execute_session_action_ix(
    program_id: &Pubkey,
    user: &Pubkey,
    config: &Pubkey,
    session: &Pubkey,
    action_type: u16,
    spend_amount: u64,
) -> Instruction {
    let mut data = vec![0u8; 8 + 2 + 8];
    let discriminator = anchor_lang::solana_program::hash::hash(b"global:execute_session_action").to_bytes();
    data[..8].copy_from_slice(&discriminator[..8]);
    data[8..10].copy_from_slice(&action_type.to_le_bytes());
    data[10..18].copy_from_slice(&spend_amount.to_le_bytes());

    Instruction {
        program_id: *program_id,
        accounts: vec![
            AccountMeta::new(*config, false),
            AccountMeta::new(*session, false),
            AccountMeta::new(*user, true),
        ],
        data,
    }
}

pub fn create_deduct_vcoin_fee_ix(
    program_id: &Pubkey,
    user: &Pubkey,
    config: &Pubkey,
    amount: u64,
) -> Instruction {
    let mut data = vec![0u8; 8 + 8];
    let discriminator = anchor_lang::solana_program::hash::hash(b"global:deduct_vcoin_fee").to_bytes();
    data[..8].copy_from_slice(&discriminator[..8]);
    data[8..16].copy_from_slice(&amount.to_le_bytes());

    Instruction {
        program_id: *program_id,
        accounts: vec![
            AccountMeta::new(*config, false),
            AccountMeta::new(*user, true),
            AccountMeta::new_readonly(spl_token_2022::id(), false),
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

