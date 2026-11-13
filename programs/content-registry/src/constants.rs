use anchor_lang::prelude::*;

#[constant]
pub const REGISTRY_CONFIG_SEED: &[u8] = b"registry-config";
#[constant]
pub const CONTENT_RECORD_SEED: &[u8] = b"content-record";
#[constant]
pub const USER_ENERGY_SEED: &[u8] = b"user-energy";

pub const HOUR: i64 = 60 * 60;
pub const DAY: i64 = 24 * HOUR;
pub const VCOIN: u64 = 1_000_000_000;

pub const MAX_URI_LEN: usize = 128;

/// Energy tiers from VCoin locked in staking-protocol (audit CR-01: not user-chosen).
pub const TIER_THRESHOLDS: [u64; 4] = [
    1_000 * VCOIN,
    5_000 * VCOIN,
    20_000 * VCOIN,
    100_000 * VCOIN,
];
pub const MAX_ENERGY: [u16; 5] = [200, 500, 800, 1_200, 2_000];
/// Energy regenerated per hour.
pub const REGEN_PER_HOUR: [u16; 5] = [20, 50, 80, 120, 200];
pub const DAILY_POST_CAP: [u16; 5] = [50, 100, 200, 400, 1_000];

/// Energy per content type: post, article, media, NFT, thread.
pub const CONTENT_COSTS: [u16; 5] = [10, 20, 50, 20, 40];
pub const EDIT_COST: u16 = 5;

/// Refunds: 24h after posting, claimable until day 7, at most 100% of the energy spent.
pub const REFUND_DELAY: i64 = DAY;
pub const REFUND_DEADLINE: i64 = 7 * DAY;
/// (engagements, refund %) from highest to lowest.
pub const REFUND_TIERS: [(u32, u16); 3] = [(100, 100), (50, 50), (10, 25)];

/// Engagement-oracle key changes take effect after this delay.
pub const ORACLE_CHANGE_DELAY: i64 = DAY;
