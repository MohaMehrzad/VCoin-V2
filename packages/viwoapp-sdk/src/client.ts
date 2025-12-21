import { Connection, PublicKey, Keypair, Transaction } from "@solana/web3.js";
import { AnchorProvider, Program, Wallet, BN } from "@coral-xyz/anchor";
import { TOKEN_2022_PROGRAM_ID } from "@solana/spl-token";

import { PROGRAM_IDS } from "./constants";
import { ViWoConnection, PDAs, WalletAdapter, ConnectionConfig } from "./core";
import { StakingClient } from "./staking";
import { GovernanceClient } from "./governance";
import { RewardsClient } from "./rewards";
import { ViLinkClient } from "./vilink";
import { GaslessClient } from "./gasless";
import { IdentityClient } from "./identity";
import { FiveAClient } from "./fivea";
import { ContentClient } from "./content";

export interface ViWoClientConfig {
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
export class ViWoClient {
  public connection: ViWoConnection;
  public pdas: PDAs;
  public wallet: WalletAdapter | null;
  public programIds: typeof PROGRAM_IDS;
  
  // Protocol clients
  public staking: StakingClient;
  public governance: GovernanceClient;
  public rewards: RewardsClient;
  public vilink: ViLinkClient;
  public gasless: GaslessClient;
  public identity: IdentityClient;
  public fivea: FiveAClient;
  public content: ContentClient;
  
  constructor(config: ViWoClientConfig) {
    // Setup connection
    if (config.connection instanceof Connection) {
      this.connection = new ViWoConnection({
        endpoint: config.connection.rpcEndpoint,
        commitment: "confirmed",
      });
    } else {
      this.connection = new ViWoConnection(config.connection);
    }
    
    // Setup wallet
    this.wallet = config.wallet || null;
    
    // Setup program IDs
    this.programIds = {
      ...PROGRAM_IDS,
      ...config.programIds,
    };
    
    // Setup PDAs
    this.pdas = new PDAs(this.programIds);
    
    // Initialize protocol clients
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
  get publicKey(): PublicKey | null {
    return this.wallet?.publicKey || null;
  }
  
  /**
   * Check if wallet is connected
   */
  get isConnected(): boolean {
    return this.wallet !== null && this.wallet.publicKey !== null;
  }
  
  /**
   * Set wallet adapter
   */
  setWallet(wallet: WalletAdapter): void {
    this.wallet = wallet;
    
    // Re-initialize clients with new wallet
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
  getProvider(): AnchorProvider | null {
    if (!this.wallet || !this.wallet.publicKey) {
      return null;
    }
    
    return new AnchorProvider(
      this.connection.connection,
      this.wallet as Wallet,
      { commitment: this.connection.commitment }
    );
  }
  
  /**
   * Send and confirm transaction
   */
  async sendTransaction(tx: Transaction): Promise<string> {
    if (!this.wallet) {
      throw new Error("Wallet not connected");
    }
    
    const { blockhash, lastValidBlockHeight } = 
      await this.connection.connection.getLatestBlockhash();
    
    tx.recentBlockhash = blockhash;
    tx.feePayer = this.wallet.publicKey!;
    
    const signedTx = await this.wallet.signTransaction(tx);
    
    const signature = await this.connection.connection.sendRawTransaction(
      signedTx.serialize()
    );
    
    await this.connection.connection.confirmTransaction({
      signature,
      blockhash,
      lastValidBlockHeight,
    });
    
    return signature;
  }
  
  /**
   * Get VCoin balance
   */
  async getVCoinBalance(user?: PublicKey): Promise<BN> {
    const target = user || this.publicKey;
    if (!target) {
      throw new Error("No user specified and wallet not connected");
    }
    
    try {
      const tokenAccounts = await this.connection.connection.getTokenAccountsByOwner(
        target,
        { programId: TOKEN_2022_PROGRAM_ID }
      );
      
      // Find VCoin token account
      // In production, filter by mint address
      let balance = new BN(0);
      for (const { account } of tokenAccounts.value) {
        const data = account.data;
        // Parse balance from account data
        const amount = data.slice(64, 72);
        balance = balance.add(new BN(amount, "le"));
      }
      
      return balance;
    } catch {
      return new BN(0);
    }
  }
  
  /**
   * Get veVCoin balance
   */
  async getVeVCoinBalance(user?: PublicKey): Promise<BN> {
    const target = user || this.publicKey;
    if (!target) {
      throw new Error("No user specified and wallet not connected");
    }
    
    try {
      const stakeData = await this.staking.getUserStake(target);
      return stakeData?.vevcoinBalance || new BN(0);
    } catch {
      return new BN(0);
    }
  }
  
  /**
   * Check connection health
   */
  async healthCheck(): Promise<{
    connected: boolean;
    slot: number | null;
    blockTime: number | null;
  }> {
    try {
      const [connected, slot] = await Promise.all([
        this.connection.isHealthy(),
        this.connection.getSlot(),
      ]);
      
      const blockTime = await this.connection.getBlockTime();
      
      return { connected, slot, blockTime };
    } catch {
      return { connected: false, slot: null, blockTime: null };
    }
  }
}

export default ViWoClient;

