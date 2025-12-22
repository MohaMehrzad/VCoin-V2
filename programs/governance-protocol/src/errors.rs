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
}

