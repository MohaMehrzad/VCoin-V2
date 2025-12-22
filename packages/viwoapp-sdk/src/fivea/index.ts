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
        accuracy: data.readUInt16LE(42),
        agility: data.readUInt16LE(44),
        activity: data.readUInt16LE(46),
        approved: data.readUInt16LE(48),
        compositeScore: data.readUInt16LE(50),
        lastUpdated: new BN(data.slice(52, 60), "le"),
        isPrivate: data[60] !== 0,
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
    description: string;
    score: string;
    weight: number;
    contribution: string;
  }[] {
    const weights = FIVE_A_CONSTANTS.scoreWeights;
    
    return [
      {
        component: "A1 - Authenticity",
        description: "Are you a real person?",
        score: this.formatScore(score.authenticity),
        weight: weights.authenticity,
        contribution: this.formatScore((score.authenticity * weights.authenticity) / 100),
      },
      {
        component: "A2 - Accuracy",
        description: "Is your content quality?",
        score: this.formatScore(score.accuracy),
        weight: weights.accuracy,
        contribution: this.formatScore((score.accuracy * weights.accuracy) / 100),
      },
      {
        component: "A3 - Agility",
        description: "Are you fast?",
        score: this.formatScore(score.agility),
        weight: weights.agility,
        contribution: this.formatScore((score.agility * weights.agility) / 100),
      },
      {
        component: "A4 - Activity",
        description: "Do you show up daily?",
        score: this.formatScore(score.activity),
        weight: weights.activity,
        contribution: this.formatScore((score.activity * weights.activity) / 100),
      },
      {
        component: "A5 - Approved",
        description: "Does the community like you?",
        score: this.formatScore(score.approved),
        weight: weights.approved,
        contribution: this.formatScore((score.approved * weights.approved) / 100),
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
    
    // Check minimum score to vouch (60% required per report)
    if (myScore.compositeScore < 6000) {
      return { canVouch: false, reason: "Score too low to vouch (min 60%)" };
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
      suggestions.push("Complete identity verification to improve Authenticity (A1)");
    }
    
    if (score.accuracy < 6000) {
      suggestions.push("Create quality content to improve Accuracy (A2)");
    }
    
    if (score.agility < 6000) {
      suggestions.push("Respond faster to improve Agility (A3)");
    }
    
    if (score.activity < 6000) {
      suggestions.push("Engage daily with content to improve Activity (A4)");
    }
    
    if (score.approved < 6000) {
      suggestions.push("Get vouched by high-score users to improve Approved (A5)");
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

