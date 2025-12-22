use anchor_lang::prelude::*;

/// Transfer Hook Error Codes
#[error_code]
pub enum TransferHookError {
    #[msg("Unauthorized: Only the authority can perform this action")]
    Unauthorized,
    
    #[msg("Transfer hook is paused")]
    HookPaused,
    
    #[msg("Invalid program owner")]
    InvalidProgramOwner,
    
    #[msg("Wash trading pattern detected")]
    WashTradingDetected,
    
    #[msg("Arithmetic overflow")]
    Overflow,
    
    #[msg("Invalid extra account metas")]
    InvalidExtraAccountMetas,
    
    #[msg("Invalid token mint")]
    InvalidMint,
}

