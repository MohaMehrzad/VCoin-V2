import { PublicKey } from "@solana/web3.js";

// ============ Program IDs (Devnet Deployed) ============

export const PROGRAM_IDS = {
  vcoinToken: new PublicKey("Gg1dtrjAfGYi6NLC31WaJjZNBoucvD98rK2h1u9qrUjn"),
  vevcoinToken: new PublicKey("FB39ae9x53FxVL3pER9LqCPEx2TRnEnQP55i838Upnjx"),
  stakingProtocol: new PublicKey("6EFcistyr2E81adLUcuBJRr8W2xzpt3D3dFYEcMewpWu"),
  transferHook: new PublicKey("9K14FcDRrBeHKD9FPNYeVJaEqJQTac2xspJyb1mM6m48"),
  identityProtocol: new PublicKey("3egAds3pFR5oog6iQCN42KPvgih8HQz2FGybNjiVWixG"),
  fiveAProtocol: new PublicKey("783PbtJw5cc7yatnr9fsvTGSnkKaV6iJe6E8VUPTYrT8"),
  contentRegistry: new PublicKey("MJn1A4MPCBPJGWWuZrtq7bHSo2G289sUwW3ej2wcmLV"),
  governanceProtocol: new PublicKey("3R256kBN9iXozjypQFRAmegBhd6HJqXWqdNG7Th78HYe"),
  sscreProtocol: new PublicKey("6AJNcQSfoiE2UAeUDyJUBumS9SBwhAdSznoAeYpXrxXZ"),
  vilinkProtocol: new PublicKey("CFGXTS2MueQwTYTMMTBQbRWzJtSTC2p4ZRuKPpLDmrv7"),
  gaslessProtocol: new PublicKey("FcXJAjzJs8eVY2WTRFXynQBpC7WZUqKZppyp9xS6PaB3"),
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
  none:     { minStake: 0,       feeDiscount: 0,  boost: 1.0, minLock: 0 },
  bronze:   { minStake: 1_000,   feeDiscount: 10, boost: 1.1, minLock: 0 },
  silver:   { minStake: 5_000,   feeDiscount: 20, boost: 1.2, minLock: 0 },
  gold:     { minStake: 20_000,  feeDiscount: 30, boost: 1.3, minLock: 0 },
  platinum: { minStake: 100_000, feeDiscount: 50, boost: 1.4, minLock: 0 },
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
    authenticity: 25, // A1 - "Are you a real person?"
    accuracy: 20,     // A2 - "Is your content quality?"
    agility: 15,      // A3 - "Are you fast?"
    activity: 25,     // A4 - "Do you show up daily?"
    approved: 15,     // A5 - "Does the community like you?"
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

