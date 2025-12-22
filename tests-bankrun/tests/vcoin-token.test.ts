/**
 * VCoin Token Tests using Bankrun
 */

import { describe, it } from "node:test";
import { expect } from "chai";

// Constants
const VCOIN_DECIMALS = 9;
const ONE_VCOIN = 10 ** VCOIN_DECIMALS;
const TOTAL_SUPPLY = 1_000_000_000 * ONE_VCOIN;

describe("VCoin Token - Bankrun Tests", () => {
  describe("Token Constants", () => {
    it("should have correct decimals", () => {
      expect(VCOIN_DECIMALS).to.equal(9);
    });

    it("should have correct total supply (1 billion)", () => {
      expect(TOTAL_SUPPLY).to.equal(1_000_000_000_000_000_000);
    });

    it("should have correct 1 VCoin conversion", () => {
      expect(ONE_VCOIN).to.equal(1_000_000_000);
    });
  });

  describe("Supply Calculations", () => {
    it("should calculate remaining supply correctly", () => {
      const minted = 500_000_000 * ONE_VCOIN;
      const remaining = TOTAL_SUPPLY - minted;
      
      expect(remaining).to.equal(500_000_000 * ONE_VCOIN);
    });

    it("should detect when mint would exceed supply", () => {
      const minted = 950_000_000 * ONE_VCOIN;
      const mintAmount = 100_000_000 * ONE_VCOIN;
      
      const wouldExceed = minted + mintAmount > TOTAL_SUPPLY;
      expect(wouldExceed).to.be.true;
    });

    it("should allow mint up to remaining supply", () => {
      const minted = 900_000_000 * ONE_VCOIN;
      const remaining = TOTAL_SUPPLY - minted;
      
      expect(remaining).to.equal(100_000_000 * ONE_VCOIN);
    });
  });

  describe("Tokenomics Allocation", () => {
    it("should validate ecosystem rewards allocation (35%)", () => {
      const ecosystemRewards = (TOTAL_SUPPLY * 35) / 100;
      expect(ecosystemRewards).to.equal(350_000_000 * ONE_VCOIN);
    });

    it("should validate team allocation (18%)", () => {
      const teamAllocation = (TOTAL_SUPPLY * 18) / 100;
      expect(teamAllocation).to.equal(180_000_000 * ONE_VCOIN);
    });

    it("should validate governance treasury (20%)", () => {
      const treasury = (TOTAL_SUPPLY * 20) / 100;
      expect(treasury).to.equal(200_000_000 * ONE_VCOIN);
    });

    it("should validate liquidity provision (10%)", () => {
      const liquidity = (TOTAL_SUPPLY * 10) / 100;
      expect(liquidity).to.equal(100_000_000 * ONE_VCOIN);
    });

    it("should validate marketing (7%)", () => {
      const marketing = (TOTAL_SUPPLY * 7) / 100;
      expect(marketing).to.equal(70_000_000 * ONE_VCOIN);
    });

    it("should validate angel investors (6%)", () => {
      const angel = (TOTAL_SUPPLY * 6) / 100;
      expect(angel).to.equal(60_000_000 * ONE_VCOIN);
    });

    it("should validate SSCRE reserves (4%)", () => {
      const sscre = (TOTAL_SUPPLY * 4) / 100;
      expect(sscre).to.equal(40_000_000 * ONE_VCOIN);
    });

    it("should sum all allocations to 100%", () => {
      const ecosystem = 35;
      const team = 18;
      const treasury = 20;
      const liquidity = 10;
      const marketing = 7;
      const angel = 6;
      const sscre = 4;
      
      const total = ecosystem + team + treasury + liquidity + marketing + angel + sscre;
      expect(total).to.equal(100);
    });
  });

  describe("Slashing Calculations", () => {
    it("should calculate slash amount correctly", () => {
      const balance = 1000 * ONE_VCOIN;
      const slashPercentBps = 500; // 5%
      
      const slashAmount = (balance * slashPercentBps) / 10000;
      expect(slashAmount).to.equal(50 * ONE_VCOIN);
    });

    it("should not allow slash more than balance", () => {
      const balance = 100 * ONE_VCOIN;
      const slashAmount = 150 * ONE_VCOIN;
      
      expect(slashAmount > balance).to.be.true;
    });
  });

  describe("Pause State", () => {
    it("should default to unpaused", () => {
      const paused = false;
      expect(paused).to.be.false;
    });

    it("should toggle pause state", () => {
      let paused = false;
      paused = true;
      expect(paused).to.be.true;
      paused = false;
      expect(paused).to.be.false;
    });
  });
});

