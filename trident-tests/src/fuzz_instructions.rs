//! Fuzz instruction definitions
//!
//! Defines the instruction data types for fuzzing.

use anchor_lang::prelude::*;
use borsh::{BorshDeserialize, BorshSerialize};

/// Fuzz data for staking operations
#[derive(Debug, Clone, BorshSerialize, BorshDeserialize)]
pub struct FuzzStakeData {
    pub amount: u64,
    pub lock_duration: i64,
}

impl FuzzStakeData {
    /// Validate stake data is within bounds
    pub fn is_valid(&self) -> bool {
        // Minimum lock: 1 week
        const MIN_LOCK: i64 = 7 * 24 * 60 * 60;
        // Maximum lock: 4 years
        const MAX_LOCK: i64 = 4 * 365 * 24 * 60 * 60;
        
        self.amount > 0 && 
        self.lock_duration >= MIN_LOCK && 
        self.lock_duration <= MAX_LOCK
    }
}

/// Fuzz data for governance voting
#[derive(Debug, Clone, BorshSerialize, BorshDeserialize)]
pub struct FuzzVoteData {
    pub choice: u8,
    pub vevcoin_balance: u64,
    pub five_a_score: u16,
    pub tier: u8,
}

impl FuzzVoteData {
    /// Validate vote data is within bounds
    pub fn is_valid(&self) -> bool {
        // Choice: 1=For, 2=Against, 3=Abstain
        let valid_choice = self.choice >= 1 && self.choice <= 3;
        // 5A score: 0-10000
        let valid_score = self.five_a_score <= 10000;
        // Tier: 0-4
        let valid_tier = self.tier <= 4;
        
        valid_choice && valid_score && valid_tier && self.vevcoin_balance > 0
    }
}

/// Fuzz data for 5A score submission
#[derive(Debug, Clone, BorshSerialize, BorshDeserialize)]
pub struct Fuzz5AScoreData {
    pub authenticity: u16,
    pub accuracy: u16,
    pub agility: u16,
    pub activity: u16,
    pub approved: u16,
}

impl Fuzz5AScoreData {
    /// Validate all scores are within bounds
    pub fn is_valid(&self) -> bool {
        const MAX: u16 = 10000;
        self.authenticity <= MAX &&
        self.accuracy <= MAX &&
        self.agility <= MAX &&
        self.activity <= MAX &&
        self.approved <= MAX
    }
    
    /// Calculate composite score
    pub fn composite(&self) -> u16 {
        // Weights: Auth=25%, Acc=20%, Agi=15%, Act=25%, App=15%
        let weighted = 
            self.authenticity as u32 * 2500 +
            self.accuracy as u32 * 2000 +
            self.agility as u32 * 1500 +
            self.activity as u32 * 2500 +
            self.approved as u32 * 1500;
        (weighted / 10000) as u16
    }
}

/// Fuzz data for proposal creation
#[derive(Debug, Clone, BorshSerialize, BorshDeserialize)]
pub struct FuzzProposalData {
    pub title_hash: [u8; 32],
    pub proposal_type: u8,
    pub enable_private_voting: bool,
}

impl FuzzProposalData {
    pub fn is_valid(&self) -> bool {
        // Proposal type: 0-3
        self.proposal_type <= 3
    }
}

/// Fuzz data for merkle claim
#[derive(Debug, Clone, BorshSerialize, BorshDeserialize)]
pub struct FuzzClaimData {
    pub amount: u64,
    pub epoch: u64,
    pub merkle_proof: Vec<[u8; 32]>,
}

impl FuzzClaimData {
    pub fn is_valid(&self) -> bool {
        self.amount > 0 && 
        self.epoch > 0 && 
        self.merkle_proof.len() <= 32 // Max tree depth
    }
}

/// Fuzz instruction types
#[derive(Debug, Clone)]
pub enum FuzzInstruction {
    // Staking
    Stake(FuzzStakeData),
    ExtendLock { new_duration: i64 },
    Unstake { amount: u64 },
    
    // Governance
    CreateProposal(FuzzProposalData),
    CastVote(FuzzVoteData),
    DelegateVotes { amount: u64, delegate: Pubkey },
    
    // 5A Protocol
    SubmitScore(Fuzz5AScoreData),
    
    // SSCRE
    ClaimRewards(FuzzClaimData),
    
    // Admin
    SetPaused(bool),
}

