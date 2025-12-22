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
}

