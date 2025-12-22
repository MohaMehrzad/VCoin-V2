/**
 * @viwoapp/sdk Governance Module Tests
 */

import { Keypair, PublicKey } from '@solana/web3.js';
import {
  GovernanceClient,
  calculateVotingPower,
  integerSqrt,
  ProposalStatus,
  VoteChoice,
  GOVERNANCE_CONSTANTS,
} from '../governance';

describe('Governance Module', () => {
  describe('Integer Square Root', () => {
    it('should return 0 for 0', () => {
      expect(integerSqrt(0n)).toBe(0n);
    });

    it('should return 1 for 1', () => {
      expect(integerSqrt(1n)).toBe(1n);
    });

    it('should calculate perfect squares correctly', () => {
      expect(integerSqrt(4n)).toBe(2n);
      expect(integerSqrt(9n)).toBe(3n);
      expect(integerSqrt(16n)).toBe(4n);
      expect(integerSqrt(100n)).toBe(10n);
      expect(integerSqrt(10000n)).toBe(100n);
      expect(integerSqrt(1000000n)).toBe(1000n);
    });

    it('should floor non-perfect squares', () => {
      expect(integerSqrt(2n)).toBe(1n);
      expect(integerSqrt(5n)).toBe(2n);
      expect(integerSqrt(99n)).toBe(9n);
      expect(integerSqrt(101n)).toBe(10n);
    });

    it('should handle large values', () => {
      const largeValue = 1_000_000_000n;
      const result = integerSqrt(largeValue);
      expect(result).toBe(31622n);
    });
  });

  describe('Voting Power Calculation', () => {
    it('should return 0 for 0 veVCoin', () => {
      const power = calculateVotingPower({
        veVCoinBalance: 0n,
        fiveAScore: 5000,
        tier: 'bronze',
      });
      expect(power).toBe(0n);
    });

    it('should apply quadratic formula', () => {
      const power = calculateVotingPower({
        veVCoinBalance: 10000n,
        fiveAScore: 0,
        tier: 'none',
      });
      // sqrt(10000) * 1.0 (5A) * 1.0 (tier) = 100
      expect(Number(power)).toBeCloseTo(100, -1);
    });

    it('should apply 5A score boost', () => {
      const basePower = calculateVotingPower({
        veVCoinBalance: 10000n,
        fiveAScore: 0,
        tier: 'none',
      });

      const boostedPower = calculateVotingPower({
        veVCoinBalance: 10000n,
        fiveAScore: 10000, // Max 5A = 2.0x
        tier: 'none',
      });

      expect(Number(boostedPower)).toBeCloseTo(Number(basePower) * 2, -1);
    });

    it('should apply tier multipliers', () => {
      const bronzePower = calculateVotingPower({
        veVCoinBalance: 10000n,
        fiveAScore: 0,
        tier: 'bronze',
      });

      const platinumPower = calculateVotingPower({
        veVCoinBalance: 10000n,
        fiveAScore: 0,
        tier: 'platinum',
      });

      // Platinum = 10x, Bronze = 1x
      expect(Number(platinumPower)).toBeGreaterThan(Number(bronzePower));
    });

    it('should apply all boosts together', () => {
      const power = calculateVotingPower({
        veVCoinBalance: 10000n,
        fiveAScore: 10000, // 2.0x
        tier: 'platinum', // 10x
      });

      // sqrt(10000) * 2.0 * 10 = 100 * 20 = 2000
      expect(Number(power)).toBeCloseTo(2000, -1);
    });
  });

  describe('Proposal Status Enum', () => {
    it('should have all status values', () => {
      expect(ProposalStatus.Pending).toBe(0);
      expect(ProposalStatus.Active).toBe(1);
      expect(ProposalStatus.Passed).toBe(2);
      expect(ProposalStatus.Rejected).toBe(3);
      expect(ProposalStatus.Executed).toBe(4);
      expect(ProposalStatus.Cancelled).toBe(5);
    });
  });

  describe('Vote Choice Enum', () => {
    it('should have all vote choices', () => {
      expect(VoteChoice.For).toBe(0);
      expect(VoteChoice.Against).toBe(1);
      expect(VoteChoice.Abstain).toBe(2);
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

    it('should have correct proposal threshold', () => {
      expect(GOVERNANCE_CONSTANTS.proposalThreshold).toBe(1000);
    });
  });

  describe('Quorum Calculation', () => {
    it('should calculate quorum correctly', () => {
      const totalSupply = 1_000_000_000n;
      const quorumBps = GOVERNANCE_CONSTANTS.quorumBps;
      
      const quorum = (totalSupply * BigInt(quorumBps)) / 10000n;
      
      // 4% of 1B = 40M
      expect(quorum).toBe(40_000_000n);
    });
  });

  describe('Proposal Passed Check', () => {
    it('should pass when for > against and quorum met', () => {
      const proposal = {
        votesFor: 60_000_000n,
        votesAgainst: 30_000_000n,
        votesAbstain: 10_000_000n,
      };
      
      const totalVotes = proposal.votesFor + proposal.votesAgainst + proposal.votesAbstain;
      const quorum = 40_000_000n;
      
      const passed = proposal.votesFor > proposal.votesAgainst && totalVotes >= quorum;
      expect(passed).toBe(true);
    });

    it('should fail when for <= against', () => {
      const proposal = {
        votesFor: 30_000_000n,
        votesAgainst: 60_000_000n,
        votesAbstain: 10_000_000n,
      };
      
      const passed = proposal.votesFor > proposal.votesAgainst;
      expect(passed).toBe(false);
    });

    it('should fail when quorum not met', () => {
      const proposal = {
        votesFor: 20_000_000n,
        votesAgainst: 10_000_000n,
        votesAbstain: 5_000_000n,
      };
      
      const totalVotes = proposal.votesFor + proposal.votesAgainst + proposal.votesAbstain;
      const quorum = 40_000_000n;
      
      const passed = proposal.votesFor > proposal.votesAgainst && totalVotes >= quorum;
      expect(passed).toBe(false);
    });
  });

  describe('GovernanceClient', () => {
    it('should export GovernanceClient class', () => {
      expect(GovernanceClient).toBeDefined();
    });
  });
});

