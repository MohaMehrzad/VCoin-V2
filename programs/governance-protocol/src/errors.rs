use anchor_lang::prelude::*;

#[error_code]
pub enum GovernanceError {
    #[msg("Unauthorized: Only the authority can perform this action")]
    Unauthorized,
    #[msg("Governance is paused")]
    GovernancePaused,
    #[msg("Insufficient veVCoin to create proposal")]
    InsufficientVeVCoin,
    #[msg("Proposal not found")]
    ProposalNotFound,
    #[msg("Voting period has not started")]
    VotingNotStarted,
    #[msg("Voting period has ended")]
    VotingEnded,
    #[msg("Voting period has not ended")]
    VotingNotEnded,
    #[msg("Already voted on this proposal")]
    AlreadyVoted,
    #[msg("Invalid vote choice")]
    InvalidVoteChoice,
    #[msg("Quorum not reached")]
    QuorumNotReached,
    #[msg("Proposal already executed")]
    ProposalAlreadyExecuted,
    #[msg("Timelock not expired")]
    TimelockNotExpired,
    #[msg("Cannot delegate to self")]
    CannotDelegateSelf,
    #[msg("Delegation already exists")]
    DelegationExists,
    #[msg("Delegation not found")]
    DelegationNotFound,
    #[msg("ZK voting not enabled for this proposal")]
    ZKVotingNotEnabled,
    #[msg("ZK reveal not started")]
    RevealNotStarted,
    #[msg("ZK reveal already complete")]
    RevealAlreadyComplete,
    #[msg("Invalid decryption share")]
    InvalidDecryptionShare,
    #[msg("Invalid ZK proof")]
    InvalidZKProof,
    #[msg("Arithmetic overflow")]
    Overflow,
    #[msg("Decryption share already submitted by this committee member")]
    DecryptionShareAlreadySubmitted,
    #[msg("Invalid committee index")]
    InvalidCommitteeIndex,
    
    // H-02: Two-step authority transfer errors
    #[msg("Not the pending authority")]
    NotPendingAuthority,
    
    #[msg("No pending authority transfer")]
    NoPendingTransfer,
    
    #[msg("Cannot propose self as new authority")]
    CannotProposeSelf,
    
    #[msg("Invalid authority address (zero)")]
    InvalidAuthority,
    
    // M-07: Delegation expiry enforcement
    #[msg("Delegation has expired and cannot be used for voting")]
    DelegationExpired,
    
    #[msg("Delegation is not active")]
    DelegationNotActive,
    
    // L-04: URI validation
    #[msg("Invalid description URI format (must start with ipfs://, https://, or ar://)")]
    InvalidDescriptionUri,
}

