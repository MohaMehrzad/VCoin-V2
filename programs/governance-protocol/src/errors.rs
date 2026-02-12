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

    #[msg("Insufficient decryption shares - threshold not met")]
    InsufficientDecryptionShares,

    #[msg("Invalid DecryptionShare PDA - account does not match expected derivation")]
    InvalidDecryptionSharePDA,
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
    
    // C-NEW-01: On-chain voting power verification errors
    #[msg("Invalid UserStake PDA - must be derived from staking program")]
    InvalidUserStakePDA,
    
    #[msg("Invalid UserScore PDA - must be derived from five-a program")]
    InvalidUserScorePDA,
    
    #[msg("UserStake account data is invalid or uninitialized")]
    InvalidUserStakeData,
    
    #[msg("UserScore account data is invalid or uninitialized")]
    InvalidUserScoreData,
    
    // H-NEW-03: Delegation amount validation
    #[msg("Claimed veVCoin balance exceeds delegated amount")]
    ExceedsDelegatedAmount,

    // H-02: Authority transfer timelock
    #[msg("Authority transfer timelock has not elapsed (24h required)")]
    AuthorityTransferTimelock,

    // ZK cryptographic verification errors
    #[msg("Invalid Ristretto point in ciphertext or proof")]
    InvalidRistrettoPoint,

    #[msg("Vote validity OR proof verification failed")]
    InvalidOrProof,

    #[msg("Vote validity sum proof verification failed")]
    InvalidSumProof,

    #[msg("Decryption share DLEQ proof verification failed")]
    InvalidDleqProof,

    #[msg("Tally verification failed: tally * H != C_sum - D")]
    TallyVerificationFailed,
}

