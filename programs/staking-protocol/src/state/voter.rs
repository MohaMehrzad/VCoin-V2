use anchor_lang::prelude::*;

use crate::constants::MAX_DEPOSITS;

/// One locked deposit.
#[derive(
    AnchorSerialize, AnchorDeserialize, Clone, Copy, Debug, Default, PartialEq, Eq, InitSpace,
)]
pub struct DepositEntry {
    pub amount: u64,
    pub lock_start: i64,
    pub lock_end: i64,
    pub is_used: bool,
    /// Counted in `Registrar::active_locked` (the base of the max voter weight). Cleared by
    /// `deactivate_expired` or a withdrawal once the lock has ended; set again by a relock.
    pub is_active: bool,
}

/// Per-owner deposits, PDA `["voter", owner]`.
#[account]
#[derive(Default, InitSpace)]
pub struct Voter {
    pub owner: Pubkey,
    pub deposits: [DepositEntry; MAX_DEPOSITS],
    /// Open slash requests against this voter; withdrawals are frozen while > 0.
    pub pending_slashes: u8,
    pub bump: u8,
}

impl Voter {
    pub const LEN: usize = 8 + Self::INIT_SPACE;

    /// Sum of all deposit principals.
    pub fn total_amount(&self) -> u64 {
        self.deposits
            .iter()
            .filter(|d| d.is_used)
            .fold(0u64, |acc, d| acc.saturating_add(d.amount))
    }

    /// Principal still locked at `now` (used for content-registry energy tiers).
    pub fn locked_amount(&self, now: i64) -> u64 {
        self.deposits
            .iter()
            .filter(|d| d.is_used && d.lock_end > now)
            .fold(0u64, |acc, d| acc.saturating_add(d.amount))
    }

    pub fn is_empty(&self) -> bool {
        self.deposits.iter().all(|d| !d.is_used)
    }
}
