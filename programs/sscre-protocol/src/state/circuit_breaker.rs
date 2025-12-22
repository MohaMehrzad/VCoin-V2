use anchor_lang::prelude::*;

/// Circuit breaker state
#[account]
#[derive(Default)]
pub struct CircuitBreaker {
    /// Authority
    pub authority: Pubkey,
    /// Whether circuit breaker is active
    pub is_active: bool,
    /// Max emission per epoch
    pub max_epoch_emission: u64,
    /// Max single claim
    pub max_single_claim: u64,
    /// Current epoch emission so far
    pub current_epoch_emission: u64,
    /// Largest claim this epoch
    pub largest_claim_this_epoch: u64,
    /// Number of triggers
    pub trigger_count: u32,
    /// Last trigger timestamp
    pub last_trigger_at: i64,
    /// Trigger reason
    pub last_trigger_reason: u8,
    /// PDA bump
    pub bump: u8,
}

impl CircuitBreaker {
    pub const LEN: usize = 8 + // discriminator
        32 + // authority
        1 +  // is_active
        8 +  // max_epoch_emission
        8 +  // max_single_claim
        8 +  // current_epoch_emission
        8 +  // largest_claim_this_epoch
        4 +  // trigger_count
        8 +  // last_trigger_at
        1 +  // last_trigger_reason
        1;   // bump
}

