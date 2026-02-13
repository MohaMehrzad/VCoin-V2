import { PublicKey } from "@solana/web3.js";
import { BN } from "@coral-xyz/anchor";

// ============ Common Types (Security - Phase 2) ============

/** Two-step authority transfer fields (H-02) */
export interface PendingAuthorityFields {
  pendingAuthority?: PublicKey;
  pendingAuthorityActivatedAt?: BN;
}

// ============ VCoin Token Types ============

export interface VCoinConfig extends PendingAuthorityFields {
  authority: PublicKey;
  mint: PublicKey;
  permanentDelegate: PublicKey;
  paused: boolean;
  totalMinted: BN;
  totalBurned: BN;
}

/** Governance-controlled slashing request (H-01) */
export enum SlashStatus {
  Proposed = 0,
  Approved = 1,
  Executed = 2,
  Cancelled = 3,
}

export interface SlashRequest {
  target: PublicKey;
  requestId: BN; // v2.8.0: Used for PDA derivation
  amount: BN;
  reason: Uint8Array;
  proposer: PublicKey;
  proposedAt: BN;
  approvedAt?: BN;
  executedAt?: BN;
  status: SlashStatus;
  governanceProposal?: PublicKey;
}

// ============ Staking Types ============

export enum StakingTier {
  None = 0,
  Bronze = 1,
  Silver = 2,
  Gold = 3,
  Platinum = 4,
}

export interface StakingPool extends PendingAuthorityFields {
  authority: PublicKey;
  vcoinMint: PublicKey;
  vevcoinMint: PublicKey;
  totalStaked: BN;
  totalVevcoinMinted: BN;
  paused: boolean;
  reentrancyGuard?: boolean; // M-01: Reentrancy protection (optional for backwards compat)
}

export interface UserStake {
  user: PublicKey;
  stakedAmount: BN;
  vevcoinBalance: BN;
  tier: StakingTier;
  lockEndTime: BN;
  lastUpdateTime: BN;
}

export interface StakeParams {
  amount: BN;
  lockDuration: number; // seconds
}

// ============ Governance Types ============

export enum ProposalStatus {
  Active = 0,
  Passed = 1,
  Rejected = 2,
  Executed = 3,
  Cancelled = 4,
}

/**
 * Vote choice for governance voting (v2.8.0 C-NEW-01)
 * Voting power params are now read from on-chain state, not passed as parameters
 */
export enum VoteChoice {
  Against = 0,
  For = 1,
  Abstain = 2,
}

export interface Proposal {
  id: BN;
  proposer: PublicKey;
  title: string;
  descriptionHash: Uint8Array;
  startTime: BN;
  endTime: BN;
  votesFor: BN;
  votesAgainst: BN;
  status: ProposalStatus;
  executed: boolean;
  category: number;
}

export interface VoteRecord {
  user: PublicKey;
  proposal: PublicKey;
  votePower: BN;
  support: boolean;
  votedAt: BN;
  /** ElGamal ciphertext for "for" vote (64 bytes: R || C) - only set for private votes */
  ctFor?: Uint8Array;
  /** ElGamal ciphertext for "against" vote (64 bytes: R || C) - only set for private votes */
  ctAgainst?: Uint8Array;
  /** ElGamal ciphertext for "abstain" vote (64 bytes: R || C) - only set for private votes */
  ctAbstain?: Uint8Array;
}

export interface CreateProposalParams {
  title: string;
  description: string;
  category: number;
  durationDays: number;
}

// ============ SSCRE Rewards Types ============

export interface RewardsPoolConfig extends PendingAuthorityFields {
  authority: PublicKey;
  vcoinMint: PublicKey;
  currentEpoch: BN;
  totalDistributed: BN;
  remainingReserves: BN;
  paused: boolean;
  circuitBreakerActive?: boolean; // M-05: Optional for backwards compat
  circuitBreakerTriggeredAt?: BN; // M-05: Circuit breaker cooldown
}

export interface EpochDistribution {
  epoch: BN;
  merkleRoot: Uint8Array;
  totalAllocation: BN;
  totalClaimed: BN;
  claimsCount: BN;
  isFinalized: boolean;
}

export interface UserClaim {
  user: PublicKey;
  lastClaimedEpoch: BN;
  totalClaimed: BN;
  claimsCount: number;
  claimedEpochsBitmap?: BN[];      // H-04: Bitmap for epochs 0-255 (optional)
  claimedEpochsBitmapExt?: BN[];   // H-04: Extended bitmap for epochs 256-511 (optional)
  highEpochsBitmap?: BN[];         // H-NEW-04: Bitmap for epochs 512-1023 (replaces array)
}

export interface ClaimRewardsParams {
  epoch: BN;
  amount: BN;
  merkleProof: Uint8Array[];
}

// ============ ViLink Types ============

export enum ActionType {
  Tip = 0,
  Vouch = 1,
  Follow = 2,
  Challenge = 3,
  Stake = 4,
  ContentReact = 5,
  Delegate = 6,
  Vote = 7,
}

/**
 * ViLinkConfig - Updated with H-02 pending authority field
 */
export interface ViLinkConfig extends PendingAuthorityFields {
  authority: PublicKey;
  /** H-02: Pending authority for two-step transfer */
  pendingAuthority?: PublicKey;
  vcoinMint: PublicKey;
  treasury: PublicKey;
  enabledActions: number;
  totalActionsCreated: BN;
  totalActionsExecuted: BN;
  totalTipVolume: BN;
  paused: boolean;
  /** M-02: Platform fee in basis points, bounded 10-1000 (0.1%-10%) */
  platformFeeBps: number;
}

export interface ViLinkAction {
  actionId: Uint8Array;
  creator: PublicKey;
  target: PublicKey;
  actionType: ActionType;
  amount: BN;
  expiresAt: BN;
  executed: boolean;
  executionCount: number;
  maxExecutions: number;
  /** M-04: Nonce used for deterministic PDA derivation (replaces timestamp) */
  actionNonce: BN;
}

export interface CreateActionParams {
  actionType: ActionType;
  target: PublicKey;
  amount?: BN;
  expirySeconds?: number;
  oneTime?: boolean;
  maxExecutions?: number;
  contentId?: Uint8Array;
  metadata?: string;
  /**
   * M-04: Nonce for deterministic PDA derivation.
   * If not provided, fetched from user's action_nonce in UserActionStats.
   */
  nonce?: BN;
}

/** M-04 + Finding #5: User action statistics with nonce tracking */
export interface UserActionStatsExtended {
  user: PublicKey;
  actionsCreated: BN;
  actionsExecuted: BN;
  tipsSent: BN;
  tipsReceived: BN;
  vcoinSent: BN;
  vcoinReceived: BN;
  vouchesGiven: BN;
  followsGiven: BN;
  firstActionAt: BN;
  lastActionAt: BN;
  /** M-04: Next nonce to use when creating an action */
  actionNonce: BN;
  /** Finding #5: Next nonce to use when creating a batch (prevents timestamp collisions) */
  batchNonce: BN;
}

// ============ Gasless Types ============

export enum FeeMethod {
  PlatformSubsidized = 0,
  VCoinDeduction = 1,
  SSCREDeduction = 2,
}

/**
 * GaslessConfig - Finding #8 Fix
 * 
 * Updated to include all fields from on-chain GaslessConfig struct.
 * Previous version was missing fields added after H-02 security fix.
 */
export interface GaslessConfig extends PendingAuthorityFields {
  authority: PublicKey;
  /** H-02: Pending authority for two-step transfer */
  pendingAuthority?: PublicKey;
  feePayer: PublicKey;
  vcoinMint: PublicKey;
  /** Fee vault for VCoin fee collection */
  feeVault?: PublicKey;
  /** SSCRE program for reward deduction integration */
  sscreProgram?: PublicKey;
  dailySubsidyBudget: BN;
  solFeePerTx: BN;
  vcoinFeeMultiplier: BN;
  /** SSCRE deduction rate in basis points */
  sscreDeductionBps?: number;
  /** Maximum subsidized transactions per user per day */
  maxSubsidizedPerUser?: number;
  totalSubsidizedTx: BN;
  /** Total SOL spent on subsidies */
  totalSolSpent?: BN;
  totalVcoinCollected: BN;
  paused: boolean;
  /** Current day number for daily budget reset */
  currentDay?: number;
  /** Today's spent budget */
  daySpent?: BN;
  /** L-03: Maximum fee slippage in basis points (default 500 = 5%) */
  maxSlippageBps?: number;
}

export interface SessionKey {
  user: PublicKey;
  sessionPubkey: PublicKey;
  scope: number;
  createdAt: BN;
  expiresAt: BN;
  actionsUsed: number;
  maxActions: number;
  vcoinSpent: BN;
  maxSpend: BN;
  isRevoked: boolean;
  feeMethod: FeeMethod;
}

export interface CreateSessionParams {
  sessionPubkey: PublicKey;
  scope: number;
  durationSeconds?: number;
  maxActions?: number;
  maxSpend?: BN;
  feeMethod?: FeeMethod;
}

export interface UserGaslessStats {
  user: PublicKey;
  totalGaslessTx: BN;
  totalSubsidized: BN;
  totalVcoinFees: BN;
  sessionsCreated: number;
  activeSession: PublicKey;
  /** H-AUDIT-12: Number of active (non-revoked, non-expired) sessions */
  activeSessions: number;
}

// ============ Identity Types ============

export enum VerificationLevel {
  None = 0,     // Wallet connected only
  Basic = 1,    // Email + phone verified
  KYC = 2,      // Identity documents verified
  Full = 3,     // KYC + biometric verification
  Enhanced = 4, // Full + UniqueHuman attestation
}

export interface Identity {
  user: PublicKey;
  didHash: Uint8Array;
  verificationLevel: VerificationLevel;
  createdAt: BN;
  updatedAt: BN;
}

// ============ 5A Protocol Types ============

export interface FiveAScore {
  user: PublicKey;
  authenticity: number;  // A1 - "Are you a real person?"
  accuracy: number;      // A2 - "Is your content quality?"
  agility: number;       // A3 - "Are you fast?"
  activity: number;      // A4 - "Do you show up daily?"
  approved: number;      // A5 - "Does the community like you?"
  compositeScore: number;
  lastUpdated: BN;
  isPrivate: boolean;
}

export interface VouchRecord {
  voucher: PublicKey;
  vouchee: PublicKey;
  vouchedAt: BN;
  isPositive: boolean;
  outcome: number;
}

// ============ Content Registry Types ============

export enum ContentState {
  Active = 0,
  Hidden = 1,
  Deleted = 2,
  Flagged = 3,
}

export interface ContentRecord {
  contentId: Uint8Array;
  creator: PublicKey;
  contentHash: Uint8Array;
  state: ContentState;
  createdAt: BN;
  editCount: number;
  tips: BN;
  engagementScore: BN;
}

export interface UserEnergy {
  user: PublicKey;
  currentEnergy: number;
  maxEnergy: number;
  lastRegenTime: BN;
  tier: number;
}

// ============ Content Registry Config ============

export interface RegistryConfig extends PendingAuthorityFields {
  authority: PublicKey;
  paused: boolean;
  totalContent: BN;
}

// ============ Identity Config ============

export interface IdentityConfig extends PendingAuthorityFields {
  authority: PublicKey;
  paused: boolean;
  totalIdentities: BN;
}

// ============ 5A Config ============

export interface FiveAConfig extends PendingAuthorityFields {
  authority: PublicKey;
  paused: boolean;
  oracleConsensusRequired: number; // H-05: Default 3
}

// ============ Governance Types (Security Updates) ============

export interface GovernanceConfig extends PendingAuthorityFields {
  authority: PublicKey;
  vevcoinMint: PublicKey;
  paused: boolean;
  proposalCount: BN;
  /** H-02: Timestamp when pending authority transfer was initiated (for 24h timelock) */
  pendingAuthorityActivatedAt: BN;
}

/** ZK voting decryption share with per-category partials and DLEQ proof */
export interface DecryptionShare {
  proposal: PublicKey;
  committeeIndex: number;
  committeeMember: PublicKey;
  /** Partial decryption: sk_j * R_for_sum (Ristretto255 point) */
  partialFor: Uint8Array;
  /** Partial decryption: sk_j * R_against_sum (Ristretto255 point) */
  partialAgainst: Uint8Array;
  /** Partial decryption: sk_j * R_abstain_sum (Ristretto255 point) */
  partialAbstain: Uint8Array;
  /** Batched DLEQ proof challenge (Scalar) */
  dleqChallenge: Uint8Array;
  /** Batched DLEQ proof response (Scalar) */
  dleqResponse: Uint8Array;
  submittedAt: BN;
  verified: boolean;
}

/** Private voting config with ZK cryptographic verification */
export interface PrivateVotingConfig {
  proposal: PublicKey;
  /** Joint ElGamal public key (Ristretto255 point, 32 bytes) */
  encryptionPubkey: Uint8Array;
  decryptionThreshold: number;
  decryptionCommittee: PublicKey[];
  /** Committee ElGamal public keys (Ristretto255 points) */
  committeeElgamalPubkeys: Uint8Array[];
  sharesSubmitted: boolean[];
  revealCompleted: boolean;
  aggregatedFor: BN;
  aggregatedAgainst: BN;
  aggregatedAbstain: BN;
  /** Verification hash for audit trail */
  verificationHash: Uint8Array;
  /** Homomorphically accumulated ciphertext components */
  accumulatedCtForR: Uint8Array;
  accumulatedCtForC: Uint8Array;
  accumulatedCtAgainstR: Uint8Array;
  accumulatedCtAgainstC: Uint8Array;
  accumulatedCtAbstainR: Uint8Array;
  accumulatedCtAbstainC: Uint8Array;
  /** Number of private votes cast */
  totalPrivateVotes: number;
}

/** Delegation with expiry (M-07) */
export interface Delegation {
  delegator: PublicKey;
  delegate: PublicKey;
  delegationType: number;
  categories: number;
  delegatedAmount: BN;
  delegatedAt: BN;
  expiresAt: BN;
  revocable: boolean;
}

/** H-NEW-03: Parameters for creating a delegation */
export interface DelegateVotesParams {
  /** Address to delegate voting power to */
  delegate: PublicKey;
  /** Delegation type (0=Full, 1=PerCategory, 2=TimeScoped) */
  delegationType: number;
  /** Category bitmap (0xFF for all) */
  categories: number;
  /** Amount of veVCoin to delegate (must be <= actual balance) */
  amount: BN;
  /** Expiration timestamp (0 = never expires) */
  expiresAt: BN;
  /** Whether the delegation can be revoked mid-vote */
  revocable: boolean;
}

// ============ 5A Oracle Consensus Types (H-05) ============

/** Pending score update for oracle consensus */
export interface PendingScoreUpdate {
  user: PublicKey;
  authenticity: number;
  accuracy: number;
  agility: number;
  activity: number;
  approved: number;
  oracleSubmissions: PublicKey[];
  submissionCount: number;
  createdAt: BN;
  expiresAt: BN;
  /** C-07: Immutable consensus count stored at creation time */
  requiredConsensus: number;
}

// ============ Transfer Hook Config ============

export interface HookConfig extends PendingAuthorityFields {
  authority: PublicKey;
  vcoinMint: PublicKey;
  blockWashTrading: boolean; // M-04: When true, blocks detected wash trades
  paused: boolean;
}

/** Pair tracking for wash trade detection */
export interface PairTracking {
  sender: PublicKey;
  receiver: PublicKey;
  transferCount: number;
  lastTransferTime: BN;
  washFlags: number;
  /** M-AUDIT-10: Timestamp of last wash flag for decay */
  lastFlagTime: BN;
}

// ============ veVCoin Types ============

export interface VeVCoinConfig extends PendingAuthorityFields {
  authority: PublicKey;
  vcoinMint: PublicKey;
  vevcoinMint: PublicKey;
  stakingProtocol: PublicKey;
  totalHolders: BN;
  totalMinted: BN;
  paused: boolean;
  /** C-01: Timestamp when pending authority transfer was initiated (for 24h timelock) */
  pendingAuthorityActivatedAt: BN;
}

// ============ ZK Private Voting Types ============

/** Parameters for casting a private vote */
export interface CastPrivateVoteParams {
  proposalId: BN;
  /** ElGamal ciphertext for "for" (64 bytes) */
  ctFor: Uint8Array;
  /** ElGamal ciphertext for "against" (64 bytes) */
  ctAgainst: Uint8Array;
  /** ElGamal ciphertext for "abstain" (64 bytes) */
  ctAbstain: Uint8Array;
  /** Vote validity proof (352 bytes) */
  proofData: Uint8Array;
}

/** Parameters for enabling private voting on a proposal */
export interface EnablePrivateVotingParams {
  proposalId: BN;
  /** Joint ElGamal public key (Ristretto255 point, 32 bytes) */
  encryptionPubkey: Uint8Array;
  /** Committee member Solana pubkeys */
  decryptionCommittee: PublicKey[];
  committeeSize: number;
  decryptionThreshold: number;
  /** Committee member ElGamal public keys (Ristretto255 points, 32 bytes each) */
  committeeElgamalPubkeys: Uint8Array[];
}

/** Parameters for submitting a decryption share */
export interface SubmitDecryptionShareParams {
  proposalId: BN;
  committeeIndex: number;
  /** Partial decryption for "for" (Ristretto255 point, 32 bytes) */
  partialFor: Uint8Array;
  /** Partial decryption for "against" (Ristretto255 point, 32 bytes) */
  partialAgainst: Uint8Array;
  /** Partial decryption for "abstain" (Ristretto255 point, 32 bytes) */
  partialAbstain: Uint8Array;
  /** DLEQ proof challenge (Scalar, 32 bytes) */
  dleqChallenge: Uint8Array;
  /** DLEQ proof response (Scalar, 32 bytes) */
  dleqResponse: Uint8Array;
}

/** Parameters for aggregating revealed votes (permissionless) */
export interface AggregateRevealedVotesParams {
  proposalId: BN;
  tallyFor: BN;
  tallyAgainst: BN;
  tallyAbstain: BN;
  /** Lagrange interpolation coefficients (Scalars, 32 bytes each) */
  lagrangeCoefficients: Uint8Array[];
}

