import { PublicKey, Connection, Commitment, Transaction, VersionedTransaction, TransactionInstruction } from '@solana/web3.js';
import { BN, AnchorProvider } from '@coral-xyz/anchor';
export { BN } from '@coral-xyz/anchor';

declare const PROGRAM_IDS: {
    vcoinToken: PublicKey;
    vevcoinToken: PublicKey;
    stakingProtocol: PublicKey;
    transferHook: PublicKey;
    identityProtocol: PublicKey;
    fiveAProtocol: PublicKey;
    contentRegistry: PublicKey;
    governanceProtocol: PublicKey;
    sscreProtocol: PublicKey;
    vilinkProtocol: PublicKey;
    gaslessProtocol: PublicKey;
    /**
     * VCoin Token Mint Address (Token-2022)
     *
     * NOTE: This is a placeholder. Override via ViWoClient config.programIds.vcoinMint
     * after deploying your VCoin mint on devnet/mainnet.
     *
     * Finding #2 Fix: SDK now filters token accounts by mint address to prevent
     * summing balances from other Token-2022 tokens.
     */
    vcoinMint: PublicKey;
};
declare const SEEDS: {
    vcoinConfig: string;
    vevcoinConfig: string;
    userVevcoin: string;
    stakingPool: string;
    userStake: string;
    governanceConfig: string;
    proposal: string;
    voteRecord: string;
    delegation: string;
    poolConfig: string;
    epoch: string;
    userClaim: string;
    vilinkConfig: string;
    action: string;
    userStats: string;
    dapp: string;
    gaslessConfig: string;
    sessionKey: string;
    userGasless: string;
    feeVault: string;
    identityConfig: string;
    identity: string;
    fiveAConfig: string;
    userScore: string;
    vouch: string;
    registryConfig: string;
    content: string;
    userEnergy: string;
    slashRequest: string;
    decryptionShare: string;
    pendingScore: string;
};
declare const VCOIN_DECIMALS = 9;
declare const VEVCOIN_DECIMALS = 9;
declare const VCOIN_TOTAL_SUPPLY = 1000000000;
declare const VCOIN_INITIAL_CIRCULATING = 100000000;
declare const STAKING_TIERS: {
    none: {
        minStake: number;
        feeDiscount: number;
        boost: number;
        minLock: number;
    };
    bronze: {
        minStake: number;
        feeDiscount: number;
        boost: number;
        minLock: number;
    };
    silver: {
        minStake: number;
        feeDiscount: number;
        boost: number;
        minLock: number;
    };
    gold: {
        minStake: number;
        feeDiscount: number;
        boost: number;
        minLock: number;
    };
    platinum: {
        minStake: number;
        feeDiscount: number;
        boost: number;
        minLock: number;
    };
};
declare const LOCK_DURATIONS: {
    none: number;
    oneMonth: number;
    threeMonths: number;
    sixMonths: number;
    oneYear: number;
};
declare const SSCRE_CONSTANTS: {
    primaryReserves: number;
    secondaryReserves: number;
    epochDuration: number;
    claimWindow: number;
    gaslessFeeBps: number;
    minClaimAmount: number;
    circuitBreakerCooldown: number;
};
declare const VILINK_CONSTANTS: {
    maxActionExpiry: number;
    minTipAmount: number;
    maxTipAmount: number;
    platformFeeBps: number;
    maxPlatformFeeBps: number;
    minPlatformFeeBps: number;
};
declare const ACTION_SCOPES: {
    tip: number;
    vouch: number;
    content: number;
    governance: number;
    transfer: number;
    stake: number;
    claim: number;
    follow: number;
    all: number;
};
declare const GASLESS_CONSTANTS: {
    sessionDuration: number;
    maxSessionActions: number;
    maxSessionSpend: number;
    defaultSolFee: number;
    vcoinFeeMultiplier: number;
    sscreDeductionBps: number;
    dailySubsidyBudget: number;
    maxSubsidizedPerUser: number;
    maxSlippageBps: number;
};
declare const FIVE_A_CONSTANTS: {
    maxScore: number;
    scoreWeights: {
        authenticity: number;
        accuracy: number;
        agility: number;
        activity: number;
        approved: number;
    };
    scoreMultipliers: {
        "0-20": number;
        "20-40": number;
        "40-60": number;
        "60-80": number;
        "80-100": number;
    };
    oracleConsensusRequired: number;
    pendingScoreExpiry: number;
    minScoreUpdateInterval: number;
};
declare const CONTENT_CONSTANTS: {
    maxEnergy: number;
    energyRegenRate: number;
    createCost: number;
    editCost: number;
    deleteCost: number;
};
declare const GOVERNANCE_CONSTANTS: {
    minProposalThreshold: number;
    votingDuration: number;
    executionDelay: number;
    vetoWindow: number;
    quorumBps: number;
    zkVotingEnabled: boolean;
};
declare const SECURITY_CONSTANTS: {
    authorityTransferTimelock: number;
    slashApprovalTimelock: number;
    slashExpiry: number;
    maxFeeSlippageBps: number;
    minScoreUpdateInterval: number;
    circuitBreakerCooldown: number;
    oracleConsensusRequired: number;
    pendingScoreExpiry: number;
    maxPlatformFeeBps: number;
    minPlatformFeeBps: number;
    merkleProofMaxSize: number;
    maxEpochBitmap: number;
    votingPowerVerifiedOnChain: boolean;
};
declare const VALID_URI_PREFIXES: readonly ["ipfs://", "https://", "ar://"];
declare const MAX_URI_LENGTH = 128;
declare const MERKLE_CONSTANTS: {
    leafDomainPrefix: string;
};
/** Maximum Merkle proof size (H-NEW-02) - prevents DoS attacks */
declare const MERKLE_PROOF_MAX_SIZE = 32;
/** Maximum supported epoch number with bitmap storage (H-NEW-04) */
declare const MAX_EPOCH_BITMAP = 1023;
/**
 * @deprecated The legacy slash_tokens function is disabled (C-NEW-02).
 * Use propose_slash -> approve_slash -> execute_slash flow instead.
 */
declare const LEGACY_SLASH_DEPRECATED = true;

interface ConnectionConfig {
    endpoint: string;
    commitment?: Commitment;
    wsEndpoint?: string;
}
interface WalletAdapter {
    publicKey: PublicKey | null;
    signTransaction<T extends Transaction | VersionedTransaction>(tx: T): Promise<T>;
    signAllTransactions<T extends Transaction | VersionedTransaction>(txs: T[]): Promise<T[]>;
}
/**
 * Core connection manager for ViWoApp SDK
 */
declare class ViWoConnection {
    connection: Connection;
    commitment: Commitment;
    constructor(config: ConnectionConfig);
    /**
     * Get current slot
     */
    getSlot(): Promise<number>;
    /**
     * Get current block time
     */
    getBlockTime(): Promise<number | null>;
    /**
     * Check if connection is healthy
     */
    isHealthy(): Promise<boolean>;
}
/**
 * PDA utility functions
 */
declare class PDAs {
    private programIds;
    constructor(programIds?: typeof PROGRAM_IDS);
    getVCoinConfig(): PublicKey;
    getStakingPool(): PublicKey;
    getUserStake(user: PublicKey): PublicKey;
    getGovernanceConfig(): PublicKey;
    getProposal(proposalId: BN): PublicKey;
    getVoteRecord(user: PublicKey, proposal: PublicKey): PublicKey;
    getRewardsPoolConfig(): PublicKey;
    getEpochDistribution(epoch: BN): PublicKey;
    getUserClaim(user: PublicKey): PublicKey;
    getViLinkConfig(): PublicKey;
    /**
     * Get ViLink action PDA
     * @param creator - The action creator's public key
     * @param nonce - M-04: The action nonce (deterministic counter, NOT timestamp)
     * @deprecated Use getViLinkActionByNonce for clarity
     */
    getViLinkAction(creator: PublicKey, nonce: BN): PublicKey;
    /**
     * Get ViLink action PDA using nonce (M-04 fix)
     * @param creator - The action creator's public key
     * @param nonce - The action nonce from UserActionStats.actionNonce
     */
    getViLinkActionByNonce(creator: PublicKey, nonce: BN): PublicKey;
    getUserActionStats(user: PublicKey): PublicKey;
    getGaslessConfig(): PublicKey;
    getSessionKey(user: PublicKey, sessionPubkey: PublicKey): PublicKey;
    getUserGaslessStats(user: PublicKey): PublicKey;
    getIdentityConfig(): PublicKey;
    getUserIdentity(user: PublicKey): PublicKey;
    getFiveAConfig(): PublicKey;
    getUserScore(user: PublicKey): PublicKey;
    getContentRegistryConfig(): PublicKey;
    getContentRecord(contentId: Uint8Array): PublicKey;
    getUserEnergy(user: PublicKey): PublicKey;
}
/**
 * Transaction builder utilities
 */
declare class TransactionBuilder {
    private instructions;
    add(instruction: TransactionInstruction): this;
    addMany(instructions: TransactionInstruction[]): this;
    build(): Transaction;
    clear(): this;
    get length(): number;
}
/**
 * Format utilities
 */
declare function formatVCoin(amount: BN | number, decimals?: number): string;
declare function parseVCoin(amount: string | number, decimals?: number): BN;
/**
 * Time utilities
 */
declare function getCurrentTimestamp(): number;
declare function timestampToDate(timestamp: number | BN): Date;
declare function dateToTimestamp(date: Date): number;

/** Two-step authority transfer fields (H-02) */
interface PendingAuthorityFields {
    pendingAuthority?: PublicKey;
    pendingAuthorityActivatedAt?: BN;
}
interface VCoinConfig extends PendingAuthorityFields {
    authority: PublicKey;
    mint: PublicKey;
    permanentDelegate: PublicKey;
    paused: boolean;
    totalMinted: BN;
    totalBurned: BN;
}
/** Governance-controlled slashing request (H-01) */
declare enum SlashStatus {
    Proposed = 0,
    Approved = 1,
    Executed = 2,
    Cancelled = 3
}
interface SlashRequest {
    target: PublicKey;
    requestId: BN;
    amount: BN;
    reason: Uint8Array;
    proposer: PublicKey;
    proposedAt: BN;
    approvedAt?: BN;
    executedAt?: BN;
    status: SlashStatus;
    governanceProposal?: PublicKey;
}
declare enum StakingTier {
    None = 0,
    Bronze = 1,
    Silver = 2,
    Gold = 3,
    Platinum = 4
}
interface StakingPool extends PendingAuthorityFields {
    authority: PublicKey;
    vcoinMint: PublicKey;
    vevcoinMint: PublicKey;
    totalStaked: BN;
    totalVevcoinMinted: BN;
    paused: boolean;
    reentrancyGuard?: boolean;
}
interface UserStake {
    user: PublicKey;
    stakedAmount: BN;
    vevcoinBalance: BN;
    tier: StakingTier;
    lockEndTime: BN;
    lastUpdateTime: BN;
}
interface StakeParams {
    amount: BN;
    lockDuration: number;
}
declare enum ProposalStatus {
    Active = 0,
    Passed = 1,
    Rejected = 2,
    Executed = 3,
    Cancelled = 4
}
/**
 * Vote choice for governance voting (v2.8.0 C-NEW-01)
 * Voting power params are now read from on-chain state, not passed as parameters
 */
declare enum VoteChoice {
    Against = 0,
    For = 1,
    Abstain = 2
}
interface Proposal {
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
interface VoteRecord {
    user: PublicKey;
    proposal: PublicKey;
    votePower: BN;
    support: boolean;
    votedAt: BN;
}
interface CreateProposalParams {
    title: string;
    description: string;
    category: number;
    durationDays: number;
}
interface RewardsPoolConfig extends PendingAuthorityFields {
    authority: PublicKey;
    vcoinMint: PublicKey;
    currentEpoch: BN;
    totalDistributed: BN;
    remainingReserves: BN;
    paused: boolean;
    circuitBreakerActive?: boolean;
    circuitBreakerTriggeredAt?: BN;
}
interface EpochDistribution {
    epoch: BN;
    merkleRoot: Uint8Array;
    totalAllocation: BN;
    totalClaimed: BN;
    claimsCount: BN;
    isFinalized: boolean;
}
interface UserClaim {
    user: PublicKey;
    lastClaimedEpoch: BN;
    totalClaimed: BN;
    claimsCount: number;
    claimedEpochsBitmap?: BN[];
    claimedEpochsBitmapExt?: BN[];
    highEpochsBitmap?: BN[];
}
interface ClaimRewardsParams {
    epoch: BN;
    amount: BN;
    merkleProof: Uint8Array[];
}
declare enum ActionType {
    Tip = 0,
    Vouch = 1,
    Follow = 2,
    Challenge = 3,
    Stake = 4,
    ContentReact = 5,
    Delegate = 6,
    Vote = 7
}
/**
 * ViLinkConfig - Updated with H-02 pending authority field
 */
interface ViLinkConfig extends PendingAuthorityFields {
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
interface ViLinkAction {
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
interface CreateActionParams {
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
interface UserActionStatsExtended {
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
declare enum FeeMethod {
    PlatformSubsidized = 0,
    VCoinDeduction = 1,
    SSCREDeduction = 2
}
/**
 * GaslessConfig - Finding #8 Fix
 *
 * Updated to include all fields from on-chain GaslessConfig struct.
 * Previous version was missing fields added after H-02 security fix.
 */
interface GaslessConfig extends PendingAuthorityFields {
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
interface SessionKey {
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
interface CreateSessionParams {
    sessionPubkey: PublicKey;
    scope: number;
    durationSeconds?: number;
    maxActions?: number;
    maxSpend?: BN;
    feeMethod?: FeeMethod;
}
interface UserGaslessStats {
    user: PublicKey;
    totalGaslessTx: BN;
    totalSubsidized: BN;
    totalVcoinFees: BN;
    sessionsCreated: number;
    activeSession: PublicKey;
}
declare enum VerificationLevel {
    None = 0,// Wallet connected only
    Basic = 1,// Email + phone verified
    KYC = 2,// Identity documents verified
    Full = 3,// KYC + biometric verification
    Enhanced = 4
}
interface Identity {
    user: PublicKey;
    didHash: Uint8Array;
    verificationLevel: VerificationLevel;
    createdAt: BN;
    updatedAt: BN;
}
interface FiveAScore {
    user: PublicKey;
    authenticity: number;
    accuracy: number;
    agility: number;
    activity: number;
    approved: number;
    compositeScore: number;
    lastUpdated: BN;
    isPrivate: boolean;
}
interface VouchRecord {
    voucher: PublicKey;
    vouchee: PublicKey;
    vouchedAt: BN;
    isPositive: boolean;
    outcome: number;
}
declare enum ContentState {
    Active = 0,
    Hidden = 1,
    Deleted = 2,
    Flagged = 3
}
interface ContentRecord {
    contentId: Uint8Array;
    creator: PublicKey;
    contentHash: Uint8Array;
    state: ContentState;
    createdAt: BN;
    editCount: number;
    tips: BN;
    engagementScore: BN;
}
interface UserEnergy {
    user: PublicKey;
    currentEnergy: number;
    maxEnergy: number;
    lastRegenTime: BN;
    tier: number;
}
interface RegistryConfig extends PendingAuthorityFields {
    authority: PublicKey;
    paused: boolean;
    totalContent: BN;
}
interface IdentityConfig extends PendingAuthorityFields {
    authority: PublicKey;
    paused: boolean;
    totalIdentities: BN;
}
interface FiveAConfig extends PendingAuthorityFields {
    authority: PublicKey;
    paused: boolean;
    oracleConsensusRequired: number;
}
interface GovernanceConfig extends PendingAuthorityFields {
    authority: PublicKey;
    vevcoinMint: PublicKey;
    paused: boolean;
    proposalCount: BN;
    zkVotingEnabled: boolean;
}
/** ZK voting decryption share storage (C-02) */
interface DecryptionShare {
    proposal: PublicKey;
    committeeIndex: number;
    committeeMember: PublicKey;
    share: Uint8Array;
    submittedAt: BN;
}
/** Private voting config with committee tracking (C-02) */
interface PrivateVotingConfig {
    proposal: PublicKey;
    encryptionPubkey: PublicKey;
    decryptionThreshold: number;
    decryptionCommittee: PublicKey[];
    sharesSubmitted: boolean[];
    revealCompleted: boolean;
    aggregatedFor: BN;
    aggregatedAgainst: BN;
}
/** Delegation with expiry (M-07) */
interface Delegation {
    delegator: PublicKey;
    delegate: PublicKey;
    delegationType: number;
    delegatedAmount: BN;
    expiresAt?: BN;
    revocable: boolean;
}
/** Pending score update for oracle consensus */
interface PendingScoreUpdate {
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
}
interface HookConfig extends PendingAuthorityFields {
    authority: PublicKey;
    vcoinMint: PublicKey;
    blockWashTrading: boolean;
    paused: boolean;
}

/**
 * Governance Client for ViWoApp governance operations
 *
 * @example
 * ```typescript
 * const govClient = client.governance;
 *
 * // Create a proposal
 * await govClient.createProposal({
 *   title: "Increase staking rewards",
 *   description: "Proposal to increase staking APY by 10%",
 *   category: 1,
 *   durationDays: 7,
 * });
 *
 * // Vote on a proposal
 * await govClient.vote(proposalId, true); // Vote in favor
 * ```
 */
declare class GovernanceClient {
    private client;
    constructor(client: ViWoClient);
    /**
     * Get governance configuration
     */
    getConfig(): Promise<any | null>;
    /**
     * Get proposal by ID
     */
    getProposal(proposalId: BN): Promise<Proposal | null>;
    /**
     * Get all active proposals
     */
    getActiveProposals(): Promise<Proposal[]>;
    /**
     * Get user's vote record for a proposal
     */
    getVoteRecord(proposalId: BN, user?: PublicKey): Promise<VoteRecord | null>;
    /**
     * Check if user has voted on a proposal
     */
    hasVoted(proposalId: BN, user?: PublicKey): Promise<boolean>;
    /**
     * Calculate user's voting power
     */
    getVotingPower(user?: PublicKey): Promise<BN>;
    /**
     * Get proposal status text
     */
    getStatusText(status: ProposalStatus): string;
    /**
     * Check if proposal can be executed
     */
    canExecute(proposalId: BN): Promise<{
        canExecute: boolean;
        reason?: string;
    }>;
    /**
     * Get proposal progress
     */
    getProposalProgress(proposalId: BN): Promise<{
        votesFor: string;
        votesAgainst: string;
        totalVotes: string;
        forPercentage: number;
        againstPercentage: number;
        quorumReached: boolean;
        timeRemaining: number;
    }>;
    /**
     * Build create proposal transaction
     */
    buildCreateProposalTransaction(params: CreateProposalParams): Promise<Transaction>;
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
    buildVoteTransaction(proposalId: BN, support: boolean): Promise<Transaction>;
    /**
     * Build execute proposal transaction
     */
    buildExecuteTransaction(proposalId: BN): Promise<Transaction>;
}

/**
 * Rewards Client for SSCRE rewards operations
 *
 * @example
 * ```typescript
 * const rewardsClient = client.rewards;
 *
 * // Check claimable rewards
 * const claimable = await rewardsClient.getClaimableRewards();
 * console.log("Claimable:", claimable);
 *
 * // Claim rewards with merkle proof
 * await rewardsClient.claim({
 *   epoch: currentEpoch,
 *   amount: claimableAmount,
 *   merkleProof: proof,
 * });
 * ```
 */
declare class RewardsClient {
    private client;
    constructor(client: ViWoClient);
    /**
     * Get rewards pool configuration
     */
    getPoolConfig(): Promise<RewardsPoolConfig | null>;
    /**
     * Get epoch distribution details
     */
    getEpochDistribution(epoch: BN): Promise<EpochDistribution | null>;
    /**
     * Get current epoch
     */
    getCurrentEpoch(): Promise<BN>;
    /**
     * Get user claim history
     */
    getUserClaim(user?: PublicKey): Promise<UserClaim | null>;
    /**
     * Check if user has claimed for an epoch
     */
    hasClaimedEpoch(epoch: BN, user?: PublicKey): Promise<boolean>;
    /**
     * Get unclaimed epochs
     */
    getUnclaimedEpochs(user?: PublicKey): Promise<BN[]>;
    /**
     * Get rewards statistics
     */
    getStats(): Promise<{
        currentEpoch: number;
        totalDistributed: string;
        remainingReserves: string;
        reservePercentage: number;
        userTotalClaimed: string | null;
        userClaimsCount: number | null;
    }>;
    /**
     * Calculate gasless fee for claim
     */
    calculateGaslessFee(amount: BN): BN;
    /**
     * Calculate net claim amount after fee
     */
    calculateNetClaim(amount: BN): BN;
    /**
     * Verify merkle proof locally
     */
    verifyMerkleProof(proof: Uint8Array[], root: Uint8Array, leaf: Uint8Array): boolean;
    /**
     * Compute leaf hash from user data
     */
    computeLeaf(user: PublicKey, amount: BN, epoch: BN): Uint8Array;
    private compareBytes;
    private hashBytes;
    /**
     * Build claim rewards transaction
     */
    buildClaimTransaction(params: ClaimRewardsParams): Promise<Transaction>;
}

/**
 * ViLink Client for cross-dApp action deep links
 *
 * @example
 * ```typescript
 * const vilinkClient = client.vilink;
 *
 * // Create a tip action link
 * const action = await vilinkClient.createTipAction({
 *   target: recipientPubkey,
 *   amount: parseVCoin("10"),
 * });
 *
 * // Generate shareable URI
 * const uri = vilinkClient.generateUri(action.actionId);
 * // => viwo://action/abc123...
 *
 * // Execute action from URI
 * await vilinkClient.executeAction(actionId);
 * ```
 */
declare class ViLinkClient {
    private client;
    constructor(client: ViWoClient);
    /**
     * Get ViLink configuration
     *
     * Finding #8 (related): Corrected byte offsets to match on-chain ViLinkConfig struct.
     * Added pending_authority field that was missing after H-02 security fix.
     */
    getConfig(): Promise<ViLinkConfig | null>;
    /**
     * Get action by nonce (M-04: deterministic PDA derivation)
     * @param creator - The action creator's public key
     * @param nonce - The action nonce (from UserActionStats.actionNonce at creation time)
     */
    getAction(creator: PublicKey, nonce: BN): Promise<ViLinkAction | null>;
    /**
     * @deprecated Use getAction with nonce parameter instead
     */
    getActionByTimestamp(creator: PublicKey, timestamp: BN): Promise<ViLinkAction | null>;
    /**
     * Get user action statistics
     */
    getUserStats(user?: PublicKey): Promise<any | null>;
    /**
     * Get action type name
     */
    getActionTypeName(actionType: ActionType): string;
    /**
     * Check if action type is enabled
     */
    isActionTypeEnabled(actionType: ActionType): Promise<boolean>;
    /**
     * Check if action is valid for execution
     * @param creator - The action creator's public key
     * @param nonce - M-04: The action nonce (NOT timestamp)
     */
    isActionValid(creator: PublicKey, nonce: BN): Promise<{
        valid: boolean;
        reason?: string;
    }>;
    /**
     * Calculate platform fee for tip
     */
    calculateFee(amount: BN): {
        fee: BN;
        net: BN;
    };
    /**
     * Generate ViLink URI from action ID
     */
    generateUri(actionId: Uint8Array, baseUrl?: string): string;
    /**
     * Parse action ID from URI
     */
    parseUri(uri: string): Uint8Array | null;
    /**
     * Generate QR code data for action
     */
    generateQRData(actionId: Uint8Array): string;
    /**
     * Generate shareable link with metadata
     */
    generateShareableLink(actionId: Uint8Array, metadata?: {
        title?: string;
        amount?: string;
    }): string;
    /**
     * Build create tip action transaction
     */
    buildCreateTipAction(params: {
        target: PublicKey;
        amount: BN;
        expirySeconds?: number;
        oneTime?: boolean;
        metadata?: string;
    }): Promise<Transaction>;
    /**
     * Build create vouch action transaction
     */
    buildCreateVouchAction(params: {
        target: PublicKey;
        expirySeconds?: number;
    }): Promise<Transaction>;
    /**
     * Build create follow action transaction
     */
    buildCreateFollowAction(params: {
        target: PublicKey;
        maxExecutions?: number;
        expirySeconds?: number;
    }): Promise<Transaction>;
    /**
     * Build execute tip action transaction
     * @param creator - The action creator's public key
     * @param nonce - M-04: The action nonce (NOT timestamp)
     */
    buildExecuteTipAction(creator: PublicKey, nonce: BN): Promise<Transaction>;
    /**
     * Get the next nonce for creating an action (M-04)
     * Fetches from UserActionStats.actionNonce on-chain
     */
    getNextNonce(user?: PublicKey): Promise<BN>;
}

/**
 * Gasless Client for session key management and gasless transactions
 *
 * @example
 * ```typescript
 * const gaslessClient = client.gasless;
 *
 * // Create a 24-hour session key
 * const sessionKeypair = Keypair.generate();
 * await gaslessClient.createSession({
 *   sessionPubkey: sessionKeypair.publicKey,
 *   scope: ACTION_SCOPES.tip | ACTION_SCOPES.vouch,
 *   feeMethod: FeeMethod.PlatformSubsidized,
 * });
 *
 * // Execute action using session
 * await gaslessClient.executeWithSession(sessionKeypair, tipAction);
 * ```
 */
declare class GaslessClient {
    private client;
    constructor(client: ViWoClient);
    /**
     * Get gasless configuration
     *
     * Finding #8 Fix: Corrected byte offsets to match on-chain GaslessConfig struct.
     * Added missing fields: pendingAuthority, feeVault, sscreProgram, sscreDeductionBps,
     * maxSubsidizedPerUser, totalSolSpent, currentDay, daySpent, maxSlippageBps.
     */
    getConfig(): Promise<GaslessConfig | null>;
    /**
     * Get session key details
     */
    getSessionKey(user: PublicKey, sessionPubkey: PublicKey): Promise<SessionKey | null>;
    /**
     * Get user gasless statistics
     */
    getUserStats(user?: PublicKey): Promise<UserGaslessStats | null>;
    /**
     * Check if session is valid
     */
    isSessionValid(user: PublicKey, sessionPubkey: PublicKey): Promise<{
        valid: boolean;
        reason?: string;
    }>;
    /**
     * Check if action is in session scope
     */
    isActionInScope(session: SessionKey, actionScope: number): boolean;
    /**
     * Get remaining session actions
     */
    getRemainingActions(session: SessionKey): number;
    /**
     * Get remaining session spend
     */
    getRemainingSpend(session: SessionKey): BN;
    /**
     * Get remaining session time
     */
    getRemainingTime(session: SessionKey): number;
    /**
     * Calculate VCoin fee equivalent
     */
    calculateVCoinFee(): Promise<BN>;
    /**
     * Check if user is eligible for subsidized transactions
     */
    isEligibleForSubsidy(user?: PublicKey): Promise<{
        eligible: boolean;
        remainingToday: number;
        reason?: string;
    }>;
    /**
     * Get scope names from scope bitmap
     */
    getScopeNames(scope: number): string[];
    /**
     * Create scope from action names
     */
    createScope(actions: string[]): number;
    /**
     * Build create session key transaction
     */
    buildCreateSessionTransaction(params: CreateSessionParams): Promise<Transaction>;
    /**
     * Build revoke session key transaction
     */
    buildRevokeSessionTransaction(sessionPubkey: PublicKey): Promise<Transaction>;
    /**
     * Build VCoin fee deduction transaction
     */
    buildDeductFeeTransaction(amount?: BN): Promise<Transaction>;
}

/**
 * Identity Client for ViWoApp identity operations
 *
 * @example
 * ```typescript
 * const identityClient = client.identity;
 *
 * // Get user identity
 * const identity = await identityClient.getIdentity(userPubkey);
 * console.log("Verification level:", identityClient.getVerificationLevelName(identity.verificationLevel));
 *
 * // Create identity
 * await identityClient.createIdentity(didHash);
 * ```
 */
declare class IdentityClient {
    private client;
    constructor(client: ViWoClient);
    /**
     * Get user identity
     */
    getIdentity(user?: PublicKey): Promise<Identity | null>;
    /**
     * Check if user has identity
     */
    hasIdentity(user?: PublicKey): Promise<boolean>;
    /**
     * Get verification level name
     */
    getVerificationLevelName(level: VerificationLevel): string;
    /**
     * Get verification level requirements
     */
    getVerificationRequirements(level: VerificationLevel): string[];
    /**
     * Get verification level benefits
     */
    getVerificationBenefits(level: VerificationLevel): string[];
    /**
     * Build create identity transaction
     */
    buildCreateIdentityTransaction(didHash: Uint8Array): Promise<Transaction>;
    /**
     * Build update DID hash transaction
     */
    buildUpdateDidHashTransaction(newDidHash: Uint8Array): Promise<Transaction>;
}

/**
 * 5A Protocol Client for reputation scoring
 *
 * @example
 * ```typescript
 * const fiveaClient = client.fivea;
 *
 * // Get user's 5A score
 * const score = await fiveaClient.getScore(userPubkey);
 * console.log("Composite score:", score.composite);
 *
 * // Vouch for another user
 * await fiveaClient.vouch(targetPubkey);
 * ```
 */
declare class FiveAClient {
    private client;
    constructor(client: ViWoClient);
    /**
     * Get user's 5A score
     */
    getScore(user?: PublicKey): Promise<FiveAScore | null>;
    /**
     * Format score as percentage
     */
    formatScore(score: number): string;
    /**
     * Get score tier
     */
    getScoreTier(composite: number): string;
    /**
     * Get reward multiplier for score
     */
    getRewardMultiplier(composite: number): number;
    /**
     * Get score breakdown
     */
    getScoreBreakdown(score: FiveAScore): {
        component: string;
        description: string;
        score: string;
        weight: number;
        contribution: string;
    }[];
    /**
     * Calculate max vouches for score
     */
    getMaxVouches(composite: number): number;
    /**
     * Check if user can vouch for another
     */
    canVouchFor(target: PublicKey): Promise<{
        canVouch: boolean;
        reason?: string;
    }>;
    /**
     * Get score improvement suggestions
     */
    getImprovementSuggestions(score: FiveAScore): string[];
    /**
     * Build vouch transaction
     */
    buildVouchTransaction(target: PublicKey): Promise<Transaction>;
}

/**
 * Content Client for content registry operations
 *
 * @example
 * ```typescript
 * const contentClient = client.content;
 *
 * // Get user's energy
 * const energy = await contentClient.getEnergy();
 * console.log("Current energy:", energy.currentEnergy);
 *
 * // Create content
 * await contentClient.createContent(contentHash);
 * ```
 */
declare class ContentClient {
    private client;
    constructor(client: ViWoClient);
    /**
     * Get content record
     */
    getContent(contentId: Uint8Array): Promise<ContentRecord | null>;
    /**
     * Get user's energy
     */
    getEnergy(user?: PublicKey): Promise<UserEnergy | null>;
    /**
     * Get content state name
     */
    getStateName(state: ContentState): string;
    /**
     * Calculate regenerated energy
     */
    calculateRegenEnergy(energy: UserEnergy): number;
    /**
     * Get time until next energy
     */
    getTimeUntilNextEnergy(energy: UserEnergy): number;
    /**
     * Get time until full energy
     */
    getTimeUntilFull(energy: UserEnergy): number;
    /**
     * Check if user can create content
     */
    canCreateContent(user?: PublicKey): Promise<{
        canCreate: boolean;
        reason?: string;
        energyNeeded?: number;
        energyAvailable?: number;
    }>;
    /**
     * Check if user can edit content
     */
    canEditContent(contentId: Uint8Array, user?: PublicKey): Promise<{
        canEdit: boolean;
        reason?: string;
    }>;
    /**
     * Get content stats
     */
    getContentStats(contentId: Uint8Array): Promise<{
        tips: string;
        engagementScore: string;
        editCount: number;
        state: string;
        age: number;
    }>;
    /**
     * Build create content transaction
     */
    buildCreateContentTransaction(contentHash: Uint8Array): Promise<Transaction>;
    /**
     * Build edit content transaction
     */
    buildEditContentTransaction(contentId: Uint8Array, newContentHash: Uint8Array): Promise<Transaction>;
    /**
     * Build delete content transaction
     */
    buildDeleteContentTransaction(contentId: Uint8Array): Promise<Transaction>;
}

interface ViWoClientConfig {
    connection: ConnectionConfig | Connection;
    wallet?: WalletAdapter;
    programIds?: Partial<typeof PROGRAM_IDS>;
}
/**
 * Main ViWoApp SDK Client
 *
 * Provides unified access to all ViWoApp protocols.
 *
 * @example
 * ```typescript
 * import { ViWoClient } from "@viwoapp/sdk";
 *
 * const client = new ViWoClient({
 *   connection: { endpoint: "https://api.devnet.solana.com" },
 *   wallet: walletAdapter,
 * });
 *
 * // Stake VCoin
 * await client.staking.stake({ amount: new BN(1000), lockDuration: 30 * 24 * 3600 });
 *
 * // Create ViLink tip action
 * await client.vilink.createTipAction({
 *   target: recipientPubkey,
 *   amount: new BN(10),
 * });
 * ```
 */
declare class ViWoClient {
    connection: ViWoConnection;
    pdas: PDAs;
    wallet: WalletAdapter | null;
    programIds: typeof PROGRAM_IDS;
    staking: StakingClient;
    governance: GovernanceClient;
    rewards: RewardsClient;
    vilink: ViLinkClient;
    gasless: GaslessClient;
    identity: IdentityClient;
    fivea: FiveAClient;
    content: ContentClient;
    constructor(config: ViWoClientConfig);
    /**
     * Get the wallet public key
     */
    get publicKey(): PublicKey | null;
    /**
     * Check if wallet is connected
     */
    get isConnected(): boolean;
    /**
     * Set wallet adapter
     */
    setWallet(wallet: WalletAdapter): void;
    /**
     * Get Anchor provider
     */
    getProvider(): AnchorProvider | null;
    /**
     * Send and confirm transaction
     */
    sendTransaction(tx: Transaction): Promise<string>;
    /**
     * Get VCoin balance
     *
     * Finding #2 Fix: Now filters by VCoin mint address instead of summing all Token-2022 accounts.
     * Make sure to set programIds.vcoinMint in your ViWoClient config.
     */
    getVCoinBalance(user?: PublicKey): Promise<BN>;
    /**
     * Get veVCoin balance
     */
    getVeVCoinBalance(user?: PublicKey): Promise<BN>;
    /**
     * Check connection health
     */
    healthCheck(): Promise<{
        connected: boolean;
        slot: number | null;
        blockTime: number | null;
    }>;
}

/**
 * Staking Client for VCoin staking operations
 *
 * @example
 * ```typescript
 * const stakingClient = client.staking;
 *
 * // Stake 1000 VCoin for 90 days
 * await stakingClient.stake({
 *   amount: parseVCoin("1000"),
 *   lockDuration: LOCK_DURATIONS.threeMonths,
 * });
 *
 * // Get stake info
 * const stakeInfo = await stakingClient.getUserStake(walletPubkey);
 * console.log("Staked:", formatVCoin(stakeInfo.stakedAmount));
 * ```
 */
declare class StakingClient {
    private client;
    constructor(client: ViWoClient);
    /**
     * Get staking pool configuration
     */
    getPool(): Promise<StakingPool | null>;
    /**
     * Get user stake information
     */
    getUserStake(user?: PublicKey): Promise<UserStake | null>;
    /**
     * Calculate tier based on stake amount
     */
    calculateTier(stakeAmount: BN | number): StakingTier;
    /**
     * Calculate veVCoin amount for given stake
     * Formula: ve_vcoin = staked_amount × (lock_duration / 4_years) × tier_boost
     */
    calculateVeVCoin(amount: BN, lockDuration: number): BN;
    /**
     * Get tier name
     */
    getTierName(tier: StakingTier): string;
    /**
     * Get tier info
     */
    getTierInfo(tier: StakingTier): typeof STAKING_TIERS.none;
    /**
     * Check if user can unstake
     */
    canUnstake(user?: PublicKey): Promise<{
        canUnstake: boolean;
        reason?: string;
    }>;
    /**
     * Get staking statistics
     */
    getStats(): Promise<{
        totalStaked: string;
        totalVevcoin: string;
        userStake: string | null;
        userVevcoin: string | null;
        userTier: string | null;
    }>;
    /**
     * Build stake instruction
     * @param params Stake parameters
     * @returns Transaction to sign and send
     */
    buildStakeTransaction(params: StakeParams): Promise<Transaction>;
    /**
     * Build unstake instruction
     * @returns Transaction to sign and send
     */
    buildUnstakeTransaction(): Promise<Transaction>;
    /**
     * Build extend lock instruction
     * @param newDuration New lock duration in seconds
     * @returns Transaction to sign and send
     */
    buildExtendLockTransaction(newDuration: number): Promise<Transaction>;
}

export { ACTION_SCOPES, ActionType, CONTENT_CONSTANTS, type ClaimRewardsParams, type ConnectionConfig, ContentClient, type ContentRecord, ContentState, type CreateActionParams, type CreateProposalParams, type CreateSessionParams, type DecryptionShare, type Delegation, type EpochDistribution, FIVE_A_CONSTANTS, FeeMethod, FiveAClient, type FiveAConfig, type FiveAScore, GASLESS_CONSTANTS, GOVERNANCE_CONSTANTS, GaslessClient, type GaslessConfig, GovernanceClient, type GovernanceConfig, type HookConfig, type Identity, IdentityClient, type IdentityConfig, LEGACY_SLASH_DEPRECATED, LOCK_DURATIONS, MAX_EPOCH_BITMAP, MAX_URI_LENGTH, MERKLE_CONSTANTS, MERKLE_PROOF_MAX_SIZE, PDAs, PROGRAM_IDS, type PendingAuthorityFields, type PendingScoreUpdate, type PrivateVotingConfig, type Proposal, ProposalStatus, type RegistryConfig, RewardsClient, type RewardsPoolConfig, SECURITY_CONSTANTS, SEEDS, SSCRE_CONSTANTS, STAKING_TIERS, type SessionKey, type SlashRequest, SlashStatus, type StakeParams, StakingClient, type StakingPool, StakingTier, TransactionBuilder, type UserActionStatsExtended, type UserClaim, type UserEnergy, type UserGaslessStats, type UserStake, VALID_URI_PREFIXES, VCOIN_DECIMALS, VCOIN_INITIAL_CIRCULATING, VCOIN_TOTAL_SUPPLY, type VCoinConfig, VEVCOIN_DECIMALS, VILINK_CONSTANTS, VerificationLevel, type ViLinkAction, ViLinkClient, type ViLinkConfig, ViWoClient, ViWoConnection, VoteChoice, type VoteRecord, type VouchRecord, type WalletAdapter, dateToTimestamp, formatVCoin, getCurrentTimestamp, parseVCoin, timestampToDate };
