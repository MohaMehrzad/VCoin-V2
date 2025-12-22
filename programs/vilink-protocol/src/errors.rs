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
}

