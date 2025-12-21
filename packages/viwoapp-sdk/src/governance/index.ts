import { PublicKey, Transaction } from "@solana/web3.js";
import { BN } from "@coral-xyz/anchor";

import type { ViWoClient } from "../client";
import type { Proposal, VoteRecord, CreateProposalParams } from "../types";
import { ProposalStatus } from "../types";
import { GOVERNANCE_CONSTANTS } from "../constants";
import { formatVCoin } from "../core";

/**
 * Governance Client for ViWoApp governance operations
 * 
 * @example
 * ```typescript
 * const govClient = client.governance;
 * 
 * // Create a proposal
 * await govClient.createProposal({
 *   title: "Increase staking rewards",
 *   description: "Proposal to increase staking APY by 10%",
 *   category: 1,
 *   durationDays: 7,
 * });
 * 
 * // Vote on a proposal
 * await govClient.vote(proposalId, true); // Vote in favor
 * ```
 */
export class GovernanceClient {
  constructor(private client: ViWoClient) {}
  
  /**
   * Get governance configuration
   */
  async getConfig(): Promise<any | null> {
    try {
      const configPda = this.client.pdas.getGovernanceConfig();
      const accountInfo = await this.client.connection.connection.getAccountInfo(configPda);
      
      if (!accountInfo) {
        return null;
      }
      
      // Parse account data
      return {
        authority: new PublicKey(accountInfo.data.slice(8, 40)),
        proposalCount: new BN(accountInfo.data.slice(40, 48), "le"),
        vevcoinMint: new PublicKey(accountInfo.data.slice(48, 80)),
        paused: accountInfo.data[80] !== 0,
      };
    } catch {
      return null;
    }
  }
  
  /**
   * Get proposal by ID
   */
  async getProposal(proposalId: BN): Promise<Proposal | null> {
    try {
      const proposalPda = this.client.pdas.getProposal(proposalId);
      const accountInfo = await this.client.connection.connection.getAccountInfo(proposalPda);
      
      if (!accountInfo) {
        return null;
      }
      
      // Parse account data (simplified)
      const data = accountInfo.data;
      return {
        id: new BN(data.slice(8, 16), "le"),
        proposer: new PublicKey(data.slice(16, 48)),
        title: Buffer.from(data.slice(48, 112)).toString("utf8").replace(/\0/g, ""),
        descriptionHash: new Uint8Array(data.slice(112, 144)),
        startTime: new BN(data.slice(144, 152), "le"),
        endTime: new BN(data.slice(152, 160), "le"),
        votesFor: new BN(data.slice(160, 168), "le"),
        votesAgainst: new BN(data.slice(168, 176), "le"),
        status: data[176] as ProposalStatus,
        executed: data[177] !== 0,
        category: data[178],
      };
    } catch {
      return null;
    }
  }
  
  /**
   * Get all active proposals
   */
  async getActiveProposals(): Promise<Proposal[]> {
    // In production, this would use getProgramAccounts with filters
    const config = await this.getConfig();
    if (!config) return [];
    
    const proposals: Proposal[] = [];
    const now = Math.floor(Date.now() / 1000);
    
    // Fetch recent proposals
    const proposalCount = config.proposalCount.toNumber();
    const startFrom = Math.max(0, proposalCount - 20); // Last 20 proposals
    
    for (let i = startFrom; i < proposalCount; i++) {
      const proposal = await this.getProposal(new BN(i));
      if (proposal && proposal.endTime.toNumber() > now && proposal.status === 0) {
        proposals.push(proposal);
      }
    }
    
    return proposals;
  }
  
  /**
   * Get user's vote record for a proposal
   */
  async getVoteRecord(proposalId: BN, user?: PublicKey): Promise<VoteRecord | null> {
    const target = user || this.client.publicKey;
    if (!target) {
      throw new Error("No user specified and wallet not connected");
    }
    
    try {
      const proposalPda = this.client.pdas.getProposal(proposalId);
      const votePda = this.client.pdas.getVoteRecord(target, proposalPda);
      const accountInfo = await this.client.connection.connection.getAccountInfo(votePda);
      
      if (!accountInfo) {
        return null;
      }
      
      const data = accountInfo.data;
      return {
        user: new PublicKey(data.slice(8, 40)),
        proposal: new PublicKey(data.slice(40, 72)),
        votePower: new BN(data.slice(72, 80), "le"),
        support: data[80] !== 0,
        votedAt: new BN(data.slice(81, 89), "le"),
      };
    } catch {
      return null;
    }
  }
  
  /**
   * Check if user has voted on a proposal
   */
  async hasVoted(proposalId: BN, user?: PublicKey): Promise<boolean> {
    const voteRecord = await this.getVoteRecord(proposalId, user);
    return voteRecord !== null;
  }
  
  /**
   * Calculate user's voting power
   */
  async getVotingPower(user?: PublicKey): Promise<BN> {
    const target = user || this.client.publicKey;
    if (!target) {
      throw new Error("No user specified and wallet not connected");
    }
    
    // Voting power = veVCoin balance × 5A multiplier
    const vevcoinBalance = await this.client.getVeVCoinBalance(target);
    
    // In production, also fetch 5A score and apply multiplier
    const fiveAMultiplier = 1.0; // Placeholder
    
    return new BN(Math.floor(vevcoinBalance.toNumber() * fiveAMultiplier));
  }
  
  /**
   * Get proposal status text
   */
  getStatusText(status: ProposalStatus): string {
    const statuses = ["Active", "Passed", "Rejected", "Executed", "Cancelled"];
    return statuses[status] || "Unknown";
  }
  
  /**
   * Check if proposal can be executed
   */
  async canExecute(proposalId: BN): Promise<{ canExecute: boolean; reason?: string }> {
    const proposal = await this.getProposal(proposalId);
    
    if (!proposal) {
      return { canExecute: false, reason: "Proposal not found" };
    }
    
    if (proposal.executed) {
      return { canExecute: false, reason: "Proposal already executed" };
    }
    
    if (proposal.status !== ProposalStatus.Passed) {
      return { canExecute: false, reason: "Proposal has not passed" };
    }
    
    const now = Math.floor(Date.now() / 1000);
    const executionDelay = proposal.endTime.toNumber() + GOVERNANCE_CONSTANTS.executionDelay;
    
    if (now < executionDelay) {
      const remaining = executionDelay - now;
      const hours = Math.ceil(remaining / 3600);
      return { canExecute: false, reason: `Execution delay: ${hours} hours remaining` };
    }
    
    return { canExecute: true };
  }
  
  /**
   * Get proposal progress
   */
  async getProposalProgress(proposalId: BN): Promise<{
    votesFor: string;
    votesAgainst: string;
    totalVotes: string;
    forPercentage: number;
    againstPercentage: number;
    quorumReached: boolean;
    timeRemaining: number;
  }> {
    const proposal = await this.getProposal(proposalId);
    
    if (!proposal) {
      throw new Error("Proposal not found");
    }
    
    const totalVotes = proposal.votesFor.add(proposal.votesAgainst);
    const forPct = totalVotes.isZero() 
      ? 0 
      : (proposal.votesFor.toNumber() / totalVotes.toNumber()) * 100;
    const againstPct = 100 - forPct;
    
    // Calculate quorum (simplified)
    const quorumThreshold = new BN(GOVERNANCE_CONSTANTS.quorumBps).mul(new BN(10000));
    const quorumReached = totalVotes.gte(quorumThreshold);
    
    const now = Math.floor(Date.now() / 1000);
    const timeRemaining = Math.max(0, proposal.endTime.toNumber() - now);
    
    return {
      votesFor: formatVCoin(proposal.votesFor),
      votesAgainst: formatVCoin(proposal.votesAgainst),
      totalVotes: formatVCoin(totalVotes),
      forPercentage: forPct,
      againstPercentage: againstPct,
      quorumReached,
      timeRemaining,
    };
  }
  
  // ============ Transaction Building ============
  
  /**
   * Build create proposal transaction
   */
  async buildCreateProposalTransaction(params: CreateProposalParams): Promise<Transaction> {
    if (!this.client.publicKey) {
      throw new Error("Wallet not connected");
    }
    
    // Check voting power meets threshold
    const votingPower = await this.getVotingPower();
    if (votingPower.toNumber() < GOVERNANCE_CONSTANTS.minProposalThreshold) {
      throw new Error(`Insufficient voting power. Need ${GOVERNANCE_CONSTANTS.minProposalThreshold} veVCoin`);
    }
    
    const tx = new Transaction();
    
    // Add create proposal instruction
    // tx.add(await this.program.methods.createProposal(...)...);
    
    return tx;
  }
  
  /**
   * Build vote transaction
   */
  async buildVoteTransaction(proposalId: BN, support: boolean): Promise<Transaction> {
    if (!this.client.publicKey) {
      throw new Error("Wallet not connected");
    }
    
    // Check if already voted
    const hasVoted = await this.hasVoted(proposalId);
    if (hasVoted) {
      throw new Error("Already voted on this proposal");
    }
    
    const tx = new Transaction();
    
    // Add vote instruction
    // tx.add(await this.program.methods.castVote(proposalId, support)...);
    
    return tx;
  }
  
  /**
   * Build execute proposal transaction
   */
  async buildExecuteTransaction(proposalId: BN): Promise<Transaction> {
    if (!this.client.publicKey) {
      throw new Error("Wallet not connected");
    }
    
    const { canExecute, reason } = await this.canExecute(proposalId);
    if (!canExecute) {
      throw new Error(reason);
    }
    
    const tx = new Transaction();
    
    // Add execute instruction
    // tx.add(await this.program.methods.executeProposal(proposalId)...);
    
    return tx;
  }
}

export { GOVERNANCE_CONSTANTS };

