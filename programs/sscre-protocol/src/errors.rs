use anchor_lang::prelude::*;

#[error_code]
pub enum SSCREError {
    #[msg("Unauthorized: Only the authority can perform this action")]
    Unauthorized,
    #[msg("SSCRE Protocol is paused")]
    ProtocolPaused,
    #[msg("Invalid merkle proof")]
    InvalidMerkleProof,
    #[msg("Already claimed for this epoch")]
    AlreadyClaimed,
    #[msg("Claim window expired")]
    ClaimWindowExpired,
    #[msg("Epoch not finalized")]
    EpochNotFinalized,
    #[msg("Insufficient pool balance")]
    InsufficientPoolBalance,
    #[msg("Claim amount below minimum")]
    ClaimBelowMinimum,
    #[msg("Circuit breaker triggered: max epoch emission exceeded")]
    CircuitBreakerEpochMax,
    #[msg("Circuit breaker triggered: max single claim exceeded")]
    CircuitBreakerClaimMax,
    #[msg("Invalid epoch number")]
    InvalidEpoch,
    #[msg("Epoch already exists")]
    EpochAlreadyExists,
    #[msg("Oracle not registered")]
    OracleNotRegistered,
    #[msg("Funding layer inactive")]
    FundingLayerInactive,
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
    
    // H-04: Epoch claim bitmap errors
    #[msg("Too many epochs claimed - high epoch array full")]
    TooManyEpochsClaimed,
    
    // M-05: Circuit breaker cooldown
    #[msg("Circuit breaker cooldown not elapsed - wait 6 hours after trigger")]
    CircuitBreakerCooldown,
    
    // H-NEW-02: Merkle proof size limit
    #[msg("Merkle proof too large - maximum 32 levels (supports 4 billion users)")]
    MerkleProofTooLarge,
}

