use anchor_lang::prelude::*;

/// Staking Protocol Error Codes
#[error_code]
pub enum StakingError {
    #[msg("Unauthorized: Only the authority can perform this action")]
    Unauthorized,
    
    #[msg("Staking pool is paused")]
    PoolPaused,
    
    #[msg("Cannot stake zero tokens")]
    ZeroStakeAmount,
    
    #[msg("Lock duration below minimum (1 week)")]
    LockDurationTooShort,
    
    #[msg("Lock duration exceeds maximum (4 years)")]
    LockDurationTooLong,
    
    #[msg("Tokens are still locked")]
    TokensStillLocked,
    
    #[msg("Cannot unstake more than staked amount")]
    InsufficientStake,
    
    #[msg("Cannot extend lock to a shorter duration")]
    CannotShortenLock,
    
    #[msg("New lock end must be after current lock end")]
    InvalidLockExtension,
    
    #[msg("Arithmetic overflow")]
    Overflow,
    
    #[msg("User has no active stake")]
    NoActiveStake,
    
    #[msg("Invalid token account owner")]
    InvalidTokenAccount,
    
    #[msg("Invalid token mint")]
    InvalidMint,
    
    #[msg("veVCoin mint CPI failed")]
    VeVCoinMintFailed,
    
    #[msg("veVCoin burn CPI failed")]
    VeVCoinBurnFailed,
    
    // H-02: Two-step authority transfer errors
    #[msg("Not the pending authority")]
    NotPendingAuthority,
    
    #[msg("No pending authority transfer")]
    NoPendingTransfer,
    
    #[msg("Cannot propose self as new authority")]
    CannotProposeSelf,
    
    #[msg("Invalid authority address (zero)")]
    InvalidAuthority,
    
    // M-01: Reentrancy protection
    #[msg("Reentrancy detected - operation already in progress")]
    ReentrancyDetected,
}

