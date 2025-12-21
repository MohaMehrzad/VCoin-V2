import { PublicKey } from "@solana/web3.js";

// ============ Program IDs ============

export const PROGRAM_IDS = {
  vcoinToken: new PublicKey("VCNtkM3xg8ihH3JY8bQbqjUWCNEAVCiqUGmPjAqPNwP"),
  vevcoinToken: new PublicKey("VEVCnmRk9hYxBGhH3JY8bQbqjUWCNEAVCiqUGmPjBqQ"),
  stakingProtocol: new PublicKey("STKGnmRk9hYxBGhH3JY8bQbqjUWCNEAVCiqUGmPjCrR"),
  transferHook: new PublicKey("E5FWQsncH5hWRYX2ysiTA9uA2vhdQtQP473tDU9GWhyi"),
  identityProtocol: new PublicKey("CnxKPyRgU3HZDUvbAFPAddYJVWM2rWhLVq9QoEnBgJdB"),
  fiveAProtocol: new PublicKey("EPVUXY5NSTxWRGU4JF3zowtc5wB6HE9aUwFHG61W9CCH"),
  contentRegistry: new PublicKey("3Ex3eTSLUcLdfkMdUD91FH3a5CaFSzydMtCwAWGvW5vY"),
  governanceProtocol: new PublicKey("3fgzSVwUho1rp4k87ZZ43K9fysxy1WqabDNWTemmD1vi"),
  sscreProtocol: new PublicKey("FZrjuWJE6VW7qSxB8Jhd4hxv1fSnYBsRCzBTfWhVN8zC"),
  vilinkProtocol: new PublicKey("FYaKjTU8fq6W8nBQB6LhFBvCzYtvNzYNd6Gdr4dQELfT"),
  gaslessProtocol: new PublicKey("FZyRfP5qeChTZ9z2M2aHkXQ8QLHbRQ5aK7dJ2BpPtYXj"),
};

// ============ PDA Seeds ============

export const SEEDS = {
  // VCoin
  vcoinConfig: "vcoin-config",
  
  // veVCoin
  vevcoinConfig: "vevcoin-config",
  userVevcoin: "user-vevcoin",
  
  // Staking
  stakingPool: "staking-pool",
  userStake: "user-stake",
  
  // Governance
  governanceConfig: "governance-config",
  proposal: "proposal",
  voteRecord: "vote",
  delegation: "delegation",
  
  // SSCRE
  poolConfig: "pool-config",
  epoch: "epoch",
  userClaim: "user-claim",
  
  // ViLink
  vilinkConfig: "vilink-config",
  action: "action",
  userStats: "user-stats",
  dapp: "dapp",
  
  // Gasless
  gaslessConfig: "gasless-config",
  sessionKey: "session-key",
  userGasless: "user-gasless",
  feeVault: "fee-vault",
  
  // Identity
  identityConfig: "identity-config",
  identity: "identity",
  
  // 5A
  fiveAConfig: "five-a-config",
  userScore: "user-score",
  vouch: "vouch",
  
  // Content
  registryConfig: "registry-config",
  content: "content",
  userEnergy: "user-energy",
};

// ============ Token Constants ============

export const VCOIN_DECIMALS = 9;
export const VEVCOIN_DECIMALS = 9;

export const VCOIN_TOTAL_SUPPLY = 1_000_000_000; // 1 billion
export const VCOIN_INITIAL_CIRCULATING = 100_000_000; // 100 million

// ============ Staking Constants ============

export const STAKING_TIERS = {
  bronze: { minStake: 100, multiplier: 1.0, minLock: 0 },
  silver: { minStake: 1000, multiplier: 1.1, minLock: 30 * 24 * 3600 },
  gold: { minStake: 10000, multiplier: 1.25, minLock: 90 * 24 * 3600 },
  platinum: { minStake: 50000, multiplier: 1.5, minLock: 180 * 24 * 3600 },
  diamond: { minStake: 100000, multiplier: 2.0, minLock: 365 * 24 * 3600 },
};

export const LOCK_DURATIONS = {
  none: 0,
  oneMonth: 30 * 24 * 3600,
  threeMonths: 90 * 24 * 3600,
  sixMonths: 180 * 24 * 3600,
  oneYear: 365 * 24 * 3600,
};

// ============ SSCRE Constants ============

export const SSCRE_CONSTANTS = {
  primaryReserves: 350_000_000, // 350M VCoin
  secondaryReserves: 40_000_000, // 40M VCoin
  epochDuration: 30 * 24 * 3600, // 30 days
  claimWindow: 90 * 24 * 3600, // 90 days
  gaslessFeeBps: 100, // 1%
  minClaimAmount: 1, // 1 VCoin
};

// ============ ViLink Constants ============

export const VILINK_CONSTANTS = {
  maxActionExpiry: 7 * 24 * 3600, // 7 days
  minTipAmount: 0.1, // 0.1 VCoin
  maxTipAmount: 10_000, // 10,000 VCoin
  platformFeeBps: 250, // 2.5%
};

export const ACTION_SCOPES = {
  tip: 1 << 0,
  vouch: 1 << 1,
  content: 1 << 2,
  governance: 1 << 3,
  transfer: 1 << 4,
  stake: 1 << 5,
  claim: 1 << 6,
  follow: 1 << 7,
  all: 0xFFFF,
};

// ============ Gasless Constants ============

export const GASLESS_CONSTANTS = {
  sessionDuration: 24 * 3600, // 24 hours
  maxSessionActions: 1000,
  maxSessionSpend: 100_000, // 100,000 VCoin
  defaultSolFee: 5000, // 0.000005 SOL
  vcoinFeeMultiplier: 100,
  sscreDeductionBps: 100, // 1%
  dailySubsidyBudget: 10, // 10 SOL
  maxSubsidizedPerUser: 50,
};

// ============ 5A Protocol Constants ============

export const FIVE_A_CONSTANTS = {
  maxScore: 10000, // 100.00 with 2 decimal precision
  scoreWeights: {
    authenticity: 25,
    activity: 25,
    age: 15,
    associations: 20,
    accumulation: 15,
  },
  scoreMultipliers: {
    "0-20": 0.1,
    "20-40": 0.4,
    "40-60": 0.7,
    "60-80": 1.0,
    "80-100": 1.2,
  },
};

// ============ Content Constants ============

export const CONTENT_CONSTANTS = {
  maxEnergy: 100,
  energyRegenRate: 10, // per hour
  createCost: 10,
  editCost: 5,
  deleteCost: 0,
};

// ============ Governance Constants ============

export const GOVERNANCE_CONSTANTS = {
  minProposalThreshold: 100, // 100 veVCoin
  votingDuration: 7 * 24 * 3600, // 7 days
  executionDelay: 2 * 24 * 3600, // 2 days
  vetoWindow: 24 * 3600, // 1 day
  quorumBps: 400, // 4%
};

