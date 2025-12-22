use anchor_lang::prelude::*;

/// Proposal account
#[account]
pub struct Proposal {
    /// Proposal ID
    pub id: u64,
    /// Proposer
    pub proposer: Pubkey,
    /// Hash of title
    pub title_hash: [u8; 32],
    /// IPFS link to description
    pub description_uri: [u8; 128],
    /// URI length
    pub uri_len: u8,
    /// Proposal type
    pub proposal_type: u8,
    /// Voting start time
    pub start_time: i64,
    /// Voting end time
    pub end_time: i64,
    /// Votes for (u128 for large-scale)
    pub votes_for: u128,
    /// Votes against
    pub votes_against: u128,
    /// Abstain votes
    pub votes_abstain: u128,
    /// Current status
    pub status: u8,
    /// Execution time (after timelock)
    pub execution_time: i64,
    /// Whether executed
    pub executed: bool,
    /// Whether ZK voting is enabled
    pub is_private_voting: bool,
    /// PDA bump
    pub bump: u8,
}

impl Default for Proposal {
    fn default() -> Self {
        Self {
            id: 0,
            proposer: Pubkey::default(),
            title_hash: [0u8; 32],
            description_uri: [0u8; 128],
            uri_len: 0,
            proposal_type: 0,
            start_time: 0,
            end_time: 0,
            votes_for: 0,
            votes_against: 0,
            votes_abstain: 0,
            status: 0,
            execution_time: 0,
            executed: false,
            is_private_voting: false,
            bump: 0,
        }
    }
}

impl Proposal {
    pub const LEN: usize = 8 + // discriminator
        8 +   // id
        32 +  // proposer
        32 +  // title_hash
        128 + // description_uri
        1 +   // uri_len
        1 +   // proposal_type
        8 +   // start_time
        8 +   // end_time
        16 +  // votes_for (u128)
        16 +  // votes_against
        16 +  // votes_abstain
        1 +   // status
        8 +   // execution_time
        1 +   // executed
        1 +   // is_private_voting
        1;    // bump
}

