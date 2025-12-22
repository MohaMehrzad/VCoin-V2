use anchor_lang::prelude::*;

/// VCoin Token Error Codes
#[error_code]
pub enum VCoinError {
    #[msg("Unauthorized: Only the authority can perform this action")]
    Unauthorized,
    
    #[msg("Mint is already initialized")]
    MintAlreadyInitialized,
    
    #[msg("Invalid mint authority")]
    InvalidMintAuthority,
    
    #[msg("Exceeds maximum supply")]
    ExceedsMaxSupply,
    
    #[msg("Slashing amount exceeds balance")]
    SlashingExceedsBalance,
    
    #[msg("Cannot slash zero tokens")]
    ZeroSlashAmount,
    
    #[msg("Token is paused")]
    TokenPaused,
    
    #[msg("Invalid token mint")]
    InvalidMint,
}

