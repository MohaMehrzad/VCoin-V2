//! Common test utilities for vcoin-token integration tests
//!
//! Provides TestContext and helpers for solana-program-test based testing.

use solana_program_test::*;
use solana_sdk::{
    account::Account,
    hash::{Hash, hash},
    instruction::{AccountMeta, Instruction},
    pubkey::Pubkey,
    signature::{Keypair, Signer},
    system_program,
    sysvar,
    transaction::Transaction,
};

/// VCoin configuration seed
pub const VCOIN_CONFIG_SEED: &[u8] = b"vcoin-config";

/// Test context wrapper for vcoin-token tests
pub struct TestContext {
    pub banks_client: BanksClient,
    pub payer: Keypair,
    pub recent_blockhash: Hash,
    pub program_id: Pubkey,
}

impl TestContext {
    /// Create a new test context with the vcoin-token program loaded
    pub async fn new() -> Self {
        let program_id = vcoin_token::id();
        let mut program_test = ProgramTest::default();
        program_test.add_program("vcoin_token", program_id, None);

        let (banks_client, payer, recent_blockhash) = program_test.start().await;

        Self {
            banks_client,
            payer,
            recent_blockhash,
            program_id,
        }
    }

    /// Refresh the blockhash
    pub async fn refresh_blockhash(&mut self) {
        self.recent_blockhash = self.banks_client.get_latest_blockhash().await.unwrap();
    }

    /// Get an account by pubkey
    pub async fn get_account(&mut self, pubkey: Pubkey) -> Option<Account> {
        self.banks_client.get_account(pubkey).await.unwrap()
    }

    /// Airdrop SOL to an account
    pub async fn airdrop(&mut self, pubkey: &Pubkey, lamports: u64) {
        let ix = solana_sdk::system_instruction::transfer(&self.payer.pubkey(), pubkey, lamports);
        let tx = Transaction::new_signed_with_payer(
            &[ix],
            Some(&self.payer.pubkey()),
            &[&self.payer],
            self.recent_blockhash,
        );
        self.banks_client.process_transaction(tx).await.unwrap();
        self.refresh_blockhash().await;
    }

    /// Get the config PDA
    pub fn get_config_pda(&self) -> (Pubkey, u8) {
        Pubkey::find_program_address(&[VCOIN_CONFIG_SEED], &self.program_id)
    }

    /// Process a transaction and return the result
    pub async fn process_transaction(
        &mut self,
        instructions: &[Instruction],
        signers: &[&Keypair],
    ) -> std::result::Result<(), BanksClientError> {
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
    treasury: &Pubkey,
    permanent_delegate: &Pubkey,
) -> Instruction {
    // Build instruction data: discriminator + permanent_delegate pubkey
    let mut data = vec![0u8; 8 + 32]; // 8 byte discriminator + 32 byte pubkey
    // Anchor discriminator for "initialize_mint"
    let discriminator = hash(b"global:initialize_mint").to_bytes();
    data[..8].copy_from_slice(&discriminator[..8]);
    data[8..40].copy_from_slice(permanent_delegate.as_ref());

    Instruction {
        program_id: *program_id,
        accounts: vec![
            AccountMeta::new(*authority, true),
            AccountMeta::new(*config, false),
            AccountMeta::new(*mint, false),
            AccountMeta::new_readonly(*treasury, false),
            AccountMeta::new_readonly(spl_token_2022::id(), false),
            AccountMeta::new_readonly(system_program::ID, false),
            AccountMeta::new_readonly(sysvar::rent::ID, false),
        ],
        data,
    }
}

/// Helper to create a set_paused instruction
pub fn create_set_paused_ix(
    program_id: &Pubkey,
    authority: &Pubkey,
    config: &Pubkey,
    paused: bool,
) -> Instruction {
    let mut data = vec![0u8; 8 + 1]; // 8 byte discriminator + 1 byte bool
    let discriminator = hash(b"global:set_paused").to_bytes();
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

/// Helper to create an update_authority instruction
pub fn create_update_authority_ix(
    program_id: &Pubkey,
    authority: &Pubkey,
    config: &Pubkey,
    new_authority: &Pubkey,
) -> Instruction {
    let mut data = vec![0u8; 8 + 32]; // 8 byte discriminator + 32 byte pubkey
    let discriminator = hash(b"global:update_authority").to_bytes();
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

/// Helper to create an update_permanent_delegate instruction
pub fn create_update_delegate_ix(
    program_id: &Pubkey,
    authority: &Pubkey,
    config: &Pubkey,
    new_delegate: &Pubkey,
) -> Instruction {
    let mut data = vec![0u8; 8 + 32]; // 8 byte discriminator + 32 byte pubkey
    let discriminator = hash(b"global:update_permanent_delegate").to_bytes();
    data[..8].copy_from_slice(&discriminator[..8]);
    data[8..40].copy_from_slice(new_delegate.as_ref());

    Instruction {
        program_id: *program_id,
        accounts: vec![
            AccountMeta::new(*authority, true),
            AccountMeta::new(*config, false),
        ],
        data,
    }
}

