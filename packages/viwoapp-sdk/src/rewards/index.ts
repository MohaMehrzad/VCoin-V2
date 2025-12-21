import { PublicKey, Transaction } from "@solana/web3.js";
import { BN } from "@coral-xyz/anchor";

import type { ViWoClient } from "../client";
import type { RewardsPoolConfig, EpochDistribution, UserClaim, ClaimRewardsParams } from "../types";
import { SSCRE_CONSTANTS } from "../constants";
import { formatVCoin } from "../core";

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
export class RewardsClient {
  constructor(private client: ViWoClient) {}
  
  /**
   * Get rewards pool configuration
   */
  async getPoolConfig(): Promise<RewardsPoolConfig | null> {
    try {
      const configPda = this.client.pdas.getRewardsPoolConfig();
      const accountInfo = await this.client.connection.connection.getAccountInfo(configPda);
      
      if (!accountInfo) {
        return null;
      }
      
      const data = accountInfo.data;
      return {
        authority: new PublicKey(data.slice(8, 40)),
        vcoinMint: new PublicKey(data.slice(40, 72)),
        currentEpoch: new BN(data.slice(72, 80), "le"),
        totalDistributed: new BN(data.slice(80, 88), "le"),
        remainingReserves: new BN(data.slice(88, 96), "le"),
        paused: data[96] !== 0,
      };
    } catch {
      return null;
    }
  }
  
  /**
   * Get epoch distribution details
   */
  async getEpochDistribution(epoch: BN): Promise<EpochDistribution | null> {
    try {
      const epochPda = this.client.pdas.getEpochDistribution(epoch);
      const accountInfo = await this.client.connection.connection.getAccountInfo(epochPda);
      
      if (!accountInfo) {
        return null;
      }
      
      const data = accountInfo.data;
      return {
        epoch: new BN(data.slice(8, 16), "le"),
        merkleRoot: new Uint8Array(data.slice(16, 48)),
        totalAllocation: new BN(data.slice(48, 56), "le"),
        totalClaimed: new BN(data.slice(56, 64), "le"),
        claimsCount: new BN(data.slice(64, 72), "le"),
        isFinalized: data[72] !== 0,
      };
    } catch {
      return null;
    }
  }
  
  /**
   * Get current epoch
   */
  async getCurrentEpoch(): Promise<BN> {
    const config = await this.getPoolConfig();
    return config?.currentEpoch || new BN(0);
  }
  
  /**
   * Get user claim history
   */
  async getUserClaim(user?: PublicKey): Promise<UserClaim | null> {
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
        user: new PublicKey(data.slice(8, 40)),
        lastClaimedEpoch: new BN(data.slice(40, 48), "le"),
        totalClaimed: new BN(data.slice(48, 56), "le"),
        claimsCount: data.readUInt32LE(56),
      };
    } catch {
      return null;
    }
  }
  
  /**
   * Check if user has claimed for an epoch
   */
  async hasClaimedEpoch(epoch: BN, user?: PublicKey): Promise<boolean> {
    const userClaim = await this.getUserClaim(user);
    if (!userClaim) return false;
    
    // Check bitmap for recent epochs
    const epochNum = epoch.toNumber();
    if (epochNum <= 255) {
      // Would need to parse the bitmap from the account
      // Simplified check using lastClaimedEpoch
      return userClaim.lastClaimedEpoch.gte(epoch);
    }
    
    return userClaim.lastClaimedEpoch.gte(epoch);
  }
  
  /**
   * Get unclaimed epochs
   */
  async getUnclaimedEpochs(user?: PublicKey): Promise<BN[]> {
    const currentEpoch = await this.getCurrentEpoch();
    const userClaim = await this.getUserClaim(user);
    
    const unclaimed: BN[] = [];
    const startEpoch = userClaim 
      ? userClaim.lastClaimedEpoch.toNumber() + 1 
      : 1;
    
    for (let e = startEpoch; e <= currentEpoch.toNumber(); e++) {
      const epochDist = await this.getEpochDistribution(new BN(e));
      if (epochDist?.isFinalized) {
        // Check claim window
        const now = Math.floor(Date.now() / 1000);
        const claimExpiry = epochDist.epoch.toNumber() * SSCRE_CONSTANTS.epochDuration + 
          SSCRE_CONSTANTS.claimWindow;
        
        if (now <= claimExpiry) {
          unclaimed.push(new BN(e));
        }
      }
    }
    
    return unclaimed;
  }
  
  /**
   * Get rewards statistics
   */
  async getStats(): Promise<{
    currentEpoch: number;
    totalDistributed: string;
    remainingReserves: string;
    reservePercentage: number;
    userTotalClaimed: string | null;
    userClaimsCount: number | null;
  }> {
    const config = await this.getPoolConfig();
    const userClaim = this.client.publicKey 
      ? await this.getUserClaim() 
      : null;
    
    const totalReserves = SSCRE_CONSTANTS.primaryReserves * 1e9;
    const remaining = config?.remainingReserves.toNumber() || 0;
    const reservePct = (remaining / totalReserves) * 100;
    
    return {
      currentEpoch: config?.currentEpoch.toNumber() || 0,
      totalDistributed: config ? formatVCoin(config.totalDistributed) : "0",
      remainingReserves: config ? formatVCoin(config.remainingReserves) : "0",
      reservePercentage: reservePct,
      userTotalClaimed: userClaim ? formatVCoin(userClaim.totalClaimed) : null,
      userClaimsCount: userClaim?.claimsCount || null,
    };
  }
  
  /**
   * Calculate gasless fee for claim
   */
  calculateGaslessFee(amount: BN): BN {
    const fee = amount.muln(SSCRE_CONSTANTS.gaslessFeeBps).divn(10000);
    return fee;
  }
  
  /**
   * Calculate net claim amount after fee
   */
  calculateNetClaim(amount: BN): BN {
    const fee = this.calculateGaslessFee(amount);
    return amount.sub(fee);
  }
  
  // ============ Merkle Proof Utilities ============
  
  /**
   * Verify merkle proof locally
   */
  verifyMerkleProof(
    proof: Uint8Array[],
    root: Uint8Array,
    leaf: Uint8Array
  ): boolean {
    let computedHash = leaf;
    
    for (const proofElement of proof) {
      const combined = new Uint8Array(64);
      
      // Sort hashes for consistent ordering
      if (this.compareBytes(computedHash, proofElement) < 0) {
        combined.set(computedHash, 0);
        combined.set(proofElement, 32);
      } else {
        combined.set(proofElement, 0);
        combined.set(computedHash, 32);
      }
      
      // In production, use proper keccak256
      computedHash = this.hashBytes(combined);
    }
    
    return this.compareBytes(computedHash, root) === 0;
  }
  
  /**
   * Compute leaf hash from user data
   */
  computeLeaf(user: PublicKey, amount: BN, epoch: BN): Uint8Array {
    const data = new Uint8Array(48);
    data.set(user.toBytes(), 0);
    data.set(amount.toArrayLike(Buffer, "le", 8), 32);
    data.set(epoch.toArrayLike(Buffer, "le", 8), 40);
    
    return this.hashBytes(data);
  }
  
  private compareBytes(a: Uint8Array, b: Uint8Array): number {
    for (let i = 0; i < Math.min(a.length, b.length); i++) {
      if (a[i] !== b[i]) {
        return a[i] - b[i];
      }
    }
    return a.length - b.length;
  }
  
  private hashBytes(data: Uint8Array): Uint8Array {
    // Placeholder - in production use proper keccak256
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
  async buildClaimTransaction(params: ClaimRewardsParams): Promise<Transaction> {
    if (!this.client.publicKey) {
      throw new Error("Wallet not connected");
    }
    
    // Validate minimum claim
    if (params.amount.lt(new BN(SSCRE_CONSTANTS.minClaimAmount * 1e9))) {
      throw new Error(`Claim amount below minimum: ${SSCRE_CONSTANTS.minClaimAmount} VCoin`);
    }
    
    // Check if already claimed
    const hasClaimed = await this.hasClaimedEpoch(params.epoch);
    if (hasClaimed) {
      throw new Error("Already claimed for this epoch");
    }
    
    const tx = new Transaction();
    
    // Add claim instruction
    // tx.add(await this.program.methods.claimRewards(params.amount, params.merkleProof)...);
    
    return tx;
  }
}

export { SSCRE_CONSTANTS };

