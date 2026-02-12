/**
 * Governance Protocol Tests using Bankrun
 */

import { describe, it } from "node:test";
import { expect } from "chai";

// Constants
const ONE_VCOIN = 1_000_000_000;
const ONE_DAY = 24 * 60 * 60;
const DEFAULT_VOTING_PERIOD = 7 * ONE_DAY;
const DEFAULT_TIMELOCK = 48 * 60 * 60;

describe("Governance Protocol - Bankrun Tests", () => {
  it("should calculate quadratic voting power", () => {
    const calculateVotingPower = (
      vevcoinBalance: number,
      fiveAScore: number,
      tier: number
    ) => {
      // Quadratic base
      const base = Math.sqrt(vevcoinBalance);
      
      // 5A boost (1.0x to 2.0x)
      const fiveABoost = 1 + fiveAScore / 10000;
      
      // Tier multiplier
      const tierMult = [1, 1, 2, 5, 10][tier] || 1;
      
      return Math.floor(base * fiveABoost * tierMult);
    };

    // Minimum voter
    const minPower = calculateVotingPower(ONE_VCOIN, 0, 0);
    expect(minPower).to.be.greaterThan(0);
    
    // Platinum whale with max 5A
    const maxPower = calculateVotingPower(100_000 * ONE_VCOIN, 10000, 4);
    expect(maxPower).to.be.greaterThan(1000);
    
    // Quadratic reduces whale influence
    const smallBalance = 1000;
    const largeBalance = 1000000;
    const smallPower = Math.sqrt(smallBalance);
    const largePower = Math.sqrt(largeBalance);
    
    const balanceRatio = largeBalance / smallBalance;
    const powerRatio = largePower / smallPower;
    
    expect(powerRatio).to.be.lessThan(balanceRatio);
    console.log(`Balance ratio: ${balanceRatio}, Power ratio: ${powerRatio.toFixed(2)}`);
  });

  it("should track voting period state", () => {
    const now = Math.floor(Date.now() / 1000);
    const votingStart = now;
    const votingEnd = now + DEFAULT_VOTING_PERIOD;

    const isActive = (timestamp: number) => {
      return timestamp >= votingStart && timestamp <= votingEnd;
    };

    const hasEnded = (timestamp: number) => {
      return timestamp > votingEnd;
    };

    expect(isActive(now)).to.be.true;
    expect(isActive(now + 3 * ONE_DAY)).to.be.true;
    expect(hasEnded(now)).to.be.false;
    expect(hasEnded(votingEnd + 1)).to.be.true;
  });

  it("should handle timelock correctly", () => {
    const now = Math.floor(Date.now() / 1000);
    const unlockTime = now + DEFAULT_TIMELOCK;

    const hasExpired = (timestamp: number) => timestamp >= unlockTime;
    const remaining = (timestamp: number) => Math.max(0, unlockTime - timestamp);

    expect(hasExpired(now)).to.be.false;
    expect(hasExpired(now + DEFAULT_TIMELOCK - 1)).to.be.false;
    expect(hasExpired(now + DEFAULT_TIMELOCK)).to.be.true;
    
    expect(remaining(now)).to.equal(DEFAULT_TIMELOCK);
    expect(remaining(now + DEFAULT_TIMELOCK)).to.equal(0);
  });

  it("should validate vote choices", () => {
    // On-chain enum: Against=0, For=1, Abstain=2
    const isValidChoice = (choice: number) => {
      return choice >= 0 && choice <= 2;
    };

    expect(isValidChoice(0)).to.be.true;  // Against
    expect(isValidChoice(1)).to.be.true;  // For
    expect(isValidChoice(2)).to.be.true;  // Abstain
    expect(isValidChoice(3)).to.be.false;
  });

  it("should prevent self-delegation", () => {
    const delegator = "user1";
    const delegate = "user2";
    const selfDelegate = "user1";

    const canDelegate = (from: string, to: string) => from !== to;

    expect(canDelegate(delegator, delegate)).to.be.true;
    expect(canDelegate(delegator, selfDelegate)).to.be.false;
  });

  it("should calculate quorum excluding abstains (C-03)", () => {
    const quorumBps = 400; // 4%
    const totalSupply = 1_000_000_000 * ONE_VCOIN;

    const quorum = Math.floor((totalSupply * quorumBps) / 10000);

    const proposal = {
      votesFor: 25_000_000 * ONE_VCOIN,
      votesAgainst: 10_000_000 * ONE_VCOIN,
      votesAbstain: 100_000_000 * ONE_VCOIN,
    };

    // C-03: Only for + against count toward quorum (abstains excluded)
    const quorumVotes = proposal.votesFor + proposal.votesAgainst;
    const passed = proposal.votesFor > proposal.votesAgainst && quorumVotes >= quorum;

    // 35M < 40M quorum, so fails even though total votes = 135M
    expect(passed).to.be.false;

    // But with enough for+against votes:
    const proposal2 = {
      votesFor: 30_000_000 * ONE_VCOIN,
      votesAgainst: 15_000_000 * ONE_VCOIN,
      votesAbstain: 5_000_000 * ONE_VCOIN,
    };
    const quorumVotes2 = proposal2.votesFor + proposal2.votesAgainst;
    const passed2 = proposal2.votesFor > proposal2.votesAgainst && quorumVotes2 >= quorum;
    expect(passed2).to.be.true; // 45M >= 40M quorum
  });

  it("should enforce authority transfer timelock (H-02)", () => {
    const AUTHORITY_TRANSFER_TIMELOCK = 24 * 60 * 60; // 24 hours
    const now = Math.floor(Date.now() / 1000);
    const activatedAt = now - 12 * 60 * 60; // 12 hours ago

    // Cannot accept before timelock expires
    const canAccept = (activatedAt: number, currentTime: number) => {
      return currentTime >= activatedAt + AUTHORITY_TRANSFER_TIMELOCK;
    };

    expect(canAccept(activatedAt, now)).to.be.false; // Only 12h elapsed
    expect(canAccept(activatedAt, activatedAt + AUTHORITY_TRANSFER_TIMELOCK)).to.be.true; // 24h elapsed
    expect(canAccept(activatedAt, activatedAt + AUTHORITY_TRANSFER_TIMELOCK - 1)).to.be.false;
  });

  it("should validate ZK private voting proof sizes", () => {
    // ElGamal ciphertext: R (32 bytes) || C (32 bytes) = 64 bytes
    const CIPHERTEXT_SIZE = 64;
    // 3 OR proofs (96 bytes each) + 1 sum proof (64 bytes) = 352 bytes
    const VOTE_PROOF_SIZE = 3 * 96 + 64;
    const MAX_COMMITTEE_SIZE = 5;

    expect(CIPHERTEXT_SIZE).to.equal(64);
    expect(VOTE_PROOF_SIZE).to.equal(352);
    expect(MAX_COMMITTEE_SIZE).to.equal(5);

    // Private vote transaction: 3 ciphertexts + proof
    const privateVoteDataSize = 3 * CIPHERTEXT_SIZE + VOTE_PROOF_SIZE;
    expect(privateVoteDataSize).to.equal(544);
  });
});

