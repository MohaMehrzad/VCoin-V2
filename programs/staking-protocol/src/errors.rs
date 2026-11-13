use anchor_lang::prelude::*;

#[error_code]
pub enum StakingError {
    #[msg("Staking is paused")]
    Paused,
    #[msg("Amount must be greater than zero")]
    ZeroAmount,
    #[msg("Lock duration outside [1 week, 4 years]")]
    InvalidLockDuration,
    #[msg("Deposit index out of range")]
    InvalidDepositIndex,
    #[msg("Deposit slot is empty")]
    DepositNotUsed,
    #[msg("Cannot add to an expired deposit; withdraw or relock it first")]
    DepositExpired,
    #[msg("A lock can only be extended")]
    LockNotExtended,
    #[msg("Deposit is still locked")]
    StillLocked,
    #[msg("Amount exceeds the deposit")]
    InsufficientDeposit,
    #[msg("Voter still has deposits")]
    VoterNotEmpty,
    #[msg("Program data account does not belong to this program")]
    InvalidProgramData,
    #[msg("VCoin mint must be the vcoin-token program's mint PDA")]
    InvalidVCoinMint,
    #[msg("veVCoin mint must be the vevcoin-token program's mint PDA")]
    InvalidVeVCoinMint,
    #[msg("No VCoin was received by the vault")]
    NothingReceived,
    #[msg("Weight parameters out of bounds")]
    InvalidWeightParams,
    #[msg(
        "Realm must be a RealmV2 of the configured governance program with VCoin as community mint"
    )]
    InvalidRealm,
    #[msg("Governance is already configured")]
    GovernanceAlreadyConfigured,
    #[msg("Governance is not configured yet")]
    GovernanceNotConfigured,
    #[msg("Token owner record does not match this realm, mint and owner")]
    InvalidTokenOwnerRecord,
    #[msg("Relinquish all votes and finish open proposals before withdrawing")]
    VotesNotRelinquished,
    #[msg("5A reputation score is below the required gate")]
    ScoreBelowGate,
    #[msg("5A score account is not the owner's UserScore PDA")]
    InvalidScoreAccount,
    #[msg("Gate scores must be at most 10000")]
    InvalidGateParams,
    #[msg("Arithmetic overflow")]
    Overflow,
}
