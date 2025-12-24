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
    
    // H-02: Two-step authority transfer errors
    #[msg("Not the pending authority")]
    NotPendingAuthority,
    
    #[msg("No pending authority transfer")]
    NoPendingTransfer,
    
    #[msg("Cannot propose self as new authority")]
    CannotProposeSelf,
    
    #[msg("Invalid authority address (zero)")]
    InvalidAuthority,
    
    // H-01: Slashing governance errors
    #[msg("Slash request not found")]
    SlashRequestNotFound,
    
    #[msg("Slash request not approved")]
    SlashRequestNotApproved,
    
    #[msg("Slash request already executed")]
    SlashRequestAlreadyExecuted,
    
    #[msg("Timelock not expired - must wait 48 hours after approval")]
    TimelockNotExpired,
    
    #[msg("Invalid slash request status")]
    InvalidSlashStatus,
    
    #[msg("Only governance can approve slash requests")]
    GovernanceApprovalRequired,
    
    // C-NEW-02: Legacy slash function deprecated
    #[msg("This function is deprecated. Use propose_slash -> approve_slash -> execute_slash flow")]
    DeprecatedSlashFunction,
    
    // H-NEW-01: Authority transfer timelock
    #[msg("Authority transfer timelock not elapsed - must wait 24 hours after proposal")]
    AuthorityTransferTimelock,
    
    // C-01: Slash PDA seed validation
    #[msg("Invalid request_id: must equal current timestamp for PDA consistency")]
    InvalidRequestId,
    
    // H-04: Slash target binding
    #[msg("Target account does not match the specified target address")]
    InvalidTarget,
}

