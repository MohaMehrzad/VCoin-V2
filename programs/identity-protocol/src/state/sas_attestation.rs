use anchor_lang::prelude::*;

/// User's SAS attestation link
#[account]
#[derive(Default)]
pub struct UserSASAttestation {
    /// User wallet
    pub user: Pubkey,
    /// SAS attestation account PDA
    pub sas_attestation_id: Pubkey,
    /// Attestation type (0=Email, 1=Phone, 2=KYC, 3=Biometric, 4=UniqueHuman)
    pub attestation_type: u8,
    /// Who issued the attestation
    pub attester: Pubkey,
    /// Bitmap of verified claims
    pub verified_claims: u16,
    /// Derived verification level from claims
    pub verification_level: u8,
    /// First verification timestamp
    pub first_verified_at: i64,
    /// Last verification timestamp
    pub last_verified_at: i64,
    /// Attestation expiry
    pub expires_at: i64,
    /// Portable score from other dApps (0-10000)
    pub portable_score: u16,
    /// PDA bump
    pub bump: u8,
}

impl UserSASAttestation {
    pub const LEN: usize = 8 + // discriminator
        32 + // user
        32 + // sas_attestation_id
        1 +  // attestation_type
        32 + // attester
        2 +  // verified_claims
        1 +  // verification_level
        8 +  // first_verified_at
        8 +  // last_verified_at
        8 +  // expires_at
        2 +  // portable_score
        1;   // bump
}

