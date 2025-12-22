use anchor_lang::prelude::*;

/// Vouch status enum
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, PartialEq, Eq, Default)]
pub enum VouchStatus {
    #[default]
    Active = 0,
    Revoked = 1,
    Slashed = 2,
    Rewarded = 3,
}

