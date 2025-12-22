//! Common test utilities for sscre-protocol integration tests

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

pub const POOL_CONFIG_SEED: &[u8] = b"pool-config";
pub const EPOCH_SEED: &[u8] = b"epoch";
pub const USER_CLAIM_SEED: &[u8] = b"user-claim";
pub const CIRCUIT_BREAKER_SEED: &[u8] = b"circuit-breaker";

pub struct TestContext {
    pub banks_client: BanksClient,
    pub payer: Keypair,
    pub recent_blockhash: Hash,
    pub program_id: Pubkey,
}

impl TestContext {
    pub async fn new() -> Self {
        let program_id = sscre_protocol::id();
        let program_test = ProgramTest::new(
            "sscre_protocol",
            program_id,
            processor!(sscre_protocol::entry),
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

    pub fn get_pool_pda(&self) -> (Pubkey, u8) {
        Pubkey::find_program_address(&[POOL_CONFIG_SEED], &self.program_id)
    }

    pub fn get_epoch_pda(&self, epoch: u64) -> (Pubkey, u8) {
        Pubkey::find_program_address(&[EPOCH_SEED, &epoch.to_le_bytes()], &self.program_id)
    }

    pub fn get_user_claim_pda(&self, user: &Pubkey) -> (Pubkey, u8) {
        Pubkey::find_program_address(&[USER_CLAIM_SEED, user.as_ref()], &self.program_id)
    }

    pub fn get_circuit_breaker_pda(&self) -> (Pubkey, u8) {
        Pubkey::find_program_address(&[CIRCUIT_BREAKER_SEED], &self.program_id)
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

pub fn create_initialize_pool_ix(
    program_id: &Pubkey,
    authority: &Pubkey,
    pool: &Pubkey,
    vcoin_mint: &Pubkey,
    pool_vault: &Pubkey,
    fee_recipient: &Pubkey,
) -> Instruction {
    let mut data = vec![0u8; 8 + 32];
    let discriminator = anchor_lang::solana_program::hash::hash(b"global:initialize_pool").to_bytes();
    data[..8].copy_from_slice(&discriminator[..8]);
    data[8..40].copy_from_slice(fee_recipient.as_ref());

    Instruction {
        program_id: *program_id,
        accounts: vec![
            AccountMeta::new(*pool, false),
            AccountMeta::new_readonly(*vcoin_mint, false),
            AccountMeta::new(*pool_vault, false),
            AccountMeta::new(*authority, true),
            AccountMeta::new_readonly(spl_token_2022::id(), false),
            AccountMeta::new_readonly(solana_sdk::system_program::id(), false),
        ],
        data,
    }
}

pub fn create_start_epoch_ix(
    program_id: &Pubkey,
    authority: &Pubkey,
    pool: &Pubkey,
    epoch: &Pubkey,
    circuit_breaker: &Pubkey,
    total_allocation: u64,
) -> Instruction {
    let mut data = vec![0u8; 8 + 8];
    let discriminator = anchor_lang::solana_program::hash::hash(b"global:start_epoch").to_bytes();
    data[..8].copy_from_slice(&discriminator[..8]);
    data[8..16].copy_from_slice(&total_allocation.to_le_bytes());

    Instruction {
        program_id: *program_id,
        accounts: vec![
            AccountMeta::new(*pool, false),
            AccountMeta::new(*epoch, false),
            AccountMeta::new(*circuit_breaker, false),
            AccountMeta::new(*authority, true),
            AccountMeta::new_readonly(solana_sdk::system_program::id(), false),
        ],
        data,
    }
}

pub fn create_claim_rewards_ix(
    program_id: &Pubkey,
    user: &Pubkey,
    pool: &Pubkey,
    epoch: &Pubkey,
    user_claim: &Pubkey,
    circuit_breaker: &Pubkey,
    amount: u64,
    merkle_proof: Vec<[u8; 32]>,
) -> Instruction {
    let proof_len = merkle_proof.len();
    let mut data = vec![0u8; 8 + 8 + 4 + proof_len * 32];
    let discriminator = anchor_lang::solana_program::hash::hash(b"global:claim_rewards").to_bytes();
    data[..8].copy_from_slice(&discriminator[..8]);
    data[8..16].copy_from_slice(&amount.to_le_bytes());
    data[16..20].copy_from_slice(&(proof_len as u32).to_le_bytes());
    for (i, proof) in merkle_proof.iter().enumerate() {
        data[20 + i * 32..20 + (i + 1) * 32].copy_from_slice(proof);
    }

    Instruction {
        program_id: *program_id,
        accounts: vec![
            AccountMeta::new(*pool, false),
            AccountMeta::new(*epoch, false),
            AccountMeta::new(*user_claim, false),
            AccountMeta::new(*circuit_breaker, false),
            AccountMeta::new(*user, true),
            AccountMeta::new_readonly(spl_token_2022::id(), false),
            AccountMeta::new_readonly(solana_sdk::system_program::id(), false),
        ],
        data,
    }
}

pub fn create_trigger_circuit_breaker_ix(
    program_id: &Pubkey,
    authority: &Pubkey,
    pool: &Pubkey,
    circuit_breaker: &Pubkey,
    reason: u8,
) -> Instruction {
    let mut data = vec![0u8; 8 + 1];
    let discriminator = anchor_lang::solana_program::hash::hash(b"global:trigger_circuit_breaker").to_bytes();
    data[..8].copy_from_slice(&discriminator[..8]);
    data[8] = reason;

    Instruction {
        program_id: *program_id,
        accounts: vec![
            AccountMeta::new(*pool, false),
            AccountMeta::new(*circuit_breaker, false),
            AccountMeta::new(*authority, true),
        ],
        data,
    }
}

pub fn create_set_paused_ix(
    program_id: &Pubkey,
    authority: &Pubkey,
    pool: &Pubkey,
    paused: bool,
) -> Instruction {
    let mut data = vec![0u8; 8 + 1];
    let discriminator = anchor_lang::solana_program::hash::hash(b"global:set_paused").to_bytes();
    data[..8].copy_from_slice(&discriminator[..8]);
    data[8] = if paused { 1 } else { 0 };

    Instruction {
        program_id: *program_id,
        accounts: vec![
            AccountMeta::new(*pool, false),
            AccountMeta::new(*authority, true),
        ],
        data,
    }
}

