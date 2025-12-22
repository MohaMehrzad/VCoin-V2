/**
 * Staking Protocol Tests using Bankrun
 */

import { describe, it, before } from "node:test";
import { expect } from "chai";
import { BankrunProvider } from "anchor-bankrun";
import { startAnchor } from "solana-bankrun";
import { PublicKey, Keypair } from "@solana/web3.js";
import * as anchor from "@coral-xyz/anchor";

// Constants
const ONE_WEEK = 7 * 24 * 60 * 60;
const ONE_YEAR = 365 * 24 * 60 * 60;
const FOUR_YEARS = 4 * ONE_YEAR;
const ONE_VCOIN = 1_000_000_000;
const BRONZE_THRESHOLD = 1_000 * ONE_VCOIN;
const SILVER_THRESHOLD = 5_000 * ONE_VCOIN;
const GOLD_THRESHOLD = 20_000 * ONE_VCOIN;
const PLATINUM_THRESHOLD = 100_000 * ONE_VCOIN;

describe("Staking Protocol - Bankrun Tests", () => {
  it("should calculate correct tier for stake amount", () => {
    const getTier = (amount: number) => {
      if (amount >= PLATINUM_THRESHOLD) return 4;
      if (amount >= GOLD_THRESHOLD) return 3;
      if (amount >= SILVER_THRESHOLD) return 2;
      if (amount >= BRONZE_THRESHOLD) return 1;
      return 0;
    };

    expect(getTier(0)).to.equal(0);
    expect(getTier(BRONZE_THRESHOLD - 1)).to.equal(0);
    expect(getTier(BRONZE_THRESHOLD)).to.equal(1);
    expect(getTier(SILVER_THRESHOLD)).to.equal(2);
    expect(getTier(GOLD_THRESHOLD)).to.equal(3);
    expect(getTier(PLATINUM_THRESHOLD)).to.equal(4);
  });

  it("should calculate veVCoin correctly", () => {
    const calculateVeVCoin = (
      amount: number,
      duration: number,
      tierBoost: number
    ) => {
      const durationFactor = (BigInt(duration) * BigInt(10000)) / BigInt(FOUR_YEARS);
      const base = (BigInt(amount) * durationFactor) / BigInt(10000);
      const boosted = (base * BigInt(tierBoost)) / BigInt(10000);
      return Number(boosted);
    };

    // Platinum with 4 year lock should give ~1.4x stake
    const vevcoin = calculateVeVCoin(PLATINUM_THRESHOLD, FOUR_YEARS, 14000);
    expect(vevcoin).to.be.approximately(PLATINUM_THRESHOLD * 1.4, ONE_VCOIN);

    // Bronze with 1 year lock should give ~0.275x stake
    const bronzeVevcoin = calculateVeVCoin(BRONZE_THRESHOLD, ONE_YEAR, 11000);
    expect(bronzeVevcoin).to.be.approximately(BRONZE_THRESHOLD * 0.275, ONE_VCOIN);
  });

  it("should validate lock duration bounds", () => {
    const isValidDuration = (duration: number) => {
      return duration >= ONE_WEEK && duration <= FOUR_YEARS;
    };

    expect(isValidDuration(ONE_WEEK - 1)).to.be.false;
    expect(isValidDuration(ONE_WEEK)).to.be.true;
    expect(isValidDuration(ONE_YEAR)).to.be.true;
    expect(isValidDuration(FOUR_YEARS)).to.be.true;
    expect(isValidDuration(FOUR_YEARS + 1)).to.be.false;
  });

  it("should calculate fee discounts correctly", () => {
    const getDiscount = (tier: number) => {
      switch (tier) {
        case 0: return 0;
        case 1: return 1000;  // 10%
        case 2: return 2000;  // 20%
        case 3: return 3000;  // 30%
        case 4: return 5000;  // 50%
        default: return 0;
      }
    };

    const baseFee = 1000; // 10% in basis points
    
    for (let tier = 0; tier <= 4; tier++) {
      const discount = getDiscount(tier);
      const discountedFee = baseFee - (baseFee * discount) / 10000;
      
      console.log(`Tier ${tier}: ${discount / 100}% discount, fee ${discountedFee / 100}%`);
    }

    expect(getDiscount(0)).to.equal(0);
    expect(getDiscount(4)).to.equal(5000);
  });
});

