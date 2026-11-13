use anchor_lang::prelude::*;

/// A DAO-approved slash of one voter's locked deposits, PDA
/// `["slash", voter, nonce]`. Executable by anyone in `[executable_at, expires_at)`;
/// the voter's withdrawals are frozen while it exists. Closed (rent to `rent_payer`) when
/// cancelled, executed or expired.
#[account]
#[derive(Default, InitSpace)]
pub struct SlashRequest {
    pub voter: Pubkey,
    pub owner: Pubkey,
    /// Requested amount; execution takes `min(amount, deposits)`.
    pub amount: u64,
    /// Hash of the off-chain evidence / DAO proposal text.
    pub reason_hash: [u8; 32],
    pub proposed_at: i64,
    pub executable_at: i64,
    pub expires_at: i64,
    pub nonce: u64,
    pub rent_payer: Pubkey,
    pub bump: u8,
}

impl SlashRequest {
    pub const LEN: usize = 8 + Self::INIT_SPACE;
}
