/// Seeds
pub const REGISTRY_CONFIG_SEED: &[u8] = b"registry-config";
pub const CONTENT_RECORD_SEED: &[u8] = b"content-record";
pub const USER_ENERGY_SEED: &[u8] = b"user-energy";
pub const RATE_LIMIT_SEED: &[u8] = b"rate-limit";
pub const ENERGY_CONFIG_SEED: &[u8] = b"energy-config";

/// Energy costs by action
pub const ENERGY_COST_TEXT_POST: u16 = 10;
pub const ENERGY_COST_IMAGE_POST: u16 = 20;
pub const ENERGY_COST_VIDEO_POST: u16 = 50;
pub const ENERGY_COST_THREAD: u16 = 40;
pub const ENERGY_COST_REPLY: u16 = 5;
pub const ENERGY_COST_REPOST: u16 = 8;
pub const ENERGY_COST_EDIT_AFTER_1H: u16 = 5;

/// Energy regen rate per hour by tier
pub const REGEN_RATE_NONE: u16 = 20;
pub const REGEN_RATE_BRONZE: u16 = 50;
pub const REGEN_RATE_SILVER: u16 = 80;
pub const REGEN_RATE_GOLD: u16 = 120;
pub const REGEN_RATE_PLATINUM: u16 = 200;

/// Max energy by tier
pub const MAX_ENERGY_NONE: u16 = 200;
pub const MAX_ENERGY_BRONZE: u16 = 500;
pub const MAX_ENERGY_SILVER: u16 = 800;
pub const MAX_ENERGY_GOLD: u16 = 1200;
pub const MAX_ENERGY_PLATINUM: u16 = 2000;

/// Engagement thresholds for refunds
pub const REFUND_THRESHOLD_10: u32 = 10;    // 25% refund
pub const REFUND_THRESHOLD_50: u32 = 50;    // 50% refund
pub const REFUND_THRESHOLD_100: u32 = 100;  // 100% refund
pub const REFUND_THRESHOLD_1000: u32 = 1000; // 150% refund (viral)

/// Timing
pub const ENGAGEMENT_CHECK_DELAY: i64 = 24 * 60 * 60; // 24 hours
pub const FREE_EDIT_WINDOW: i64 = 60 * 60; // 1 hour
