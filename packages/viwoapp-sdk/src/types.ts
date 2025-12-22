import { PublicKey } from "@solana/web3.js";
import { BN } from "@coral-xyz/anchor";

// ============ VCoin Token Types ============

export interface VCoinConfig {
  authority: PublicKey;
  mint: PublicKey;
  permanentDelegate: PublicKey;
  paused: boolean;
  totalMinted: BN;
  totalBurned: BN;
}

// ============ Staking Types ============

export enum StakingTier {
  None = 0,
  Bronze = 1,
  Silver = 2,
  Gold = 3,
  Platinum = 4,
}

export interface StakingPool {
  authority: PublicKey;
  vcoinMint: PublicKey;
  vevcoinMint: PublicKey;
  totalStaked: BN;
  totalVevcoinMinted: BN;
  paused: boolean;
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
}

export interface CreateProposalParams {
  title: string;
  description: string;
  category: number;
  durationDays: number;
}

// ============ SSCRE Rewards Types ============

export interface RewardsPoolConfig {
  authority: PublicKey;
  vcoinMint: PublicKey;
  currentEpoch: BN;
  totalDistributed: BN;
  remainingReserves: BN;
  paused: boolean;
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

export interface ViLinkConfig {
  authority: PublicKey;
  vcoinMint: PublicKey;
  treasury: PublicKey;
  enabledActions: number;
  totalActionsCreated: BN;
  totalActionsExecuted: BN;
  totalTipVolume: BN;
  paused: boolean;
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
}

// ============ Gasless Types ============

export enum FeeMethod {
  PlatformSubsidized = 0,
  VCoinDeduction = 1,
  SSCREDeduction = 2,
}

export interface GaslessConfig {
  authority: PublicKey;
  feePayer: PublicKey;
  vcoinMint: PublicKey;
  dailySubsidyBudget: BN;
  solFeePerTx: BN;
  vcoinFeeMultiplier: BN;
  totalSubsidizedTx: BN;
  totalVcoinCollected: BN;
  paused: boolean;
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

