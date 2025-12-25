"use strict";
var __defProp = Object.defineProperty;
var __getOwnPropDesc = Object.getOwnPropertyDescriptor;
var __getOwnPropNames = Object.getOwnPropertyNames;
var __hasOwnProp = Object.prototype.hasOwnProperty;
var __export = (target, all) => {
  for (var name in all)
    __defProp(target, name, { get: all[name], enumerable: true });
};
var __copyProps = (to, from, except, desc) => {
  if (from && typeof from === "object" || typeof from === "function") {
    for (let key of __getOwnPropNames(from))
      if (!__hasOwnProp.call(to, key) && key !== except)
        __defProp(to, key, { get: () => from[key], enumerable: !(desc = __getOwnPropDesc(from, key)) || desc.enumerable });
  }
  return to;
};
var __toCommonJS = (mod) => __copyProps(__defProp({}, "__esModule", { value: true }), mod);

// src/index.ts
var index_exports = {};
__export(index_exports, {
  ACTION_SCOPES: () => ACTION_SCOPES,
  ActionType: () => ActionType,
  BN: () => import_anchor.BN,
  CONTENT_CONSTANTS: () => CONTENT_CONSTANTS,
  ContentClient: () => ContentClient,
  ContentState: () => ContentState,
  FIVE_A_CONSTANTS: () => FIVE_A_CONSTANTS,
  FeeMethod: () => FeeMethod,
  FiveAClient: () => FiveAClient,
  GASLESS_CONSTANTS: () => GASLESS_CONSTANTS,
  GOVERNANCE_CONSTANTS: () => GOVERNANCE_CONSTANTS,
  GaslessClient: () => GaslessClient,
  GovernanceClient: () => GovernanceClient,
  IdentityClient: () => IdentityClient,
  LEGACY_SLASH_DEPRECATED: () => LEGACY_SLASH_DEPRECATED,
  LOCK_DURATIONS: () => LOCK_DURATIONS,
  MAX_EPOCH_BITMAP: () => MAX_EPOCH_BITMAP,
  MAX_URI_LENGTH: () => MAX_URI_LENGTH,
  MERKLE_CONSTANTS: () => MERKLE_CONSTANTS,
  MERKLE_PROOF_MAX_SIZE: () => MERKLE_PROOF_MAX_SIZE,
  PDAs: () => PDAs,
  PROGRAM_IDS: () => PROGRAM_IDS,
  ProposalStatus: () => ProposalStatus,
  RewardsClient: () => RewardsClient,
  SECURITY_CONSTANTS: () => SECURITY_CONSTANTS,
  SEEDS: () => SEEDS,
  SSCRE_CONSTANTS: () => SSCRE_CONSTANTS,
  STAKING_TIERS: () => STAKING_TIERS,
  SlashStatus: () => SlashStatus,
  StakingClient: () => StakingClient,
  StakingTier: () => StakingTier,
  TransactionBuilder: () => TransactionBuilder,
  VALID_URI_PREFIXES: () => VALID_URI_PREFIXES,
  VCOIN_DECIMALS: () => VCOIN_DECIMALS,
  VCOIN_INITIAL_CIRCULATING: () => VCOIN_INITIAL_CIRCULATING,
  VCOIN_TOTAL_SUPPLY: () => VCOIN_TOTAL_SUPPLY,
  VEVCOIN_DECIMALS: () => VEVCOIN_DECIMALS,
  VILINK_CONSTANTS: () => VILINK_CONSTANTS,
  VerificationLevel: () => VerificationLevel,
  ViLinkClient: () => ViLinkClient,
  ViWoClient: () => ViWoClient,
  ViWoConnection: () => ViWoConnection,
  VoteChoice: () => VoteChoice,
  dateToTimestamp: () => dateToTimestamp,
  formatVCoin: () => formatVCoin,
  getCurrentTimestamp: () => getCurrentTimestamp,
  parseVCoin: () => parseVCoin,
  timestampToDate: () => timestampToDate
});
module.exports = __toCommonJS(index_exports);

// src/core/index.ts
var import_web32 = require("@solana/web3.js");
var import_anchor = require("@coral-xyz/anchor");

// src/constants.ts
var import_web3 = require("@solana/web3.js");
var PROGRAM_IDS = {
  vcoinToken: new import_web3.PublicKey("Gg1dtrjAfGYi6NLC31WaJjZNBoucvD98rK2h1u9qrUjn"),
  vevcoinToken: new import_web3.PublicKey("FB39ae9x53FxVL3pER9LqCPEx2TRnEnQP55i838Upnjx"),
  stakingProtocol: new import_web3.PublicKey("6EFcistyr2E81adLUcuBJRr8W2xzpt3D3dFYEcMewpWu"),
  transferHook: new import_web3.PublicKey("9K14FcDRrBeHKD9FPNYeVJaEqJQTac2xspJyb1mM6m48"),
  identityProtocol: new import_web3.PublicKey("3egAds3pFR5oog6iQCN42KPvgih8HQz2FGybNjiVWixG"),
  fiveAProtocol: new import_web3.PublicKey("783PbtJw5cc7yatnr9fsvTGSnkKaV6iJe6E8VUPTYrT8"),
  contentRegistry: new import_web3.PublicKey("MJn1A4MPCBPJGWWuZrtq7bHSo2G289sUwW3ej2wcmLV"),
  governanceProtocol: new import_web3.PublicKey("3R256kBN9iXozjypQFRAmegBhd6HJqXWqdNG7Th78HYe"),
  sscreProtocol: new import_web3.PublicKey("6AJNcQSfoiE2UAeUDyJUBumS9SBwhAdSznoAeYpXrxXZ"),
  vilinkProtocol: new import_web3.PublicKey("CFGXTS2MueQwTYTMMTBQbRWzJtSTC2p4ZRuKPpLDmrv7"),
  gaslessProtocol: new import_web3.PublicKey("FcXJAjzJs8eVY2WTRFXynQBpC7WZUqKZppyp9xS6PaB3"),
  /**
   * VCoin Token Mint Address (Token-2022)
   * 
   * NOTE: This is a placeholder. Override via ViWoClient config.programIds.vcoinMint
   * after deploying your VCoin mint on devnet/mainnet.
   * 
   * Finding #2 Fix: SDK now filters token accounts by mint address to prevent
   * summing balances from other Token-2022 tokens.
   */
  vcoinMint: new import_web3.PublicKey("11111111111111111111111111111111")
  // Placeholder - override in config
};
var SEEDS = {
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
  // Security (Phase 2-4)
  slashRequest: "slash-request",
  // H-01: Governance slashing
  decryptionShare: "decryption-share",
  // C-02: ZK voting shares
  pendingScore: "pending-score"
  // H-05: Oracle consensus
};
var VCOIN_DECIMALS = 9;
var VEVCOIN_DECIMALS = 9;
var VCOIN_TOTAL_SUPPLY = 1e9;
var VCOIN_INITIAL_CIRCULATING = 1e8;
var STAKING_TIERS = {
  none: { minStake: 0, feeDiscount: 0, boost: 1, minLock: 0 },
  bronze: { minStake: 1e3, feeDiscount: 10, boost: 1.1, minLock: 0 },
  silver: { minStake: 5e3, feeDiscount: 20, boost: 1.2, minLock: 0 },
  gold: { minStake: 2e4, feeDiscount: 30, boost: 1.3, minLock: 0 },
  platinum: { minStake: 1e5, feeDiscount: 50, boost: 1.4, minLock: 0 }
};
var LOCK_DURATIONS = {
  none: 0,
  oneMonth: 30 * 24 * 3600,
  threeMonths: 90 * 24 * 3600,
  sixMonths: 180 * 24 * 3600,
  oneYear: 365 * 24 * 3600
};
var SSCRE_CONSTANTS = {
  primaryReserves: 35e7,
  // 350M VCoin
  secondaryReserves: 4e7,
  // 40M VCoin
  epochDuration: 30 * 24 * 3600,
  // 30 days
  claimWindow: 90 * 24 * 3600,
  // 90 days
  gaslessFeeBps: 100,
  // 1%
  minClaimAmount: 1,
  // 1 VCoin
  circuitBreakerCooldown: 21600
  // M-05: 6 hours before reset
};
var VILINK_CONSTANTS = {
  maxActionExpiry: 7 * 24 * 3600,
  // 7 days
  minTipAmount: 0.1,
  // 0.1 VCoin
  maxTipAmount: 1e4,
  // 10,000 VCoin
  platformFeeBps: 250,
  // 2.5%
  maxPlatformFeeBps: 1e3,
  // M-02: 10% max
  minPlatformFeeBps: 10
  // M-02: 0.1% min
};
var ACTION_SCOPES = {
  tip: 1 << 0,
  vouch: 1 << 1,
  content: 1 << 2,
  governance: 1 << 3,
  transfer: 1 << 4,
  stake: 1 << 5,
  claim: 1 << 6,
  follow: 1 << 7,
  all: 65535
};
var GASLESS_CONSTANTS = {
  sessionDuration: 24 * 3600,
  // 24 hours
  maxSessionActions: 1e3,
  maxSessionSpend: 1e5,
  // 100,000 VCoin
  defaultSolFee: 5e3,
  // 0.000005 SOL
  vcoinFeeMultiplier: 100,
  sscreDeductionBps: 100,
  // 1%
  dailySubsidyBudget: 10,
  // 10 SOL
  maxSubsidizedPerUser: 50,
  maxSlippageBps: 500
  // L-03: 5% max slippage for fee conversion
};
var FIVE_A_CONSTANTS = {
  maxScore: 1e4,
  // 100.00 with 2 decimal precision
  scoreWeights: {
    authenticity: 25,
    // A1 - "Are you a real person?"
    accuracy: 20,
    // A2 - "Is your content quality?"
    agility: 15,
    // A3 - "Are you fast?"
    activity: 25,
    // A4 - "Do you show up daily?"
    approved: 15
    // A5 - "Does the community like you?"
  },
  scoreMultipliers: {
    "0-20": 0.1,
    "20-40": 0.4,
    "40-60": 0.7,
    "60-80": 1,
    "80-100": 1.2
  },
  // H-05: Oracle consensus
  oracleConsensusRequired: 3,
  // 3-of-N oracles must agree
  pendingScoreExpiry: 3600,
  // 1 hour
  // L-07: Rate limiting
  minScoreUpdateInterval: 3600
  // 1 hour between updates for same user
};
var CONTENT_CONSTANTS = {
  maxEnergy: 100,
  energyRegenRate: 10,
  // per hour
  createCost: 10,
  editCost: 5,
  deleteCost: 0
};
var GOVERNANCE_CONSTANTS = {
  minProposalThreshold: 100,
  // 100 veVCoin
  votingDuration: 7 * 24 * 3600,
  // 7 days
  executionDelay: 2 * 24 * 3600,
  // 2 days
  vetoWindow: 24 * 3600,
  // 1 day
  quorumBps: 400,
  // 4%
  zkVotingEnabled: false
  // C-01: Disabled until proper ZK infrastructure
};
var SECURITY_CONSTANTS = {
  // H-02: Two-step authority transfer
  authorityTransferTimelock: 24 * 3600,
  // 24 hours
  // H-01: Governance-controlled slashing
  slashApprovalTimelock: 48 * 3600,
  // 48 hours
  slashExpiry: 7 * 24 * 3600,
  // 7 days
  // L-03: Slippage protection for gasless fees
  maxFeeSlippageBps: 500,
  // 5% max slippage
  // L-07: Oracle rate limiting
  minScoreUpdateInterval: 3600,
  // 1 hour between updates for same user
  // M-05: Circuit breaker cooldown
  circuitBreakerCooldown: 21600,
  // 6 hours (6 * 3600)
  // H-05: Oracle consensus
  oracleConsensusRequired: 3,
  // 3-of-N oracles must agree
  pendingScoreExpiry: 3600,
  // 1 hour
  // M-02: Platform fee bounds (ViLink)
  maxPlatformFeeBps: 1e3,
  // 10% max
  minPlatformFeeBps: 10,
  // 0.1% min
  // v2.8.0 Phase 5 additions
  merkleProofMaxSize: 32,
  // H-NEW-02: Max proof levels (supports 4B+ users)
  maxEpochBitmap: 1023,
  // H-NEW-04: Max epoch with bitmap storage (85+ years)
  votingPowerVerifiedOnChain: true
  // C-NEW-01: Params read from chain, not passed
};
var VALID_URI_PREFIXES = ["ipfs://", "https://", "ar://"];
var MAX_URI_LENGTH = 128;
var MERKLE_CONSTANTS = {
  leafDomainPrefix: "SSCRE_CLAIM_V1"
  // Domain separation for merkle leaves
};
var MERKLE_PROOF_MAX_SIZE = 32;
var MAX_EPOCH_BITMAP = 1023;
var LEGACY_SLASH_DEPRECATED = true;

// src/core/index.ts
var ViWoConnection = class {
  constructor(config) {
    this.commitment = config.commitment || "confirmed";
    this.connection = new import_web32.Connection(
      config.endpoint,
      {
        commitment: this.commitment,
        wsEndpoint: config.wsEndpoint
      }
    );
  }
  /**
   * Get current slot
   */
  async getSlot() {
    return this.connection.getSlot(this.commitment);
  }
  /**
   * Get current block time
   */
  async getBlockTime() {
    const slot = await this.getSlot();
    return this.connection.getBlockTime(slot);
  }
  /**
   * Check if connection is healthy
   */
  async isHealthy() {
    try {
      await this.connection.getVersion();
      return true;
    } catch {
      return false;
    }
  }
};
var PDAs = class {
  constructor(programIds = PROGRAM_IDS) {
    this.programIds = programIds;
  }
  // VCoin PDAs
  getVCoinConfig() {
    const [pda] = import_web32.PublicKey.findProgramAddressSync(
      [Buffer.from(SEEDS.vcoinConfig)],
      this.programIds.vcoinToken
    );
    return pda;
  }
  // Staking PDAs
  getStakingPool() {
    const [pda] = import_web32.PublicKey.findProgramAddressSync(
      [Buffer.from(SEEDS.stakingPool)],
      this.programIds.stakingProtocol
    );
    return pda;
  }
  getUserStake(user) {
    const [pda] = import_web32.PublicKey.findProgramAddressSync(
      [Buffer.from(SEEDS.userStake), user.toBuffer()],
      this.programIds.stakingProtocol
    );
    return pda;
  }
  // Governance PDAs
  getGovernanceConfig() {
    const [pda] = import_web32.PublicKey.findProgramAddressSync(
      [Buffer.from(SEEDS.governanceConfig)],
      this.programIds.governanceProtocol
    );
    return pda;
  }
  getProposal(proposalId) {
    const [pda] = import_web32.PublicKey.findProgramAddressSync(
      [Buffer.from(SEEDS.proposal), proposalId.toArrayLike(Buffer, "le", 8)],
      this.programIds.governanceProtocol
    );
    return pda;
  }
  getVoteRecord(user, proposal) {
    const [pda] = import_web32.PublicKey.findProgramAddressSync(
      [Buffer.from(SEEDS.voteRecord), user.toBuffer(), proposal.toBuffer()],
      this.programIds.governanceProtocol
    );
    return pda;
  }
  // SSCRE PDAs
  getRewardsPoolConfig() {
    const [pda] = import_web32.PublicKey.findProgramAddressSync(
      [Buffer.from(SEEDS.poolConfig)],
      this.programIds.sscreProtocol
    );
    return pda;
  }
  getEpochDistribution(epoch) {
    const [pda] = import_web32.PublicKey.findProgramAddressSync(
      [Buffer.from(SEEDS.epoch), epoch.toArrayLike(Buffer, "le", 8)],
      this.programIds.sscreProtocol
    );
    return pda;
  }
  getUserClaim(user) {
    const [pda] = import_web32.PublicKey.findProgramAddressSync(
      [Buffer.from(SEEDS.userClaim), user.toBuffer()],
      this.programIds.sscreProtocol
    );
    return pda;
  }
  // ViLink PDAs
  getViLinkConfig() {
    const [pda] = import_web32.PublicKey.findProgramAddressSync(
      [Buffer.from(SEEDS.vilinkConfig)],
      this.programIds.vilinkProtocol
    );
    return pda;
  }
  /**
   * Get ViLink action PDA
   * @param creator - The action creator's public key
   * @param nonce - M-04: The action nonce (deterministic counter, NOT timestamp)
   * @deprecated Use getViLinkActionByNonce for clarity
   */
  getViLinkAction(creator, nonce) {
    return this.getViLinkActionByNonce(creator, nonce);
  }
  /**
   * Get ViLink action PDA using nonce (M-04 fix)
   * @param creator - The action creator's public key
   * @param nonce - The action nonce from UserActionStats.actionNonce
   */
  getViLinkActionByNonce(creator, nonce) {
    const [pda] = import_web32.PublicKey.findProgramAddressSync(
      [
        Buffer.from(SEEDS.action),
        creator.toBuffer(),
        nonce.toArrayLike(Buffer, "le", 8)
      ],
      this.programIds.vilinkProtocol
    );
    return pda;
  }
  getUserActionStats(user) {
    const [pda] = import_web32.PublicKey.findProgramAddressSync(
      [Buffer.from(SEEDS.userStats), user.toBuffer()],
      this.programIds.vilinkProtocol
    );
    return pda;
  }
  // Gasless PDAs
  getGaslessConfig() {
    const [pda] = import_web32.PublicKey.findProgramAddressSync(
      [Buffer.from(SEEDS.gaslessConfig)],
      this.programIds.gaslessProtocol
    );
    return pda;
  }
  getSessionKey(user, sessionPubkey) {
    const [pda] = import_web32.PublicKey.findProgramAddressSync(
      [
        Buffer.from(SEEDS.sessionKey),
        user.toBuffer(),
        sessionPubkey.toBuffer()
      ],
      this.programIds.gaslessProtocol
    );
    return pda;
  }
  getUserGaslessStats(user) {
    const [pda] = import_web32.PublicKey.findProgramAddressSync(
      [Buffer.from(SEEDS.userGasless), user.toBuffer()],
      this.programIds.gaslessProtocol
    );
    return pda;
  }
  // Identity PDAs
  getIdentityConfig() {
    const [pda] = import_web32.PublicKey.findProgramAddressSync(
      [Buffer.from(SEEDS.identityConfig)],
      this.programIds.identityProtocol
    );
    return pda;
  }
  getUserIdentity(user) {
    const [pda] = import_web32.PublicKey.findProgramAddressSync(
      [Buffer.from(SEEDS.identity), user.toBuffer()],
      this.programIds.identityProtocol
    );
    return pda;
  }
  // 5A Protocol PDAs
  getFiveAConfig() {
    const [pda] = import_web32.PublicKey.findProgramAddressSync(
      [Buffer.from(SEEDS.fiveAConfig)],
      this.programIds.fiveAProtocol
    );
    return pda;
  }
  getUserScore(user) {
    const [pda] = import_web32.PublicKey.findProgramAddressSync(
      [Buffer.from(SEEDS.userScore), user.toBuffer()],
      this.programIds.fiveAProtocol
    );
    return pda;
  }
  // Content PDAs
  getContentRegistryConfig() {
    const [pda] = import_web32.PublicKey.findProgramAddressSync(
      [Buffer.from(SEEDS.registryConfig)],
      this.programIds.contentRegistry
    );
    return pda;
  }
  getContentRecord(contentId) {
    const [pda] = import_web32.PublicKey.findProgramAddressSync(
      [Buffer.from(SEEDS.content), Buffer.from(contentId)],
      this.programIds.contentRegistry
    );
    return pda;
  }
  getUserEnergy(user) {
    const [pda] = import_web32.PublicKey.findProgramAddressSync(
      [Buffer.from(SEEDS.userEnergy), user.toBuffer()],
      this.programIds.contentRegistry
    );
    return pda;
  }
};
var TransactionBuilder = class {
  constructor() {
    this.instructions = [];
  }
  add(instruction) {
    this.instructions.push(instruction);
    return this;
  }
  addMany(instructions) {
    this.instructions.push(...instructions);
    return this;
  }
  build() {
    const tx = new import_web32.Transaction();
    for (const ix of this.instructions) {
      tx.add(ix);
    }
    return tx;
  }
  clear() {
    this.instructions = [];
    return this;
  }
  get length() {
    return this.instructions.length;
  }
};
function formatVCoin(amount, decimals = 9) {
  const amountBN = typeof amount === "number" ? new import_anchor.BN(amount) : amount;
  const divisor = new import_anchor.BN(10).pow(new import_anchor.BN(decimals));
  const whole = amountBN.div(divisor).toString();
  const fraction = amountBN.mod(divisor).toString().padStart(decimals, "0");
  return `${whole}.${fraction}`;
}
function parseVCoin(amount, decimals = 9) {
  if (typeof amount === "number") {
    amount = amount.toString();
  }
  const [whole, fraction = ""] = amount.split(".");
  const paddedFraction = fraction.padEnd(decimals, "0").slice(0, decimals);
  return new import_anchor.BN(whole + paddedFraction);
}
function getCurrentTimestamp() {
  return Math.floor(Date.now() / 1e3);
}
function timestampToDate(timestamp) {
  const ts = typeof timestamp === "number" ? timestamp : timestamp.toNumber();
  return new Date(ts * 1e3);
}
function dateToTimestamp(date) {
  return Math.floor(date.getTime() / 1e3);
}

// src/types.ts
var SlashStatus = /* @__PURE__ */ ((SlashStatus2) => {
  SlashStatus2[SlashStatus2["Proposed"] = 0] = "Proposed";
  SlashStatus2[SlashStatus2["Approved"] = 1] = "Approved";
  SlashStatus2[SlashStatus2["Executed"] = 2] = "Executed";
  SlashStatus2[SlashStatus2["Cancelled"] = 3] = "Cancelled";
  return SlashStatus2;
})(SlashStatus || {});
var StakingTier = /* @__PURE__ */ ((StakingTier2) => {
  StakingTier2[StakingTier2["None"] = 0] = "None";
  StakingTier2[StakingTier2["Bronze"] = 1] = "Bronze";
  StakingTier2[StakingTier2["Silver"] = 2] = "Silver";
  StakingTier2[StakingTier2["Gold"] = 3] = "Gold";
  StakingTier2[StakingTier2["Platinum"] = 4] = "Platinum";
  return StakingTier2;
})(StakingTier || {});
var ProposalStatus = /* @__PURE__ */ ((ProposalStatus2) => {
  ProposalStatus2[ProposalStatus2["Active"] = 0] = "Active";
  ProposalStatus2[ProposalStatus2["Passed"] = 1] = "Passed";
  ProposalStatus2[ProposalStatus2["Rejected"] = 2] = "Rejected";
  ProposalStatus2[ProposalStatus2["Executed"] = 3] = "Executed";
  ProposalStatus2[ProposalStatus2["Cancelled"] = 4] = "Cancelled";
  return ProposalStatus2;
})(ProposalStatus || {});
var VoteChoice = /* @__PURE__ */ ((VoteChoice2) => {
  VoteChoice2[VoteChoice2["Against"] = 0] = "Against";
  VoteChoice2[VoteChoice2["For"] = 1] = "For";
  VoteChoice2[VoteChoice2["Abstain"] = 2] = "Abstain";
  return VoteChoice2;
})(VoteChoice || {});
var ActionType = /* @__PURE__ */ ((ActionType2) => {
  ActionType2[ActionType2["Tip"] = 0] = "Tip";
  ActionType2[ActionType2["Vouch"] = 1] = "Vouch";
  ActionType2[ActionType2["Follow"] = 2] = "Follow";
  ActionType2[ActionType2["Challenge"] = 3] = "Challenge";
  ActionType2[ActionType2["Stake"] = 4] = "Stake";
  ActionType2[ActionType2["ContentReact"] = 5] = "ContentReact";
  ActionType2[ActionType2["Delegate"] = 6] = "Delegate";
  ActionType2[ActionType2["Vote"] = 7] = "Vote";
  return ActionType2;
})(ActionType || {});
var FeeMethod = /* @__PURE__ */ ((FeeMethod2) => {
  FeeMethod2[FeeMethod2["PlatformSubsidized"] = 0] = "PlatformSubsidized";
  FeeMethod2[FeeMethod2["VCoinDeduction"] = 1] = "VCoinDeduction";
  FeeMethod2[FeeMethod2["SSCREDeduction"] = 2] = "SSCREDeduction";
  return FeeMethod2;
})(FeeMethod || {});
var VerificationLevel = /* @__PURE__ */ ((VerificationLevel2) => {
  VerificationLevel2[VerificationLevel2["None"] = 0] = "None";
  VerificationLevel2[VerificationLevel2["Basic"] = 1] = "Basic";
  VerificationLevel2[VerificationLevel2["KYC"] = 2] = "KYC";
  VerificationLevel2[VerificationLevel2["Full"] = 3] = "Full";
  VerificationLevel2[VerificationLevel2["Enhanced"] = 4] = "Enhanced";
  return VerificationLevel2;
})(VerificationLevel || {});
var ContentState = /* @__PURE__ */ ((ContentState2) => {
  ContentState2[ContentState2["Active"] = 0] = "Active";
  ContentState2[ContentState2["Hidden"] = 1] = "Hidden";
  ContentState2[ContentState2["Deleted"] = 2] = "Deleted";
  ContentState2[ContentState2["Flagged"] = 3] = "Flagged";
  return ContentState2;
})(ContentState || {});

// src/staking/index.ts
var import_web33 = require("@solana/web3.js");
var import_anchor2 = require("@coral-xyz/anchor");
var StakingClient = class {
  constructor(client) {
    this.client = client;
  }
  /**
   * Get staking pool configuration
   */
  async getPool() {
    try {
      const poolPda = this.client.pdas.getStakingPool();
      const accountInfo = await this.client.connection.connection.getAccountInfo(poolPda);
      if (!accountInfo) {
        return null;
      }
      return {
        authority: new import_web33.PublicKey(accountInfo.data.slice(8, 40)),
        vcoinMint: new import_web33.PublicKey(accountInfo.data.slice(40, 72)),
        vevcoinMint: new import_web33.PublicKey(accountInfo.data.slice(72, 104)),
        totalStaked: new import_anchor2.BN(accountInfo.data.slice(104, 112), "le"),
        totalVevcoinMinted: new import_anchor2.BN(accountInfo.data.slice(112, 120), "le"),
        paused: accountInfo.data[120] !== 0
      };
    } catch {
      return null;
    }
  }
  /**
   * Get user stake information
   */
  async getUserStake(user) {
    const target = user || this.client.publicKey;
    if (!target) {
      throw new Error("No user specified and wallet not connected");
    }
    try {
      const stakePda = this.client.pdas.getUserStake(target);
      const accountInfo = await this.client.connection.connection.getAccountInfo(stakePda);
      if (!accountInfo) {
        return null;
      }
      return {
        user: new import_web33.PublicKey(accountInfo.data.slice(8, 40)),
        stakedAmount: new import_anchor2.BN(accountInfo.data.slice(40, 48), "le"),
        vevcoinBalance: new import_anchor2.BN(accountInfo.data.slice(48, 56), "le"),
        tier: accountInfo.data[56],
        lockEndTime: new import_anchor2.BN(accountInfo.data.slice(57, 65), "le"),
        lastUpdateTime: new import_anchor2.BN(accountInfo.data.slice(65, 73), "le")
      };
    } catch {
      return null;
    }
  }
  /**
   * Calculate tier based on stake amount
   */
  calculateTier(stakeAmount) {
    const amount = typeof stakeAmount === "number" ? stakeAmount : stakeAmount.toNumber() / Math.pow(10, VCOIN_DECIMALS);
    if (amount >= STAKING_TIERS.platinum.minStake) return 4;
    if (amount >= STAKING_TIERS.gold.minStake) return 3;
    if (amount >= STAKING_TIERS.silver.minStake) return 2;
    if (amount >= STAKING_TIERS.bronze.minStake) return 1;
    return 0;
  }
  /**
   * Calculate veVCoin amount for given stake
   * Formula: ve_vcoin = staked_amount × (lock_duration / 4_years) × tier_boost
   */
  calculateVeVCoin(amount, lockDuration) {
    const FOUR_YEARS = 4 * 365 * 24 * 3600;
    const lockRatio = lockDuration / FOUR_YEARS;
    const tier = this.calculateTier(amount);
    const tierBoosts = [1, 1.1, 1.2, 1.3, 1.4];
    const tierBoost = tierBoosts[tier];
    const multiplier = lockRatio * tierBoost;
    const vevcoinAmount = amount.toNumber() * multiplier;
    return new import_anchor2.BN(Math.floor(vevcoinAmount));
  }
  /**
   * Get tier name
   */
  getTierName(tier) {
    const names = ["None", "Bronze", "Silver", "Gold", "Platinum"];
    return names[tier] || "Unknown";
  }
  /**
   * Get tier info
   */
  getTierInfo(tier) {
    const tiers = [
      STAKING_TIERS.none,
      STAKING_TIERS.bronze,
      STAKING_TIERS.silver,
      STAKING_TIERS.gold,
      STAKING_TIERS.platinum
    ];
    return tiers[tier] || STAKING_TIERS.none;
  }
  /**
   * Check if user can unstake
   */
  async canUnstake(user) {
    const stakeInfo = await this.getUserStake(user);
    if (!stakeInfo) {
      return { canUnstake: false, reason: "No active stake found" };
    }
    if (stakeInfo.stakedAmount.isZero()) {
      return { canUnstake: false, reason: "No staked amount" };
    }
    const now = Math.floor(Date.now() / 1e3);
    if (stakeInfo.lockEndTime.toNumber() > now) {
      const remaining = stakeInfo.lockEndTime.toNumber() - now;
      const days = Math.ceil(remaining / 86400);
      return { canUnstake: false, reason: `Lock period active: ${days} days remaining` };
    }
    return { canUnstake: true };
  }
  /**
   * Get staking statistics
   */
  async getStats() {
    const pool = await this.getPool();
    const userStake = this.client.publicKey ? await this.getUserStake() : null;
    return {
      totalStaked: pool ? formatVCoin(pool.totalStaked) : "0",
      totalVevcoin: pool ? formatVCoin(pool.totalVevcoinMinted) : "0",
      userStake: userStake ? formatVCoin(userStake.stakedAmount) : null,
      userVevcoin: userStake ? formatVCoin(userStake.vevcoinBalance) : null,
      userTier: userStake ? this.getTierName(userStake.tier) : null
    };
  }
  // ============ Transaction Building ============
  // Note: Full implementation would use Anchor IDL
  /**
   * Build stake instruction
   * @param params Stake parameters
   * @returns Transaction to sign and send
   */
  async buildStakeTransaction(params) {
    if (!this.client.publicKey) {
      throw new Error("Wallet not connected");
    }
    const tx = new import_web33.Transaction();
    return tx;
  }
  /**
   * Build unstake instruction
   * @returns Transaction to sign and send
   */
  async buildUnstakeTransaction() {
    if (!this.client.publicKey) {
      throw new Error("Wallet not connected");
    }
    const { canUnstake, reason } = await this.canUnstake();
    if (!canUnstake) {
      throw new Error(reason);
    }
    const tx = new import_web33.Transaction();
    return tx;
  }
  /**
   * Build extend lock instruction
   * @param newDuration New lock duration in seconds
   * @returns Transaction to sign and send
   */
  async buildExtendLockTransaction(newDuration) {
    if (!this.client.publicKey) {
      throw new Error("Wallet not connected");
    }
    const tx = new import_web33.Transaction();
    return tx;
  }
};

// src/governance/index.ts
var import_web34 = require("@solana/web3.js");
var import_anchor3 = require("@coral-xyz/anchor");
var GovernanceClient = class {
  constructor(client) {
    this.client = client;
  }
  /**
   * Get governance configuration
   */
  async getConfig() {
    try {
      const configPda = this.client.pdas.getGovernanceConfig();
      const accountInfo = await this.client.connection.connection.getAccountInfo(configPda);
      if (!accountInfo) {
        return null;
      }
      return {
        authority: new import_web34.PublicKey(accountInfo.data.slice(8, 40)),
        proposalCount: new import_anchor3.BN(accountInfo.data.slice(40, 48), "le"),
        vevcoinMint: new import_web34.PublicKey(accountInfo.data.slice(48, 80)),
        paused: accountInfo.data[80] !== 0
      };
    } catch {
      return null;
    }
  }
  /**
   * Get proposal by ID
   */
  async getProposal(proposalId) {
    try {
      const proposalPda = this.client.pdas.getProposal(proposalId);
      const accountInfo = await this.client.connection.connection.getAccountInfo(proposalPda);
      if (!accountInfo) {
        return null;
      }
      const data = accountInfo.data;
      return {
        id: new import_anchor3.BN(data.slice(8, 16), "le"),
        proposer: new import_web34.PublicKey(data.slice(16, 48)),
        title: Buffer.from(data.slice(48, 112)).toString("utf8").replace(/\0/g, ""),
        descriptionHash: new Uint8Array(data.slice(112, 144)),
        startTime: new import_anchor3.BN(data.slice(144, 152), "le"),
        endTime: new import_anchor3.BN(data.slice(152, 160), "le"),
        votesFor: new import_anchor3.BN(data.slice(160, 168), "le"),
        votesAgainst: new import_anchor3.BN(data.slice(168, 176), "le"),
        status: data[176],
        executed: data[177] !== 0,
        category: data[178]
      };
    } catch {
      return null;
    }
  }
  /**
   * Get all active proposals
   */
  async getActiveProposals() {
    const config = await this.getConfig();
    if (!config) return [];
    const proposals = [];
    const now = Math.floor(Date.now() / 1e3);
    const proposalCount = config.proposalCount.toNumber();
    const startFrom = Math.max(0, proposalCount - 20);
    for (let i = startFrom; i < proposalCount; i++) {
      const proposal = await this.getProposal(new import_anchor3.BN(i));
      if (proposal && proposal.endTime.toNumber() > now && proposal.status === 0) {
        proposals.push(proposal);
      }
    }
    return proposals;
  }
  /**
   * Get user's vote record for a proposal
   */
  async getVoteRecord(proposalId, user) {
    const target = user || this.client.publicKey;
    if (!target) {
      throw new Error("No user specified and wallet not connected");
    }
    try {
      const proposalPda = this.client.pdas.getProposal(proposalId);
      const votePda = this.client.pdas.getVoteRecord(target, proposalPda);
      const accountInfo = await this.client.connection.connection.getAccountInfo(votePda);
      if (!accountInfo) {
        return null;
      }
      const data = accountInfo.data;
      return {
        user: new import_web34.PublicKey(data.slice(8, 40)),
        proposal: new import_web34.PublicKey(data.slice(40, 72)),
        votePower: new import_anchor3.BN(data.slice(72, 80), "le"),
        support: data[80] !== 0,
        votedAt: new import_anchor3.BN(data.slice(81, 89), "le")
      };
    } catch {
      return null;
    }
  }
  /**
   * Check if user has voted on a proposal
   */
  async hasVoted(proposalId, user) {
    const voteRecord = await this.getVoteRecord(proposalId, user);
    return voteRecord !== null;
  }
  /**
   * Calculate user's voting power
   */
  async getVotingPower(user) {
    const target = user || this.client.publicKey;
    if (!target) {
      throw new Error("No user specified and wallet not connected");
    }
    const vevcoinBalance = await this.client.getVeVCoinBalance(target);
    const fiveAMultiplier = 1;
    return new import_anchor3.BN(Math.floor(vevcoinBalance.toNumber() * fiveAMultiplier));
  }
  /**
   * Get proposal status text
   */
  getStatusText(status) {
    const statuses = ["Active", "Passed", "Rejected", "Executed", "Cancelled"];
    return statuses[status] || "Unknown";
  }
  /**
   * Check if proposal can be executed
   */
  async canExecute(proposalId) {
    const proposal = await this.getProposal(proposalId);
    if (!proposal) {
      return { canExecute: false, reason: "Proposal not found" };
    }
    if (proposal.executed) {
      return { canExecute: false, reason: "Proposal already executed" };
    }
    if (proposal.status !== 1 /* Passed */) {
      return { canExecute: false, reason: "Proposal has not passed" };
    }
    const now = Math.floor(Date.now() / 1e3);
    const executionDelay = proposal.endTime.toNumber() + GOVERNANCE_CONSTANTS.executionDelay;
    if (now < executionDelay) {
      const remaining = executionDelay - now;
      const hours = Math.ceil(remaining / 3600);
      return { canExecute: false, reason: `Execution delay: ${hours} hours remaining` };
    }
    return { canExecute: true };
  }
  /**
   * Get proposal progress
   */
  async getProposalProgress(proposalId) {
    const proposal = await this.getProposal(proposalId);
    if (!proposal) {
      throw new Error("Proposal not found");
    }
    const totalVotes = proposal.votesFor.add(proposal.votesAgainst);
    const forPct = totalVotes.isZero() ? 0 : proposal.votesFor.toNumber() / totalVotes.toNumber() * 100;
    const againstPct = 100 - forPct;
    const quorumThreshold = new import_anchor3.BN(GOVERNANCE_CONSTANTS.quorumBps).mul(new import_anchor3.BN(1e4));
    const quorumReached = totalVotes.gte(quorumThreshold);
    const now = Math.floor(Date.now() / 1e3);
    const timeRemaining = Math.max(0, proposal.endTime.toNumber() - now);
    return {
      votesFor: formatVCoin(proposal.votesFor),
      votesAgainst: formatVCoin(proposal.votesAgainst),
      totalVotes: formatVCoin(totalVotes),
      forPercentage: forPct,
      againstPercentage: againstPct,
      quorumReached,
      timeRemaining
    };
  }
  // ============ Transaction Building ============
  /**
   * Build create proposal transaction
   */
  async buildCreateProposalTransaction(params) {
    if (!this.client.publicKey) {
      throw new Error("Wallet not connected");
    }
    const votingPower = await this.getVotingPower();
    if (votingPower.toNumber() < GOVERNANCE_CONSTANTS.minProposalThreshold) {
      throw new Error(`Insufficient voting power. Need ${GOVERNANCE_CONSTANTS.minProposalThreshold} veVCoin`);
    }
    const tx = new import_web34.Transaction();
    return tx;
  }
  /**
   * Build vote transaction
   * 
   * @note v2.8.0 (C-NEW-01): Voting power parameters (vevcoin_balance, five_a_score, tier) 
   * are now read from on-chain state, not passed as parameters. This prevents vote manipulation.
   * The transaction only needs: proposal_id and choice (VoteChoice enum)
   * 
   * @param proposalId - The proposal to vote on
   * @param support - true = For, false = Against (use VoteChoice for more options)
   */
  async buildVoteTransaction(proposalId, support) {
    if (!this.client.publicKey) {
      throw new Error("Wallet not connected");
    }
    const hasVoted = await this.hasVoted(proposalId);
    if (hasVoted) {
      throw new Error("Already voted on this proposal");
    }
    const tx = new import_web34.Transaction();
    const choice = support ? 1 /* For */ : 0 /* Against */;
    return tx;
  }
  /**
   * Build execute proposal transaction
   */
  async buildExecuteTransaction(proposalId) {
    if (!this.client.publicKey) {
      throw new Error("Wallet not connected");
    }
    const { canExecute, reason } = await this.canExecute(proposalId);
    if (!canExecute) {
      throw new Error(reason);
    }
    const tx = new import_web34.Transaction();
    return tx;
  }
};

// src/rewards/index.ts
var import_web35 = require("@solana/web3.js");
var import_anchor4 = require("@coral-xyz/anchor");
var RewardsClient = class {
  constructor(client) {
    this.client = client;
  }
  /**
   * Get rewards pool configuration
   */
  async getPoolConfig() {
    try {
      const configPda = this.client.pdas.getRewardsPoolConfig();
      const accountInfo = await this.client.connection.connection.getAccountInfo(configPda);
      if (!accountInfo) {
        return null;
      }
      const data = accountInfo.data;
      return {
        authority: new import_web35.PublicKey(data.slice(8, 40)),
        vcoinMint: new import_web35.PublicKey(data.slice(40, 72)),
        currentEpoch: new import_anchor4.BN(data.slice(72, 80), "le"),
        totalDistributed: new import_anchor4.BN(data.slice(80, 88), "le"),
        remainingReserves: new import_anchor4.BN(data.slice(88, 96), "le"),
        paused: data[96] !== 0
      };
    } catch {
      return null;
    }
  }
  /**
   * Get epoch distribution details
   */
  async getEpochDistribution(epoch) {
    try {
      const epochPda = this.client.pdas.getEpochDistribution(epoch);
      const accountInfo = await this.client.connection.connection.getAccountInfo(epochPda);
      if (!accountInfo) {
        return null;
      }
      const data = accountInfo.data;
      return {
        epoch: new import_anchor4.BN(data.slice(8, 16), "le"),
        merkleRoot: new Uint8Array(data.slice(16, 48)),
        totalAllocation: new import_anchor4.BN(data.slice(48, 56), "le"),
        totalClaimed: new import_anchor4.BN(data.slice(56, 64), "le"),
        claimsCount: new import_anchor4.BN(data.slice(64, 72), "le"),
        isFinalized: data[72] !== 0
      };
    } catch {
      return null;
    }
  }
  /**
   * Get current epoch
   */
  async getCurrentEpoch() {
    const config = await this.getPoolConfig();
    return config?.currentEpoch || new import_anchor4.BN(0);
  }
  /**
   * Get user claim history
   */
  async getUserClaim(user) {
    const target = user || this.client.publicKey;
    if (!target) {
      throw new Error("No user specified and wallet not connected");
    }
    try {
      const claimPda = this.client.pdas.getUserClaim(target);
      const accountInfo = await this.client.connection.connection.getAccountInfo(claimPda);
      if (!accountInfo) {
        return null;
      }
      const data = accountInfo.data;
      return {
        user: new import_web35.PublicKey(data.slice(8, 40)),
        lastClaimedEpoch: new import_anchor4.BN(data.slice(40, 48), "le"),
        totalClaimed: new import_anchor4.BN(data.slice(48, 56), "le"),
        claimsCount: data.readUInt32LE(56)
      };
    } catch {
      return null;
    }
  }
  /**
   * Check if user has claimed for an epoch
   */
  async hasClaimedEpoch(epoch, user) {
    const userClaim = await this.getUserClaim(user);
    if (!userClaim) return false;
    const epochNum = epoch.toNumber();
    if (epochNum <= 255) {
      return userClaim.lastClaimedEpoch.gte(epoch);
    }
    return userClaim.lastClaimedEpoch.gte(epoch);
  }
  /**
   * Get unclaimed epochs
   */
  async getUnclaimedEpochs(user) {
    const currentEpoch = await this.getCurrentEpoch();
    const userClaim = await this.getUserClaim(user);
    const unclaimed = [];
    const startEpoch = userClaim ? userClaim.lastClaimedEpoch.toNumber() + 1 : 1;
    for (let e = startEpoch; e <= currentEpoch.toNumber(); e++) {
      const epochDist = await this.getEpochDistribution(new import_anchor4.BN(e));
      if (epochDist?.isFinalized) {
        const now = Math.floor(Date.now() / 1e3);
        const claimExpiry = epochDist.epoch.toNumber() * SSCRE_CONSTANTS.epochDuration + SSCRE_CONSTANTS.claimWindow;
        if (now <= claimExpiry) {
          unclaimed.push(new import_anchor4.BN(e));
        }
      }
    }
    return unclaimed;
  }
  /**
   * Get rewards statistics
   */
  async getStats() {
    const config = await this.getPoolConfig();
    const userClaim = this.client.publicKey ? await this.getUserClaim() : null;
    const totalReserves = SSCRE_CONSTANTS.primaryReserves * 1e9;
    const remaining = config?.remainingReserves.toNumber() || 0;
    const reservePct = remaining / totalReserves * 100;
    return {
      currentEpoch: config?.currentEpoch.toNumber() || 0,
      totalDistributed: config ? formatVCoin(config.totalDistributed) : "0",
      remainingReserves: config ? formatVCoin(config.remainingReserves) : "0",
      reservePercentage: reservePct,
      userTotalClaimed: userClaim ? formatVCoin(userClaim.totalClaimed) : null,
      userClaimsCount: userClaim?.claimsCount || null
    };
  }
  /**
   * Calculate gasless fee for claim
   */
  calculateGaslessFee(amount) {
    const fee = amount.muln(SSCRE_CONSTANTS.gaslessFeeBps).divn(1e4);
    return fee;
  }
  /**
   * Calculate net claim amount after fee
   */
  calculateNetClaim(amount) {
    const fee = this.calculateGaslessFee(amount);
    return amount.sub(fee);
  }
  // ============ Merkle Proof Utilities ============
  /**
   * Verify merkle proof locally
   */
  verifyMerkleProof(proof, root, leaf) {
    let computedHash = leaf;
    for (const proofElement of proof) {
      const combined = new Uint8Array(64);
      if (this.compareBytes(computedHash, proofElement) < 0) {
        combined.set(computedHash, 0);
        combined.set(proofElement, 32);
      } else {
        combined.set(proofElement, 0);
        combined.set(computedHash, 32);
      }
      computedHash = this.hashBytes(combined);
    }
    return this.compareBytes(computedHash, root) === 0;
  }
  /**
   * Compute leaf hash from user data
   */
  computeLeaf(user, amount, epoch) {
    const data = new Uint8Array(48);
    data.set(user.toBytes(), 0);
    data.set(amount.toArrayLike(Buffer, "le", 8), 32);
    data.set(epoch.toArrayLike(Buffer, "le", 8), 40);
    return this.hashBytes(data);
  }
  compareBytes(a, b) {
    for (let i = 0; i < Math.min(a.length, b.length); i++) {
      if (a[i] !== b[i]) {
        return a[i] - b[i];
      }
    }
    return a.length - b.length;
  }
  hashBytes(data) {
    const hash = new Uint8Array(32);
    for (let i = 0; i < data.length; i++) {
      hash[i % 32] ^= data[i];
    }
    return hash;
  }
  // ============ Transaction Building ============
  /**
   * Build claim rewards transaction
   */
  async buildClaimTransaction(params) {
    if (!this.client.publicKey) {
      throw new Error("Wallet not connected");
    }
    if (params.amount.lt(new import_anchor4.BN(SSCRE_CONSTANTS.minClaimAmount * 1e9))) {
      throw new Error(`Claim amount below minimum: ${SSCRE_CONSTANTS.minClaimAmount} VCoin`);
    }
    const hasClaimed = await this.hasClaimedEpoch(params.epoch);
    if (hasClaimed) {
      throw new Error("Already claimed for this epoch");
    }
    const tx = new import_web35.Transaction();
    return tx;
  }
};

// src/vilink/index.ts
var import_web36 = require("@solana/web3.js");
var import_anchor5 = require("@coral-xyz/anchor");
var ViLinkClient = class {
  constructor(client) {
    this.client = client;
  }
  /**
   * Get ViLink configuration
   * 
   * Finding #8 (related): Corrected byte offsets to match on-chain ViLinkConfig struct.
   * Added pending_authority field that was missing after H-02 security fix.
   */
  async getConfig() {
    try {
      const configPda = this.client.pdas.getViLinkConfig();
      const accountInfo = await this.client.connection.connection.getAccountInfo(configPda);
      if (!accountInfo) {
        return null;
      }
      const data = accountInfo.data;
      return {
        authority: new import_web36.PublicKey(data.slice(8, 40)),
        pendingAuthority: new import_web36.PublicKey(data.slice(40, 72)),
        vcoinMint: new import_web36.PublicKey(data.slice(72, 104)),
        treasury: new import_web36.PublicKey(data.slice(104, 136)),
        enabledActions: data[296],
        totalActionsCreated: new import_anchor5.BN(data.slice(297, 305), "le"),
        totalActionsExecuted: new import_anchor5.BN(data.slice(305, 313), "le"),
        totalTipVolume: new import_anchor5.BN(data.slice(313, 321), "le"),
        paused: data[321] !== 0,
        platformFeeBps: data.readUInt16LE(322)
      };
    } catch (error) {
      console.warn("[ViWoSDK] vilink.getConfig failed:", error instanceof Error ? error.message : error);
      return null;
    }
  }
  /**
   * Get action by nonce (M-04: deterministic PDA derivation)
   * @param creator - The action creator's public key
   * @param nonce - The action nonce (from UserActionStats.actionNonce at creation time)
   */
  async getAction(creator, nonce) {
    try {
      const actionPda = this.client.pdas.getViLinkActionByNonce(creator, nonce);
      const accountInfo = await this.client.connection.connection.getAccountInfo(actionPda);
      if (!accountInfo) {
        return null;
      }
      const data = accountInfo.data;
      return {
        actionId: new Uint8Array(data.slice(8, 40)),
        creator: new import_web36.PublicKey(data.slice(40, 72)),
        target: new import_web36.PublicKey(data.slice(72, 104)),
        actionType: data[104],
        amount: new import_anchor5.BN(data.slice(105, 113), "le"),
        expiresAt: new import_anchor5.BN(data.slice(145, 153), "le"),
        executed: data[153] !== 0,
        executionCount: data.readUInt32LE(193),
        maxExecutions: data.readUInt32LE(197),
        actionNonce: nonce
        // M-04: Store nonce for reference
      };
    } catch (error) {
      console.warn("[ViWoSDK] vilink.getAction failed:", error instanceof Error ? error.message : error);
      return null;
    }
  }
  /**
   * @deprecated Use getAction with nonce parameter instead
   */
  async getActionByTimestamp(creator, timestamp) {
    return this.getAction(creator, timestamp);
  }
  /**
   * Get user action statistics
   */
  async getUserStats(user) {
    const target = user || this.client.publicKey;
    if (!target) {
      throw new Error("No user specified and wallet not connected");
    }
    try {
      const statsPda = this.client.pdas.getUserActionStats(target);
      const accountInfo = await this.client.connection.connection.getAccountInfo(statsPda);
      if (!accountInfo) {
        return null;
      }
      const data = accountInfo.data;
      return {
        user: new import_web36.PublicKey(data.slice(8, 40)),
        actionsCreated: new import_anchor5.BN(data.slice(40, 48), "le"),
        actionsExecuted: new import_anchor5.BN(data.slice(48, 56), "le"),
        tipsSent: new import_anchor5.BN(data.slice(56, 64), "le"),
        tipsReceived: new import_anchor5.BN(data.slice(64, 72), "le"),
        vcoinSent: new import_anchor5.BN(data.slice(72, 80), "le"),
        vcoinReceived: new import_anchor5.BN(data.slice(80, 88), "le")
      };
    } catch (error) {
      console.warn("[ViWoSDK] vilink.getUserStats failed:", error instanceof Error ? error.message : error);
      return null;
    }
  }
  /**
   * Get action type name
   */
  getActionTypeName(actionType) {
    const names = [
      "Tip",
      "Vouch",
      "Follow",
      "Challenge",
      "Stake",
      "ContentReact",
      "Delegate",
      "Vote"
    ];
    return names[actionType] || "Unknown";
  }
  /**
   * Check if action type is enabled
   */
  async isActionTypeEnabled(actionType) {
    const config = await this.getConfig();
    if (!config) return false;
    return (config.enabledActions & 1 << actionType) !== 0;
  }
  /**
   * Check if action is valid for execution
   * @param creator - The action creator's public key
   * @param nonce - M-04: The action nonce (NOT timestamp)
   */
  async isActionValid(creator, nonce) {
    const action = await this.getAction(creator, nonce);
    if (!action) {
      return { valid: false, reason: "Action not found" };
    }
    const now = getCurrentTimestamp();
    if (now > action.expiresAt.toNumber()) {
      return { valid: false, reason: "Action has expired" };
    }
    if (action.executed && action.maxExecutions === 1) {
      return { valid: false, reason: "Action already executed" };
    }
    if (action.maxExecutions > 0 && action.executionCount >= action.maxExecutions) {
      return { valid: false, reason: "Max executions reached" };
    }
    return { valid: true };
  }
  /**
   * Calculate platform fee for tip
   */
  calculateFee(amount) {
    const fee = amount.muln(VILINK_CONSTANTS.platformFeeBps).divn(1e4);
    return {
      fee,
      net: amount.sub(fee)
    };
  }
  // ============ URI Utilities ============
  /**
   * Generate ViLink URI from action ID
   */
  generateUri(actionId, baseUrl = "viwo://action") {
    const idHex = Buffer.from(actionId).toString("hex");
    return `${baseUrl}/${idHex}`;
  }
  /**
   * Parse action ID from URI
   */
  parseUri(uri) {
    const match = uri.match(/viwo:\/\/action\/([a-f0-9]{64})/i);
    if (!match) return null;
    return new Uint8Array(Buffer.from(match[1], "hex"));
  }
  /**
   * Generate QR code data for action
   */
  generateQRData(actionId) {
    return this.generateUri(actionId, "https://viwoapp.com/action");
  }
  /**
   * Generate shareable link with metadata
   */
  generateShareableLink(actionId, metadata) {
    const baseUri = this.generateUri(actionId, "https://viwoapp.com/action");
    if (!metadata) return baseUri;
    const params = new URLSearchParams();
    if (metadata.title) params.set("t", metadata.title);
    if (metadata.amount) params.set("a", metadata.amount);
    return `${baseUri}?${params.toString()}`;
  }
  // ============ Transaction Building ============
  /**
   * Build create tip action transaction
   */
  async buildCreateTipAction(params) {
    if (!this.client.publicKey) {
      throw new Error("Wallet not connected");
    }
    const minAmount = parseVCoin(VILINK_CONSTANTS.minTipAmount.toString());
    const maxAmount = parseVCoin(VILINK_CONSTANTS.maxTipAmount.toString());
    if (params.amount.lt(minAmount)) {
      throw new Error(`Tip amount below minimum: ${VILINK_CONSTANTS.minTipAmount} VCoin`);
    }
    if (params.amount.gt(maxAmount)) {
      throw new Error(`Tip amount exceeds maximum: ${VILINK_CONSTANTS.maxTipAmount} VCoin`);
    }
    const tx = new import_web36.Transaction();
    return tx;
  }
  /**
   * Build create vouch action transaction
   */
  async buildCreateVouchAction(params) {
    if (!this.client.publicKey) {
      throw new Error("Wallet not connected");
    }
    const tx = new import_web36.Transaction();
    return tx;
  }
  /**
   * Build create follow action transaction
   */
  async buildCreateFollowAction(params) {
    if (!this.client.publicKey) {
      throw new Error("Wallet not connected");
    }
    const tx = new import_web36.Transaction();
    return tx;
  }
  /**
   * Build execute tip action transaction
   * @param creator - The action creator's public key
   * @param nonce - M-04: The action nonce (NOT timestamp)
   */
  async buildExecuteTipAction(creator, nonce) {
    if (!this.client.publicKey) {
      throw new Error("Wallet not connected");
    }
    const { valid, reason } = await this.isActionValid(creator, nonce);
    if (!valid) {
      throw new Error(reason);
    }
    const action = await this.getAction(creator, nonce);
    if (action?.creator.equals(this.client.publicKey)) {
      throw new Error("Cannot execute own action");
    }
    const tx = new import_web36.Transaction();
    return tx;
  }
  /**
   * Get the next nonce for creating an action (M-04)
   * Fetches from UserActionStats.actionNonce on-chain
   */
  async getNextNonce(user) {
    const stats = await this.getUserStats(user);
    if (!stats) {
      return new import_anchor5.BN(0);
    }
    return new import_anchor5.BN(stats.actionsCreated.toNumber());
  }
};

// src/gasless/index.ts
var import_web37 = require("@solana/web3.js");
var import_anchor6 = require("@coral-xyz/anchor");
var GaslessClient = class {
  constructor(client) {
    this.client = client;
  }
  /**
   * Get gasless configuration
   * 
   * Finding #8 Fix: Corrected byte offsets to match on-chain GaslessConfig struct.
   * Added missing fields: pendingAuthority, feeVault, sscreProgram, sscreDeductionBps,
   * maxSubsidizedPerUser, totalSolSpent, currentDay, daySpent, maxSlippageBps.
   */
  async getConfig() {
    try {
      const configPda = this.client.pdas.getGaslessConfig();
      const accountInfo = await this.client.connection.connection.getAccountInfo(configPda);
      if (!accountInfo) {
        return null;
      }
      const data = accountInfo.data;
      return {
        authority: new import_web37.PublicKey(data.slice(8, 40)),
        pendingAuthority: new import_web37.PublicKey(data.slice(40, 72)),
        feePayer: new import_web37.PublicKey(data.slice(72, 104)),
        vcoinMint: new import_web37.PublicKey(data.slice(104, 136)),
        feeVault: new import_web37.PublicKey(data.slice(136, 168)),
        sscreProgram: new import_web37.PublicKey(data.slice(168, 200)),
        dailySubsidyBudget: new import_anchor6.BN(data.slice(200, 208), "le"),
        solFeePerTx: new import_anchor6.BN(data.slice(208, 216), "le"),
        vcoinFeeMultiplier: new import_anchor6.BN(data.slice(216, 224), "le"),
        sscreDeductionBps: data.readUInt16LE(224),
        maxSubsidizedPerUser: data.readUInt32LE(226),
        totalSubsidizedTx: new import_anchor6.BN(data.slice(230, 238), "le"),
        totalSolSpent: new import_anchor6.BN(data.slice(238, 246), "le"),
        totalVcoinCollected: new import_anchor6.BN(data.slice(246, 254), "le"),
        paused: data[254] !== 0,
        currentDay: data.readUInt32LE(255),
        daySpent: new import_anchor6.BN(data.slice(259, 267), "le"),
        maxSlippageBps: data.readUInt16LE(267)
      };
    } catch (error) {
      console.warn("[ViWoSDK] gasless.getConfig failed:", error instanceof Error ? error.message : error);
      return null;
    }
  }
  /**
   * Get session key details
   */
  async getSessionKey(user, sessionPubkey) {
    try {
      const sessionPda = this.client.pdas.getSessionKey(user, sessionPubkey);
      const accountInfo = await this.client.connection.connection.getAccountInfo(sessionPda);
      if (!accountInfo) {
        return null;
      }
      const data = accountInfo.data;
      return {
        user: new import_web37.PublicKey(data.slice(8, 40)),
        sessionPubkey: new import_web37.PublicKey(data.slice(40, 72)),
        scope: data.readUInt16LE(72),
        createdAt: new import_anchor6.BN(data.slice(74, 82), "le"),
        expiresAt: new import_anchor6.BN(data.slice(82, 90), "le"),
        actionsUsed: data.readUInt32LE(90),
        maxActions: data.readUInt32LE(94),
        vcoinSpent: new import_anchor6.BN(data.slice(98, 106), "le"),
        maxSpend: new import_anchor6.BN(data.slice(106, 114), "le"),
        isRevoked: data[114] !== 0,
        feeMethod: data[123]
      };
    } catch (error) {
      console.warn("[ViWoSDK] gasless.getSessionKey failed:", error instanceof Error ? error.message : error);
      return null;
    }
  }
  /**
   * Get user gasless statistics
   */
  async getUserStats(user) {
    const target = user || this.client.publicKey;
    if (!target) {
      throw new Error("No user specified and wallet not connected");
    }
    try {
      const statsPda = this.client.pdas.getUserGaslessStats(target);
      const accountInfo = await this.client.connection.connection.getAccountInfo(statsPda);
      if (!accountInfo) {
        return null;
      }
      const data = accountInfo.data;
      return {
        user: new import_web37.PublicKey(data.slice(8, 40)),
        totalGaslessTx: new import_anchor6.BN(data.slice(40, 48), "le"),
        totalSubsidized: new import_anchor6.BN(data.slice(48, 56), "le"),
        totalVcoinFees: new import_anchor6.BN(data.slice(56, 64), "le"),
        sessionsCreated: data.readUInt32LE(72),
        activeSession: new import_web37.PublicKey(data.slice(76, 108))
      };
    } catch (error) {
      console.warn("[ViWoSDK] gasless.getUserStats failed:", error instanceof Error ? error.message : error);
      return null;
    }
  }
  /**
   * Check if session is valid
   */
  async isSessionValid(user, sessionPubkey) {
    const session = await this.getSessionKey(user, sessionPubkey);
    if (!session) {
      return { valid: false, reason: "Session not found" };
    }
    if (session.isRevoked) {
      return { valid: false, reason: "Session has been revoked" };
    }
    const now = getCurrentTimestamp();
    if (now > session.expiresAt.toNumber()) {
      return { valid: false, reason: "Session has expired" };
    }
    if (session.actionsUsed >= session.maxActions) {
      return { valid: false, reason: "Session action limit reached" };
    }
    return { valid: true };
  }
  /**
   * Check if action is in session scope
   */
  isActionInScope(session, actionScope) {
    return (session.scope & actionScope) !== 0;
  }
  /**
   * Get remaining session actions
   */
  getRemainingActions(session) {
    return session.maxActions - session.actionsUsed;
  }
  /**
   * Get remaining session spend
   */
  getRemainingSpend(session) {
    return session.maxSpend.sub(session.vcoinSpent);
  }
  /**
   * Get remaining session time
   */
  getRemainingTime(session) {
    const now = getCurrentTimestamp();
    return Math.max(0, session.expiresAt.toNumber() - now);
  }
  /**
   * Calculate VCoin fee equivalent
   */
  async calculateVCoinFee() {
    const config = await this.getConfig();
    if (!config) {
      return new import_anchor6.BN(GASLESS_CONSTANTS.defaultSolFee * GASLESS_CONSTANTS.vcoinFeeMultiplier);
    }
    return config.solFeePerTx.mul(config.vcoinFeeMultiplier);
  }
  /**
   * Check if user is eligible for subsidized transactions
   */
  async isEligibleForSubsidy(user) {
    const target = user || this.client.publicKey;
    if (!target) {
      throw new Error("No user specified and wallet not connected");
    }
    const [config, userStats] = await Promise.all([
      this.getConfig(),
      this.getUserStats(target)
    ]);
    if (!config) {
      return { eligible: false, remainingToday: 0, reason: "Config not found" };
    }
    const maxPerUser = GASLESS_CONSTANTS.maxSubsidizedPerUser;
    const usedToday = 0;
    const remaining = maxPerUser - usedToday;
    if (remaining <= 0) {
      return {
        eligible: false,
        remainingToday: 0,
        reason: "Daily limit reached"
      };
    }
    return { eligible: true, remainingToday: remaining };
  }
  /**
   * Get scope names from scope bitmap
   */
  getScopeNames(scope) {
    const names = [];
    const scopeMap = [
      { bit: ACTION_SCOPES.tip, name: "Tip" },
      { bit: ACTION_SCOPES.vouch, name: "Vouch" },
      { bit: ACTION_SCOPES.content, name: "Content" },
      { bit: ACTION_SCOPES.governance, name: "Governance" },
      { bit: ACTION_SCOPES.transfer, name: "Transfer" },
      { bit: ACTION_SCOPES.stake, name: "Stake" },
      { bit: ACTION_SCOPES.claim, name: "Claim" },
      { bit: ACTION_SCOPES.follow, name: "Follow" }
    ];
    for (const { bit, name } of scopeMap) {
      if (scope & bit) {
        names.push(name);
      }
    }
    return names;
  }
  /**
   * Create scope from action names
   */
  createScope(actions) {
    let scope = 0;
    const scopeMap = {
      tip: ACTION_SCOPES.tip,
      vouch: ACTION_SCOPES.vouch,
      content: ACTION_SCOPES.content,
      governance: ACTION_SCOPES.governance,
      transfer: ACTION_SCOPES.transfer,
      stake: ACTION_SCOPES.stake,
      claim: ACTION_SCOPES.claim,
      follow: ACTION_SCOPES.follow
    };
    for (const action of actions) {
      const bit = scopeMap[action.toLowerCase()];
      if (bit) {
        scope |= bit;
      }
    }
    return scope;
  }
  // ============ Transaction Building ============
  /**
   * Build create session key transaction
   */
  async buildCreateSessionTransaction(params) {
    if (!this.client.publicKey) {
      throw new Error("Wallet not connected");
    }
    if (!params.sessionPubkey) {
      throw new Error("Session public key required");
    }
    if (!params.scope || params.scope === 0) {
      throw new Error("At least one scope required");
    }
    const duration = params.durationSeconds || GASLESS_CONSTANTS.sessionDuration;
    const maxActions = params.maxActions || GASLESS_CONSTANTS.maxSessionActions;
    const maxSpend = params.maxSpend || new import_anchor6.BN(GASLESS_CONSTANTS.maxSessionSpend * 1e9);
    const feeMethod = params.feeMethod ?? 1;
    const tx = new import_web37.Transaction();
    return tx;
  }
  /**
   * Build revoke session key transaction
   */
  async buildRevokeSessionTransaction(sessionPubkey) {
    if (!this.client.publicKey) {
      throw new Error("Wallet not connected");
    }
    const session = await this.getSessionKey(this.client.publicKey, sessionPubkey);
    if (!session) {
      throw new Error("Session not found");
    }
    if (session.isRevoked) {
      throw new Error("Session already revoked");
    }
    const tx = new import_web37.Transaction();
    return tx;
  }
  /**
   * Build VCoin fee deduction transaction
   */
  async buildDeductFeeTransaction(amount) {
    if (!this.client.publicKey) {
      throw new Error("Wallet not connected");
    }
    const tx = new import_web37.Transaction();
    return tx;
  }
};

// src/identity/index.ts
var import_web38 = require("@solana/web3.js");
var import_anchor7 = require("@coral-xyz/anchor");
var IdentityClient = class {
  constructor(client) {
    this.client = client;
  }
  /**
   * Get user identity
   */
  async getIdentity(user) {
    const target = user || this.client.publicKey;
    if (!target) {
      throw new Error("No user specified and wallet not connected");
    }
    try {
      const identityPda = this.client.pdas.getUserIdentity(target);
      const accountInfo = await this.client.connection.connection.getAccountInfo(identityPda);
      if (!accountInfo) {
        return null;
      }
      const data = accountInfo.data;
      return {
        user: new import_web38.PublicKey(data.slice(8, 40)),
        didHash: new Uint8Array(data.slice(40, 72)),
        verificationLevel: data[72],
        createdAt: new import_anchor7.BN(data.slice(73, 81), "le"),
        updatedAt: new import_anchor7.BN(data.slice(81, 89), "le")
      };
    } catch {
      return null;
    }
  }
  /**
   * Check if user has identity
   */
  async hasIdentity(user) {
    const identity = await this.getIdentity(user);
    return identity !== null;
  }
  /**
   * Get verification level name
   */
  getVerificationLevelName(level) {
    const levels = ["None", "Basic", "Standard", "Enhanced", "Premium"];
    return levels[level] || "Unknown";
  }
  /**
   * Get verification level requirements
   */
  getVerificationRequirements(level) {
    const requirements = {
      0: [],
      1: ["Email verification", "Phone verification"],
      2: ["Basic requirements", "Social account linking"],
      3: ["Standard requirements", "ID verification"],
      4: ["Enhanced requirements", "Face verification", "Address verification"]
    };
    return requirements[level] || [];
  }
  /**
   * Get verification level benefits
   */
  getVerificationBenefits(level) {
    const benefits = {
      0: ["Basic platform access"],
      1: ["Higher withdrawal limits", "Basic rewards eligibility"],
      2: ["Full rewards eligibility", "Vouch capabilities"],
      3: ["Priority support", "Enhanced trust score"],
      4: ["VIP status", "Governance proposal creation", "Maximum limits"]
    };
    return benefits[level] || [];
  }
  // ============ Transaction Building ============
  /**
   * Build create identity transaction
   */
  async buildCreateIdentityTransaction(didHash) {
    if (!this.client.publicKey) {
      throw new Error("Wallet not connected");
    }
    const existing = await this.getIdentity();
    if (existing) {
      throw new Error("Identity already exists");
    }
    if (didHash.length !== 32) {
      throw new Error("DID hash must be 32 bytes");
    }
    const tx = new import_web38.Transaction();
    return tx;
  }
  /**
   * Build update DID hash transaction
   */
  async buildUpdateDidHashTransaction(newDidHash) {
    if (!this.client.publicKey) {
      throw new Error("Wallet not connected");
    }
    if (newDidHash.length !== 32) {
      throw new Error("DID hash must be 32 bytes");
    }
    const tx = new import_web38.Transaction();
    return tx;
  }
};

// src/fivea/index.ts
var import_web39 = require("@solana/web3.js");
var import_anchor8 = require("@coral-xyz/anchor");
var FiveAClient = class {
  constructor(client) {
    this.client = client;
  }
  /**
   * Get user's 5A score
   */
  async getScore(user) {
    const target = user || this.client.publicKey;
    if (!target) {
      throw new Error("No user specified and wallet not connected");
    }
    try {
      const scorePda = this.client.pdas.getUserScore(target);
      const accountInfo = await this.client.connection.connection.getAccountInfo(scorePda);
      if (!accountInfo) {
        return null;
      }
      const data = accountInfo.data;
      return {
        user: new import_web39.PublicKey(data.slice(8, 40)),
        authenticity: data.readUInt16LE(40),
        accuracy: data.readUInt16LE(42),
        agility: data.readUInt16LE(44),
        activity: data.readUInt16LE(46),
        approved: data.readUInt16LE(48),
        compositeScore: data.readUInt16LE(50),
        lastUpdated: new import_anchor8.BN(data.slice(52, 60), "le"),
        isPrivate: data[60] !== 0
      };
    } catch {
      return null;
    }
  }
  /**
   * Format score as percentage
   */
  formatScore(score) {
    return `${(score / 100).toFixed(2)}%`;
  }
  /**
   * Get score tier
   */
  getScoreTier(composite) {
    if (composite >= 8e3) return "Excellent";
    if (composite >= 6e3) return "Good";
    if (composite >= 4e3) return "Average";
    if (composite >= 2e3) return "Below Average";
    return "Low";
  }
  /**
   * Get reward multiplier for score
   */
  getRewardMultiplier(composite) {
    if (composite >= 8e3) return FIVE_A_CONSTANTS.scoreMultipliers["80-100"];
    if (composite >= 6e3) return FIVE_A_CONSTANTS.scoreMultipliers["60-80"];
    if (composite >= 4e3) return FIVE_A_CONSTANTS.scoreMultipliers["40-60"];
    if (composite >= 2e3) return FIVE_A_CONSTANTS.scoreMultipliers["20-40"];
    return FIVE_A_CONSTANTS.scoreMultipliers["0-20"];
  }
  /**
   * Get score breakdown
   */
  getScoreBreakdown(score) {
    const weights = FIVE_A_CONSTANTS.scoreWeights;
    return [
      {
        component: "A1 - Authenticity",
        description: "Are you a real person?",
        score: this.formatScore(score.authenticity),
        weight: weights.authenticity,
        contribution: this.formatScore(score.authenticity * weights.authenticity / 100)
      },
      {
        component: "A2 - Accuracy",
        description: "Is your content quality?",
        score: this.formatScore(score.accuracy),
        weight: weights.accuracy,
        contribution: this.formatScore(score.accuracy * weights.accuracy / 100)
      },
      {
        component: "A3 - Agility",
        description: "Are you fast?",
        score: this.formatScore(score.agility),
        weight: weights.agility,
        contribution: this.formatScore(score.agility * weights.agility / 100)
      },
      {
        component: "A4 - Activity",
        description: "Do you show up daily?",
        score: this.formatScore(score.activity),
        weight: weights.activity,
        contribution: this.formatScore(score.activity * weights.activity / 100)
      },
      {
        component: "A5 - Approved",
        description: "Does the community like you?",
        score: this.formatScore(score.approved),
        weight: weights.approved,
        contribution: this.formatScore(score.approved * weights.approved / 100)
      }
    ];
  }
  /**
   * Calculate max vouches for score
   */
  getMaxVouches(composite) {
    if (composite >= 9e3) return 20;
    if (composite >= 8e3) return 15;
    if (composite >= 7e3) return 10;
    if (composite >= 6e3) return 7;
    if (composite >= 5e3) return 5;
    if (composite >= 4e3) return 3;
    return 2;
  }
  /**
   * Check if user can vouch for another
   */
  async canVouchFor(target) {
    if (!this.client.publicKey) {
      return { canVouch: false, reason: "Wallet not connected" };
    }
    if (this.client.publicKey.equals(target)) {
      return { canVouch: false, reason: "Cannot vouch for yourself" };
    }
    const myScore = await this.getScore();
    if (!myScore) {
      return { canVouch: false, reason: "No 5A score found" };
    }
    if (myScore.compositeScore < 6e3) {
      return { canVouch: false, reason: "Score too low to vouch (min 60%)" };
    }
    return { canVouch: true };
  }
  /**
   * Get score improvement suggestions
   */
  getImprovementSuggestions(score) {
    const suggestions = [];
    if (score.authenticity < 6e3) {
      suggestions.push("Complete identity verification to improve Authenticity (A1)");
    }
    if (score.accuracy < 6e3) {
      suggestions.push("Create quality content to improve Accuracy (A2)");
    }
    if (score.agility < 6e3) {
      suggestions.push("Respond faster to improve Agility (A3)");
    }
    if (score.activity < 6e3) {
      suggestions.push("Engage daily with content to improve Activity (A4)");
    }
    if (score.approved < 6e3) {
      suggestions.push("Get vouched by high-score users to improve Approved (A5)");
    }
    return suggestions;
  }
  // ============ Transaction Building ============
  /**
   * Build vouch transaction
   */
  async buildVouchTransaction(target) {
    if (!this.client.publicKey) {
      throw new Error("Wallet not connected");
    }
    const { canVouch, reason } = await this.canVouchFor(target);
    if (!canVouch) {
      throw new Error(reason);
    }
    const tx = new import_web39.Transaction();
    return tx;
  }
};

// src/content/index.ts
var import_web310 = require("@solana/web3.js");
var import_anchor9 = require("@coral-xyz/anchor");
var ContentClient = class {
  constructor(client) {
    this.client = client;
  }
  /**
   * Get content record
   */
  async getContent(contentId) {
    try {
      const contentPda = this.client.pdas.getContentRecord(contentId);
      const accountInfo = await this.client.connection.connection.getAccountInfo(contentPda);
      if (!accountInfo) {
        return null;
      }
      const data = accountInfo.data;
      return {
        contentId: new Uint8Array(data.slice(8, 40)),
        creator: new import_web310.PublicKey(data.slice(40, 72)),
        contentHash: new Uint8Array(data.slice(72, 104)),
        state: data[104],
        createdAt: new import_anchor9.BN(data.slice(105, 113), "le"),
        editCount: data.readUInt16LE(113),
        tips: new import_anchor9.BN(data.slice(115, 123), "le"),
        engagementScore: new import_anchor9.BN(data.slice(123, 131), "le")
      };
    } catch {
      return null;
    }
  }
  /**
   * Get user's energy
   */
  async getEnergy(user) {
    const target = user || this.client.publicKey;
    if (!target) {
      throw new Error("No user specified and wallet not connected");
    }
    try {
      const energyPda = this.client.pdas.getUserEnergy(target);
      const accountInfo = await this.client.connection.connection.getAccountInfo(energyPda);
      if (!accountInfo) {
        return null;
      }
      const data = accountInfo.data;
      return {
        user: new import_web310.PublicKey(data.slice(8, 40)),
        currentEnergy: data.readUInt16LE(40),
        maxEnergy: data.readUInt16LE(42),
        lastRegenTime: new import_anchor9.BN(data.slice(44, 52), "le"),
        tier: data[52]
      };
    } catch {
      return null;
    }
  }
  /**
   * Get content state name
   */
  getStateName(state) {
    const states = ["Active", "Hidden", "Deleted", "Flagged"];
    return states[state] || "Unknown";
  }
  /**
   * Calculate regenerated energy
   */
  calculateRegenEnergy(energy) {
    const now = getCurrentTimestamp();
    const secondsSinceRegen = now - energy.lastRegenTime.toNumber();
    const hoursSinceRegen = secondsSinceRegen / 3600;
    const regenAmount = Math.floor(hoursSinceRegen * CONTENT_CONSTANTS.energyRegenRate);
    const newEnergy = Math.min(
      energy.maxEnergy,
      energy.currentEnergy + regenAmount
    );
    return newEnergy;
  }
  /**
   * Get time until next energy
   */
  getTimeUntilNextEnergy(energy) {
    if (energy.currentEnergy >= energy.maxEnergy) {
      return 0;
    }
    const now = getCurrentTimestamp();
    const secondsSinceRegen = now - energy.lastRegenTime.toNumber();
    const secondsPerEnergy = 3600 / CONTENT_CONSTANTS.energyRegenRate;
    const nextRegenIn = secondsPerEnergy - secondsSinceRegen % secondsPerEnergy;
    return Math.max(0, Math.ceil(nextRegenIn));
  }
  /**
   * Get time until full energy
   */
  getTimeUntilFull(energy) {
    const currentEnergy = this.calculateRegenEnergy(energy);
    if (currentEnergy >= energy.maxEnergy) {
      return 0;
    }
    const energyNeeded = energy.maxEnergy - currentEnergy;
    const secondsPerEnergy = 3600 / CONTENT_CONSTANTS.energyRegenRate;
    return Math.ceil(energyNeeded * secondsPerEnergy);
  }
  /**
   * Check if user can create content
   */
  async canCreateContent(user) {
    const energy = await this.getEnergy(user);
    if (!energy) {
      return { canCreate: false, reason: "Energy account not found" };
    }
    const currentEnergy = this.calculateRegenEnergy(energy);
    const energyNeeded = CONTENT_CONSTANTS.createCost;
    if (currentEnergy < energyNeeded) {
      return {
        canCreate: false,
        reason: `Insufficient energy (${currentEnergy}/${energyNeeded})`,
        energyNeeded,
        energyAvailable: currentEnergy
      };
    }
    return { canCreate: true, energyNeeded, energyAvailable: currentEnergy };
  }
  /**
   * Check if user can edit content
   */
  async canEditContent(contentId, user) {
    const target = user || this.client.publicKey;
    if (!target) {
      return { canEdit: false, reason: "Wallet not connected" };
    }
    const content = await this.getContent(contentId);
    if (!content) {
      return { canEdit: false, reason: "Content not found" };
    }
    if (!content.creator.equals(target)) {
      return { canEdit: false, reason: "Not content creator" };
    }
    if (content.state === 2) {
      return { canEdit: false, reason: "Content is deleted" };
    }
    const energy = await this.getEnergy(target);
    if (!energy) {
      return { canEdit: false, reason: "Energy account not found" };
    }
    const currentEnergy = this.calculateRegenEnergy(energy);
    if (currentEnergy < CONTENT_CONSTANTS.editCost) {
      return { canEdit: false, reason: "Insufficient energy" };
    }
    return { canEdit: true };
  }
  /**
   * Get content stats
   */
  async getContentStats(contentId) {
    const content = await this.getContent(contentId);
    if (!content) {
      throw new Error("Content not found");
    }
    const now = getCurrentTimestamp();
    const age = now - content.createdAt.toNumber();
    return {
      tips: formatVCoin(content.tips),
      engagementScore: content.engagementScore.toString(),
      editCount: content.editCount,
      state: this.getStateName(content.state),
      age
    };
  }
  // ============ Transaction Building ============
  /**
   * Build create content transaction
   */
  async buildCreateContentTransaction(contentHash) {
    if (!this.client.publicKey) {
      throw new Error("Wallet not connected");
    }
    const { canCreate, reason } = await this.canCreateContent();
    if (!canCreate) {
      throw new Error(reason);
    }
    if (contentHash.length !== 32) {
      throw new Error("Content hash must be 32 bytes");
    }
    const tx = new import_web310.Transaction();
    return tx;
  }
  /**
   * Build edit content transaction
   */
  async buildEditContentTransaction(contentId, newContentHash) {
    if (!this.client.publicKey) {
      throw new Error("Wallet not connected");
    }
    const { canEdit, reason } = await this.canEditContent(contentId);
    if (!canEdit) {
      throw new Error(reason);
    }
    const tx = new import_web310.Transaction();
    return tx;
  }
  /**
   * Build delete content transaction
   */
  async buildDeleteContentTransaction(contentId) {
    if (!this.client.publicKey) {
      throw new Error("Wallet not connected");
    }
    const content = await this.getContent(contentId);
    if (!content) {
      throw new Error("Content not found");
    }
    if (!content.creator.equals(this.client.publicKey)) {
      throw new Error("Not content creator");
    }
    const tx = new import_web310.Transaction();
    return tx;
  }
};

// src/client.ts
var import_web311 = require("@solana/web3.js");
var import_anchor10 = require("@coral-xyz/anchor");
var import_spl_token = require("@solana/spl-token");
var ViWoClient = class {
  constructor(config) {
    if (config.connection instanceof import_web311.Connection) {
      this.connection = new ViWoConnection({
        endpoint: config.connection.rpcEndpoint,
        commitment: "confirmed"
      });
    } else {
      this.connection = new ViWoConnection(config.connection);
    }
    this.wallet = config.wallet || null;
    this.programIds = {
      ...PROGRAM_IDS,
      ...config.programIds
    };
    this.pdas = new PDAs(this.programIds);
    this.staking = new StakingClient(this);
    this.governance = new GovernanceClient(this);
    this.rewards = new RewardsClient(this);
    this.vilink = new ViLinkClient(this);
    this.gasless = new GaslessClient(this);
    this.identity = new IdentityClient(this);
    this.fivea = new FiveAClient(this);
    this.content = new ContentClient(this);
  }
  /**
   * Get the wallet public key
   */
  get publicKey() {
    return this.wallet?.publicKey || null;
  }
  /**
   * Check if wallet is connected
   */
  get isConnected() {
    return this.wallet !== null && this.wallet.publicKey !== null;
  }
  /**
   * Set wallet adapter
   */
  setWallet(wallet) {
    this.wallet = wallet;
    this.staking = new StakingClient(this);
    this.governance = new GovernanceClient(this);
    this.rewards = new RewardsClient(this);
    this.vilink = new ViLinkClient(this);
    this.gasless = new GaslessClient(this);
    this.identity = new IdentityClient(this);
    this.fivea = new FiveAClient(this);
    this.content = new ContentClient(this);
  }
  /**
   * Get Anchor provider
   */
  getProvider() {
    if (!this.wallet || !this.wallet.publicKey) {
      return null;
    }
    return new import_anchor10.AnchorProvider(
      this.connection.connection,
      this.wallet,
      { commitment: this.connection.commitment }
    );
  }
  /**
   * Send and confirm transaction
   */
  async sendTransaction(tx) {
    if (!this.wallet) {
      throw new Error("Wallet not connected");
    }
    const { blockhash, lastValidBlockHeight } = await this.connection.connection.getLatestBlockhash();
    tx.recentBlockhash = blockhash;
    tx.feePayer = this.wallet.publicKey;
    const signedTx = await this.wallet.signTransaction(tx);
    const signature = await this.connection.connection.sendRawTransaction(
      signedTx.serialize()
    );
    await this.connection.connection.confirmTransaction({
      signature,
      blockhash,
      lastValidBlockHeight
    });
    return signature;
  }
  /**
   * Get VCoin balance
   * 
   * Finding #2 Fix: Now filters by VCoin mint address instead of summing all Token-2022 accounts.
   * Make sure to set programIds.vcoinMint in your ViWoClient config.
   */
  async getVCoinBalance(user) {
    const target = user || this.publicKey;
    if (!target) {
      throw new Error("No user specified and wallet not connected");
    }
    try {
      const tokenAccounts = await this.connection.connection.getTokenAccountsByOwner(
        target,
        { mint: this.programIds.vcoinMint, programId: import_spl_token.TOKEN_2022_PROGRAM_ID }
      );
      let balance = new import_anchor10.BN(0);
      for (const { account } of tokenAccounts.value) {
        const data = account.data;
        const amount = data.slice(64, 72);
        balance = balance.add(new import_anchor10.BN(amount, "le"));
      }
      return balance;
    } catch (error) {
      console.warn("[ViWoSDK] getVCoinBalance failed:", error instanceof Error ? error.message : error);
      return new import_anchor10.BN(0);
    }
  }
  /**
   * Get veVCoin balance
   */
  async getVeVCoinBalance(user) {
    const target = user || this.publicKey;
    if (!target) {
      throw new Error("No user specified and wallet not connected");
    }
    try {
      const stakeData = await this.staking.getUserStake(target);
      return stakeData?.vevcoinBalance || new import_anchor10.BN(0);
    } catch (error) {
      console.warn("[ViWoSDK] getVeVCoinBalance failed:", error instanceof Error ? error.message : error);
      return new import_anchor10.BN(0);
    }
  }
  /**
   * Check connection health
   */
  async healthCheck() {
    try {
      const [connected, slot] = await Promise.all([
        this.connection.isHealthy(),
        this.connection.getSlot()
      ]);
      const blockTime = await this.connection.getBlockTime();
      return { connected, slot, blockTime };
    } catch (error) {
      console.warn("[ViWoSDK] healthCheck failed:", error instanceof Error ? error.message : error);
      return { connected: false, slot: null, blockTime: null };
    }
  }
};
// Annotate the CommonJS export names for ESM import in node:
0 && (module.exports = {
  ACTION_SCOPES,
  ActionType,
  BN,
  CONTENT_CONSTANTS,
  ContentClient,
  ContentState,
  FIVE_A_CONSTANTS,
  FeeMethod,
  FiveAClient,
  GASLESS_CONSTANTS,
  GOVERNANCE_CONSTANTS,
  GaslessClient,
  GovernanceClient,
  IdentityClient,
  LEGACY_SLASH_DEPRECATED,
  LOCK_DURATIONS,
  MAX_EPOCH_BITMAP,
  MAX_URI_LENGTH,
  MERKLE_CONSTANTS,
  MERKLE_PROOF_MAX_SIZE,
  PDAs,
  PROGRAM_IDS,
  ProposalStatus,
  RewardsClient,
  SECURITY_CONSTANTS,
  SEEDS,
  SSCRE_CONSTANTS,
  STAKING_TIERS,
  SlashStatus,
  StakingClient,
  StakingTier,
  TransactionBuilder,
  VALID_URI_PREFIXES,
  VCOIN_DECIMALS,
  VCOIN_INITIAL_CIRCULATING,
  VCOIN_TOTAL_SUPPLY,
  VEVCOIN_DECIMALS,
  VILINK_CONSTANTS,
  VerificationLevel,
  ViLinkClient,
  ViWoClient,
  ViWoConnection,
  VoteChoice,
  dateToTimestamp,
  formatVCoin,
  getCurrentTimestamp,
  parseVCoin,
  timestampToDate
});
