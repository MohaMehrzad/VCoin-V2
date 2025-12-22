use anchor_lang::prelude::*;

/// Global governance configuration
#[account]
#[derive(Default)]
pub struct GovernanceConfig {
    /// Admin authority
    pub authority: Pubkey,
    /// Pending authority for two-step transfer (H-02 security fix)
    pub pending_authority: Pubkey,
    /// Staking program
    pub staking_program: Pubkey,
    /// 5A Protocol program
    pub five_a_program: Pubkey,
    /// veVCoin required to propose
    pub proposal_threshold: u64,
    /// Minimum votes for valid proposal
    pub quorum: u64,
    /// Voting period in seconds
    pub voting_period: i64,
    /// Timelock delay before execution
    pub timelock_delay: i64,
    /// Total proposals created
    pub proposal_count: u64,
    /// Treasury balance (200M VCoin)
    pub treasury_balance: u64,
    /// Whether governance is paused
    pub paused: bool,
    /// PDA bump
    pub bump: u8,
}

impl GovernanceConfig {
    pub const LEN: usize = 8 + // discriminator
        32 + // authority
        32 + // pending_authority (NEW - H-02)
        32 + // staking_program
        32 + // five_a_program
        8 +  // proposal_threshold
        8 +  // quorum
        8 +  // voting_period
        8 +  // timelock_delay
        8 +  // proposal_count
        8 +  // treasury_balance
        1 +  // paused
        1;   // bump
}

