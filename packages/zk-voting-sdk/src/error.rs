use zk_poll_core::ZkError;

#[derive(Debug, thiserror::Error, PartialEq, Eq)]
pub enum SdkError {
    #[error("verifier rejected the artifact: {0}")]
    Zk(#[from] ZkError),
    #[error("bytes are not a valid Ristretto point")]
    InvalidPoint,
    #[error("committee index {0} is outside 1..=size")]
    InvalidIndex(u8),
    #[error("dealer {dealer} sent an invalid share, commitments or proof of knowledge; exclude it and rerun the DKG")]
    BadDealing { dealer: u8 },
    #[error("members {members:?} arranged for an identity or shared key; exclude them and rerun the DKG")]
    DegenerateMemberKeys { members: Vec<u8> },
    #[error("expected exactly one dealing and one share from each of the {expected} members")]
    IncompleteDealings { expected: u8 },
    #[error("committee index {0} appears more than once")]
    DuplicateIndex(u8),
    #[error("need at least {needed} partial decryptions, got {got}")]
    NotEnoughShares { needed: u8, got: usize },
    #[error("a tally exceeds the search bound {0}")]
    TallyOutOfRange(u64),
    #[error("weight must be positive")]
    ZeroWeight,
    #[error("unknown choice {0}")]
    InvalidChoice(u8),
}

pub type SdkResult<T> = Result<T, SdkError>;
