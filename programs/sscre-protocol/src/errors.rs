use anchor_lang::prelude::*;

#[error_code]
pub enum SscreError {
    #[msg("Rewards are paused")]
    Paused,
    #[msg("Program data account does not belong to this program")]
    InvalidProgramData,
    #[msg("VCoin mint must be the vcoin-token program's mint PDA")]
    InvalidVCoinMint,
    #[msg("Parameters out of bounds")]
    InvalidParams,
    #[msg("Fee above 1%")]
    FeeTooHigh,
    #[msg("Invalid oracle key")]
    InvalidOracle,
    #[msg("Oracle already registered")]
    OracleExists,
    #[msg("All oracle slots are used")]
    TooManyOracles,
    #[msg("Oracle not registered")]
    OracleNotFound,
    #[msg("Quorum must be between 2 and the number of registered oracles")]
    InvalidQuorum,
    #[msg("Signer is not an active oracle")]
    NotActiveOracle,
    #[msg("The previous epoch has not run its full duration")]
    EpochTooSoon,
    #[msg("Reserve is empty")]
    ReserveEmpty,
    #[msg("Budget above the configured ceiling")]
    BudgetTooHigh,
    #[msg("The epoch has not ended")]
    EpochNotEnded,
    #[msg("Epoch is not accepting root votes")]
    NotVoting,
    #[msg("Root must be non-zero and max_total_claim within (0, budget]")]
    InvalidRoot,
    #[msg("No root has reached quorum")]
    NoProposedRoot,
    #[msg("Dispute delay has not passed")]
    DisputeDelayActive,
    #[msg("The root deadline passed; claw the budget back instead")]
    RootDeadlinePassed,
    #[msg("Proposed root lost quorum")]
    QuorumLost,
    #[msg("Epoch has no active root")]
    NotActive,
    #[msg("Claim window closed")]
    ClaimWindowClosed,
    #[msg("Claim window still open")]
    ClaimWindowOpen,
    #[msg("Amount must be greater than zero")]
    ZeroAmount,
    #[msg("Proof too long")]
    ProofTooLong,
    #[msg("Invalid merkle proof")]
    InvalidProof,
    #[msg("Claim exceeds the epoch's max_total_claim")]
    ExceedsMaxTotalClaim,
    #[msg("Epoch not closed yet")]
    EpochNotClosed,
    #[msg("No fee change pending")]
    NoPendingFee,
    #[msg("Fee change timelock still active")]
    FeeTimelockActive,
    #[msg("Arithmetic overflow")]
    Overflow,
}
