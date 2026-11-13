use anchor_lang::prelude::*;

#[error_code]
pub enum VCoinError {
    #[msg("Token minting is paused")]
    TokenPaused,
    #[msg("Amount must be greater than zero")]
    ZeroAmount,
    #[msg("Minting would exceed the 1B VCoin supply cap")]
    ExceedsMaxSupply,
    #[msg("Program data account does not belong to this program")]
    InvalidProgramData,
    #[msg("Metadata field exceeds its maximum length")]
    MetadataTooLong,
    #[msg("Arithmetic overflow")]
    Overflow,
}
