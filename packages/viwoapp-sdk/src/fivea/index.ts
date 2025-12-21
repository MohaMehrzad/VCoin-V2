import { PublicKey, Transaction } from "@solana/web3.js";
import { BN } from "@coral-xyz/anchor";

import type { ViWoClient } from "../client";
import type { FiveAScore, VouchRecord } from "../types";
import { FIVE_A_CONSTANTS } from "../constants";

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
export class FiveAClient {
  constructor(private client: ViWoClient) {}
  
  /**
   * Get user's 5A score
   */
  async getScore(user?: PublicKey): Promise<FiveAScore | null> {
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
        user: new PublicKey(data.slice(8, 40)),
        authenticity: data.readUInt16LE(40),
        activity: data.readUInt16LE(42),
        age: data.readUInt16LE(44),
        associations: data.readUInt16LE(46),
        accumulation: data.readUInt16LE(48),
        composite: data.readUInt16LE(50),
        lastUpdated: new BN(data.slice(52, 60), "le"),
      };
    } catch {
      return null;
    }
  }
  
  /**
   * Format score as percentage
   */
  formatScore(score: number): string {
    return `${(score / 100).toFixed(2)}%`;
  }
  
  /**
   * Get score tier
   */
  getScoreTier(composite: number): string {
    if (composite >= 8000) return "Excellent";
    if (composite >= 6000) return "Good";
    if (composite >= 4000) return "Average";
    if (composite >= 2000) return "Below Average";
    return "Low";
  }
  
  /**
   * Get reward multiplier for score
   */
  getRewardMultiplier(composite: number): number {
    if (composite >= 8000) return FIVE_A_CONSTANTS.scoreMultipliers["80-100"];
    if (composite >= 6000) return FIVE_A_CONSTANTS.scoreMultipliers["60-80"];
    if (composite >= 4000) return FIVE_A_CONSTANTS.scoreMultipliers["40-60"];
    if (composite >= 2000) return FIVE_A_CONSTANTS.scoreMultipliers["20-40"];
    return FIVE_A_CONSTANTS.scoreMultipliers["0-20"];
  }
  
  /**
   * Get score breakdown
   */
  getScoreBreakdown(score: FiveAScore): {
    component: string;
    score: string;
    weight: number;
    contribution: string;
  }[] {
    const weights = FIVE_A_CONSTANTS.scoreWeights;
    
    return [
      {
        component: "Authenticity",
        score: this.formatScore(score.authenticity),
        weight: weights.authenticity,
        contribution: this.formatScore((score.authenticity * weights.authenticity) / 100),
      },
      {
        component: "Activity",
        score: this.formatScore(score.activity),
        weight: weights.activity,
        contribution: this.formatScore((score.activity * weights.activity) / 100),
      },
      {
        component: "Age",
        score: this.formatScore(score.age),
        weight: weights.age,
        contribution: this.formatScore((score.age * weights.age) / 100),
      },
      {
        component: "Associations",
        score: this.formatScore(score.associations),
        weight: weights.associations,
        contribution: this.formatScore((score.associations * weights.associations) / 100),
      },
      {
        component: "Accumulation",
        score: this.formatScore(score.accumulation),
        weight: weights.accumulation,
        contribution: this.formatScore((score.accumulation * weights.accumulation) / 100),
      },
    ];
  }
  
  /**
   * Calculate max vouches for score
   */
  getMaxVouches(composite: number): number {
    // Higher score = more vouch capacity
    if (composite >= 9000) return 20;
    if (composite >= 8000) return 15;
    if (composite >= 7000) return 10;
    if (composite >= 6000) return 7;
    if (composite >= 5000) return 5;
    if (composite >= 4000) return 3;
    return 2;
  }
  
  /**
   * Check if user can vouch for another
   */
  async canVouchFor(target: PublicKey): Promise<{
    canVouch: boolean;
    reason?: string;
  }> {
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
    
    // Check minimum score to vouch
    if (myScore.composite < 4000) {
      return { canVouch: false, reason: "Score too low to vouch (min 40%)" };
    }
    
    // Would also check vouch limits, existing vouches, etc.
    
    return { canVouch: true };
  }
  
  /**
   * Get score improvement suggestions
   */
  getImprovementSuggestions(score: FiveAScore): string[] {
    const suggestions: string[] = [];
    
    if (score.authenticity < 6000) {
      suggestions.push("Complete identity verification to improve Authenticity");
    }
    
    if (score.activity < 6000) {
      suggestions.push("Engage more with content and users to improve Activity");
    }
    
    if (score.age < 6000) {
      suggestions.push("Account age improves over time - stay active!");
    }
    
    if (score.associations < 6000) {
      suggestions.push("Get vouched by high-score users to improve Associations");
    }
    
    if (score.accumulation < 6000) {
      suggestions.push("Stake VCoin and earn rewards to improve Accumulation");
    }
    
    return suggestions;
  }
  
  // ============ Transaction Building ============
  
  /**
   * Build vouch transaction
   */
  async buildVouchTransaction(target: PublicKey): Promise<Transaction> {
    if (!this.client.publicKey) {
      throw new Error("Wallet not connected");
    }
    
    const { canVouch, reason } = await this.canVouchFor(target);
    if (!canVouch) {
      throw new Error(reason);
    }
    
    const tx = new Transaction();
    
    // Add vouch instruction
    // tx.add(await this.program.methods.vouchForUser(target)...);
    
    return tx;
  }
}

export { FIVE_A_CONSTANTS };

