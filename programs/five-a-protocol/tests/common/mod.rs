//! Common test utilities for five-a-protocol integration tests

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

pub const FIVE_A_CONFIG_SEED: &[u8] = b"five-a-config";
pub const USER_SCORE_SEED: &[u8] = b"user-score";
pub const ORACLE_SEED: &[u8] = b"oracle";
pub const VOUCH_RECORD_SEED: &[u8] = b"vouch-record";

pub struct TestContext {
    pub banks_client: BanksClient,
    pub payer: Keypair,
    pub recent_blockhash: Hash,
    pub program_id: Pubkey,
}

impl TestContext {
    pub async fn new() -> Self {
        let program_id = five_a_protocol::id();
        let program_test = ProgramTest::new(
            "five_a_protocol",
            program_id,
            processor!(five_a_protocol::entry),
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
        Pubkey::find_program_address(&[FIVE_A_CONFIG_SEED], &self.program_id)
    }

    pub fn get_user_score_pda(&self, user: &Pubkey) -> (Pubkey, u8) {
        Pubkey::find_program_address(&[USER_SCORE_SEED, user.as_ref()], &self.program_id)
    }

    pub fn get_oracle_pda(&self, oracle: &Pubkey) -> (Pubkey, u8) {
        Pubkey::find_program_address(&[ORACLE_SEED, oracle.as_ref()], &self.program_id)
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
    vcoin_mint: &Pubkey,
) -> Instruction {
    let mut data = vec![0u8; 8 + 32 + 32];
    let discriminator = anchor_lang::solana_program::hash::hash(b"global:initialize").to_bytes();
    data[..8].copy_from_slice(&discriminator[..8]);
    data[8..40].copy_from_slice(identity_program.as_ref());
    data[40..72].copy_from_slice(vcoin_mint.as_ref());

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

pub fn create_register_oracle_ix(
    program_id: &Pubkey,
    authority: &Pubkey,
    config: &Pubkey,
    oracle_account: &Pubkey,
    oracle_pda: &Pubkey,
    name: &str,
) -> Instruction {
    let name_bytes = name.as_bytes();
    let mut data = vec![0u8; 8 + 4 + name_bytes.len()];
    let discriminator = anchor_lang::solana_program::hash::hash(b"global:register_oracle").to_bytes();
    data[..8].copy_from_slice(&discriminator[..8]);
    data[8..12].copy_from_slice(&(name_bytes.len() as u32).to_le_bytes());
    data[12..].copy_from_slice(name_bytes);

    Instruction {
        program_id: *program_id,
        accounts: vec![
            AccountMeta::new(*authority, true),
            AccountMeta::new_readonly(*config, false),
            AccountMeta::new_readonly(*oracle_account, false),
            AccountMeta::new(*oracle_pda, false),
            AccountMeta::new_readonly(solana_sdk::system_program::id(), false),
        ],
        data,
    }
}

pub fn create_submit_score_ix(
    program_id: &Pubkey,
    oracle: &Pubkey,
    config: &Pubkey,
    user_score: &Pubkey,
    authenticity: u16,
    accuracy: u16,
    agility: u16,
    activity: u16,
    approved: u16,
) -> Instruction {
    let mut data = vec![0u8; 8 + 5 * 2];
    let discriminator = anchor_lang::solana_program::hash::hash(b"global:submit_score").to_bytes();
    data[..8].copy_from_slice(&discriminator[..8]);
    data[8..10].copy_from_slice(&authenticity.to_le_bytes());
    data[10..12].copy_from_slice(&accuracy.to_le_bytes());
    data[12..14].copy_from_slice(&agility.to_le_bytes());
    data[14..16].copy_from_slice(&activity.to_le_bytes());
    data[16..18].copy_from_slice(&approved.to_le_bytes());

    Instruction {
        program_id: *program_id,
        accounts: vec![
            AccountMeta::new(*oracle, true),
            AccountMeta::new_readonly(*config, false),
            AccountMeta::new(*user_score, false),
        ],
        data,
    }
}

pub fn create_vouch_for_user_ix(
    program_id: &Pubkey,
    voucher: &Pubkey,
    vouchee: &Pubkey,
    config: &Pubkey,
) -> Instruction {
    let mut data = vec![0u8; 8];
    let discriminator = anchor_lang::solana_program::hash::hash(b"global:vouch_for_user").to_bytes();
    data[..8].copy_from_slice(&discriminator[..8]);

    Instruction {
        program_id: *program_id,
        accounts: vec![
            AccountMeta::new(*voucher, true),
            AccountMeta::new_readonly(*vouchee, false),
            AccountMeta::new_readonly(*config, false),
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
            AccountMeta::new(*authority, true),
            AccountMeta::new(*config, false),
        ],
        data,
    }
}

