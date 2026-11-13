use anchor_lang::prelude::*;
use vcoin_common::AdminState;

use crate::constants::*;

/// Singleton, PDA `["registry-config"]`. Read-only on every user path (audit CR-05).
#[account]
#[derive(InitSpace)]
pub struct RegistryConfig {
    pub admin: AdminState,
    /// Backend key that reports engagement; separate from the admin (audit CR-07).
    pub engagement_oracle: Pubkey,
    /// Timelocked oracle change (`pending_oracle_eta == 0` = none).
    pub pending_oracle: Pubkey,
    pub pending_oracle_eta: i64,
    pub paused: bool,
    pub bump: u8,
}

impl RegistryConfig {
    pub const LEN: usize = 8 + Self::INIT_SPACE;
}

/// Per-user energy and daily post counter, PDA `["user-energy", user]`.
#[account]
#[derive(Default, InitSpace)]
pub struct UserEnergy {
    pub user: Pubkey,
    pub tier: u8,
    pub current_energy: u16,
    pub max_energy: u16,
    pub regen_per_hour: u16,
    /// Regeneration is accounted up to this time; partial hours carry over (audit CR-03).
    pub last_regen_time: i64,
    pub day_start: i64,
    pub posts_today: u16,
    pub refunded_today: u16,
    pub bump: u8,
}

impl UserEnergy {
    pub const LEN: usize = 8 + Self::INIT_SPACE;

    /// Adds whole units earned since `last_regen_time`, keeping the fractional remainder.
    pub fn regenerate(&mut self, now: i64) {
        if self.current_energy >= self.max_energy || self.regen_per_hour == 0 {
            self.last_regen_time = now;
            return;
        }
        let elapsed = now.saturating_sub(self.last_regen_time);
        let units = (elapsed as u128 * self.regen_per_hour as u128 / HOUR as u128) as u64;
        if units == 0 {
            return;
        }
        let room = (self.max_energy - self.current_energy) as u64;
        if units >= room {
            self.current_energy = self.max_energy;
            self.last_regen_time = now;
        } else {
            self.current_energy += units as u16;
            // Advance only by the time those units cost.
            self.last_regen_time += (units as i64 * HOUR) / self.regen_per_hour as i64;
        }
    }

    /// Settles regeneration at the old rate, then applies `tier` (audit CR-09).
    pub fn set_tier(&mut self, tier: u8, now: i64) {
        self.regenerate(now);
        let t = tier as usize;
        self.tier = tier;
        self.max_energy = MAX_ENERGY[t];
        self.regen_per_hour = REGEN_PER_HOUR[t];
        self.current_energy = self.current_energy.min(self.max_energy);
    }

    pub fn roll_day(&mut self, now: i64) {
        if now >= self.day_start.saturating_add(DAY) {
            self.day_start = now;
            self.posts_today = 0;
            self.refunded_today = 0;
        }
    }
}

/// Tier for an amount of VCoin locked in staking (0..=4).
pub fn tier_for_locked(locked: u64) -> u8 {
    TIER_THRESHOLDS.iter().filter(|t| locked >= **t).count() as u8
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, Debug, PartialEq, Eq, InitSpace)]
pub enum ContentState {
    Active,
    Edited,
}

/// PDA `["content-record", author, tracking_id]`: the author is part of the address, so
/// nobody can squat another author's id (audit CR-02).
#[account]
#[derive(InitSpace)]
pub struct ContentRecord {
    pub author: Pubkey,
    pub tracking_id: [u8; 32],
    pub content_hash: [u8; 32],
    pub uri: [u8; MAX_URI_LEN],
    pub uri_len: u8,
    pub content_type: u8,
    pub state: ContentState,
    pub version: u16,
    pub created_at: i64,
    pub updated_at: i64,
    pub energy_spent: u16,
    pub engagement_count: u32,
    pub refund_claimed: bool,
    /// Paid the record's rent and gets it back on delete.
    pub rent_payer: Pubkey,
    pub bump: u8,
}

impl ContentRecord {
    pub const LEN: usize = 8 + Self::INIT_SPACE;
}

/// Refund percentage for an engagement count (0 = not eligible yet).
pub fn refund_percent(engagement: u32) -> u16 {
    REFUND_TIERS
        .iter()
        .find(|(min, _)| engagement >= *min)
        .map_or(0, |(_, pct)| *pct)
}
