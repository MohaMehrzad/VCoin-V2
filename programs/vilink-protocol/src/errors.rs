use anchor_lang::prelude::*;

#[error_code]
pub enum ViLinkError {
    #[msg("Unauthorized: Only the authority can perform this action")]
    Unauthorized,
    #[msg("ViLink Protocol is paused")]
    ProtocolPaused,
    #[msg("Action has expired")]
    ActionExpired,
    #[msg("Action already executed")]
    ActionAlreadyExecuted,
    #[msg("Invalid action type")]
    InvalidActionType,
    #[msg("Invalid action amount")]
    InvalidAmount,
    #[msg("Action creator cannot execute own action")]
    SelfExecutionNotAllowed,
    #[msg("dApp not registered")]
    DAppNotRegistered,
    #[msg("Batch size exceeds maximum")]
    BatchTooLarge,
    #[msg("Action not found")]
    ActionNotFound,
    #[msg("Insufficient balance")]
    InsufficientBalance,
    #[msg("Action type disabled")]
    ActionTypeDisabled,
    #[msg("Target user not valid")]
    InvalidTarget,
    #[msg("Arithmetic overflow")]
    Overflow,
    #[msg("Invalid token account owner")]
    InvalidTokenAccount,
    #[msg("Invalid token mint")]
    InvalidMint,
    #[msg("Invalid treasury account")]
    InvalidTreasury,
    
    // H-02: Two-step authority transfer errors
    #[msg("Not the pending authority")]
    NotPendingAuthority,
    
    #[msg("No pending authority transfer")]
    NoPendingTransfer,
    
    #[msg("Cannot propose self as new authority")]
    CannotProposeSelf,
    
    #[msg("Invalid authority address (zero)")]
    InvalidAuthority,
    
    // M-02: Platform fee bounds validation
    #[msg("Platform fee must be between 0.1% and 10%")]
    InvalidFeeRange,
    
    // M-04: Nonce validation for deterministic PDA derivation
    #[msg("Invalid nonce: must match expected action_nonce from user stats")]
    InvalidNonce,
}

