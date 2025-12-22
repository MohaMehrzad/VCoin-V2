//! Common test utilities for governance-protocol integration tests

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

pub const GOV_CONFIG_SEED: &[u8] = b"gov-config";
pub const PROPOSAL_SEED: &[u8] = b"proposal";
pub const VOTE_RECORD_SEED: &[u8] = b"vote-record";
pub const DELEGATION_SEED: &[u8] = b"delegation";

pub struct TestContext {
    pub banks_client: BanksClient,
    pub payer: Keypair,
    pub recent_blockhash: Hash,
    pub program_id: Pubkey,
}

impl TestContext {
    pub async fn new() -> Self {
        let program_id = governance_protocol::id();
        let program_test = ProgramTest::new(
            "governance_protocol",
            program_id,
            processor!(governance_protocol::entry),
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
        Pubkey::find_program_address(&[GOV_CONFIG_SEED], &self.program_id)
    }

    pub fn get_proposal_pda(&self, proposal_id: u64) -> (Pubkey, u8) {
        Pubkey::find_program_address(&[PROPOSAL_SEED, &proposal_id.to_le_bytes()], &self.program_id)
    }

    pub fn get_vote_record_pda(&self, proposal: &Pubkey, voter: &Pubkey) -> (Pubkey, u8) {
        Pubkey::find_program_address(&[VOTE_RECORD_SEED, proposal.as_ref(), voter.as_ref()], &self.program_id)
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
    staking_program: &Pubkey,
    five_a_program: &Pubkey,
) -> Instruction {
    let mut data = vec![0u8; 8 + 32 + 32];
    let discriminator = anchor_lang::solana_program::hash::hash(b"global:initialize").to_bytes();
    data[..8].copy_from_slice(&discriminator[..8]);
    data[8..40].copy_from_slice(staking_program.as_ref());
    data[40..72].copy_from_slice(five_a_program.as_ref());

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

pub fn create_proposal_ix(
    program_id: &Pubkey,
    proposer: &Pubkey,
    proposal: &Pubkey,
    config: &Pubkey,
    title_hash: [u8; 32],
    description_uri: &str,
    proposal_type: u8,
    enable_private_voting: bool,
) -> Instruction {
    let uri_bytes = description_uri.as_bytes();
    let mut data = vec![0u8; 8 + 32 + 4 + uri_bytes.len() + 1 + 1];
    let discriminator = anchor_lang::solana_program::hash::hash(b"global:create_proposal").to_bytes();
    data[..8].copy_from_slice(&discriminator[..8]);
    data[8..40].copy_from_slice(&title_hash);
    data[40..44].copy_from_slice(&(uri_bytes.len() as u32).to_le_bytes());
    data[44..44 + uri_bytes.len()].copy_from_slice(uri_bytes);
    data[44 + uri_bytes.len()] = proposal_type;
    data[45 + uri_bytes.len()] = if enable_private_voting { 1 } else { 0 };

    Instruction {
        program_id: *program_id,
        accounts: vec![
            AccountMeta::new(*proposer, true),
            AccountMeta::new(*proposal, false),
            AccountMeta::new_readonly(*config, false),
            AccountMeta::new_readonly(solana_sdk::system_program::id(), false),
        ],
        data,
    }
}

pub fn create_cast_vote_ix(
    program_id: &Pubkey,
    voter: &Pubkey,
    proposal: &Pubkey,
    vote_record: &Pubkey,
    config: &Pubkey,
    choice: u8,
    vevcoin_balance: u64,
    five_a_score: u16,
    tier: u8,
) -> Instruction {
    let mut data = vec![0u8; 8 + 1 + 8 + 2 + 1];
    let discriminator = anchor_lang::solana_program::hash::hash(b"global:cast_vote").to_bytes();
    data[..8].copy_from_slice(&discriminator[..8]);
    data[8] = choice;
    data[9..17].copy_from_slice(&vevcoin_balance.to_le_bytes());
    data[17..19].copy_from_slice(&five_a_score.to_le_bytes());
    data[19] = tier;

    Instruction {
        program_id: *program_id,
        accounts: vec![
            AccountMeta::new(*voter, true),
            AccountMeta::new(*proposal, false),
            AccountMeta::new(*vote_record, false),
            AccountMeta::new_readonly(*config, false),
            AccountMeta::new_readonly(solana_sdk::system_program::id(), false),
        ],
        data,
    }
}

pub fn create_delegate_votes_ix(
    program_id: &Pubkey,
    delegator: &Pubkey,
    delegate: &Pubkey,
    delegation: &Pubkey,
    config: &Pubkey,
    delegation_type: u8,
    categories: u8,
    vevcoin_amount: u64,
    expires_at: i64,
    revocable: bool,
) -> Instruction {
    let mut data = vec![0u8; 8 + 1 + 1 + 8 + 8 + 1];
    let discriminator = anchor_lang::solana_program::hash::hash(b"global:delegate_votes").to_bytes();
    data[..8].copy_from_slice(&discriminator[..8]);
    data[8] = delegation_type;
    data[9] = categories;
    data[10..18].copy_from_slice(&vevcoin_amount.to_le_bytes());
    data[18..26].copy_from_slice(&expires_at.to_le_bytes());
    data[26] = if revocable { 1 } else { 0 };

    Instruction {
        program_id: *program_id,
        accounts: vec![
            AccountMeta::new(*delegator, true),
            AccountMeta::new_readonly(*delegate, false),
            AccountMeta::new(*delegation, false),
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

