import { PublicKey, Transaction } from "@solana/web3.js";
import { BN } from "@coral-xyz/anchor";

import type { ViWoClient } from "../client";
import type { ContentRecord, UserEnergy, ContentState } from "../types";
import { CONTENT_CONSTANTS } from "../constants";
import { formatVCoin, getCurrentTimestamp } from "../core";

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
export class ContentClient {
  constructor(private client: ViWoClient) {}
  
  /**
   * Get content record
   */
  async getContent(contentId: Uint8Array): Promise<ContentRecord | null> {
    try {
      const contentPda = this.client.pdas.getContentRecord(contentId);
      const accountInfo = await this.client.connection.connection.getAccountInfo(contentPda);
      
      if (!accountInfo) {
        return null;
      }
      
      const data = accountInfo.data;
      return {
        contentId: new Uint8Array(data.slice(8, 40)),
        creator: new PublicKey(data.slice(40, 72)),
        contentHash: new Uint8Array(data.slice(72, 104)),
        state: data[104] as ContentState,
        createdAt: new BN(data.slice(105, 113), "le"),
        editCount: data.readUInt16LE(113),
        tips: new BN(data.slice(115, 123), "le"),
        engagementScore: new BN(data.slice(123, 131), "le"),
      };
    } catch {
      return null;
    }
  }
  
  /**
   * Get user's energy
   */
  async getEnergy(user?: PublicKey): Promise<UserEnergy | null> {
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
        user: new PublicKey(data.slice(8, 40)),
        currentEnergy: data.readUInt16LE(40),
        maxEnergy: data.readUInt16LE(42),
        lastRegenTime: new BN(data.slice(44, 52), "le"),
        tier: data[52],
      };
    } catch {
      return null;
    }
  }
  
  /**
   * Get content state name
   */
  getStateName(state: ContentState): string {
    const states = ["Active", "Hidden", "Deleted", "Flagged"];
    return states[state] || "Unknown";
  }
  
  /**
   * Calculate regenerated energy
   */
  calculateRegenEnergy(energy: UserEnergy): number {
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
  getTimeUntilNextEnergy(energy: UserEnergy): number {
    if (energy.currentEnergy >= energy.maxEnergy) {
      return 0;
    }
    
    const now = getCurrentTimestamp();
    const secondsSinceRegen = now - energy.lastRegenTime.toNumber();
    const secondsPerEnergy = 3600 / CONTENT_CONSTANTS.energyRegenRate;
    const nextRegenIn = secondsPerEnergy - (secondsSinceRegen % secondsPerEnergy);
    
    return Math.max(0, Math.ceil(nextRegenIn));
  }
  
  /**
   * Get time until full energy
   */
  getTimeUntilFull(energy: UserEnergy): number {
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
  async canCreateContent(user?: PublicKey): Promise<{
    canCreate: boolean;
    reason?: string;
    energyNeeded?: number;
    energyAvailable?: number;
  }> {
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
        energyAvailable: currentEnergy,
      };
    }
    
    return { canCreate: true, energyNeeded, energyAvailable: currentEnergy };
  }
  
  /**
   * Check if user can edit content
   */
  async canEditContent(contentId: Uint8Array, user?: PublicKey): Promise<{
    canEdit: boolean;
    reason?: string;
  }> {
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
    
    if (content.state === 2) { // Deleted
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
  async getContentStats(contentId: Uint8Array): Promise<{
    tips: string;
    engagementScore: string;
    editCount: number;
    state: string;
    age: number;
  }> {
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
      age,
    };
  }
  
  // ============ Transaction Building ============
  
  /**
   * Build create content transaction
   */
  async buildCreateContentTransaction(contentHash: Uint8Array): Promise<Transaction> {
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
    
    const tx = new Transaction();
    
    // Add create content instruction
    // tx.add(await this.program.methods.createContent(Array.from(contentHash))...);
    
    return tx;
  }
  
  /**
   * Build edit content transaction
   */
  async buildEditContentTransaction(
    contentId: Uint8Array,
    newContentHash: Uint8Array
  ): Promise<Transaction> {
    if (!this.client.publicKey) {
      throw new Error("Wallet not connected");
    }
    
    const { canEdit, reason } = await this.canEditContent(contentId);
    if (!canEdit) {
      throw new Error(reason);
    }
    
    const tx = new Transaction();
    
    // Add edit content instruction
    // tx.add(await this.program.methods.editContent(
    //   Array.from(contentId),
    //   Array.from(newContentHash)
    // )...);
    
    return tx;
  }
  
  /**
   * Build delete content transaction
   */
  async buildDeleteContentTransaction(contentId: Uint8Array): Promise<Transaction> {
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
    
    const tx = new Transaction();
    
    // Add delete content instruction
    // tx.add(await this.program.methods.deleteContent(Array.from(contentId))...);
    
    return tx;
  }
}

export { CONTENT_CONSTANTS };

