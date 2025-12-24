import { 
  Connection, 
  PublicKey, 
  Commitment,
  TransactionInstruction,
  Transaction,
  VersionedTransaction,
} from "@solana/web3.js";
import { AnchorProvider, Program, Wallet, BN } from "@coral-xyz/anchor";
import { PROGRAM_IDS, SEEDS } from "../constants";

export interface ConnectionConfig {
  endpoint: string;
  commitment?: Commitment;
  wsEndpoint?: string;
}

export interface WalletAdapter {
  publicKey: PublicKey | null;
  signTransaction<T extends Transaction | VersionedTransaction>(tx: T): Promise<T>;
  signAllTransactions<T extends Transaction | VersionedTransaction>(txs: T[]): Promise<T[]>;
}

/**
 * Core connection manager for ViWoApp SDK
 */
export class ViWoConnection {
  public connection: Connection;
  public commitment: Commitment;
  
  constructor(config: ConnectionConfig) {
    this.commitment = config.commitment || "confirmed";
    this.connection = new Connection(
      config.endpoint,
      {
        commitment: this.commitment,
        wsEndpoint: config.wsEndpoint,
      }
    );
  }
  
  /**
   * Get current slot
   */
  async getSlot(): Promise<number> {
    return this.connection.getSlot(this.commitment);
  }
  
  /**
   * Get current block time
   */
  async getBlockTime(): Promise<number | null> {
    const slot = await this.getSlot();
    return this.connection.getBlockTime(slot);
  }
  
  /**
   * Check if connection is healthy
   */
  async isHealthy(): Promise<boolean> {
    try {
      await this.connection.getVersion();
      return true;
    } catch {
      return false;
    }
  }
}

/**
 * PDA utility functions
 */
export class PDAs {
  constructor(private programIds: typeof PROGRAM_IDS = PROGRAM_IDS) {}
  
  // VCoin PDAs
  getVCoinConfig(): PublicKey {
    const [pda] = PublicKey.findProgramAddressSync(
      [Buffer.from(SEEDS.vcoinConfig)],
      this.programIds.vcoinToken
    );
    return pda;
  }
  
  // Staking PDAs
  getStakingPool(): PublicKey {
    const [pda] = PublicKey.findProgramAddressSync(
      [Buffer.from(SEEDS.stakingPool)],
      this.programIds.stakingProtocol
    );
    return pda;
  }
  
  getUserStake(user: PublicKey): PublicKey {
    const [pda] = PublicKey.findProgramAddressSync(
      [Buffer.from(SEEDS.userStake), user.toBuffer()],
      this.programIds.stakingProtocol
    );
    return pda;
  }
  
  // Governance PDAs
  getGovernanceConfig(): PublicKey {
    const [pda] = PublicKey.findProgramAddressSync(
      [Buffer.from(SEEDS.governanceConfig)],
      this.programIds.governanceProtocol
    );
    return pda;
  }
  
  getProposal(proposalId: BN): PublicKey {
    const [pda] = PublicKey.findProgramAddressSync(
      [Buffer.from(SEEDS.proposal), proposalId.toArrayLike(Buffer, "le", 8)],
      this.programIds.governanceProtocol
    );
    return pda;
  }
  
  getVoteRecord(user: PublicKey, proposal: PublicKey): PublicKey {
    const [pda] = PublicKey.findProgramAddressSync(
      [Buffer.from(SEEDS.voteRecord), user.toBuffer(), proposal.toBuffer()],
      this.programIds.governanceProtocol
    );
    return pda;
  }
  
  // SSCRE PDAs
  getRewardsPoolConfig(): PublicKey {
    const [pda] = PublicKey.findProgramAddressSync(
      [Buffer.from(SEEDS.poolConfig)],
      this.programIds.sscreProtocol
    );
    return pda;
  }
  
  getEpochDistribution(epoch: BN): PublicKey {
    const [pda] = PublicKey.findProgramAddressSync(
      [Buffer.from(SEEDS.epoch), epoch.toArrayLike(Buffer, "le", 8)],
      this.programIds.sscreProtocol
    );
    return pda;
  }
  
  getUserClaim(user: PublicKey): PublicKey {
    const [pda] = PublicKey.findProgramAddressSync(
      [Buffer.from(SEEDS.userClaim), user.toBuffer()],
      this.programIds.sscreProtocol
    );
    return pda;
  }
  
  // ViLink PDAs
  getViLinkConfig(): PublicKey {
    const [pda] = PublicKey.findProgramAddressSync(
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
  getViLinkAction(creator: PublicKey, nonce: BN): PublicKey {
    return this.getViLinkActionByNonce(creator, nonce);
  }

  /**
   * Get ViLink action PDA using nonce (M-04 fix)
   * @param creator - The action creator's public key
   * @param nonce - The action nonce from UserActionStats.actionNonce
   */
  getViLinkActionByNonce(creator: PublicKey, nonce: BN): PublicKey {
    const [pda] = PublicKey.findProgramAddressSync(
      [
        Buffer.from(SEEDS.action),
        creator.toBuffer(),
        nonce.toArrayLike(Buffer, "le", 8),
      ],
      this.programIds.vilinkProtocol
    );
    return pda;
  }
  
  getUserActionStats(user: PublicKey): PublicKey {
    const [pda] = PublicKey.findProgramAddressSync(
      [Buffer.from(SEEDS.userStats), user.toBuffer()],
      this.programIds.vilinkProtocol
    );
    return pda;
  }
  
  // Gasless PDAs
  getGaslessConfig(): PublicKey {
    const [pda] = PublicKey.findProgramAddressSync(
      [Buffer.from(SEEDS.gaslessConfig)],
      this.programIds.gaslessProtocol
    );
    return pda;
  }
  
  getSessionKey(user: PublicKey, sessionPubkey: PublicKey): PublicKey {
    const [pda] = PublicKey.findProgramAddressSync(
      [
        Buffer.from(SEEDS.sessionKey),
        user.toBuffer(),
        sessionPubkey.toBuffer(),
      ],
      this.programIds.gaslessProtocol
    );
    return pda;
  }
  
  getUserGaslessStats(user: PublicKey): PublicKey {
    const [pda] = PublicKey.findProgramAddressSync(
      [Buffer.from(SEEDS.userGasless), user.toBuffer()],
      this.programIds.gaslessProtocol
    );
    return pda;
  }
  
  // Identity PDAs
  getIdentityConfig(): PublicKey {
    const [pda] = PublicKey.findProgramAddressSync(
      [Buffer.from(SEEDS.identityConfig)],
      this.programIds.identityProtocol
    );
    return pda;
  }
  
  getUserIdentity(user: PublicKey): PublicKey {
    const [pda] = PublicKey.findProgramAddressSync(
      [Buffer.from(SEEDS.identity), user.toBuffer()],
      this.programIds.identityProtocol
    );
    return pda;
  }
  
  // 5A Protocol PDAs
  getFiveAConfig(): PublicKey {
    const [pda] = PublicKey.findProgramAddressSync(
      [Buffer.from(SEEDS.fiveAConfig)],
      this.programIds.fiveAProtocol
    );
    return pda;
  }
  
  getUserScore(user: PublicKey): PublicKey {
    const [pda] = PublicKey.findProgramAddressSync(
      [Buffer.from(SEEDS.userScore), user.toBuffer()],
      this.programIds.fiveAProtocol
    );
    return pda;
  }
  
  // Content PDAs
  getContentRegistryConfig(): PublicKey {
    const [pda] = PublicKey.findProgramAddressSync(
      [Buffer.from(SEEDS.registryConfig)],
      this.programIds.contentRegistry
    );
    return pda;
  }
  
  getContentRecord(contentId: Uint8Array): PublicKey {
    const [pda] = PublicKey.findProgramAddressSync(
      [Buffer.from(SEEDS.content), Buffer.from(contentId)],
      this.programIds.contentRegistry
    );
    return pda;
  }
  
  getUserEnergy(user: PublicKey): PublicKey {
    const [pda] = PublicKey.findProgramAddressSync(
      [Buffer.from(SEEDS.userEnergy), user.toBuffer()],
      this.programIds.contentRegistry
    );
    return pda;
  }
}

/**
 * Transaction builder utilities
 */
export class TransactionBuilder {
  private instructions: TransactionInstruction[] = [];
  
  add(instruction: TransactionInstruction): this {
    this.instructions.push(instruction);
    return this;
  }
  
  addMany(instructions: TransactionInstruction[]): this {
    this.instructions.push(...instructions);
    return this;
  }
  
  build(): Transaction {
    const tx = new Transaction();
    for (const ix of this.instructions) {
      tx.add(ix);
    }
    return tx;
  }
  
  clear(): this {
    this.instructions = [];
    return this;
  }
  
  get length(): number {
    return this.instructions.length;
  }
}

/**
 * Format utilities
 */
export function formatVCoin(amount: BN | number, decimals: number = 9): string {
  const amountBN = typeof amount === "number" ? new BN(amount) : amount;
  const divisor = new BN(10).pow(new BN(decimals));
  const whole = amountBN.div(divisor).toString();
  const fraction = amountBN.mod(divisor).toString().padStart(decimals, "0");
  return `${whole}.${fraction}`;
}

export function parseVCoin(amount: string | number, decimals: number = 9): BN {
  if (typeof amount === "number") {
    amount = amount.toString();
  }
  
  const [whole, fraction = ""] = amount.split(".");
  const paddedFraction = fraction.padEnd(decimals, "0").slice(0, decimals);
  return new BN(whole + paddedFraction);
}

/**
 * Time utilities
 */
export function getCurrentTimestamp(): number {
  return Math.floor(Date.now() / 1000);
}

export function timestampToDate(timestamp: number | BN): Date {
  const ts = typeof timestamp === "number" ? timestamp : timestamp.toNumber();
  return new Date(ts * 1000);
}

export function dateToTimestamp(date: Date): number {
  return Math.floor(date.getTime() / 1000);
}

export { BN };

