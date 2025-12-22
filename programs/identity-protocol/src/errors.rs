use anchor_lang::prelude::*;

/// Identity Protocol Error Codes
#[error_code]
pub enum IdentityError {
    #[msg("Unauthorized: Only the authority can perform this action")]
    Unauthorized,
    
    #[msg("Identity protocol is paused")]
    ProtocolPaused,
    
    #[msg("Identity already exists for this wallet")]
    IdentityAlreadyExists,
    
    #[msg("Identity does not exist")]
    IdentityNotFound,
    
    #[msg("Invalid verification level")]
    InvalidVerificationLevel,
    
    #[msg("Verification level cannot be downgraded")]
    CannotDowngradeVerification,
    
    #[msg("SAS attestation required for this verification level")]
    SASAttestationRequired,
    
    #[msg("SAS attestation has expired")]
    SASAttestationExpired,
    
    #[msg("Invalid subscription tier")]
    InvalidSubscriptionTier,
    
    #[msg("Subscription has expired")]
    SubscriptionExpired,
    
    #[msg("Insufficient payment for subscription")]
    InsufficientPayment,
    
    #[msg("Attestation not from trusted attester")]
    UntrustedAttester,
    
    #[msg("Arithmetic overflow")]
    Overflow,
}

