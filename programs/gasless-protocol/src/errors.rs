use anchor_lang::prelude::*;

#[error_code]
pub enum GaslessError {
    #[msg("Gasless protocol is paused")]
    Paused,
    #[msg("Scope must be non-empty and use only allowed bits")]
    InvalidScope,
    #[msg("Session validity must be in the future and at most 24h")]
    InvalidValidity,
    #[msg("Session has expired")]
    SessionExpired,
    #[msg("Session does not belong to this authority, signer or program")]
    SessionMismatch,
    #[msg("Exactly one in-scope action bit must be requested")]
    ActionNotInScope,
    #[msg("Only the session authority may revoke an unexpired session")]
    RevokeNotAllowed,
    #[msg("Fee exceeds the caller's max_fee")]
    FeeAboveMax,
    #[msg("Fee above the allowed maximum")]
    FeeTooHigh,
    #[msg("No change is scheduled")]
    NothingScheduled,
    #[msg("The scheduled change is not yet effective")]
    TimelockActive,
    #[msg("Program data account does not belong to this program")]
    InvalidProgramData,
    #[msg("Amount must be greater than zero")]
    ZeroAmount,
    #[msg("Arithmetic overflow")]
    Overflow,
}
