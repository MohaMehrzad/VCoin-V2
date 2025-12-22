/// 5A Protocol Constants
// ============================================================================
// L-02: Hardcoded Constants - Governance Path for Future Updates
// ============================================================================
// These constants define 5A scoring weights, vouch economics, and thresholds.
// To modify these values in production:
//
// Option 1: Program Upgrade (Current)
//   - Create a governance proposal to upgrade the program
//   - Requires community vote approval
//   - Deploy new program version with updated constants
//
// Option 2: Config Account Migration (Future Enhancement)
//   - Add these parameters to FiveAConfig account
//   - Implement governance-protected setter instructions
//   - Allows runtime parameter changes without upgrades
//
// All changes MUST go through governance to maintain scoring fairness.
// ============================================================================

/// PDA Seeds
pub const FIVE_A_CONFIG_SEED: &[u8] = b"five-a-config";
pub const USER_SCORE_SEED: &[u8] = b"user-score";
pub const SCORE_SNAPSHOT_SEED: &[u8] = b"score-snapshot";
pub const VOUCH_RECORD_SEED: &[u8] = b"vouch-record";
pub const VOUCH_STATUS_SEED: &[u8] = b"vouch-status";
pub const VOUCHER_STATS_SEED: &[u8] = b"voucher-stats";
pub const ORACLE_SEED: &[u8] = b"oracle";
pub const PENDING_SCORE_SEED: &[u8] = b"pending-score"; // H-05

/// Score weights (out of 10000)
/// L-02: Configurable via program upgrade - critical for fair scoring
pub const AUTHENTICITY_WEIGHT: u16 = 2500;  // 25%
pub const ACCURACY_WEIGHT: u16 = 2000;      // 20%
pub const AGILITY_WEIGHT: u16 = 1500;       // 15%
pub const ACTIVITY_WEIGHT: u16 = 2500;      // 25%
pub const APPROVED_WEIGHT: u16 = 1500;      // 15%

/// Vouch system
/// L-02: Configurable via program upgrade - affects vouching economics
pub const MIN_VOUCHER_SCORE: u16 = 6000;    // 60% 5A score to vouch
pub const VOUCH_STAKE_AMOUNT: u64 = 5_000_000_000; // 5 VCoin
pub const VOUCHES_REQUIRED: u8 = 3;
pub const VOUCH_EVALUATION_PERIOD: i64 = 90 * 24 * 60 * 60; // 90 days
pub const VOUCH_REWARD: u64 = 10_000_000_000; // 10 VCoin bonus for successful vouch

/// Score update intervals
/// L-02: Configurable via program upgrade
pub const SNAPSHOT_INTERVAL: i64 = 24 * 60 * 60; // Daily snapshots

/// H-05: Oracle consensus constants
pub const SCORE_UPDATE_EXPIRY: i64 = 60 * 60; // 1 hour for consensus
pub const MAX_CONFIRMING_ORACLES: usize = 5;

/// L-07: Rate limiting on oracle score submissions
/// Minimum time between score updates for the same user
pub const MIN_SCORE_UPDATE_INTERVAL: i64 = 60 * 60; // 1 hour minimum between updates

