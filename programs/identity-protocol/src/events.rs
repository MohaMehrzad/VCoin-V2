use anchor_lang::prelude::*;

#[event]
pub struct IdentityCreated {
    pub owner: Pubkey,
    pub username: String,
    pub timestamp: i64,
}

#[event]
pub struct VerificationUpdated {
    pub owner: Pubkey,
    pub old_level: u8,
    pub new_level: u8,
    pub timestamp: i64,
}

#[event]
pub struct SASAttestationLinked {
    pub user: Pubkey,
    pub attestation_id: Pubkey,
    pub attester: Pubkey,
    pub verification_level: u8,
}

#[event]
pub struct SubscriptionUpdated {
    pub user: Pubkey,
    pub tier: u8,
    pub expires_at: i64,
}

