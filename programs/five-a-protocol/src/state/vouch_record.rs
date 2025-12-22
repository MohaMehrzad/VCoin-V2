use anchor_lang::prelude::*;

/// Vouch record (PDA per vouch)
#[account]
#[derive(Default)]
pub struct VouchRecord {
    /// Voucher (must have 60%+ 5A)
    pub voucher: Pubkey,
    /// Vouchee (new user)
    pub vouchee: Pubkey,
    /// Timestamp of vouch
    pub vouched_at: i64,
    /// VCoin staked (5 VCoin)
    pub vouch_stake: u64,
    /// Status: 0=Active, 1=Revoked, 2=Slashed, 3=Rewarded
    pub status: u8,
    /// Whether outcome has been evaluated
    pub outcome_evaluated: bool,
    /// PDA bump
    pub bump: u8,
}

impl VouchRecord {
    pub const LEN: usize = 8 + // discriminator
        32 + // voucher
        32 + // vouchee
        8 +  // vouched_at
        8 +  // vouch_stake
        1 +  // status
        1 +  // outcome_evaluated
        1;   // bump
}

