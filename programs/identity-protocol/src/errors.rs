use anchor_lang::prelude::*;

#[error_code]
pub enum IdentityError {
    #[msg("Identity is paused")]
    Paused,
    #[msg("Program data account does not belong to this program")]
    InvalidProgramData,
    #[msg("USDC mint must have 6 decimals")]
    InvalidUsdcMint,
    #[msg("Usernames are 3-32 characters of a-z, 0-9 and _")]
    InvalidUsername,
    #[msg("Invalid attester key")]
    InvalidAttester,
    #[msg("Attester already trusted")]
    AttesterExists,
    #[msg("All attester slots are used")]
    TooManyAttesters,
    #[msg("Attester not found")]
    AttesterNotFound,
    #[msg("Signer is not a trusted attester")]
    NotTrustedAttester,
    #[msg("An attester cannot attest itself")]
    SelfAttestation,
    #[msg("Unknown claim bits")]
    InvalidClaims,
    #[msg("Expiry must be in the future and at most two years away")]
    InvalidExpiry,
    #[msg("This attestation is linked to another wallet")]
    AttestationAlreadyLinked,
    #[msg("Only the issuing attester or the authority may revoke")]
    NotRevoker,
    #[msg("Admin can only lower a level")]
    NotADowngrade,
    #[msg("Tier must be 1, 2 or 3")]
    InvalidTier,
    #[msg("Cannot downgrade while a higher tier is active")]
    CannotDowngradeActive,
    #[msg("Price exceeds the maximum you signed")]
    PriceAboveMax,
    #[msg("No payment change pending")]
    NoPendingChange,
    #[msg("Payment change timelock still active")]
    TimelockActive,
    #[msg("Only the attester that issued this attestation may refresh it")]
    AttesterMismatch,
    #[msg("Arithmetic overflow")]
    Overflow,
}
