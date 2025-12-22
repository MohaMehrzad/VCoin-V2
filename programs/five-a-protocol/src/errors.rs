use anchor_lang::prelude::*;

/// 5A Protocol Error Codes
#[error_code]
pub enum FiveAError {
    #[msg("Unauthorized: Only the authority can perform this action")]
    Unauthorized,
    
    #[msg("Protocol is paused")]
    ProtocolPaused,
    
    #[msg("Caller is not a registered oracle")]
    NotOracle,
    
    #[msg("Invalid score value (must be 0-10000)")]
    InvalidScore,
    
    #[msg("Voucher 5A score too low (need 60%+)")]
    VoucherScoreTooLow,
    
    #[msg("User already has 3 vouches")]
    AlreadyFullyVouched,
    
    #[msg("Cannot vouch for self")]
    CannotVouchSelf,
    
    #[msg("Already vouched for this user")]
    AlreadyVouched,
    
    #[msg("Max concurrent vouches reached")]
    MaxVouchesReached,
    
    #[msg("Vouch stake amount incorrect")]
    InvalidStakeAmount,
    
    #[msg("Vouch evaluation period not complete")]
    EvaluationNotComplete,
    
    #[msg("Vouch already evaluated")]
    AlreadyEvaluated,
    
    #[msg("User not found")]
    UserNotFound,
    
    #[msg("Oracle already registered")]
    OracleAlreadyRegistered,
    
    #[msg("Maximum oracles reached")]
    MaxOraclesReached,
    
    #[msg("Arithmetic overflow")]
    Overflow,
    
    // H-02: Two-step authority transfer errors
    #[msg("Not the pending authority")]
    NotPendingAuthority,
    
    #[msg("No pending authority transfer")]
    NoPendingTransfer,
    
    #[msg("Cannot propose self as new authority")]
    CannotProposeSelf,
    
    #[msg("Invalid authority address (zero)")]
    InvalidAuthority,
    
    // H-05: Oracle consensus errors
    #[msg("Score update has expired")]
    ScoreUpdateExpired,
    
    #[msg("Oracle has already submitted for this update")]
    OracleAlreadySubmitted,
    
    #[msg("Score mismatch with pending update")]
    ScoreMismatch,
    
    // L-07: Rate limiting
    #[msg("Score update too frequent - minimum 1 hour between updates for same user")]
    ScoreUpdateTooFrequent,
}

