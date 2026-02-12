/**
 * @viwoapp/sdk Governance Module Tests
 */

import { Keypair, PublicKey } from '@solana/web3.js';
import { BN } from '@coral-xyz/anchor';
import {
  GovernanceClient,
  GOVERNANCE_CONSTANTS,
  VoteChoice,
} from '../governance';
import { ProposalStatus } from '../types';

describe('Governance Module', () => {
  describe('Proposal Status Enum', () => {
    it('should have all status values', () => {
      expect(ProposalStatus.Active).toBe(0);
      expect(ProposalStatus.Passed).toBe(1);
      expect(ProposalStatus.Rejected).toBe(2);
      expect(ProposalStatus.Executed).toBe(3);
      expect(ProposalStatus.Cancelled).toBe(4);
    });
  });

  describe('Vote Choice Enum', () => {
    it('should have all vote choices', () => {
      expect(VoteChoice.Against).toBe(0);
      expect(VoteChoice.For).toBe(1);
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
      expect(GOVERNANCE_CONSTANTS.minProposalThreshold).toBe(100);
    });

    it('should have authority transfer timelock (H-02)', () => {
      expect(GOVERNANCE_CONSTANTS.authorityTransferTimelock).toBe(24 * 3600);
    });

    it('should have ZK voting constants', () => {
      expect(GOVERNANCE_CONSTANTS.zk).toBeDefined();
      expect(GOVERNANCE_CONSTANTS.zk.voteProofSize).toBe(352);
      expect(GOVERNANCE_CONSTANTS.zk.ciphertextSize).toBe(64);
      expect(GOVERNANCE_CONSTANTS.zk.maxCommitteeSize).toBe(5);
      expect(GOVERNANCE_CONSTANTS.zk.privateVotingConfigSize).toBe(680);
      expect(GOVERNANCE_CONSTANTS.zk.decryptionShareSize).toBe(242);
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

      // C-03: Quorum now counts only for + against (abstains excluded)
      const quorumVotes = proposal.votesFor + proposal.votesAgainst;
      const quorum = 40_000_000n;

      const passed = proposal.votesFor > proposal.votesAgainst && quorumVotes >= quorum;
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

    it('should fail when quorum not met (C-03: abstains excluded)', () => {
      const proposal = {
        votesFor: 20_000_000n,
        votesAgainst: 10_000_000n,
        votesAbstain: 50_000_000n, // lots of abstains, but they don't count
      };

      // C-03: Only for + against count toward quorum
      const quorumVotes = proposal.votesFor + proposal.votesAgainst;
      const quorum = 40_000_000n;

      const passed = proposal.votesFor > proposal.votesAgainst && quorumVotes >= quorum;
      expect(passed).toBe(false); // 30M < 40M quorum
    });

    it('should demonstrate C-03 quorum change: abstains no longer help reach quorum', () => {
      const proposal = {
        votesFor: 25_000_000n,
        votesAgainst: 10_000_000n,
        votesAbstain: 100_000_000n, // massive abstains
      };

      // Old quorum (all votes): 135M >= 40M would pass
      const oldTotalVotes = proposal.votesFor + proposal.votesAgainst + proposal.votesAbstain;
      const quorum = 40_000_000n;
      const oldPassed = proposal.votesFor > proposal.votesAgainst && oldTotalVotes >= quorum;
      expect(oldPassed).toBe(true); // would have passed under old rules

      // New quorum (C-03: only for + against): 35M < 40M fails
      const newQuorumVotes = proposal.votesFor + proposal.votesAgainst;
      const newPassed = proposal.votesFor > proposal.votesAgainst && newQuorumVotes >= quorum;
      expect(newPassed).toBe(false); // fails under new rules
    });
  });

  describe('ZK Voting Constants', () => {
    it('should have correct vote proof size (3 OR proofs + 1 sum proof)', () => {
      // 3 OR proofs at 96 bytes each + 1 sum proof at 64 bytes = 352
      const expectedSize = 3 * 96 + 64;
      expect(GOVERNANCE_CONSTANTS.zk.voteProofSize).toBe(expectedSize);
    });

    it('should have correct ciphertext size (R || C)', () => {
      // ElGamal ciphertext: 32-byte R point + 32-byte C point
      expect(GOVERNANCE_CONSTANTS.zk.ciphertextSize).toBe(64);
    });

    it('should support threshold decryption with committee', () => {
      expect(GOVERNANCE_CONSTANTS.zk.maxCommitteeSize).toBe(5);
    });
  });

  describe('GovernanceClient', () => {
    it('should export GovernanceClient class', () => {
      expect(GovernanceClient).toBeDefined();
    });
  });
});
