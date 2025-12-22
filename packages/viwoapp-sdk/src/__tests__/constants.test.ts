/**
 * @viwoapp/sdk Constants Tests
 */

import {
  PROGRAM_IDS,
  SEEDS,
  VCOIN_DECIMALS,
  VEVCOIN_DECIMALS,
  VCOIN_TOTAL_SUPPLY,
  STAKING_TIERS,
  LOCK_DURATIONS,
  SSCRE_CONSTANTS,
  VILINK_CONSTANTS,
  GASLESS_CONSTANTS,
  FIVE_A_CONSTANTS,
  CONTENT_CONSTANTS,
  GOVERNANCE_CONSTANTS,
  ACTION_SCOPES,
} from '../constants';
import { PublicKey } from '@solana/web3.js';

describe('SDK Constants', () => {
  describe('Program IDs', () => {
    it('should have valid program ID for vcoinToken', () => {
      expect(PROGRAM_IDS.vcoinToken).toBeInstanceOf(PublicKey);
      expect(PROGRAM_IDS.vcoinToken.toBase58()).toBe('Gg1dtrjAfGYi6NLC31WaJjZNBoucvD98rK2h1u9qrUjn');
    });

    it('should have valid program ID for vevcoinToken', () => {
      expect(PROGRAM_IDS.vevcoinToken).toBeInstanceOf(PublicKey);
      expect(PROGRAM_IDS.vevcoinToken.toBase58()).toBe('FB39ae9x53FxVL3pER9LqCPEx2TRnEnQP55i838Upnjx');
    });

    it('should have valid program ID for stakingProtocol', () => {
      expect(PROGRAM_IDS.stakingProtocol).toBeInstanceOf(PublicKey);
      expect(PROGRAM_IDS.stakingProtocol.toBase58()).toBe('6EFcistyr2E81adLUcuBJRr8W2xzpt3D3dFYEcMewpWu');
    });

    it('should have all 11 program IDs defined', () => {
      const programKeys = Object.keys(PROGRAM_IDS);
      expect(programKeys.length).toBe(11);
    });
  });

  describe('PDA Seeds', () => {
    it('should have vcoin-config seed', () => {
      expect(SEEDS.vcoinConfig).toBe('vcoin-config');
    });

    it('should have staking pool seed', () => {
      expect(SEEDS.stakingPool).toBe('staking-pool');
    });

    it('should have user stake seed', () => {
      expect(SEEDS.userStake).toBe('user-stake');
    });

    it('should have governance seeds', () => {
      expect(SEEDS.governanceConfig).toBe('governance-config');
      expect(SEEDS.proposal).toBe('proposal');
      expect(SEEDS.voteRecord).toBe('vote');
    });

    it('should have all required seeds', () => {
      const seedKeys = Object.keys(SEEDS);
      expect(seedKeys.length).toBeGreaterThan(15);
    });
  });

  describe('Token Constants', () => {
    it('should have correct VCoin decimals', () => {
      expect(VCOIN_DECIMALS).toBe(9);
    });

    it('should have correct veVCoin decimals', () => {
      expect(VEVCOIN_DECIMALS).toBe(9);
    });

    it('should have correct total supply (1 billion)', () => {
      expect(VCOIN_TOTAL_SUPPLY).toBe(1_000_000_000);
    });
  });

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
  });

  describe('Lock Durations', () => {
    it('should have correct durations', () => {
      expect(LOCK_DURATIONS.none).toBe(0);
      expect(LOCK_DURATIONS.oneMonth).toBe(30 * 24 * 3600);
      expect(LOCK_DURATIONS.threeMonths).toBe(90 * 24 * 3600);
      expect(LOCK_DURATIONS.sixMonths).toBe(180 * 24 * 3600);
      expect(LOCK_DURATIONS.oneYear).toBe(365 * 24 * 3600);
    });
  });

  describe('SSCRE Constants', () => {
    it('should have correct pool reserves', () => {
      expect(SSCRE_CONSTANTS.primaryReserves).toBe(350_000_000);
      expect(SSCRE_CONSTANTS.secondaryReserves).toBe(40_000_000);
    });

    it('should have correct epoch duration', () => {
      expect(SSCRE_CONSTANTS.epochDuration).toBe(30 * 24 * 3600);
    });

    it('should have correct claim window', () => {
      expect(SSCRE_CONSTANTS.claimWindow).toBe(90 * 24 * 3600);
    });

    it('should have correct gasless fee', () => {
      expect(SSCRE_CONSTANTS.gaslessFeeBps).toBe(100); // 1%
    });
  });

  describe('ViLink Constants', () => {
    it('should have correct tip limits', () => {
      expect(VILINK_CONSTANTS.minTipAmount).toBe(0.1);
      expect(VILINK_CONSTANTS.maxTipAmount).toBe(10_000);
    });

    it('should have correct platform fee', () => {
      expect(VILINK_CONSTANTS.platformFeeBps).toBe(250); // 2.5%
    });

    it('should have correct action expiry', () => {
      expect(VILINK_CONSTANTS.maxActionExpiry).toBe(7 * 24 * 3600);
    });
  });

  describe('Gasless Constants', () => {
    it('should have correct session duration', () => {
      expect(GASLESS_CONSTANTS.sessionDuration).toBe(24 * 3600);
    });

    it('should have correct session limits', () => {
      expect(GASLESS_CONSTANTS.maxSessionActions).toBe(1000);
      expect(GASLESS_CONSTANTS.maxSessionSpend).toBe(100_000);
    });

    it('should have correct fee config', () => {
      expect(GASLESS_CONSTANTS.sscreDeductionBps).toBe(100); // 1%
    });
  });

  describe('5A Protocol Constants', () => {
    it('should have correct max score', () => {
      expect(FIVE_A_CONSTANTS.maxScore).toBe(10000);
    });

    it('should have weights that sum to 100', () => {
      const { scoreWeights } = FIVE_A_CONSTANTS;
      const total = scoreWeights.authenticity + scoreWeights.accuracy + 
                   scoreWeights.agility + scoreWeights.activity + scoreWeights.approved;
      expect(total).toBe(100);
    });

    it('should have correct individual weights', () => {
      expect(FIVE_A_CONSTANTS.scoreWeights.authenticity).toBe(25);
      expect(FIVE_A_CONSTANTS.scoreWeights.accuracy).toBe(20);
      expect(FIVE_A_CONSTANTS.scoreWeights.agility).toBe(15);
      expect(FIVE_A_CONSTANTS.scoreWeights.activity).toBe(25);
      expect(FIVE_A_CONSTANTS.scoreWeights.approved).toBe(15);
    });
  });

  describe('Content Constants', () => {
    it('should have correct energy values', () => {
      expect(CONTENT_CONSTANTS.maxEnergy).toBe(100);
      expect(CONTENT_CONSTANTS.energyRegenRate).toBe(10);
    });

    it('should have correct action costs', () => {
      expect(CONTENT_CONSTANTS.createCost).toBe(10);
      expect(CONTENT_CONSTANTS.editCost).toBe(5);
      expect(CONTENT_CONSTANTS.deleteCost).toBe(0);
    });
  });

  describe('Governance Constants', () => {
    it('should have correct voting duration', () => {
      expect(GOVERNANCE_CONSTANTS.votingDuration).toBe(7 * 24 * 3600);
    });

    it('should have correct execution delay', () => {
      expect(GOVERNANCE_CONSTANTS.executionDelay).toBe(2 * 24 * 3600);
    });

    it('should have correct quorum', () => {
      expect(GOVERNANCE_CONSTANTS.quorumBps).toBe(400); // 4%
    });
  });

  describe('Action Scopes', () => {
    it('should have correct scope bits', () => {
      expect(ACTION_SCOPES.tip).toBe(1 << 0);
      expect(ACTION_SCOPES.vouch).toBe(1 << 1);
      expect(ACTION_SCOPES.content).toBe(1 << 2);
      expect(ACTION_SCOPES.governance).toBe(1 << 3);
    });

    it('should have all scope', () => {
      expect(ACTION_SCOPES.all).toBe(0xFFFF);
    });
  });
});

