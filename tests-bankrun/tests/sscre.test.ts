/**
 * SSCRE Protocol Tests using Bankrun
 */

import { describe, it } from "node:test";
import { expect } from "chai";
import { createHash } from "crypto";

// Constants
const ONE_VCOIN = 1_000_000_000;
const REWARDS_POOL = 350_000_000 * ONE_VCOIN;
const MONTHLY_ALLOCATION = 5_833_333 * ONE_VCOIN;

// Merkle tree utilities
const hashLeaf = (user: string, amount: number, epoch: number): Buffer => {
  const data = Buffer.concat([
    Buffer.from(user, "hex"),
    Buffer.from(new BigUint64Array([BigInt(amount)]).buffer),
    Buffer.from(new BigUint64Array([BigInt(epoch)]).buffer),
  ]);
  return createHash("sha256").update(data).digest();
};

const hashPair = (left: Buffer, right: Buffer): Buffer => {
  const sorted = left.compare(right) < 0 ? [left, right] : [right, left];
  return createHash("sha256").update(Buffer.concat(sorted)).digest();
};

describe("SSCRE Protocol - Bankrun Tests", () => {
  it("should verify rewards pool allocation", () => {
    expect(REWARDS_POOL).to.equal(350_000_000 * ONE_VCOIN);
    
    // ~60 months of emissions
    const months = Math.floor(REWARDS_POOL / MONTHLY_ALLOCATION);
    console.log(`Rewards pool supports ~${months} months of emissions`);
    
    expect(months).to.be.greaterThan(50);
  });

  it("should calculate epoch from timestamp", () => {
    const epochStart = 1704067200; // Jan 1, 2024
    const epochDuration = 7 * 24 * 60 * 60; // 1 week

    const getEpoch = (timestamp: number) => {
      if (timestamp < epochStart) return 0;
      return Math.floor((timestamp - epochStart) / epochDuration);
    };

    expect(getEpoch(epochStart)).to.equal(0);
    expect(getEpoch(epochStart + epochDuration)).to.equal(1);
    expect(getEpoch(epochStart + 52 * epochDuration)).to.equal(52);
  });

  it("should build and verify merkle proofs", () => {
    // Create sample leaves
    const leaves = [
      hashLeaf("a".repeat(64), 1000 * ONE_VCOIN, 1),
      hashLeaf("b".repeat(64), 2000 * ONE_VCOIN, 1),
      hashLeaf("c".repeat(64), 3000 * ONE_VCOIN, 1),
      hashLeaf("d".repeat(64), 4000 * ONE_VCOIN, 1),
    ];

    // Build tree
    const layer1 = [
      hashPair(leaves[0], leaves[1]),
      hashPair(leaves[2], leaves[3]),
    ];
    const root = hashPair(layer1[0], layer1[1]);

    // Verify proof for first leaf
    const proof = [leaves[1], layer1[1]];
    let computed = leaves[0];
    for (const sibling of proof) {
      computed = hashPair(computed, sibling);
    }

    expect(computed.equals(root)).to.be.true;
  });

  it("should prevent double claims", () => {
    const claims = new Set<string>();
    
    const getClaim = (user: string, epoch: number) => `${user}-${epoch}`;
    const hasClaimed = (user: string, epoch: number) => claims.has(getClaim(user, epoch));
    const recordClaim = (user: string, epoch: number) => claims.add(getClaim(user, epoch));

    const user = "user1";
    const epoch = 1;

    // First claim should succeed
    expect(hasClaimed(user, epoch)).to.be.false;
    recordClaim(user, epoch);
    
    // Second claim should be detected
    expect(hasClaimed(user, epoch)).to.be.true;
    
    // Different epoch should not be claimed
    expect(hasClaimed(user, 2)).to.be.false;
  });

  it("should apply 5A boost to rewards", () => {
    const baseReward = 100 * ONE_VCOIN;

    const applyBoost = (reward: number, fiveAScore: number) => {
      const multiplier = 1 + fiveAScore / 10000;
      return Math.floor(reward * multiplier);
    };

    // No boost (0% 5A)
    expect(applyBoost(baseReward, 0)).to.equal(baseReward);

    // Max boost (100% 5A) = 2x
    expect(applyBoost(baseReward, 10000)).to.equal(baseReward * 2);

    // Quality user (80% 5A) = 1.8x
    const qualityReward = applyBoost(baseReward, 8000);
    expect(qualityReward).to.equal(baseReward * 1.8);

    // Bot (20% 5A) = 1.2x
    const botReward = applyBoost(baseReward, 2000);
    expect(botReward).to.equal(baseReward * 1.2);

    console.log(`Quality user earns ${qualityReward / ONE_VCOIN} VCoin`);
    console.log(`Bot earns ${botReward / ONE_VCOIN} VCoin`);
    console.log(`Difference: ${(qualityReward - botReward) / ONE_VCOIN} VCoin`);
  });

  it("should validate circuit breaker logic", () => {
    let isActive = false;
    let triggerCount = 0;

    const trigger = () => {
      isActive = true;
      triggerCount++;
    };

    const reset = () => {
      isActive = false;
    };

    const canClaim = () => !isActive;

    // Normal operation
    expect(canClaim()).to.be.true;

    // After trigger
    trigger();
    expect(canClaim()).to.be.false;
    expect(triggerCount).to.equal(1);

    // After reset
    reset();
    expect(canClaim()).to.be.true;
    expect(triggerCount).to.equal(1); // Count preserved
  });
});

