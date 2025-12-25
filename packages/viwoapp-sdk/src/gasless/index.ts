import { PublicKey, Transaction, Keypair } from "@solana/web3.js";
import { BN } from "@coral-xyz/anchor";

import type { ViWoClient } from "../client";
import type { 
  GaslessConfig, 
  SessionKey, 
  UserGaslessStats, 
  CreateSessionParams,
  FeeMethod,
} from "../types";
import { GASLESS_CONSTANTS, ACTION_SCOPES } from "../constants";
import { getCurrentTimestamp } from "../core";

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
export class GaslessClient {
  constructor(private client: ViWoClient) {}
  
  /**
   * Get gasless configuration
   * 
   * Finding #8 Fix: Corrected byte offsets to match on-chain GaslessConfig struct.
   * Added missing fields: pendingAuthority, feeVault, sscreProgram, sscreDeductionBps,
   * maxSubsidizedPerUser, totalSolSpent, currentDay, daySpent, maxSlippageBps.
   */
  async getConfig(): Promise<GaslessConfig | null> {
    try {
      const configPda = this.client.pdas.getGaslessConfig();
      const accountInfo = await this.client.connection.connection.getAccountInfo(configPda);
      
      if (!accountInfo) {
        return null;
      }
      
      const data = accountInfo.data;
      
      // Finding #8 Fix: Corrected byte offsets matching gasless_config.rs
      // Offset 8:   authority (32)
      // Offset 40:  pending_authority (32)
      // Offset 72:  fee_payer (32)
      // Offset 104: vcoin_mint (32)
      // Offset 136: fee_vault (32)
      // Offset 168: sscre_program (32)
      // Offset 200: daily_subsidy_budget (8)
      // Offset 208: sol_fee_per_tx (8)
      // Offset 216: vcoin_fee_multiplier (8)
      // Offset 224: sscre_deduction_bps (2)
      // Offset 226: max_subsidized_per_user (4)
      // Offset 230: total_subsidized_tx (8)
      // Offset 238: total_sol_spent (8)
      // Offset 246: total_vcoin_collected (8)
      // Offset 254: paused (1)
      // Offset 255: current_day (4)
      // Offset 259: day_spent (8)
      // Offset 267: max_slippage_bps (2)
      // Offset 269: bump (1)
      
      return {
        authority: new PublicKey(data.slice(8, 40)),
        pendingAuthority: new PublicKey(data.slice(40, 72)),
        feePayer: new PublicKey(data.slice(72, 104)),
        vcoinMint: new PublicKey(data.slice(104, 136)),
        feeVault: new PublicKey(data.slice(136, 168)),
        sscreProgram: new PublicKey(data.slice(168, 200)),
        dailySubsidyBudget: new BN(data.slice(200, 208), "le"),
        solFeePerTx: new BN(data.slice(208, 216), "le"),
        vcoinFeeMultiplier: new BN(data.slice(216, 224), "le"),
        sscreDeductionBps: data.readUInt16LE(224),
        maxSubsidizedPerUser: data.readUInt32LE(226),
        totalSubsidizedTx: new BN(data.slice(230, 238), "le"),
        totalSolSpent: new BN(data.slice(238, 246), "le"),
        totalVcoinCollected: new BN(data.slice(246, 254), "le"),
        paused: data[254] !== 0,
        currentDay: data.readUInt32LE(255),
        daySpent: new BN(data.slice(259, 267), "le"),
        maxSlippageBps: data.readUInt16LE(267),
      };
    } catch (error) {
      // Finding #9 Fix: Log errors instead of silently returning null
      console.warn("[ViWoSDK] gasless.getConfig failed:", error instanceof Error ? error.message : error);
      return null;
    }
  }
  
  /**
   * Get session key details
   */
  async getSessionKey(user: PublicKey, sessionPubkey: PublicKey): Promise<SessionKey | null> {
    try {
      const sessionPda = this.client.pdas.getSessionKey(user, sessionPubkey);
      const accountInfo = await this.client.connection.connection.getAccountInfo(sessionPda);
      
      if (!accountInfo) {
        return null;
      }
      
      const data = accountInfo.data;
      return {
        user: new PublicKey(data.slice(8, 40)),
        sessionPubkey: new PublicKey(data.slice(40, 72)),
        scope: data.readUInt16LE(72),
        createdAt: new BN(data.slice(74, 82), "le"),
        expiresAt: new BN(data.slice(82, 90), "le"),
        actionsUsed: data.readUInt32LE(90),
        maxActions: data.readUInt32LE(94),
        vcoinSpent: new BN(data.slice(98, 106), "le"),
        maxSpend: new BN(data.slice(106, 114), "le"),
        isRevoked: data[114] !== 0,
        feeMethod: data[123] as FeeMethod,
      };
    } catch (error) {
      // Finding #9 Fix: Log errors instead of silently returning null
      console.warn("[ViWoSDK] gasless.getSessionKey failed:", error instanceof Error ? error.message : error);
      return null;
    }
  }
  
  /**
   * Get user gasless statistics
   */
  async getUserStats(user?: PublicKey): Promise<UserGaslessStats | null> {
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
        user: new PublicKey(data.slice(8, 40)),
        totalGaslessTx: new BN(data.slice(40, 48), "le"),
        totalSubsidized: new BN(data.slice(48, 56), "le"),
        totalVcoinFees: new BN(data.slice(56, 64), "le"),
        sessionsCreated: data.readUInt32LE(72),
        activeSession: new PublicKey(data.slice(76, 108)),
      };
    } catch (error) {
      // Finding #9 Fix: Log errors instead of silently returning null
      console.warn("[ViWoSDK] gasless.getUserStats failed:", error instanceof Error ? error.message : error);
      return null;
    }
  }
  
  /**
   * Check if session is valid
   */
  async isSessionValid(user: PublicKey, sessionPubkey: PublicKey): Promise<{
    valid: boolean;
    reason?: string;
  }> {
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
  isActionInScope(session: SessionKey, actionScope: number): boolean {
    return (session.scope & actionScope) !== 0;
  }
  
  /**
   * Get remaining session actions
   */
  getRemainingActions(session: SessionKey): number {
    return session.maxActions - session.actionsUsed;
  }
  
  /**
   * Get remaining session spend
   */
  getRemainingSpend(session: SessionKey): BN {
    return session.maxSpend.sub(session.vcoinSpent);
  }
  
  /**
   * Get remaining session time
   */
  getRemainingTime(session: SessionKey): number {
    const now = getCurrentTimestamp();
    return Math.max(0, session.expiresAt.toNumber() - now);
  }
  
  /**
   * Calculate VCoin fee equivalent
   */
  async calculateVCoinFee(): Promise<BN> {
    const config = await this.getConfig();
    if (!config) {
      return new BN(GASLESS_CONSTANTS.defaultSolFee * GASLESS_CONSTANTS.vcoinFeeMultiplier);
    }
    
    return config.solFeePerTx.mul(config.vcoinFeeMultiplier);
  }
  
  /**
   * Check if user is eligible for subsidized transactions
   */
  async isEligibleForSubsidy(user?: PublicKey): Promise<{
    eligible: boolean;
    remainingToday: number;
    reason?: string;
  }> {
    const target = user || this.client.publicKey;
    if (!target) {
      throw new Error("No user specified and wallet not connected");
    }
    
    const [config, userStats] = await Promise.all([
      this.getConfig(),
      this.getUserStats(target),
    ]);
    
    if (!config) {
      return { eligible: false, remainingToday: 0, reason: "Config not found" };
    }
    
    // Check daily budget
    // In production, check current day's spending
    
    // Check user's daily limit
    const maxPerUser = GASLESS_CONSTANTS.maxSubsidizedPerUser;
    const usedToday = 0; // Would track per-day usage
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
  getScopeNames(scope: number): string[] {
    const names: string[] = [];
    const scopeMap = [
      { bit: ACTION_SCOPES.tip, name: "Tip" },
      { bit: ACTION_SCOPES.vouch, name: "Vouch" },
      { bit: ACTION_SCOPES.content, name: "Content" },
      { bit: ACTION_SCOPES.governance, name: "Governance" },
      { bit: ACTION_SCOPES.transfer, name: "Transfer" },
      { bit: ACTION_SCOPES.stake, name: "Stake" },
      { bit: ACTION_SCOPES.claim, name: "Claim" },
      { bit: ACTION_SCOPES.follow, name: "Follow" },
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
  createScope(actions: string[]): number {
    let scope = 0;
    const scopeMap: { [key: string]: number } = {
      tip: ACTION_SCOPES.tip,
      vouch: ACTION_SCOPES.vouch,
      content: ACTION_SCOPES.content,
      governance: ACTION_SCOPES.governance,
      transfer: ACTION_SCOPES.transfer,
      stake: ACTION_SCOPES.stake,
      claim: ACTION_SCOPES.claim,
      follow: ACTION_SCOPES.follow,
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
  async buildCreateSessionTransaction(params: CreateSessionParams): Promise<Transaction> {
    if (!this.client.publicKey) {
      throw new Error("Wallet not connected");
    }
    
    // Validate session pubkey
    if (!params.sessionPubkey) {
      throw new Error("Session public key required");
    }
    
    // Validate scope
    if (!params.scope || params.scope === 0) {
      throw new Error("At least one scope required");
    }
    
    // Apply defaults
    const duration = params.durationSeconds || GASLESS_CONSTANTS.sessionDuration;
    const maxActions = params.maxActions || GASLESS_CONSTANTS.maxSessionActions;
    const maxSpend = params.maxSpend || new BN(GASLESS_CONSTANTS.maxSessionSpend * 1e9);
    const feeMethod = params.feeMethod ?? 1; // VCoin by default
    
    const tx = new Transaction();
    
    // Add create session key instruction
    // tx.add(await this.program.methods.createSessionKey(
    //   params.sessionPubkey,
    //   params.scope,
    //   duration,
    //   maxActions,
    //   maxSpend,
    //   feeMethod,
    // )...);
    
    return tx;
  }
  
  /**
   * Build revoke session key transaction
   */
  async buildRevokeSessionTransaction(sessionPubkey: PublicKey): Promise<Transaction> {
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
    
    const tx = new Transaction();
    
    // Add revoke session instruction
    // tx.add(await this.program.methods.revokeSessionKey()...);
    
    return tx;
  }
  
  /**
   * Build VCoin fee deduction transaction
   */
  async buildDeductFeeTransaction(amount?: BN): Promise<Transaction> {
    if (!this.client.publicKey) {
      throw new Error("Wallet not connected");
    }
    
    const tx = new Transaction();
    
    // Add deduct fee instruction
    // tx.add(await this.program.methods.deductVcoinFee(amount || new BN(0))...);
    
    return tx;
  }
}

export { GASLESS_CONSTANTS, ACTION_SCOPES };

