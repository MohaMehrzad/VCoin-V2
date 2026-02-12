/**
 * @viwoapp/sdk Staking Module Tests
 */

import { Keypair, PublicKey } from '@solana/web3.js';
import { BN } from '@coral-xyz/anchor';
import {
  StakingClient,
  LOCK_DURATIONS,
  STAKING_TIERS,
} from '../staking';
import { StakingTier } from '../types';
import { VCOIN_DECIMALS } from '../constants';

describe('Staking Module', () => {
  describe('Staking Tiers', () => {
    it('should have correct tier thresholds', () => {
      expect(STAKING_TIERS.none.minStake).toBe(0);
      expect(STAKING_TIERS.bronze.minStake).toBe(1_000);
      expect(STAKING_TIERS.silver.minStake).toBe(5_000);
      expect(STAKING_TIERS.gold.minStake).toBe(20_000);
      expect(STAKING_TIERS.platinum.minStake).toBe(100_000);
    });

    it('should have correct fee discounts', () => {
      expect(STAKING_TIERS.none.feeDiscount).toBe(0);
      expect(STAKING_TIERS.bronze.feeDiscount).toBe(10);
      expect(STAKING_TIERS.silver.feeDiscount).toBe(20);
      expect(STAKING_TIERS.gold.feeDiscount).toBe(30);
      expect(STAKING_TIERS.platinum.feeDiscount).toBe(50);
    });

    it('should have correct boost multipliers', () => {
      expect(STAKING_TIERS.none.boost).toBe(1.0);
      expect(STAKING_TIERS.bronze.boost).toBe(1.1);
      expect(STAKING_TIERS.silver.boost).toBe(1.2);
      expect(STAKING_TIERS.gold.boost).toBe(1.3);
      expect(STAKING_TIERS.platinum.boost).toBe(1.4);
    });

    it('should have increasing benefits per tier', () => {
      const tiers = ['none', 'bronze', 'silver', 'gold', 'platinum'] as const;

      let prevDiscount = -1;
      let prevBoost = 0;

      for (const tier of tiers) {
        const info = STAKING_TIERS[tier];
        expect(info.feeDiscount).toBeGreaterThan(prevDiscount);
        expect(info.boost).toBeGreaterThanOrEqual(prevBoost);
        prevDiscount = info.feeDiscount;
        prevBoost = info.boost;
      }
    });
  });

  describe('Lock Duration Constants', () => {
    it('should have correct duration values', () => {
      expect(LOCK_DURATIONS.none).toBe(0);
      expect(LOCK_DURATIONS.oneMonth).toBe(30 * 24 * 3600);
      expect(LOCK_DURATIONS.threeMonths).toBe(90 * 24 * 3600);
      expect(LOCK_DURATIONS.sixMonths).toBe(180 * 24 * 3600);
      expect(LOCK_DURATIONS.oneYear).toBe(365 * 24 * 3600);
    });

    it('should have increasing durations', () => {
      expect(LOCK_DURATIONS.oneMonth).toBeLessThan(LOCK_DURATIONS.threeMonths);
      expect(LOCK_DURATIONS.threeMonths).toBeLessThan(LOCK_DURATIONS.sixMonths);
      expect(LOCK_DURATIONS.sixMonths).toBeLessThan(LOCK_DURATIONS.oneYear);
    });
  });

  describe('StakingTier Enum', () => {
    it('should have correct values', () => {
      expect(StakingTier.None).toBe(0);
      expect(StakingTier.Bronze).toBe(1);
      expect(StakingTier.Silver).toBe(2);
      expect(StakingTier.Gold).toBe(3);
      expect(StakingTier.Platinum).toBe(4);
    });
  });

  describe('M-05: TierUnchanged Error', () => {
    it('should document that no-op tier updates are rejected on-chain', () => {
      // M-05: The on-chain update_tier instruction rejects calls where
      // the new tier equals the old tier (StakingError::TierUnchanged).
      // SDK clients should check tier would change before calling.
      const currentTier = StakingTier.Bronze;
      const stakeAmount = 1500; // Still within Bronze range

      // Simple tier calculation matching on-chain logic
      let newTier: StakingTier;
      if (stakeAmount >= 100_000) newTier = StakingTier.Platinum;
      else if (stakeAmount >= 20_000) newTier = StakingTier.Gold;
      else if (stakeAmount >= 5_000) newTier = StakingTier.Silver;
      else if (stakeAmount >= 1_000) newTier = StakingTier.Bronze;
      else newTier = StakingTier.None;

      // Would result in TierUnchanged error on-chain
      expect(newTier).toBe(currentTier);
    });
  });

  describe('StakingClient', () => {
    it('should export StakingClient class', () => {
      expect(StakingClient).toBeDefined();
    });
  });
});
