use anchor_lang::prelude::*;
use vcoin_common::AdminState;

/// Singleton configuration, PDA `["vevcoin-config"]`. It is the veVCoin mint authority
/// and permanent delegate, so only this program (on the registrar's instruction) can
/// mint or burn receipts.
#[account]
#[derive(Default, InitSpace)]
pub struct VeVCoinConfig {
    pub admin: AdminState,
    /// veVCoin mint, PDA `["vevcoin-mint"]`.
    pub mint: Pubkey,
    /// Staking registrar PDA `["registrar"]` under the staking program, derived once at init.
    pub staking_authority: Pubkey,
    pub bump: u8,
    pub mint_bump: u8,
}

impl VeVCoinConfig {
    pub const LEN: usize = 8 + Self::INIT_SPACE;
}
