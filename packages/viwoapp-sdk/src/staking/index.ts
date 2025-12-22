import { PublicKey, Transaction, SystemProgram } from "@solana/web3.js";
import { BN } from "@coral-xyz/anchor";
import { TOKEN_2022_PROGRAM_ID } from "@solana/spl-token";

import type { ViWoClient } from "../client";
import type { StakingPool, UserStake, StakeParams, StakingTier } from "../types";
import { STAKING_TIERS, LOCK_DURATIONS, VCOIN_DECIMALS } from "../constants";
import { formatVCoin, parseVCoin } from "../core";

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
export class StakingClient {
  constructor(private client: ViWoClient) {}
  
  /**
   * Get staking pool configuration
   */
  async getPool(): Promise<StakingPool | null> {
    try {
      const poolPda = this.client.pdas.getStakingPool();
      const accountInfo = await this.client.connection.connection.getAccountInfo(poolPda);
      
      if (!accountInfo) {
        return null;
      }
      
      // Parse account data
      // In production, use proper IDL deserialization
      return {
        authority: new PublicKey(accountInfo.data.slice(8, 40)),
        vcoinMint: new PublicKey(accountInfo.data.slice(40, 72)),
        vevcoinMint: new PublicKey(accountInfo.data.slice(72, 104)),
        totalStaked: new BN(accountInfo.data.slice(104, 112), "le"),
        totalVevcoinMinted: new BN(accountInfo.data.slice(112, 120), "le"),
        paused: accountInfo.data[120] !== 0,
      };
    } catch {
      return null;
    }
  }
  
  /**
   * Get user stake information
   */
  async getUserStake(user?: PublicKey): Promise<UserStake | null> {
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
      
      // Parse account data
      return {
        user: new PublicKey(accountInfo.data.slice(8, 40)),
        stakedAmount: new BN(accountInfo.data.slice(40, 48), "le"),
        vevcoinBalance: new BN(accountInfo.data.slice(48, 56), "le"),
        tier: accountInfo.data[56] as StakingTier,
        lockEndTime: new BN(accountInfo.data.slice(57, 65), "le"),
        lastUpdateTime: new BN(accountInfo.data.slice(65, 73), "le"),
      };
    } catch {
      return null;
    }
  }
  
  /**
   * Calculate tier based on stake amount
   */
  calculateTier(stakeAmount: BN | number): StakingTier {
    const amount = typeof stakeAmount === "number" 
      ? stakeAmount 
      : stakeAmount.toNumber() / Math.pow(10, VCOIN_DECIMALS);
    
    if (amount >= STAKING_TIERS.platinum.minStake) return 4; // Platinum
    if (amount >= STAKING_TIERS.gold.minStake) return 3;     // Gold
    if (amount >= STAKING_TIERS.silver.minStake) return 2;   // Silver
    if (amount >= STAKING_TIERS.bronze.minStake) return 1;   // Bronze
    return 0; // None
  }
  
  /**
   * Calculate veVCoin amount for given stake
   * Formula: ve_vcoin = staked_amount × (lock_duration / 4_years) × tier_boost
   */
  calculateVeVCoin(amount: BN, lockDuration: number): BN {
    const FOUR_YEARS = 4 * 365 * 24 * 3600; // 126,144,000 seconds
    const lockRatio = lockDuration / FOUR_YEARS;
    
    const tier = this.calculateTier(amount);
    const tierBoosts = [1.0, 1.1, 1.2, 1.3, 1.4]; // None, Bronze, Silver, Gold, Platinum
    const tierBoost = tierBoosts[tier];
    
    const multiplier = lockRatio * tierBoost;
    const vevcoinAmount = amount.toNumber() * multiplier;
    
    return new BN(Math.floor(vevcoinAmount));
  }
  
  /**
   * Get tier name
   */
  getTierName(tier: StakingTier): string {
    const names = ["None", "Bronze", "Silver", "Gold", "Platinum"];
    return names[tier] || "Unknown";
  }
  
  /**
   * Get tier info
   */
  getTierInfo(tier: StakingTier): typeof STAKING_TIERS.none {
    const tiers = [
      STAKING_TIERS.none,
      STAKING_TIERS.bronze,
      STAKING_TIERS.silver,
      STAKING_TIERS.gold,
      STAKING_TIERS.platinum,
    ];
    return tiers[tier] || STAKING_TIERS.none;
  }
  
  /**
   * Check if user can unstake
   */
  async canUnstake(user?: PublicKey): Promise<{ canUnstake: boolean; reason?: string }> {
    const stakeInfo = await this.getUserStake(user);
    
    if (!stakeInfo) {
      return { canUnstake: false, reason: "No active stake found" };
    }
    
    if (stakeInfo.stakedAmount.isZero()) {
      return { canUnstake: false, reason: "No staked amount" };
    }
    
    const now = Math.floor(Date.now() / 1000);
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
  async getStats(): Promise<{
    totalStaked: string;
    totalVevcoin: string;
    userStake: string | null;
    userVevcoin: string | null;
    userTier: string | null;
  }> {
    const pool = await this.getPool();
    const userStake = this.client.publicKey 
      ? await this.getUserStake() 
      : null;
    
    return {
      totalStaked: pool ? formatVCoin(pool.totalStaked) : "0",
      totalVevcoin: pool ? formatVCoin(pool.totalVevcoinMinted) : "0",
      userStake: userStake ? formatVCoin(userStake.stakedAmount) : null,
      userVevcoin: userStake ? formatVCoin(userStake.vevcoinBalance) : null,
      userTier: userStake ? this.getTierName(userStake.tier) : null,
    };
  }
  
  // ============ Transaction Building ============
  // Note: Full implementation would use Anchor IDL
  
  /**
   * Build stake instruction
   * @param params Stake parameters
   * @returns Transaction to sign and send
   */
  async buildStakeTransaction(params: StakeParams): Promise<Transaction> {
    if (!this.client.publicKey) {
      throw new Error("Wallet not connected");
    }
    
    // In production, this would build using Anchor program methods
    // For now, return placeholder transaction
    const tx = new Transaction();
    
    // Add stake instruction
    // tx.add(await this.program.methods.stake(params.amount, params.lockDuration)...);
    
    return tx;
  }
  
  /**
   * Build unstake instruction
   * @returns Transaction to sign and send
   */
  async buildUnstakeTransaction(): Promise<Transaction> {
    if (!this.client.publicKey) {
      throw new Error("Wallet not connected");
    }
    
    const { canUnstake, reason } = await this.canUnstake();
    if (!canUnstake) {
      throw new Error(reason);
    }
    
    const tx = new Transaction();
    
    // Add unstake instruction
    // tx.add(await this.program.methods.unstake()...);
    
    return tx;
  }
  
  /**
   * Build extend lock instruction
   * @param newDuration New lock duration in seconds
   * @returns Transaction to sign and send
   */
  async buildExtendLockTransaction(newDuration: number): Promise<Transaction> {
    if (!this.client.publicKey) {
      throw new Error("Wallet not connected");
    }
    
    const tx = new Transaction();
    
    // Add extend lock instruction
    // tx.add(await this.program.methods.extendLock(newDuration)...);
    
    return tx;
  }
}

export { STAKING_TIERS, LOCK_DURATIONS };

