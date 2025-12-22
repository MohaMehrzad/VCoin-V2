use anchor_lang::prelude::*;

/// Pending score update awaiting multi-oracle consensus (H-05 Security Fix)
/// 
/// Requires M-of-N oracles to agree on the exact same scores before
/// the update is applied to the user's score account.
#[account]
#[derive(Default)]
pub struct PendingScoreUpdate {
    /// User whose score is being updated
    pub user: Pubkey,
    /// Proposed authenticity score
    pub authenticity: u16,
    /// Proposed accuracy score
    pub accuracy: u16,
    /// Proposed agility score
    pub agility: u16,
    /// Proposed activity score
    pub activity: u16,
    /// Proposed approved score
    pub approved: u16,
    /// Oracles that have confirmed this exact score set
    pub confirming_oracles: [Pubkey; 5],
    /// Number of confirming oracles
    pub confirmation_count: u8,
    /// Timestamp when first oracle submitted
    pub initiated_at: i64,
    /// Expiry timestamp (1 hour after initiation)
    pub expires_at: i64,
    /// Whether update has been applied to user score
    pub is_applied: bool,
    /// PDA bump
    pub bump: u8,
}

impl PendingScoreUpdate {
    pub const LEN: usize = 8 + // discriminator
        32 + // user
        2 +  // authenticity
        2 +  // accuracy
        2 +  // agility
        2 +  // activity
        2 +  // approved
        (32 * 5) + // confirming_oracles
        1 +  // confirmation_count
        8 +  // initiated_at
        8 +  // expires_at
        1 +  // is_applied
        1;   // bump
    
    /// Check if an oracle has already submitted
    pub fn has_oracle_submitted(&self, oracle: &Pubkey) -> bool {
        for i in 0..self.confirmation_count as usize {
            if self.confirming_oracles[i] == *oracle {
                return true;
            }
        }
        false
    }
    
    /// Add confirming oracle
    pub fn add_confirming_oracle(&mut self, oracle: Pubkey) {
        if (self.confirmation_count as usize) < 5 {
            self.confirming_oracles[self.confirmation_count as usize] = oracle;
            self.confirmation_count += 1;
        }
    }
    
    /// Check if scores match
    pub fn scores_match(&self, authenticity: u16, accuracy: u16, agility: u16, activity: u16, approved: u16) -> bool {
        self.authenticity == authenticity &&
        self.accuracy == accuracy &&
        self.agility == agility &&
        self.activity == activity &&
        self.approved == approved
    }
}

