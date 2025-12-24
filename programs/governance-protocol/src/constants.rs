/// Seeds
pub const GOV_CONFIG_SEED: &[u8] = b"gov-config";
pub const PROPOSAL_SEED: &[u8] = b"proposal";
pub const VOTE_RECORD_SEED: &[u8] = b"vote-record";
pub const DELEGATION_SEED: &[u8] = b"delegation";
pub const DELEGATE_STATS_SEED: &[u8] = b"delegate-stats";
pub const PRIVATE_VOTING_SEED: &[u8] = b"private-voting";

// ============================================================================
// L-02: Hardcoded Constants - Governance Path for Future Updates
// ============================================================================
// The following constants are hardcoded for security and determinism.
// To modify these values in production:
//
// Option 1: Program Upgrade (Current)
//   - Create a proposal to upgrade the program with new constants
//   - Requires governance vote approval
//   - Deploy new program version via BPF upgrade authority
//
// Option 2: Governance Config Migration (Future Enhancement)
//   - Add these parameters to GovernanceConfig account
//   - Implement set_* instructions protected by governance vote
//   - Migration instruction to move values from constants to config
//
// Any changes to these parameters MUST go through governance to prevent
// centralized manipulation of protocol economics.
// ============================================================================

/// Governance thresholds (in veVCoin)
/// L-02: Configurable via program upgrade
pub const COMMUNITY_THRESHOLD: u64 = 1;
pub const DELEGATE_THRESHOLD: u64 = 1_000;
pub const COUNCIL_THRESHOLD: u64 = 10_000;

/// Default governance parameters
/// L-02: Configurable via program upgrade
pub const DEFAULT_VOTING_PERIOD: i64 = 7 * 24 * 60 * 60;  // 7 days
pub const DEFAULT_TIMELOCK_DELAY: i64 = 48 * 60 * 60;     // 48 hours
pub const DEFAULT_QUORUM: u64 = 1_000_000;                 // 1M effective votes
pub const DEFAULT_PROPOSAL_THRESHOLD: u64 = 1_000;         // 1000 veVCoin to propose

/// Tier multipliers (x1000 for precision)
/// L-02: Configurable via program upgrade
pub const TIER_MULT_NONE: u64 = 1000;      // 1.0x
pub const TIER_MULT_BRONZE: u64 = 1000;    // 1.0x
pub const TIER_MULT_SILVER: u64 = 2000;    // 2.0x
pub const TIER_MULT_GOLD: u64 = 5000;      // 5.0x
pub const TIER_MULT_PLATINUM: u64 = 10000; // 10.0x

/// Anti-plutocracy threshold
/// L-02: Configurable via program upgrade
pub const DIMINISHING_THRESHOLD: u64 = 100_000;

/// ZK voting constants
pub const MIN_DECRYPTION_THRESHOLD: u8 = 3;
pub const MAX_COMMITTEE_SIZE: usize = 5;

/// ZK voting feature flag - MUST be false until ZK verifier implemented
/// CRITICAL SECURITY: Setting this to true without implementing proper ZK
/// verification will allow vote manipulation attacks (C-01, C-02, C-03)
pub const ZK_VOTING_ENABLED: bool = false;

/// Decryption share PDA seed
pub const DECRYPTION_SHARE_SEED: &[u8] = b"decryption-share";

/// L-04: Valid URI prefixes for proposal descriptions
pub const VALID_URI_PREFIX_IPFS: &[u8] = b"ipfs://";
pub const VALID_URI_PREFIX_HTTPS: &[u8] = b"https://";
pub const VALID_URI_PREFIX_AR: &[u8] = b"ar://";
pub const MAX_URI_LENGTH: usize = 128;

/// C-NEW-01: External program PDA seeds for on-chain voting power verification
/// These must match the seeds used by staking-protocol and five-a-protocol
pub const USER_STAKE_SEED: &[u8] = b"user-stake";
pub const USER_SCORE_SEED: &[u8] = b"user-score";

