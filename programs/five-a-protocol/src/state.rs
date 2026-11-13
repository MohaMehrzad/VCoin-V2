use anchor_lang::prelude::*;
use vcoin_common::AdminState;

use crate::constants::*;

#[derive(
    AnchorSerialize, AnchorDeserialize, Clone, Copy, Debug, Default, PartialEq, Eq, InitSpace,
)]
pub struct OracleEntry {
    /// `Pubkey::default()` = empty slot.
    pub key: Pubkey,
    pub active_from: i64,
}

/// Singleton, PDA `["five-a-config"]`. Read-only on the score path, so submissions for
/// different users never contend for it (audit 5A-L1).
#[account]
#[derive(InitSpace)]
pub struct FiveAConfig {
    pub admin: AdminState,
    pub oracles: [OracleEntry; MAX_ORACLES],
    pub required_consensus: u8,
    pub paused: bool,
    pub bump: u8,
}

impl FiveAConfig {
    pub const LEN: usize = 8 + Self::INIT_SPACE;

    pub fn registered_oracles(&self) -> usize {
        self.oracles
            .iter()
            .filter(|o| o.key != Pubkey::default())
            .count()
    }

    pub fn is_active_oracle(&self, key: &Pubkey, now: i64) -> bool {
        *key != Pubkey::default()
            && self
                .oracles
                .iter()
                .any(|o| o.key == *key && o.active_from <= now)
    }
}

/// The five dimensions, each `0..=MAX_SCORE`.
#[derive(
    AnchorSerialize, AnchorDeserialize, Clone, Copy, Debug, Default, PartialEq, Eq, InitSpace,
)]
pub struct ScoreValues {
    pub authenticity: u16,
    pub accuracy: u16,
    pub agility: u16,
    pub activity: u16,
    pub approved: u16,
}

impl ScoreValues {
    pub fn is_valid(&self) -> bool {
        [
            self.authenticity,
            self.accuracy,
            self.agility,
            self.activity,
            self.approved,
        ]
        .iter()
        .all(|v| *v <= MAX_SCORE)
    }

    /// Weighted composite (weights sum to 10000, so the result is ≤ MAX_SCORE).
    pub fn composite(&self) -> u16 {
        ((self.authenticity as u32 * AUTHENTICITY_WEIGHT as u32
            + self.accuracy as u32 * ACCURACY_WEIGHT as u32
            + self.agility as u32 * AGILITY_WEIGHT as u32
            + self.activity as u32 * ACTIVITY_WEIGHT as u32
            + self.approved as u32 * APPROVED_WEIGHT as u32)
            / MAX_SCORE as u32) as u16
    }
}

/// A user's applied 5A score, PDA `["user-score", user]`. Read by staking-protocol and
/// private-poll as a participation gate.
#[account]
#[derive(Default, InitSpace)]
pub struct UserScore {
    pub user: Pubkey,
    pub authenticity: u16,
    pub accuracy: u16,
    pub agility: u16,
    pub activity: u16,
    pub approved: u16,
    pub composite_score: u16,
    pub last_updated: i64,
    pub update_count: u32,
    pub bump: u8,
}

impl UserScore {
    pub const LEN: usize = 8 + Self::INIT_SPACE;

    pub fn values(&self) -> ScoreValues {
        ScoreValues {
            authenticity: self.authenticity,
            accuracy: self.accuracy,
            agility: self.agility,
            activity: self.activity,
            approved: self.approved,
        }
    }
}

#[derive(
    AnchorSerialize, AnchorDeserialize, Clone, Copy, Debug, Default, PartialEq, Eq, InitSpace,
)]
pub struct OracleSubmission {
    /// `Pubkey::default()` = empty slot.
    pub oracle: Pubkey,
    pub values: ScoreValues,
}

/// One update round for a user, PDA `["pending-score", user]`. Each oracle has one
/// changeable submission; competing value sets are tallied independently, so a dishonest
/// first submission cannot block honest ones (audit 5A-04). Closed (rent to the opener)
/// when a value set reaches consensus or the expired round is cleaned up.
#[account]
#[derive(InitSpace)]
pub struct PendingScoreUpdate {
    pub user: Pubkey,
    pub submissions: [OracleSubmission; MAX_ORACLES],
    pub opened_at: i64,
    pub expires_at: i64,
    pub rent_payer: Pubkey,
    pub bump: u8,
}

impl PendingScoreUpdate {
    pub const LEN: usize = 8 + Self::INIT_SPACE;

    /// Active oracles whose submission equals `values`.
    pub fn support(&self, config: &FiveAConfig, values: &ScoreValues, now: i64) -> u8 {
        self.submissions
            .iter()
            .filter(|s| s.values == *values && config.is_active_oracle(&s.oracle, now))
            .count() as u8
    }
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, Debug, PartialEq, Eq, InitSpace)]
pub enum VouchOutcome {
    Succeeded,
    Failed,
    Expired,
}

/// A live vouch, PDA `["vouch-record", voucher, vouchee]`. Closed (rent to the voucher) when
/// evaluated or expired, which also frees the voucher's slot (audit 5A-06).
#[account]
#[derive(InitSpace)]
pub struct VouchRecord {
    pub voucher: Pubkey,
    pub vouchee: Pubkey,
    pub vouched_at: i64,
    pub bump: u8,
}

impl VouchRecord {
    pub const LEN: usize = 8 + Self::INIT_SPACE;
}

/// PDA `["voucher-stats", voucher]`.
#[account]
#[derive(Default, InitSpace)]
pub struct VoucherStats {
    pub user: Pubkey,
    pub total_vouches_given: u32,
    pub successful_vouches: u32,
    pub failed_vouches: u32,
    pub expired_vouches: u32,
    pub vouches_active: u8,
    pub bump: u8,
}

impl VoucherStats {
    pub const LEN: usize = 8 + Self::INIT_SPACE;

    /// Concurrent vouches allowed for a voucher's composite score.
    pub fn max_vouches_for_score(score: u16) -> u8 {
        match score {
            9000.. => 10,
            8000.. => 8,
            7000.. => 5,
            6000.. => 3,
            _ => 0,
        }
    }
}

/// PDA `["vouch-status", vouchee]`.
#[account]
#[derive(Default, InitSpace)]
pub struct UserVouchStatus {
    pub user: Pubkey,
    pub vouches_received: u8,
    pub bump: u8,
}

impl UserVouchStatus {
    pub const LEN: usize = 8 + Self::INIT_SPACE;
}
