import { PublicKey, Transaction } from "@solana/web3.js";
import { BN } from "@coral-xyz/anchor";

import type { ViWoClient } from "../client";
import type {
  Proposal,
  VoteRecord,
  CreateProposalParams,
  GovernanceConfig,
  PrivateVotingConfig,
  DecryptionShare,
  CastPrivateVoteParams,
  EnablePrivateVotingParams,
  SubmitDecryptionShareParams,
  AggregateRevealedVotesParams,
} from "../types";
import { ProposalStatus, VoteChoice } from "../types";
import { GOVERNANCE_CONSTANTS } from "../constants";
import { formatVCoin } from "../core";

/**
 * Governance Client for ViWoApp governance operations
 *
 * Supports both public voting and ZK private voting with Twisted ElGamal
 * encryption on Ristretto255.
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
 * // Public vote on a proposal
 * await govClient.vote(proposalId, true);
 *
 * // Private vote (using zk-voting-sdk for encryption)
 * await govClient.buildCastPrivateVoteTransaction({
 *   proposalId, ctFor, ctAgainst, ctAbstain, proofData,
 * });
 * ```
 */
export class GovernanceClient {
  constructor(private client: ViWoClient) {}

  /**
   * Get governance configuration
   */
  async getConfig(): Promise<GovernanceConfig | null> {
    try {
      const configPda = this.client.pdas.getGovernanceConfig();
      const accountInfo = await this.client.connection.connection.getAccountInfo(configPda);

      if (!accountInfo) {
        return null;
      }

      const data = accountInfo.data;
      return {
        authority: new PublicKey(data.slice(8, 40)),
        proposalCount: new BN(data.slice(40, 48), "le"),
        vevcoinMint: new PublicKey(data.slice(48, 80)),
        paused: data[80] !== 0,
        pendingAuthority: new PublicKey(data.slice(81, 113)),
        pendingAuthorityActivatedAt: new BN(data.slice(113, 121), "le"),
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
        // Private vote ciphertexts (if present, at byte offsets after public vote fields)
        ctFor: new Uint8Array(data.slice(89, 153)),
        ctAgainst: new Uint8Array(data.slice(153, 217)),
        ctAbstain: new Uint8Array(data.slice(217, 281)),
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
   *
   * @note M-01 fix: 5A boost formula is now `1000 + ((five_a_score * 100) / 1000)`
   * to fix precision loss for small scores.
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
   *
   * @note C-03: Quorum is now calculated as votesFor + votesAgainst only.
   * Abstains do NOT count toward quorum.
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

    // C-03: Quorum uses only for + against (abstains excluded)
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

  // ============ Transaction Building (Public Voting) ============

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
   * Build vote transaction (public)
   *
   * @note v2.8.0 (C-NEW-01): Voting power parameters (vevcoin_balance, five_a_score, tier)
   * are now read from on-chain state, not passed as parameters. This prevents vote manipulation.
   * The transaction only needs: proposal_id and choice (VoteChoice enum)
   *
   * @param proposalId - The proposal to vote on
   * @param support - true = For, false = Against (use VoteChoice for more options)
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

    // v2.8.0: cast_vote now only takes (ctx, choice) - voting power is read on-chain
    const choice = support ? VoteChoice.For : VoteChoice.Against;
    // tx.add(await this.program.methods.castVote(choice)...);

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

  // ============ ZK Private Voting ============

  /**
   * Build cast private vote transaction
   *
   * Uses Twisted ElGamal encryption on Ristretto255 with compressed sigma proofs.
   * The voter encrypts their choice into 3 ciphertexts (for/against/abstain) and
   * generates a validity proof that exactly one ciphertext encrypts their weight.
   *
   * Use the `zk-voting-sdk` crate to generate ciphertexts and proofs off-chain:
   * ```rust
   * let (ct_for, ct_against, ct_abstain, proof) = encrypt_and_prove(&pubkey, choice, weight);
   * ```
   *
   * @param params - Private vote parameters with ciphertexts and proof
   */
  async buildCastPrivateVoteTransaction(params: CastPrivateVoteParams): Promise<Transaction> {
    if (!this.client.publicKey) {
      throw new Error("Wallet not connected");
    }

    // Validate ciphertext sizes
    if (params.ctFor.length !== GOVERNANCE_CONSTANTS.zk.ciphertextSize) {
      throw new Error(`ctFor must be ${GOVERNANCE_CONSTANTS.zk.ciphertextSize} bytes`);
    }
    if (params.ctAgainst.length !== GOVERNANCE_CONSTANTS.zk.ciphertextSize) {
      throw new Error(`ctAgainst must be ${GOVERNANCE_CONSTANTS.zk.ciphertextSize} bytes`);
    }
    if (params.ctAbstain.length !== GOVERNANCE_CONSTANTS.zk.ciphertextSize) {
      throw new Error(`ctAbstain must be ${GOVERNANCE_CONSTANTS.zk.ciphertextSize} bytes`);
    }

    // Validate proof size
    if (params.proofData.length !== GOVERNANCE_CONSTANTS.zk.voteProofSize) {
      throw new Error(`proofData must be ${GOVERNANCE_CONSTANTS.zk.voteProofSize} bytes`);
    }

    // Check if already voted
    const hasVoted = await this.hasVoted(params.proposalId);
    if (hasVoted) {
      throw new Error("Already voted on this proposal");
    }

    const tx = new Transaction();

    // tx.add(await this.program.methods.castPrivateVote(
    //   Array.from(params.ctFor),
    //   Array.from(params.ctAgainst),
    //   Array.from(params.ctAbstain),
    //   Buffer.from(params.proofData),
    // )...);

    return tx;
  }

  /**
   * Build enable private voting transaction
   *
   * Called by the governance authority to enable ZK private voting on a proposal.
   * Requires specifying the decryption committee and their ElGamal public keys.
   *
   * @param params - Private voting configuration
   */
  async buildEnablePrivateVotingTransaction(params: EnablePrivateVotingParams): Promise<Transaction> {
    if (!this.client.publicKey) {
      throw new Error("Wallet not connected");
    }

    // Validate encryption pubkey (must be 32 bytes Ristretto point)
    if (params.encryptionPubkey.length !== 32) {
      throw new Error("encryptionPubkey must be 32 bytes (Ristretto255 point)");
    }

    // Validate committee size
    if (params.committeeSize > GOVERNANCE_CONSTANTS.zk.maxCommitteeSize) {
      throw new Error(`Committee size exceeds max of ${GOVERNANCE_CONSTANTS.zk.maxCommitteeSize}`);
    }

    if (params.decryptionThreshold > params.committeeSize) {
      throw new Error("Threshold cannot exceed committee size");
    }

    // Validate ElGamal pubkeys
    for (let i = 0; i < params.committeeSize; i++) {
      if (!params.committeeElgamalPubkeys[i] || params.committeeElgamalPubkeys[i].length !== 32) {
        throw new Error(`Committee member ${i} ElGamal pubkey must be 32 bytes`);
      }
    }

    const tx = new Transaction();

    // tx.add(await this.program.methods.enablePrivateVoting(
    //   Array.from(params.encryptionPubkey),
    //   params.decryptionCommittee,
    //   params.committeeSize,
    //   params.decryptionThreshold,
    //   params.committeeElgamalPubkeys.map(pk => Array.from(pk)),
    // )...);

    return tx;
  }

  /**
   * Build submit decryption share transaction
   *
   * Called by a committee member during the reveal phase.
   * Each member computes partial decryptions and a DLEQ proof off-chain:
   * ```rust
   * let partial = generate_partial_decryption(&secret_share, &pk, &r_for, &r_against, &r_abstain);
   * ```
   *
   * @param params - Decryption share with DLEQ proof
   */
  async buildSubmitDecryptionShareTransaction(params: SubmitDecryptionShareParams): Promise<Transaction> {
    if (!this.client.publicKey) {
      throw new Error("Wallet not connected");
    }

    // Validate all fields are 32 bytes
    const fields = [
      { name: "partialFor", value: params.partialFor },
      { name: "partialAgainst", value: params.partialAgainst },
      { name: "partialAbstain", value: params.partialAbstain },
      { name: "dleqChallenge", value: params.dleqChallenge },
      { name: "dleqResponse", value: params.dleqResponse },
    ];

    for (const { name, value } of fields) {
      if (value.length !== 32) {
        throw new Error(`${name} must be 32 bytes`);
      }
    }

    const tx = new Transaction();

    // tx.add(await this.program.methods.submitDecryptionShare(
    //   params.committeeIndex,
    //   Array.from(params.partialFor),
    //   Array.from(params.partialAgainst),
    //   Array.from(params.partialAbstain),
    //   Array.from(params.dleqChallenge),
    //   Array.from(params.dleqResponse),
    // )...);

    return tx;
  }

  /**
   * Build aggregate revealed votes transaction (permissionless)
   *
   * Anyone can submit the aggregated tally since the on-chain program
   * cryptographically verifies: tally * H == C_sum - D for each category.
   * This prevents fabrication of results.
   *
   * Use the `zk-voting-sdk` to compute the tally off-chain:
   * ```rust
   * let lagrange = compute_lagrange_coefficients(&selected_indices);
   * let d = combine_partials(&lagrange, &partials);
   * let tally = recover_tally(&d, &accumulated_c, max_votes).unwrap();
   * ```
   *
   * @param params - Tally values and Lagrange coefficients
   */
  async buildAggregateRevealedVotesTransaction(params: AggregateRevealedVotesParams): Promise<Transaction> {
    // No wallet check needed - this is permissionless

    // Validate Lagrange coefficients
    for (let i = 0; i < params.lagrangeCoefficients.length; i++) {
      if (params.lagrangeCoefficients[i].length !== 32) {
        throw new Error(`Lagrange coefficient ${i} must be 32 bytes`);
      }
    }

    const tx = new Transaction();

    // tx.add(await this.program.methods.aggregateRevealedVotes(
    //   params.tallyFor,
    //   params.tallyAgainst,
    //   params.tallyAbstain,
    //   params.lagrangeCoefficients.map(c => Array.from(c)),
    // )...);

    return tx;
  }

  /**
   * Check if authority transfer timelock has elapsed (H-02)
   */
  async canAcceptAuthority(): Promise<{ canAccept: boolean; reason?: string }> {
    const config = await this.getConfig();
    if (!config) {
      return { canAccept: false, reason: "Config not found" };
    }

    if (!config.pendingAuthorityActivatedAt || config.pendingAuthorityActivatedAt.isZero()) {
      return { canAccept: false, reason: "No pending authority transfer" };
    }

    const now = Math.floor(Date.now() / 1000);
    const timelockEnd = config.pendingAuthorityActivatedAt.toNumber() + GOVERNANCE_CONSTANTS.authorityTransferTimelock;

    if (now < timelockEnd) {
      const remaining = timelockEnd - now;
      const hours = Math.ceil(remaining / 3600);
      return { canAccept: false, reason: `Timelock: ${hours} hours remaining` };
    }

    return { canAccept: true };
  }
}

export { GOVERNANCE_CONSTANTS, VoteChoice };
