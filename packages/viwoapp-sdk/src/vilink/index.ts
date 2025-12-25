import { PublicKey, Transaction, Keypair } from "@solana/web3.js";
import { BN } from "@coral-xyz/anchor";

import type { ViWoClient } from "../client";
import type { ViLinkConfig, ViLinkAction, CreateActionParams, ActionType } from "../types";
import { VILINK_CONSTANTS, ACTION_SCOPES } from "../constants";
import { formatVCoin, parseVCoin, getCurrentTimestamp } from "../core";

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
export class ViLinkClient {
  constructor(private client: ViWoClient) {}
  
  /**
   * Get ViLink configuration
   * 
   * Finding #8 (related): Corrected byte offsets to match on-chain ViLinkConfig struct.
   * Added pending_authority field that was missing after H-02 security fix.
   */
  async getConfig(): Promise<ViLinkConfig | null> {
    try {
      const configPda = this.client.pdas.getViLinkConfig();
      const accountInfo = await this.client.connection.connection.getAccountInfo(configPda);
      
      if (!accountInfo) {
        return null;
      }
      
      const data = accountInfo.data;
      
      // Corrected byte offsets matching vilink_config.rs:
      // Offset 8:   authority (32)
      // Offset 40:  pending_authority (32) <- was missing
      // Offset 72:  vcoin_mint (32)
      // Offset 104: treasury (32)
      // Offset 136: five_a_program (32)
      // Offset 168: staking_program (32)
      // Offset 200: content_registry (32)
      // Offset 232: governance_program (32)
      // Offset 264: gasless_program (32)
      // Offset 296: enabled_actions (1)
      // Offset 297: total_actions_created (8)
      // Offset 305: total_actions_executed (8)
      // Offset 313: total_tip_volume (8)
      // Offset 321: paused (1)
      // Offset 322: platform_fee_bps (2)
      // Offset 324: bump (1)
      
      return {
        authority: new PublicKey(data.slice(8, 40)),
        pendingAuthority: new PublicKey(data.slice(40, 72)),
        vcoinMint: new PublicKey(data.slice(72, 104)),
        treasury: new PublicKey(data.slice(104, 136)),
        enabledActions: data[296],
        totalActionsCreated: new BN(data.slice(297, 305), "le"),
        totalActionsExecuted: new BN(data.slice(305, 313), "le"),
        totalTipVolume: new BN(data.slice(313, 321), "le"),
        paused: data[321] !== 0,
        platformFeeBps: data.readUInt16LE(322),
      };
    } catch (error) {
      // Finding #9 Fix: Log errors instead of silently returning null
      console.warn("[ViWoSDK] vilink.getConfig failed:", error instanceof Error ? error.message : error);
      return null;
    }
  }
  
  /**
   * Get action by nonce (M-04: deterministic PDA derivation)
   * @param creator - The action creator's public key
   * @param nonce - The action nonce (from UserActionStats.actionNonce at creation time)
   */
  async getAction(creator: PublicKey, nonce: BN): Promise<ViLinkAction | null> {
    try {
      const actionPda = this.client.pdas.getViLinkActionByNonce(creator, nonce);
      const accountInfo = await this.client.connection.connection.getAccountInfo(actionPda);
      
      if (!accountInfo) {
        return null;
      }
      
      const data = accountInfo.data;
      return {
        actionId: new Uint8Array(data.slice(8, 40)),
        creator: new PublicKey(data.slice(40, 72)),
        target: new PublicKey(data.slice(72, 104)),
        actionType: data[104] as ActionType,
        amount: new BN(data.slice(105, 113), "le"),
        expiresAt: new BN(data.slice(145, 153), "le"),
        executed: data[153] !== 0,
        executionCount: data.readUInt32LE(193),
        maxExecutions: data.readUInt32LE(197),
        actionNonce: nonce, // M-04: Store nonce for reference
      };
    } catch (error) {
      // Finding #9 Fix: Log errors instead of silently returning null
      console.warn("[ViWoSDK] vilink.getAction failed:", error instanceof Error ? error.message : error);
      return null;
    }
  }

  /**
   * @deprecated Use getAction with nonce parameter instead
   */
  async getActionByTimestamp(creator: PublicKey, timestamp: BN): Promise<ViLinkAction | null> {
    // For backwards compatibility during migration
    // New actions use nonce, not timestamp
    return this.getAction(creator, timestamp);
  }
  
  /**
   * Get user action statistics
   */
  async getUserStats(user?: PublicKey): Promise<any | null> {
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
        user: new PublicKey(data.slice(8, 40)),
        actionsCreated: new BN(data.slice(40, 48), "le"),
        actionsExecuted: new BN(data.slice(48, 56), "le"),
        tipsSent: new BN(data.slice(56, 64), "le"),
        tipsReceived: new BN(data.slice(64, 72), "le"),
        vcoinSent: new BN(data.slice(72, 80), "le"),
        vcoinReceived: new BN(data.slice(80, 88), "le"),
      };
    } catch (error) {
      // Finding #9 Fix: Log errors instead of silently returning null
      console.warn("[ViWoSDK] vilink.getUserStats failed:", error instanceof Error ? error.message : error);
      return null;
    }
  }
  
  /**
   * Get action type name
   */
  getActionTypeName(actionType: ActionType): string {
    const names = [
      "Tip",
      "Vouch",
      "Follow",
      "Challenge",
      "Stake",
      "ContentReact",
      "Delegate",
      "Vote",
    ];
    return names[actionType] || "Unknown";
  }
  
  /**
   * Check if action type is enabled
   */
  async isActionTypeEnabled(actionType: ActionType): Promise<boolean> {
    const config = await this.getConfig();
    if (!config) return false;
    return (config.enabledActions & (1 << actionType)) !== 0;
  }
  
  /**
   * Check if action is valid for execution
   * @param creator - The action creator's public key
   * @param nonce - M-04: The action nonce (NOT timestamp)
   */
  async isActionValid(creator: PublicKey, nonce: BN): Promise<{
    valid: boolean;
    reason?: string;
  }> {
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
  calculateFee(amount: BN): { fee: BN; net: BN } {
    const fee = amount.muln(VILINK_CONSTANTS.platformFeeBps).divn(10000);
    return {
      fee,
      net: amount.sub(fee),
    };
  }
  
  // ============ URI Utilities ============
  
  /**
   * Generate ViLink URI from action ID
   */
  generateUri(actionId: Uint8Array, baseUrl: string = "viwo://action"): string {
    const idHex = Buffer.from(actionId).toString("hex");
    return `${baseUrl}/${idHex}`;
  }
  
  /**
   * Parse action ID from URI
   */
  parseUri(uri: string): Uint8Array | null {
    const match = uri.match(/viwo:\/\/action\/([a-f0-9]{64})/i);
    if (!match) return null;
    return new Uint8Array(Buffer.from(match[1], "hex"));
  }
  
  /**
   * Generate QR code data for action
   */
  generateQRData(actionId: Uint8Array): string {
    return this.generateUri(actionId, "https://viwoapp.com/action");
  }
  
  /**
   * Generate shareable link with metadata
   */
  generateShareableLink(
    actionId: Uint8Array,
    metadata?: { title?: string; amount?: string }
  ): string {
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
  async buildCreateTipAction(params: {
    target: PublicKey;
    amount: BN;
    expirySeconds?: number;
    oneTime?: boolean;
    metadata?: string;
  }): Promise<Transaction> {
    if (!this.client.publicKey) {
      throw new Error("Wallet not connected");
    }
    
    // Validate amount
    const minAmount = parseVCoin(VILINK_CONSTANTS.minTipAmount.toString());
    const maxAmount = parseVCoin(VILINK_CONSTANTS.maxTipAmount.toString());
    
    if (params.amount.lt(minAmount)) {
      throw new Error(`Tip amount below minimum: ${VILINK_CONSTANTS.minTipAmount} VCoin`);
    }
    
    if (params.amount.gt(maxAmount)) {
      throw new Error(`Tip amount exceeds maximum: ${VILINK_CONSTANTS.maxTipAmount} VCoin`);
    }
    
    const tx = new Transaction();
    
    // Add create action instruction
    // tx.add(await this.program.methods.createAction(0, params.amount, params.target, ...)...);
    
    return tx;
  }
  
  /**
   * Build create vouch action transaction
   */
  async buildCreateVouchAction(params: {
    target: PublicKey;
    expirySeconds?: number;
  }): Promise<Transaction> {
    if (!this.client.publicKey) {
      throw new Error("Wallet not connected");
    }
    
    const tx = new Transaction();
    
    // Add create vouch action instruction
    // tx.add(await this.program.methods.createAction(1, new BN(0), params.target, ...)...);
    
    return tx;
  }
  
  /**
   * Build create follow action transaction
   */
  async buildCreateFollowAction(params: {
    target: PublicKey;
    maxExecutions?: number;
    expirySeconds?: number;
  }): Promise<Transaction> {
    if (!this.client.publicKey) {
      throw new Error("Wallet not connected");
    }
    
    const tx = new Transaction();
    
    // Add create follow action instruction
    
    return tx;
  }
  
  /**
   * Build execute tip action transaction
   * @param creator - The action creator's public key
   * @param nonce - M-04: The action nonce (NOT timestamp)
   */
  async buildExecuteTipAction(
    creator: PublicKey,
    nonce: BN
  ): Promise<Transaction> {
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
    
    const tx = new Transaction();
    
    // Add execute tip action instruction
    // tx.add(await this.program.methods.executeTipAction()...);
    
    return tx;
  }

  /**
   * Get the next nonce for creating an action (M-04)
   * Fetches from UserActionStats.actionNonce on-chain
   */
  async getNextNonce(user?: PublicKey): Promise<BN> {
    const stats = await this.getUserStats(user);
    if (!stats) {
      // First action for this user - nonce starts at 0
      return new BN(0);
    }
    // The nonce in stats is the NEXT nonce to use
    // This matches the on-chain action_nonce field
    return new BN(stats.actionsCreated.toNumber());
  }
}

export { VILINK_CONSTANTS, ACTION_SCOPES };

