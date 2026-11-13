use anchor_lang::prelude::*;

#[error_code]
pub enum VeVCoinError {
    #[msg("Only the staking registrar may mint or burn veVCoin")]
    Unauthorized,
    #[msg("Amount must be greater than zero")]
    ZeroAmount,
    #[msg("Program data account does not belong to this program")]
    InvalidProgramData,
    #[msg("Mint account does not match the expected PDA")]
    InvalidMint,
}
