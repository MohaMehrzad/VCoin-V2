import { PublicKey, Transaction } from "@solana/web3.js";
import { BN } from "@coral-xyz/anchor";

import type { ViWoClient } from "../client";
import type { Identity, VerificationLevel } from "../types";

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
export class IdentityClient {
  constructor(private client: ViWoClient) {}
  
  /**
   * Get user identity
   */
  async getIdentity(user?: PublicKey): Promise<Identity | null> {
    const target = user || this.client.publicKey;
    if (!target) {
      throw new Error("No user specified and wallet not connected");
    }
    
    try {
      const identityPda = this.client.pdas.getUserIdentity(target);
      const accountInfo = await this.client.connection.connection.getAccountInfo(identityPda);
      
      if (!accountInfo) {
        return null;
      }
      
      const data = accountInfo.data;
      return {
        user: new PublicKey(data.slice(8, 40)),
        didHash: new Uint8Array(data.slice(40, 72)),
        verificationLevel: data[72] as VerificationLevel,
        createdAt: new BN(data.slice(73, 81), "le"),
        updatedAt: new BN(data.slice(81, 89), "le"),
      };
    } catch {
      return null;
    }
  }
  
  /**
   * Check if user has identity
   */
  async hasIdentity(user?: PublicKey): Promise<boolean> {
    const identity = await this.getIdentity(user);
    return identity !== null;
  }
  
  /**
   * Get verification level name
   */
  getVerificationLevelName(level: VerificationLevel): string {
    const levels = ["None", "Basic", "Standard", "Enhanced", "Premium"];
    return levels[level] || "Unknown";
  }
  
  /**
   * Get verification level requirements
   */
  getVerificationRequirements(level: VerificationLevel): string[] {
    const requirements: { [key: number]: string[] } = {
      0: [],
      1: ["Email verification", "Phone verification"],
      2: ["Basic requirements", "Social account linking"],
      3: ["Standard requirements", "ID verification"],
      4: ["Enhanced requirements", "Face verification", "Address verification"],
    };
    return requirements[level] || [];
  }
  
  /**
   * Get verification level benefits
   */
  getVerificationBenefits(level: VerificationLevel): string[] {
    const benefits: { [key: number]: string[] } = {
      0: ["Basic platform access"],
      1: ["Higher withdrawal limits", "Basic rewards eligibility"],
      2: ["Full rewards eligibility", "Vouch capabilities"],
      3: ["Priority support", "Enhanced trust score"],
      4: ["VIP status", "Governance proposal creation", "Maximum limits"],
    };
    return benefits[level] || [];
  }
  
  // ============ Transaction Building ============
  
  /**
   * Build create identity transaction
   */
  async buildCreateIdentityTransaction(didHash: Uint8Array): Promise<Transaction> {
    if (!this.client.publicKey) {
      throw new Error("Wallet not connected");
    }
    
    // Check if identity already exists
    const existing = await this.getIdentity();
    if (existing) {
      throw new Error("Identity already exists");
    }
    
    if (didHash.length !== 32) {
      throw new Error("DID hash must be 32 bytes");
    }
    
    const tx = new Transaction();
    
    // Add create identity instruction
    // tx.add(await this.program.methods.createIdentity(Array.from(didHash))...);
    
    return tx;
  }
  
  /**
   * Build update DID hash transaction
   */
  async buildUpdateDidHashTransaction(newDidHash: Uint8Array): Promise<Transaction> {
    if (!this.client.publicKey) {
      throw new Error("Wallet not connected");
    }
    
    if (newDidHash.length !== 32) {
      throw new Error("DID hash must be 32 bytes");
    }
    
    const tx = new Transaction();
    
    // Add update DID hash instruction
    // tx.add(await this.program.methods.updateDidHash(Array.from(newDidHash))...);
    
    return tx;
  }
}

