use anchor_lang::prelude::*;

#[error_code]
pub enum ViLinkError {
    #[msg("ViLink is paused")]
    Paused,
    #[msg("Program data account does not belong to this program")]
    InvalidProgramData,
    #[msg("VCoin mint must be the vcoin-token program's mint PDA")]
    InvalidVCoinMint,
    #[msg("Fee must be between 0.1% and 10%")]
    InvalidFee,
    #[msg("Action type is unknown or disabled")]
    ActionDisabled,
    #[msg("Wrong action type for this instruction")]
    WrongActionType,
    #[msg("Tip amount must be between 0.1 and 10,000 VCoin; other actions carry no amount")]
    InvalidAmount,
    #[msg("Invalid target")]
    InvalidTarget,
    #[msg("Expiry must be between 1 hour and 7 days")]
    InvalidExpiry,
    #[msg("max_executions must be between 1 and 10,000")]
    InvalidMaxExecutions,
    #[msg("Nonce must equal the creator's actions_created")]
    InvalidNonce,
    #[msg("Action expired or fully executed")]
    ActionClosed,
    #[msg("Action is still open")]
    ActionStillOpen,
    #[msg("The creator or target cannot execute this action")]
    InvalidExecutor,
    #[msg("Signer is neither the executor nor a valid session for the executor")]
    Unauthorized,
    #[msg("No config change pending")]
    NoPendingChange,
    #[msg("Config change timelock still active")]
    TimelockActive,
    #[msg("Arithmetic overflow")]
    Overflow,
}
