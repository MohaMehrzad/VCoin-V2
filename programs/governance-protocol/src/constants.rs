/// Seeds
pub const GOV_CONFIG_SEED: &[u8] = b"gov-config";
pub const PROPOSAL_SEED: &[u8] = b"proposal";
pub const VOTE_RECORD_SEED: &[u8] = b"vote-record";
pub const DELEGATION_SEED: &[u8] = b"delegation";
pub const DELEGATE_STATS_SEED: &[u8] = b"delegate-stats";
pub const PRIVATE_VOTING_SEED: &[u8] = b"private-voting";

/// Governance thresholds (in veVCoin)
pub const COMMUNITY_THRESHOLD: u64 = 1;
pub const DELEGATE_THRESHOLD: u64 = 1_000;
pub const COUNCIL_THRESHOLD: u64 = 10_000;

/// Default governance parameters
pub const DEFAULT_VOTING_PERIOD: i64 = 7 * 24 * 60 * 60;  // 7 days
pub const DEFAULT_TIMELOCK_DELAY: i64 = 48 * 60 * 60;     // 48 hours
pub const DEFAULT_QUORUM: u64 = 1_000_000;                 // 1M effective votes
pub const DEFAULT_PROPOSAL_THRESHOLD: u64 = 1_000;         // 1000 veVCoin to propose

/// Tier multipliers (x1000 for precision)
pub const TIER_MULT_NONE: u64 = 1000;      // 1.0x
pub const TIER_MULT_BRONZE: u64 = 1000;    // 1.0x
pub const TIER_MULT_SILVER: u64 = 2000;    // 2.0x
pub const TIER_MULT_GOLD: u64 = 5000;      // 5.0x
pub const TIER_MULT_PLATINUM: u64 = 10000; // 10.0x

/// Anti-plutocracy threshold
pub const DIMINISHING_THRESHOLD: u64 = 100_000;

/// ZK voting constants
pub const MIN_DECRYPTION_THRESHOLD: u8 = 3;
pub const MAX_COMMITTEE_SIZE: usize = 5;

