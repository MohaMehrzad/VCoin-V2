use anchor_lang::prelude::*;

use crate::constants::NUM_TIERS;

#[event]
pub struct IdentityInitialized {
    pub authority: Pubkey,
    pub usdc_mint: Pubkey,
    pub treasury_token: Pubkey,
}

#[event]
pub struct AttesterAdded {
    pub attester: Pubkey,
}

#[event]
pub struct AttesterRemoved {
    pub attester: Pubkey,
}

#[event]
pub struct PauseChanged {
    pub paused: bool,
}

#[event]
pub struct PaymentChangeProposed {
    pub prices: [u64; NUM_TIERS],
    pub treasury_token: Pubkey,
    pub eta: i64,
}

#[event]
pub struct PaymentChangeApplied {
    pub prices: [u64; NUM_TIERS],
    pub treasury_token: Pubkey,
}

#[event]
pub struct PaymentChangeCancelled {}

#[event]
pub struct IdentityCreated {
    pub owner: Pubkey,
    pub username: String,
    pub did_hash: [u8; 32],
}

#[event]
pub struct DidHashUpdated {
    pub owner: Pubkey,
    pub did_hash: [u8; 32],
}

#[event]
pub struct IdentityClosed {
    pub owner: Pubkey,
}

#[event]
pub struct AttestationLinked {
    pub owner: Pubkey,
    pub attester: Pubkey,
    pub attestation_id: [u8; 32],
    pub claims: u16,
    pub level: u8,
    pub expires_at: i64,
}

#[event]
pub struct AttestationRevoked {
    pub owner: Pubkey,
    pub attestation_id: [u8; 32],
    pub revoked_by: Pubkey,
}

#[event]
pub struct LevelLowered {
    pub owner: Pubkey,
    pub old_level: u8,
    pub new_level: u8,
}

#[event]
pub struct Subscribed {
    pub user: Pubkey,
    pub tier: u8,
    pub paid: u64,
    pub expires_at: i64,
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
