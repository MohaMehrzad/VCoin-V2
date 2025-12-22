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
    
    // H-02: Two-step authority transfer errors
    #[msg("Not the pending authority")]
    NotPendingAuthority,
    
    #[msg("No pending authority transfer")]
    NoPendingTransfer,
    
    #[msg("Cannot propose self as new authority")]
    CannotProposeSelf,
    
    #[msg("Invalid authority address (zero)")]
    InvalidAuthority,
    
    // H-03: Session key signature error
    #[msg("Invalid session signer - session key must sign")]
    InvalidSessionSigner,
    
    // L-03: Slippage protection
    #[msg("Fee slippage exceeded maximum allowed")]
    SlippageExceeded,
}

