use anchor_lang::prelude::*;

#[error_code]
pub enum ContentError {
    #[msg("Content registry is paused")]
    Paused,
    #[msg("Program data account does not belong to this program")]
    InvalidProgramData,
    #[msg("Invalid key")]
    InvalidKey,
    #[msg("Signer is neither the author nor a valid content session for the author")]
    Unauthorized,
    #[msg("URI longer than 128 bytes")]
    UriTooLong,
    #[msg("Unknown content type")]
    InvalidContentType,
    #[msg("Not enough energy")]
    InsufficientEnergy,
    #[msg("Daily post cap reached")]
    DailyCapExceeded,
    #[msg("Signer is not the engagement oracle")]
    NotEngagementOracle,
    #[msg("Refunds open 24h after posting")]
    RefundTooEarly,
    #[msg("The 7-day refund window has closed")]
    RefundWindowClosed,
    #[msg("Engagement does not qualify for a refund yet")]
    RefundNotReady,
    #[msg("Refund already claimed")]
    RefundAlreadyClaimed,
    #[msg("Daily refund cap reached")]
    DailyRefundCapExceeded,
    #[msg("No oracle change pending")]
    NoPendingChange,
    #[msg("Oracle change timelock still active")]
    TimelockActive,
    #[msg("Arithmetic overflow")]
    Overflow,
}
