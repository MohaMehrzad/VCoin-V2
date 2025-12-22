//! Common test utilities for staking-protocol integration tests

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

pub const STAKING_POOL_SEED: &[u8] = b"staking-pool";
pub const USER_STAKE_SEED: &[u8] = b"user-stake";

pub struct TestContext {
    pub banks_client: BanksClient,
    pub payer: Keypair,
    pub recent_blockhash: Hash,
    pub program_id: Pubkey,
}

impl TestContext {
    pub async fn new() -> Self {
        let program_id = staking_protocol::id();
        let program_test = ProgramTest::new(
            "staking_protocol",
            program_id,
            processor!(staking_protocol::entry),
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

    pub fn get_pool_pda(&self) -> (Pubkey, u8) {
        Pubkey::find_program_address(&[STAKING_POOL_SEED], &self.program_id)
    }

    pub fn get_user_stake_pda(&self, user: &Pubkey) -> (Pubkey, u8) {
        Pubkey::find_program_address(&[USER_STAKE_SEED, user.as_ref()], &self.program_id)
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
    vevcoin_program: &Pubkey,
) -> Instruction {
    let mut data = vec![0u8; 8 + 32];
    let discriminator = anchor_lang::solana_program::hash::hash(b"global:initialize_pool").to_bytes();
    data[..8].copy_from_slice(&discriminator[..8]);
    data[8..40].copy_from_slice(vevcoin_program.as_ref());

    Instruction {
        program_id: *program_id,
        accounts: vec![
            AccountMeta::new(*authority, true),
            AccountMeta::new(*pool, false),
            AccountMeta::new_readonly(solana_sdk::system_program::id(), false),
        ],
        data,
    }
}

pub fn create_stake_ix(
    program_id: &Pubkey,
    user: &Pubkey,
    pool: &Pubkey,
    user_stake: &Pubkey,
    amount: u64,
    lock_duration: i64,
) -> Instruction {
    let mut data = vec![0u8; 8 + 8 + 8];
    let discriminator = anchor_lang::solana_program::hash::hash(b"global:stake").to_bytes();
    data[..8].copy_from_slice(&discriminator[..8]);
    data[8..16].copy_from_slice(&amount.to_le_bytes());
    data[16..24].copy_from_slice(&lock_duration.to_le_bytes());

    Instruction {
        program_id: *program_id,
        accounts: vec![
            AccountMeta::new(*user, true),
            AccountMeta::new(*pool, false),
            AccountMeta::new(*user_stake, false),
            AccountMeta::new_readonly(spl_token_2022::id(), false),
            AccountMeta::new_readonly(solana_sdk::system_program::id(), false),
        ],
        data,
    }
}

pub fn create_unstake_ix(
    program_id: &Pubkey,
    user: &Pubkey,
    pool: &Pubkey,
    user_stake: &Pubkey,
    amount: u64,
) -> Instruction {
    let mut data = vec![0u8; 8 + 8];
    let discriminator = anchor_lang::solana_program::hash::hash(b"global:unstake").to_bytes();
    data[..8].copy_from_slice(&discriminator[..8]);
    data[8..16].copy_from_slice(&amount.to_le_bytes());

    Instruction {
        program_id: *program_id,
        accounts: vec![
            AccountMeta::new(*user, true),
            AccountMeta::new(*pool, false),
            AccountMeta::new(*user_stake, false),
            AccountMeta::new_readonly(spl_token_2022::id(), false),
        ],
        data,
    }
}

pub fn create_extend_lock_ix(
    program_id: &Pubkey,
    user: &Pubkey,
    pool: &Pubkey,
    user_stake: &Pubkey,
    new_lock_duration: i64,
) -> Instruction {
    let mut data = vec![0u8; 8 + 8];
    let discriminator = anchor_lang::solana_program::hash::hash(b"global:extend_lock").to_bytes();
    data[..8].copy_from_slice(&discriminator[..8]);
    data[8..16].copy_from_slice(&new_lock_duration.to_le_bytes());

    Instruction {
        program_id: *program_id,
        accounts: vec![
            AccountMeta::new(*user, true),
            AccountMeta::new_readonly(*pool, false),
            AccountMeta::new(*user_stake, false),
        ],
        data,
    }
}

pub fn create_update_tier_ix(
    program_id: &Pubkey,
    user: &Pubkey,
    pool: &Pubkey,
    user_stake: &Pubkey,
) -> Instruction {
    let mut data = vec![0u8; 8];
    let discriminator = anchor_lang::solana_program::hash::hash(b"global:update_tier").to_bytes();
    data[..8].copy_from_slice(&discriminator[..8]);

    Instruction {
        program_id: *program_id,
        accounts: vec![
            AccountMeta::new(*user, true),
            AccountMeta::new_readonly(*pool, false),
            AccountMeta::new(*user_stake, false),
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
            AccountMeta::new(*authority, true),
            AccountMeta::new(*pool, false),
        ],
        data,
    }
}

