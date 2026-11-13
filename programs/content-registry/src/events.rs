use anchor_lang::prelude::*;

#[event]
pub struct RegistryInitialized {
    pub authority: Pubkey,
    pub engagement_oracle: Pubkey,
}

#[event]
pub struct OracleChangeProposed {
    pub oracle: Pubkey,
    pub eta: i64,
}

#[event]
pub struct OracleChanged {
    pub oracle: Pubkey,
}

#[event]
pub struct OracleChangeCancelled {}

#[event]
pub struct PauseChanged {
    pub paused: bool,
}

#[event]
pub struct EnergyInitialized {
    pub user: Pubkey,
    pub tier: u8,
}

#[event]
pub struct TierChanged {
    pub user: Pubkey,
    pub old_tier: u8,
    pub new_tier: u8,
}

#[event]
pub struct EnergySpent {
    pub user: Pubkey,
    pub amount: u16,
    pub remaining: u16,
}

/// Every version's hash is in the event log (the record keeps only the latest).
#[event]
pub struct ContentCreated {
    pub author: Pubkey,
    pub tracking_id: [u8; 32],
    pub content_hash: [u8; 32],
    pub content_type: u8,
    pub signer: Pubkey,
}

#[event]
pub struct ContentEdited {
    pub author: Pubkey,
    pub tracking_id: [u8; 32],
    pub previous_hash: [u8; 32],
    pub content_hash: [u8; 32],
    pub version: u16,
    pub signer: Pubkey,
}

#[event]
pub struct ContentDeleted {
    pub author: Pubkey,
    pub tracking_id: [u8; 32],
    pub content_hash: [u8; 32],
}

#[event]
pub struct EngagementUpdated {
    pub author: Pubkey,
    pub tracking_id: [u8; 32],
    pub engagement_count: u32,
}

#[event]
pub struct RefundClaimed {
    pub author: Pubkey,
    pub tracking_id: [u8; 32],
    pub percent: u16,
    pub amount: u16,
}

#[event]
pub struct AuthorityProposed {
    pub current: Pubkey,
    pub proposed: Pubkey,
    pub ready_at: i64,
}

#[event]
pub struct AuthorityTransferCancelled {
    pub authority: Pubkey,
}

#[event]
pub struct AuthorityTransferred {
    pub previous: Pubkey,
    pub new_authority: Pubkey,
}
