use anchor_lang::prelude::*;

#[event]
pub struct ProposalCreated {
    pub id: u64,
    pub proposer: Pubkey,
    pub proposal_type: u8,
    pub start_time: i64,
    pub end_time: i64,
}

#[event]
pub struct VoteCast {
    pub proposal_id: u64,
    pub voter: Pubkey,
    pub choice: u8,
    pub weight: u64,
    pub is_private: bool,
}

#[event]
pub struct ProposalExecuted {
    pub id: u64,
    pub executor: Pubkey,
    pub timestamp: i64,
}

#[event]
pub struct DelegationCreated {
    pub delegator: Pubkey,
    pub delegate: Pubkey,
    pub amount: u64,
    pub delegation_type: u8,
}

#[event]
pub struct ZKRevealComplete {
    pub proposal_id: u64,
    pub votes_for: u128,
    pub votes_against: u128,
    pub votes_abstain: u128,
}

