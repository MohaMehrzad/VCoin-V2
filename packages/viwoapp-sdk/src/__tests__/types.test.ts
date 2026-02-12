/**
 * @viwoapp/sdk Types Tests
 */

import { PublicKey, Keypair } from '@solana/web3.js';
import { BN } from '@coral-xyz/anchor';
import {
  StakingTier,
  ProposalStatus,
  VoteChoice,
  ActionType,
  FeeMethod,
  VerificationLevel,
  ContentState,
  SlashStatus,
} from '../types';
import type {
  UserStake,
  StakingPool,
  Proposal,
  VoteRecord,
  UserClaim,
  ViLinkAction,
  SessionKey,
  FiveAScore,
  ContentRecord,
  GovernanceConfig,
  DecryptionShare,
  PrivateVotingConfig,
  PairTracking,
  VeVCoinConfig,
  UserGaslessStats,
  PendingScoreUpdate,
  CastPrivateVoteParams,
  EnablePrivateVotingParams,
  SubmitDecryptionShareParams,
  AggregateRevealedVotesParams,
} from '../types';

describe('Types Module', () => {
  describe('StakingTier Enum', () => {
    it('should have all tier values', () => {
      expect(StakingTier.None).toBe(0);
      expect(StakingTier.Bronze).toBe(1);
      expect(StakingTier.Silver).toBe(2);
      expect(StakingTier.Gold).toBe(3);
      expect(StakingTier.Platinum).toBe(4);
    });
  });

  describe('ProposalStatus Enum', () => {
    it('should have all status values', () => {
      expect(ProposalStatus.Active).toBe(0);
      expect(ProposalStatus.Passed).toBe(1);
      expect(ProposalStatus.Rejected).toBe(2);
      expect(ProposalStatus.Executed).toBe(3);
      expect(ProposalStatus.Cancelled).toBe(4);
    });

    it('should reject invalid status values', () => {
      expect(ProposalStatus[5]).toBeUndefined();
    });
  });

  describe('VoteChoice Enum', () => {
    it('should have all vote choices', () => {
      expect(VoteChoice.Against).toBe(0);
      expect(VoteChoice.For).toBe(1);
      expect(VoteChoice.Abstain).toBe(2);
    });
  });

  describe('ActionType Enum', () => {
    it('should have all action types', () => {
      expect(ActionType.Tip).toBe(0);
      expect(ActionType.Vouch).toBe(1);
      expect(ActionType.Follow).toBe(2);
      expect(ActionType.Challenge).toBe(3);
      expect(ActionType.Stake).toBe(4);
      expect(ActionType.ContentReact).toBe(5);
      expect(ActionType.Delegate).toBe(6);
      expect(ActionType.Vote).toBe(7);
    });
  });

  describe('FeeMethod Enum', () => {
    it('should have all fee methods', () => {
      expect(FeeMethod.PlatformSubsidized).toBe(0);
      expect(FeeMethod.VCoinDeduction).toBe(1);
      expect(FeeMethod.SSCREDeduction).toBe(2);
    });
  });

  describe('VerificationLevel Enum', () => {
    it('should have all verification levels', () => {
      expect(VerificationLevel.None).toBe(0);
      expect(VerificationLevel.Basic).toBe(1);
      expect(VerificationLevel.KYC).toBe(2);
      expect(VerificationLevel.Full).toBe(3);
      expect(VerificationLevel.Enhanced).toBe(4);
    });
  });

  describe('ContentState Enum', () => {
    it('should have all content states', () => {
      expect(ContentState.Active).toBe(0);
      expect(ContentState.Hidden).toBe(1);
      expect(ContentState.Deleted).toBe(2);
      expect(ContentState.Flagged).toBe(3);
    });
  });

  describe('SlashStatus Enum', () => {
    it('should have all slash statuses', () => {
      expect(SlashStatus.Proposed).toBe(0);
      expect(SlashStatus.Approved).toBe(1);
      expect(SlashStatus.Executed).toBe(2);
      expect(SlashStatus.Cancelled).toBe(3);
    });
  });

  describe('UserStake Interface', () => {
    it('should define correct structure', () => {
      const account: UserStake = {
        user: Keypair.generate().publicKey,
        stakedAmount: new BN(10000),
        vevcoinBalance: new BN(15000),
        tier: StakingTier.Silver,
        lockEndTime: new BN(Date.now() / 1000 + 86400),
        lastUpdateTime: new BN(Date.now() / 1000),
      };

      expect(account.user).toBeInstanceOf(PublicKey);
      expect(account.tier).toBe(StakingTier.Silver);
    });
  });

  describe('Proposal Interface', () => {
    it('should define correct structure', () => {
      const account: Proposal = {
        id: new BN(1),
        proposer: Keypair.generate().publicKey,
        title: 'Test Proposal',
        descriptionHash: new Uint8Array(32),
        startTime: new BN(Date.now() / 1000),
        endTime: new BN(Date.now() / 1000 + 604800),
        votesFor: new BN(0),
        votesAgainst: new BN(0),
        status: ProposalStatus.Active,
        executed: false,
        category: 0,
      };

      expect(account.proposer).toBeInstanceOf(PublicKey);
      expect(account.executed).toBe(false);
      expect(account.status).toBe(ProposalStatus.Active);
    });
  });

  describe('VoteRecord Interface', () => {
    it('should define correct structure with optional ZK fields', () => {
      const account: VoteRecord = {
        user: Keypair.generate().publicKey,
        proposal: Keypair.generate().publicKey,
        votePower: new BN(100),
        support: true,
        votedAt: new BN(Date.now() / 1000),
      };

      expect(account.user).toBeInstanceOf(PublicKey);
      expect(account.support).toBe(true);

      // ZK fields are optional
      expect(account.ctFor).toBeUndefined();
      expect(account.ctAgainst).toBeUndefined();
      expect(account.ctAbstain).toBeUndefined();
    });

    it('should support ZK ciphertext fields for private votes', () => {
      const account: VoteRecord = {
        user: Keypair.generate().publicKey,
        proposal: Keypair.generate().publicKey,
        votePower: new BN(100),
        support: true,
        votedAt: new BN(Date.now() / 1000),
        ctFor: new Uint8Array(64),
        ctAgainst: new Uint8Array(64),
        ctAbstain: new Uint8Array(64),
      };

      expect(account.ctFor).toHaveLength(64);
      expect(account.ctAgainst).toHaveLength(64);
      expect(account.ctAbstain).toHaveLength(64);
    });
  });

  describe('GovernanceConfig Interface', () => {
    it('should include authority transfer fields (H-02)', () => {
      const config: GovernanceConfig = {
        authority: Keypair.generate().publicKey,
        pendingAuthority: PublicKey.default,
        pendingAuthorityActivatedAt: new BN(0),
        vevcoinMint: Keypair.generate().publicKey,
        paused: false,
        proposalCount: new BN(0),
      };

      expect(config.pendingAuthorityActivatedAt).toBeDefined();
      expect(config.proposalCount.toNumber()).toBe(0);
    });
  });

  describe('DecryptionShare Interface', () => {
    it('should include per-category partials and DLEQ proof', () => {
      const share: DecryptionShare = {
        proposal: Keypair.generate().publicKey,
        committeeIndex: 0,
        committeeMember: Keypair.generate().publicKey,
        partialFor: new Uint8Array(32),
        partialAgainst: new Uint8Array(32),
        partialAbstain: new Uint8Array(32),
        dleqChallenge: new Uint8Array(32),
        dleqResponse: new Uint8Array(32),
        submittedAt: new BN(Date.now() / 1000),
        verified: true,
      };

      expect(share.partialFor).toHaveLength(32);
      expect(share.dleqChallenge).toHaveLength(32);
      expect(share.verified).toBe(true);
    });
  });

  describe('PrivateVotingConfig Interface', () => {
    it('should include ZK voting fields', () => {
      const config: PrivateVotingConfig = {
        proposal: Keypair.generate().publicKey,
        encryptionPubkey: new Uint8Array(32),
        committeeElgamalPubkeys: [new Uint8Array(32)],
        decryptionCommittee: [Keypair.generate().publicKey],
        decryptionThreshold: 1,
        sharesSubmitted: [false],
        revealCompleted: false,
        accumulatedCtForR: new Uint8Array(32),
        accumulatedCtForC: new Uint8Array(32),
        accumulatedCtAgainstR: new Uint8Array(32),
        accumulatedCtAgainstC: new Uint8Array(32),
        accumulatedCtAbstainR: new Uint8Array(32),
        accumulatedCtAbstainC: new Uint8Array(32),
        totalPrivateVotes: 0,
        verificationHash: new Uint8Array(32),
        aggregatedFor: new BN(0),
        aggregatedAgainst: new BN(0),
        aggregatedAbstain: new BN(0),
      };

      expect(config.encryptionPubkey).toHaveLength(32);
      expect(config.accumulatedCtForR).toHaveLength(32);
      expect(config.totalPrivateVotes).toBe(0);
    });
  });

  describe('UserGaslessStats Interface', () => {
    it('should include activeSessions field (H-AUDIT-12)', () => {
      const stats: UserGaslessStats = {
        user: Keypair.generate().publicKey,
        totalGaslessTx: new BN(0),
        totalSubsidized: new BN(0),
        totalVcoinFees: new BN(0),
        sessionsCreated: 0,
        activeSession: Keypair.generate().publicKey,
        activeSessions: 2,
      };

      expect(stats.activeSessions).toBe(2);
    });
  });

  describe('PairTracking Interface', () => {
    it('should define correct structure', () => {
      const tracking: PairTracking = {
        sender: Keypair.generate().publicKey,
        receiver: Keypair.generate().publicKey,
        transferCount: 5,
        lastTransferTime: new BN(Date.now() / 1000),
        washFlags: 0,
        lastFlagTime: new BN(0),
      };

      expect(tracking.sender).toBeInstanceOf(PublicKey);
      expect(tracking.washFlags).toBe(0);
    });
  });

  describe('ZK Voting Params Interfaces', () => {
    it('should define CastPrivateVoteParams', () => {
      const params: CastPrivateVoteParams = {
        proposalId: new BN(1),
        ctFor: new Uint8Array(64),
        ctAgainst: new Uint8Array(64),
        ctAbstain: new Uint8Array(64),
        proofData: new Uint8Array(352),
      };

      expect(params.ctFor).toHaveLength(64);
      expect(params.proofData).toHaveLength(352);
    });

    it('should define EnablePrivateVotingParams', () => {
      const params: EnablePrivateVotingParams = {
        proposalId: new BN(1),
        encryptionPubkey: new Uint8Array(32),
        committeeElgamalPubkeys: Array(5).fill(new Uint8Array(32)),
        decryptionCommittee: Array(5).fill(Keypair.generate().publicKey),
        committeeSize: 5,
        decryptionThreshold: 3,
      };

      expect(params.decryptionThreshold).toBe(3);
      expect(params.committeeElgamalPubkeys).toHaveLength(5);
    });

    it('should define SubmitDecryptionShareParams', () => {
      const params: SubmitDecryptionShareParams = {
        proposalId: new BN(1),
        committeeIndex: 0,
        partialFor: new Uint8Array(32),
        partialAgainst: new Uint8Array(32),
        partialAbstain: new Uint8Array(32),
        dleqChallenge: new Uint8Array(32),
        dleqResponse: new Uint8Array(32),
      };

      expect(params.committeeIndex).toBe(0);
      expect(params.partialFor).toHaveLength(32);
    });

    it('should define AggregateRevealedVotesParams', () => {
      const params: AggregateRevealedVotesParams = {
        proposalId: new BN(1),
        tallyFor: new BN(100),
        tallyAgainst: new BN(50),
        tallyAbstain: new BN(10),
        lagrangeCoefficients: [new Uint8Array(32)],
      };

      expect(params.tallyFor.toNumber()).toBe(100);
      expect(params.lagrangeCoefficients).toHaveLength(1);
    });
  });

  describe('Type Exports', () => {
    it('should export all required enum types', () => {
      expect(StakingTier).toBeDefined();
      expect(ProposalStatus).toBeDefined();
      expect(VoteChoice).toBeDefined();
      expect(ActionType).toBeDefined();
      expect(FeeMethod).toBeDefined();
      expect(VerificationLevel).toBeDefined();
      expect(ContentState).toBeDefined();
      expect(SlashStatus).toBeDefined();
    });
  });
});
