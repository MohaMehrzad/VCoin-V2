use anchor_lang::prelude::*;
use zk_poll_core::ZkError;

#[error_code]
pub enum PollError {
    #[msg("Private polls are paused")]
    Paused,
    #[msg("Program data account does not belong to this program")]
    InvalidProgramData,
    #[msg("Poll parameters out of bounds")]
    InvalidParams,
    #[msg("No decryption committee has been registered")]
    CommitteeNotSet,
    #[msg("Committee must satisfy 3 <= threshold <= size <= 5 with distinct, non-default members")]
    InvalidCommittee,
    #[msg("A committee member key does not match the Feldman commitments")]
    FeldmanMismatch,
    #[msg("Start must be within 30 days and the voting period within the configured bounds")]
    InvalidSchedule,
    #[msg("Creator's lock-weighted stake is below the minimum to open a poll")]
    CreatorWeightTooLow,
    #[msg("Consumer must be set (e.g. the Realms governance PDA)")]
    InvalidConsumer,
    #[msg("Poll is not open for voting")]
    PollNotOpen,
    #[msg("No lock-weighted stake at the poll's end time")]
    NoVotingWeight,
    #[msg("5A reputation score is below the voting gate")]
    ScoreBelowGate,
    #[msg("Ballot proof rejected")]
    InvalidBallotProof,
    #[msg("Invalid curve point")]
    InvalidPoint,
    #[msg("Non-canonical scalar")]
    NonCanonicalScalar,
    #[msg("Voting has not ended")]
    VotingNotEnded,
    #[msg("Reveal window is closed")]
    RevealClosed,
    #[msg("Reveal window is still open")]
    RevealStillOpen,
    #[msg("Voting was paused during this poll, so it is void; run it again")]
    VotingWasPaused,
    #[msg("Signer is not this committee member")]
    NotCommitteeMember,
    #[msg("This member already submitted a decryption share")]
    ShareAlreadySubmitted,
    #[msg("Decryption share proof rejected")]
    InvalidDleqProof,
    #[msg("Fewer decryption shares than the threshold")]
    NotEnoughShares,
    #[msg("One Lagrange coefficient per submitted share is required")]
    InvalidLagrange,
    #[msg("Claimed tally does not match the encrypted ballots")]
    TallyMismatch,
    #[msg("Poll already finalized or expired")]
    PollAlreadyFinal,
    #[msg("Poll is not final yet")]
    PollNotFinal,
    #[msg("Poll did not pass")]
    PollNotPassed,
    #[msg("Poll outcome was already consumed")]
    AlreadyConsumed,
    #[msg("Signer is not the poll's consumer")]
    WrongConsumer,
    #[msg("A passed, unconsumed poll can only be closed after the grace period")]
    CloseTooEarly,
    #[msg("Enough shares were submitted; finalize the poll instead")]
    FinalizeInstead,
    #[msg("Arithmetic overflow")]
    Overflow,
}

impl From<ZkError> for PollError {
    fn from(e: ZkError) -> Self {
        match e {
            ZkError::InvalidPoint => PollError::InvalidPoint,
            ZkError::NonCanonicalScalar => PollError::NonCanonicalScalar,
            ZkError::InvalidBallotProof => PollError::InvalidBallotProof,
            ZkError::InvalidDleqProof => PollError::InvalidDleqProof,
            ZkError::FeldmanMismatch => PollError::FeldmanMismatch,
            ZkError::InvalidCommittee => PollError::InvalidCommittee,
            ZkError::TallyMismatch => PollError::TallyMismatch,
        }
    }
}

/// `?`-friendly conversion of verifier results.
pub fn zk(result: core::result::Result<(), ZkError>) -> Result<()> {
    result.map_err(|e| error!(PollError::from(e)))
}
