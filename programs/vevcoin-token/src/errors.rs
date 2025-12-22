use anchor_lang::prelude::*;

/// veVCoin Token Error Codes
#[error_code]
pub enum VeVCoinError {
    #[msg("Unauthorized: Only the staking protocol can perform this action")]
    Unauthorized,
    
    #[msg("Mint is already initialized")]
    MintAlreadyInitialized,
    
    #[msg("Cannot transfer soulbound tokens")]
    TransferNotAllowed,
    
    #[msg("Invalid staking protocol")]
    InvalidStakingProtocol,
    
    #[msg("Cannot burn more tokens than balance")]
    InsufficientBalance,
    
    #[msg("Cannot mint/burn zero tokens")]
    ZeroAmount,
    
    #[msg("Invalid token account owner")]
    InvalidTokenAccount,
    
    #[msg("Invalid token mint")]
    InvalidMint,
}

