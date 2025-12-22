use anchor_lang::prelude::*;

/// Delegate statistics
#[account]
#[derive(Default)]
pub struct DelegateStats {
    /// Delegate wallet
    pub delegate: Pubkey,
    /// League tier (0=Bronze, 1=Silver, 2=Gold, 3=Diamond)
    pub league_tier: u8,
    /// Total proposals voted on
    pub total_proposals_voted: u32,
    /// Proposals voted with winning outcome
    pub proposals_with_outcome: u32,
    /// Voting accuracy (0-10000)
    pub voting_accuracy: u16,
    /// Participation rate (0-10000)
    pub participation_rate: u16,
    /// Number of unique delegators
    pub unique_delegators: u32,
    /// Total veVCoin delegated to this delegate
    pub total_delegated_vevcoin: u64,
    /// Delegator satisfaction score (0-10000)
    pub delegator_satisfaction: u16,
    /// Last vote timestamp
    pub last_vote_at: i64,
    /// Tier last updated
    pub tier_updated_at: i64,
    /// Whether eligible for promotion
    pub promotion_eligible: bool,
    /// Whether warned about demotion
    pub demotion_warning: bool,
    /// PDA bump
    pub bump: u8,
}

impl DelegateStats {
    pub const LEN: usize = 8 + // discriminator
        32 + // delegate
        1 +  // league_tier
        4 +  // total_proposals_voted
        4 +  // proposals_with_outcome
        2 +  // voting_accuracy
        2 +  // participation_rate
        4 +  // unique_delegators
        8 +  // total_delegated_vevcoin
        2 +  // delegator_satisfaction
        8 +  // last_vote_at
        8 +  // tier_updated_at
        1 +  // promotion_eligible
        1 +  // demotion_warning
        1;   // bump
    
    /// Get max delegation percent based on tier
    pub fn max_delegation_pct(&self) -> u16 {
        match self.league_tier {
            0 => 100,  // 1% Bronze
            1 => 300,  // 3% Silver
            2 => 500,  // 5% Gold
            3 => 1000, // 10% Diamond
            _ => 100,
        }
    }
}

