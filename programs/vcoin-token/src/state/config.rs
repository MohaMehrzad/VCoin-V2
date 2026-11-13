use anchor_lang::prelude::*;
use vcoin_common::AdminState;

/// Singleton configuration, PDA `["vcoin-config-v2"]`. Also the VCoin mint authority
/// and the metadata update authority.
#[account]
#[derive(Default, InitSpace)]
pub struct VCoinConfig {
    /// Authority with a 24h two-step transfer (audit X-2).
    pub admin: AdminState,
    /// The VCoin mint, PDA `["vcoin-mint"]`, created by `initialize`.
    pub mint: Pubkey,
    /// Pauses `mint_tokens`. Transfers are never pausable (the mint has no
    /// pausable/freeze capability by design).
    pub paused: bool,
    pub bump: u8,
    pub mint_bump: u8,
}

impl VCoinConfig {
    pub const LEN: usize = 8 + Self::INIT_SPACE;
}

/// Metadata fields the authority may update.
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, Debug, PartialEq, Eq)]
pub enum MetadataField {
    Name,
    Symbol,
    Uri,
}
