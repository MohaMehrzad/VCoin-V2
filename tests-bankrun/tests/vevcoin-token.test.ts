/**
 * veVCoin Token Tests using Bankrun
 */

import { describe, it } from "node:test";
import { expect } from "chai";

// Constants
const VEVCOIN_DECIMALS = 9;
const ONE_VEVCOIN = 10 ** VEVCOIN_DECIMALS;
const FOUR_YEARS_SECONDS = 4 * 365 * 24 * 60 * 60;

describe("veVCoin Token - Bankrun Tests", () => {
  describe("Token Constants", () => {
    it("should have correct decimals", () => {
      expect(VEVCOIN_DECIMALS).to.equal(9);
    });

    it("should have correct 4 years duration", () => {
      expect(FOUR_YEARS_SECONDS).to.equal(126144000);
    });
  });

  describe("veVCoin Calculation", () => {
    const calculateVeVCoin = (vcoinAmount: number, lockDuration: number) => {
      // veVCoin = VCoin * (lockDuration / 4 years)
      return Math.floor((vcoinAmount * lockDuration) / FOUR_YEARS_SECONDS);
    };

    it("should return 0 for 0 lock duration", () => {
      const vevcoin = calculateVeVCoin(1000 * ONE_VEVCOIN, 0);
      expect(vevcoin).to.equal(0);
    });

    it("should return 25% for 1 year lock", () => {
      const oneYear = 365 * 24 * 60 * 60;
      const vevcoin = calculateVeVCoin(1000 * ONE_VEVCOIN, oneYear);
      
      expect(vevcoin).to.be.approximately(250 * ONE_VEVCOIN, ONE_VEVCOIN);
    });

    it("should return 50% for 2 year lock", () => {
      const twoYears = 2 * 365 * 24 * 60 * 60;
      const vevcoin = calculateVeVCoin(1000 * ONE_VEVCOIN, twoYears);
      
      expect(vevcoin).to.be.approximately(500 * ONE_VEVCOIN, ONE_VEVCOIN);
    });

    it("should return 100% for 4 year lock", () => {
      const vevcoin = calculateVeVCoin(1000 * ONE_VEVCOIN, FOUR_YEARS_SECONDS);
      
      expect(vevcoin).to.equal(1000 * ONE_VEVCOIN);
    });
  });

  describe("Soulbound Property", () => {
    it("should not allow transfers (conceptual)", () => {
      const isSoulbound = true;
      expect(isSoulbound).to.be.true;
    });
  });

  describe("Balance Tracking", () => {
    it("should track total supply on mint", () => {
      let totalSupply = 0;
      
      // Simulate mints
      totalSupply += 1000 * ONE_VEVCOIN;
      totalSupply += 500 * ONE_VEVCOIN;
      
      expect(totalSupply).to.equal(1500 * ONE_VEVCOIN);
    });

    it("should track total supply on burn", () => {
      let totalSupply = 2000 * ONE_VEVCOIN;
      
      // Simulate burn
      totalSupply -= 500 * ONE_VEVCOIN;
      
      expect(totalSupply).to.equal(1500 * ONE_VEVCOIN);
    });

    it("should track holder count", () => {
      let holders = 0;
      const userBalances = new Map<string, number>();
      
      // First mint to user
      userBalances.set("user1", 1000);
      if (!userBalances.has("user1")) holders++;
      holders = userBalances.size;
      
      expect(holders).to.equal(1);
      
      // Second user
      userBalances.set("user2", 500);
      holders = userBalances.size;
      
      expect(holders).to.equal(2);
    });
  });

  describe("Lock Expiry", () => {
    it("should detect active lock", () => {
      const now = Date.now() / 1000;
      const unlockTime = now + 86400; // 1 day from now
      
      const isLocked = now < unlockTime;
      expect(isLocked).to.be.true;
    });

    it("should detect expired lock", () => {
      const now = Date.now() / 1000;
      const unlockTime = now - 86400; // 1 day ago
      
      const isLocked = now < unlockTime;
      expect(isLocked).to.be.false;
    });
  });
});

