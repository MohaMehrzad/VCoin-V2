use anchor_lang::prelude::*;

#[error_code]
pub enum GaslessError {
    #[msg("Unauthorized: Only the authority can perform this action")]
    Unauthorized,
    #[msg("Gasless Protocol is paused")]
    ProtocolPaused,
    #[msg("Session key expired")]
    SessionExpired,
    #[msg("Session key revoked")]
    SessionRevoked,
    #[msg("Action not in session scope")]
    ActionNotInScope,
    #[msg("Session action limit exceeded")]
    SessionActionLimitExceeded,
    #[msg("Session spend limit exceeded")]
    SessionSpendLimitExceeded,
    #[msg("Daily subsidy budget exceeded")]
    DailyBudgetExceeded,
    #[msg("User daily limit exceeded")]
    UserDailyLimitExceeded,
    #[msg("Insufficient VCoin balance for fee")]
    InsufficientVCoinBalance,
    #[msg("Invalid session key")]
    InvalidSessionKey,
    #[msg("Session already exists")]
    SessionAlreadyExists,
    #[msg("Fee deduction method not allowed")]
    FeeMethodNotAllowed,
    #[msg("Invalid action type")]
    InvalidActionType,
    #[msg("Arithmetic overflow")]
    Overflow,
    #[msg("Invalid token account owner")]
    InvalidTokenAccount,
    #[msg("Invalid token mint")]
    InvalidMint,
}

