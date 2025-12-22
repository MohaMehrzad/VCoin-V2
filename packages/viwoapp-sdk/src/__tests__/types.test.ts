/**
 * @viwoapp/sdk Types Tests
 */

import { PublicKey, Keypair } from '@solana/web3.js';
import {
  StakingTier,
  UserStakeAccount,
  StakingPoolAccount,
  ProposalAccount,
  VoteRecordAccount,
  UserClaimAccount,
  ViLinkActionAccount,
  SessionKeyAccount,
  FiveAScoreAccount,
  ContentRecordAccount,
  isStakingTier,
  isValidProposalStatus,
  isValidVoteChoice,
  isValidActionType,
  isValidFeeMethod,
} from '../types';

describe('Types Module', () => {
  describe('StakingTier Type Guard', () => {
    it('should validate valid tiers', () => {
      expect(isStakingTier('none')).toBe(true);
      expect(isStakingTier('bronze')).toBe(true);
      expect(isStakingTier('silver')).toBe(true);
      expect(isStakingTier('gold')).toBe(true);
      expect(isStakingTier('platinum')).toBe(true);
    });

    it('should reject invalid tiers', () => {
      expect(isStakingTier('diamond')).toBe(false);
      expect(isStakingTier('invalid')).toBe(false);
      expect(isStakingTier('')).toBe(false);
      expect(isStakingTier(null)).toBe(false);
      expect(isStakingTier(undefined)).toBe(false);
      expect(isStakingTier(1)).toBe(false);
    });
  });

  describe('ProposalStatus Type Guard', () => {
    it('should validate valid status values', () => {
      expect(isValidProposalStatus(0)).toBe(true); // Pending
      expect(isValidProposalStatus(1)).toBe(true); // Active
      expect(isValidProposalStatus(2)).toBe(true); // Passed
      expect(isValidProposalStatus(3)).toBe(true); // Rejected
      expect(isValidProposalStatus(4)).toBe(true); // Executed
      expect(isValidProposalStatus(5)).toBe(true); // Cancelled
    });

    it('should reject invalid status values', () => {
      expect(isValidProposalStatus(6)).toBe(false);
      expect(isValidProposalStatus(-1)).toBe(false);
      expect(isValidProposalStatus(100)).toBe(false);
    });
  });

  describe('VoteChoice Type Guard', () => {
    it('should validate valid vote choices', () => {
      expect(isValidVoteChoice(0)).toBe(true); // For
      expect(isValidVoteChoice(1)).toBe(true); // Against
      expect(isValidVoteChoice(2)).toBe(true); // Abstain
    });

    it('should reject invalid vote choices', () => {
      expect(isValidVoteChoice(3)).toBe(false);
      expect(isValidVoteChoice(-1)).toBe(false);
    });
  });

  describe('ActionType Type Guard', () => {
    it('should validate valid action types', () => {
      expect(isValidActionType(0)).toBe(true); // Tip
      expect(isValidActionType(1)).toBe(true); // Vouch
      expect(isValidActionType(2)).toBe(true); // Follow
      expect(isValidActionType(3)).toBe(true); // Challenge
      expect(isValidActionType(4)).toBe(true); // Stake
      expect(isValidActionType(5)).toBe(true); // ContentReact
      expect(isValidActionType(6)).toBe(true); // Delegate
      expect(isValidActionType(7)).toBe(true); // Vote
    });

    it('should reject invalid action types', () => {
      expect(isValidActionType(8)).toBe(false);
      expect(isValidActionType(-1)).toBe(false);
    });
  });

  describe('FeeMethod Type Guard', () => {
    it('should validate valid fee methods', () => {
      expect(isValidFeeMethod(0)).toBe(true); // PlatformSubsidized
      expect(isValidFeeMethod(1)).toBe(true); // VCoinDeduction
      expect(isValidFeeMethod(2)).toBe(true); // SSCREDeduction
    });

    it('should reject invalid fee methods', () => {
      expect(isValidFeeMethod(3)).toBe(false);
      expect(isValidFeeMethod(-1)).toBe(false);
    });
  });

  describe('UserStakeAccount Interface', () => {
    it('should define correct structure', () => {
      const account: UserStakeAccount = {
        owner: Keypair.generate().publicKey,
        stakedAmount: BigInt(10000),
        veVCoinAmount: BigInt(15000),
        lockEndTime: BigInt(Date.now() / 1000 + 86400),
        tier: 'silver',
        stakedAt: BigInt(Date.now() / 1000),
        lastUpdatedAt: BigInt(Date.now() / 1000),
        bump: 255,
      };

      expect(account.owner).toBeInstanceOf(PublicKey);
      expect(typeof account.stakedAmount).toBe('bigint');
      expect(typeof account.tier).toBe('string');
    });
  });

  describe('ProposalAccount Interface', () => {
    it('should define correct structure', () => {
      const account: ProposalAccount = {
        id: BigInt(1),
        proposer: Keypair.generate().publicKey,
        titleHash: new Uint8Array(32),
        descriptionUri: 'ipfs://...',
        proposalType: 0,
        startTime: BigInt(Date.now() / 1000),
        endTime: BigInt(Date.now() / 1000 + 604800),
        votesFor: BigInt(0),
        votesAgainst: BigInt(0),
        votesAbstain: BigInt(0),
        status: 1,
        executionTime: BigInt(0),
        executed: false,
        isPrivateVoting: false,
        bump: 255,
      };

      expect(account.proposer).toBeInstanceOf(PublicKey);
      expect(account.executed).toBe(false);
    });
  });

  describe('UserClaimAccount Interface', () => {
    it('should define correct structure', () => {
      const account: UserClaimAccount = {
        user: Keypair.generate().publicKey,
        lastClaimedEpoch: BigInt(5),
        totalClaimed: BigInt(1000000000),
        claimsCount: 10,
        firstClaimAt: BigInt(Date.now() / 1000 - 86400 * 30),
        lastClaimAt: BigInt(Date.now() / 1000),
        claimedEpochsBitmap: [BigInt(0), BigInt(0), BigInt(0), BigInt(0)],
        bump: 255,
      };

      expect(account.user).toBeInstanceOf(PublicKey);
      expect(account.claimedEpochsBitmap.length).toBe(4);
    });
  });

  describe('ViLinkActionAccount Interface', () => {
    it('should define correct structure', () => {
      const account: ViLinkActionAccount = {
        actionId: new Uint8Array(32),
        creator: Keypair.generate().publicKey,
        target: Keypair.generate().publicKey,
        actionType: 0,
        amount: BigInt(100000000000),
        metadataHash: new Uint8Array(32),
        createdAt: BigInt(Date.now() / 1000),
        expiresAt: BigInt(Date.now() / 1000 + 604800),
        executed: false,
        executor: PublicKey.default,
        executedAt: BigInt(0),
        contentId: null,
        sourceDapp: PublicKey.default,
        oneTime: true,
        executionCount: 0,
        maxExecutions: 1,
        bump: 255,
      };

      expect(account.creator).toBeInstanceOf(PublicKey);
      expect(account.oneTime).toBe(true);
    });
  });

  describe('SessionKeyAccount Interface', () => {
    it('should define correct structure', () => {
      const account: SessionKeyAccount = {
        user: Keypair.generate().publicKey,
        sessionPubkey: Keypair.generate().publicKey,
        scope: 0xFFFF,
        createdAt: BigInt(Date.now() / 1000),
        expiresAt: BigInt(Date.now() / 1000 + 86400),
        actionsUsed: 10,
        maxActions: 1000,
        vcoinSpent: BigInt(0),
        maxSpend: BigInt(100000000000000),
        isRevoked: false,
        lastActionAt: BigInt(Date.now() / 1000),
        feeMethod: 0,
        bump: 255,
      };

      expect(account.sessionPubkey).toBeInstanceOf(PublicKey);
      expect(account.isRevoked).toBe(false);
    });
  });

  describe('FiveAScoreAccount Interface', () => {
    it('should define correct structure', () => {
      const account: FiveAScoreAccount = {
        user: Keypair.generate().publicKey,
        authenticity: 2000,
        accuracy: 1800,
        agility: 1500,
        activity: 2500,
        approved: 1200,
        totalScore: 9000,
        lastUpdatedAt: BigInt(Date.now() / 1000),
        updateCount: 100,
        bump: 255,
      };

      expect(account.user).toBeInstanceOf(PublicKey);
      expect(account.totalScore).toBeLessThanOrEqual(10000);
    });
  });

  describe('ContentRecordAccount Interface', () => {
    it('should define correct structure', () => {
      const account: ContentRecordAccount = {
        trackingId: new Uint8Array(32),
        author: Keypair.generate().publicKey,
        contentHash: new Uint8Array(32),
        contentUri: 'ipfs://...',
        contentType: 0,
        state: 0,
        version: 1,
        createdAt: BigInt(Date.now() / 1000),
        updatedAt: BigInt(Date.now() / 1000),
        previousHash: new Uint8Array(32),
        energySpent: 10,
        refundClaimed: false,
        engagementCount: 0,
        bump: 255,
      };

      expect(account.author).toBeInstanceOf(PublicKey);
      expect(account.refundClaimed).toBe(false);
    });
  });

  describe('Type Exports', () => {
    it('should export all required types', () => {
      // These are type-only checks - they pass if compilation succeeds
      const _tier: StakingTier = 'bronze';
      const _userStake: Partial<UserStakeAccount> = {};
      const _pool: Partial<StakingPoolAccount> = {};
      const _proposal: Partial<ProposalAccount> = {};
      const _vote: Partial<VoteRecordAccount> = {};
      const _claim: Partial<UserClaimAccount> = {};
      const _action: Partial<ViLinkActionAccount> = {};
      const _session: Partial<SessionKeyAccount> = {};
      const _fiveA: Partial<FiveAScoreAccount> = {};
      const _content: Partial<ContentRecordAccount> = {};
    });
  });
});

