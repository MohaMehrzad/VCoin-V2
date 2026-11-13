use anchor_lang::prelude::*;

#[error_code]
pub enum FiveAError {
    #[msg("5A is paused")]
    Paused,
    #[msg("Program data account does not belong to this program")]
    InvalidProgramData,
    #[msg("Invalid oracle key")]
    InvalidOracle,
    #[msg("Oracle already registered")]
    OracleExists,
    #[msg("All oracle slots are used")]
    TooManyOracles,
    #[msg("Oracle not registered")]
    OracleNotFound,
    #[msg("Consensus must be between 2 and min(registered oracles, 5)")]
    InvalidConsensus,
    #[msg("Signer is not an active oracle")]
    NotActiveOracle,
    #[msg("Each score must be at most 10000")]
    InvalidScore,
    #[msg("This user's score was updated less than an hour ago")]
    ScoreUpdateTooFrequent,
    #[msg("The update round has not expired")]
    RoundNotExpired,
    #[msg("Rent payer does not match the round")]
    WrongRentPayer,
    #[msg("Score account is not the user's PDA")]
    InvalidScoreAccount,
    #[msg("Voucher's 5A score is too low to vouch")]
    VoucherScoreTooLow,
    #[msg("Vouchee is already established; vouching is for new or low-score users")]
    VoucheeAlreadyEstablished,
    #[msg("Cannot vouch for yourself")]
    CannotVouchSelf,
    #[msg("Mutual vouching is not allowed")]
    MutualVouch,
    #[msg("Voucher has no free vouch slots")]
    MaxVouchesReached,
    #[msg("Vouchee already has the maximum number of vouches")]
    VoucheeFullyVouched,
    #[msg("The evaluation period has not passed")]
    EvaluationTooEarly,
    #[msg("The vouch expired; use expire_vouch")]
    VouchExpired,
    #[msg("The vouch has not expired")]
    VouchNotExpired,
    #[msg("Signer is not the vouchee of this vouch")]
    NotVouchee,
    #[msg("A vouch can only be declined before its evaluation period starts")]
    DeclineWindowClosed,
    #[msg("Arithmetic overflow")]
    Overflow,
}
