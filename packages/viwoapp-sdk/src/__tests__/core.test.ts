/**
 * @viwoapp/sdk Core Module Tests
 */

import { PublicKey, Keypair, Connection } from '@solana/web3.js';
import {
  getStakingPoolPDA,
  getUserStakePDA,
  getGovernanceConfigPDA,
  getProposalPDA,
  getVoteRecordPDA,
  getSscrePoolPDA,
  getUserClaimPDA,
  getVilinkConfigPDA,
  getViwoAppClient,
  formatVCoin,
  parseVCoin,
  formatVeVCoin,
  parseVeVCoin,
  calculateTier,
  calculateVeVCoinMultiplier,
  calculateVotingPower,
} from '../core';
import { PROGRAM_IDS, STAKING_TIERS, VCOIN_DECIMALS } from '../constants';

describe('Core Module - PDA Derivation', () => {
  describe('Staking PDAs', () => {
    it('should derive staking pool PDA correctly', () => {
      const [pda, bump] = getStakingPoolPDA();
      
      expect(pda).toBeInstanceOf(PublicKey);
      expect(bump).toBeGreaterThanOrEqual(0);
      expect(bump).toBeLessThanOrEqual(255);
    });

    it('should derive user stake PDA correctly', () => {
      const user = Keypair.generate().publicKey;
      const [pda, bump] = getUserStakePDA(user);
      
      expect(pda).toBeInstanceOf(PublicKey);
      expect(bump).toBeGreaterThanOrEqual(0);
      expect(bump).toBeLessThanOrEqual(255);
    });

    it('should derive different PDAs for different users', () => {
      const user1 = Keypair.generate().publicKey;
      const user2 = Keypair.generate().publicKey;
      
      const [pda1] = getUserStakePDA(user1);
      const [pda2] = getUserStakePDA(user2);
      
      expect(pda1.toBase58()).not.toBe(pda2.toBase58());
    });
  });

  describe('Governance PDAs', () => {
    it('should derive governance config PDA correctly', () => {
      const [pda, bump] = getGovernanceConfigPDA();
      
      expect(pda).toBeInstanceOf(PublicKey);
      expect(bump).toBeGreaterThanOrEqual(0);
    });

    it('should derive proposal PDA correctly', () => {
      const proposalId = 1;
      const [pda, bump] = getProposalPDA(proposalId);
      
      expect(pda).toBeInstanceOf(PublicKey);
      expect(bump).toBeGreaterThanOrEqual(0);
    });

    it('should derive different PDAs for different proposals', () => {
      const [pda1] = getProposalPDA(1);
      const [pda2] = getProposalPDA(2);
      
      expect(pda1.toBase58()).not.toBe(pda2.toBase58());
    });

    it('should derive vote record PDA correctly', () => {
      const voter = Keypair.generate().publicKey;
      const proposal = Keypair.generate().publicKey;
      
      const [pda, bump] = getVoteRecordPDA(voter, proposal);
      
      expect(pda).toBeInstanceOf(PublicKey);
      expect(bump).toBeGreaterThanOrEqual(0);
    });
  });

  describe('SSCRE PDAs', () => {
    it('should derive SSCRE pool PDA correctly', () => {
      const [pda, bump] = getSscrePoolPDA();
      
      expect(pda).toBeInstanceOf(PublicKey);
      expect(bump).toBeGreaterThanOrEqual(0);
    });

    it('should derive user claim PDA correctly', () => {
      const user = Keypair.generate().publicKey;
      const [pda, bump] = getUserClaimPDA(user);
      
      expect(pda).toBeInstanceOf(PublicKey);
      expect(bump).toBeGreaterThanOrEqual(0);
    });
  });

  describe('ViLink PDAs', () => {
    it('should derive ViLink config PDA correctly', () => {
      const [pda, bump] = getVilinkConfigPDA();
      
      expect(pda).toBeInstanceOf(PublicKey);
      expect(bump).toBeGreaterThanOrEqual(0);
    });
  });
});

describe('Core Module - Formatting', () => {
  describe('VCoin Formatting', () => {
    it('should format VCoin from base units', () => {
      const baseUnits = BigInt(1_000_000_000); // 1 VCoin
      const formatted = formatVCoin(baseUnits);
      
      expect(formatted).toBe('1.000000000');
    });

    it('should format zero correctly', () => {
      const formatted = formatVCoin(BigInt(0));
      expect(formatted).toBe('0.000000000');
    });

    it('should format fractional amounts', () => {
      const baseUnits = BigInt(1_500_000_000); // 1.5 VCoin
      const formatted = formatVCoin(baseUnits);
      
      expect(formatted).toBe('1.500000000');
    });

    it('should format large amounts', () => {
      const baseUnits = BigInt(1_000_000_000_000_000_000n); // 1 billion VCoin
      const formatted = formatVCoin(baseUnits);
      
      expect(formatted).toBe('1000000000.000000000');
    });
  });

  describe('VCoin Parsing', () => {
    it('should parse whole VCoin to base units', () => {
      const parsed = parseVCoin('1');
      expect(parsed).toBe(BigInt(1_000_000_000));
    });

    it('should parse fractional VCoin', () => {
      const parsed = parseVCoin('1.5');
      expect(parsed).toBe(BigInt(1_500_000_000));
    });

    it('should parse zero', () => {
      const parsed = parseVCoin('0');
      expect(parsed).toBe(BigInt(0));
    });

    it('should be inverse of format', () => {
      const original = BigInt(12_345_678_901);
      const formatted = formatVCoin(original);
      const parsed = parseVCoin(formatted);
      
      expect(parsed).toBe(original);
    });
  });

  describe('veVCoin Formatting', () => {
    it('should format veVCoin correctly', () => {
      const baseUnits = BigInt(2_500_000_000);
      const formatted = formatVeVCoin(baseUnits);
      
      expect(formatted).toBe('2.500000000');
    });

    it('should parse veVCoin correctly', () => {
      const parsed = parseVeVCoin('2.5');
      expect(parsed).toBe(BigInt(2_500_000_000));
    });
  });
});

describe('Core Module - Tier Calculation', () => {
  it('should return None tier for 0 stake', () => {
    const tier = calculateTier(0);
    expect(tier).toBe('none');
  });

  it('should return None tier for stake below Bronze', () => {
    const tier = calculateTier(500);
    expect(tier).toBe('none');
  });

  it('should return Bronze tier for 1000 stake', () => {
    const tier = calculateTier(1_000);
    expect(tier).toBe('bronze');
  });

  it('should return Bronze tier for stake between 1000 and 5000', () => {
    const tier = calculateTier(3_000);
    expect(tier).toBe('bronze');
  });

  it('should return Silver tier for 5000 stake', () => {
    const tier = calculateTier(5_000);
    expect(tier).toBe('silver');
  });

  it('should return Gold tier for 20000 stake', () => {
    const tier = calculateTier(20_000);
    expect(tier).toBe('gold');
  });

  it('should return Platinum tier for 100000 stake', () => {
    const tier = calculateTier(100_000);
    expect(tier).toBe('platinum');
  });

  it('should return Platinum for very large stakes', () => {
    const tier = calculateTier(1_000_000);
    expect(tier).toBe('platinum');
  });
});

describe('Core Module - veVCoin Multiplier', () => {
  it('should return 1.0x for no lock', () => {
    const mult = calculateVeVCoinMultiplier(0);
    expect(mult).toBe(1.0);
  });

  it('should return correct multiplier for 1 month lock', () => {
    const oneMonthSeconds = 30 * 24 * 3600;
    const mult = calculateVeVCoinMultiplier(oneMonthSeconds);
    expect(mult).toBeCloseTo(1.1, 1);
  });

  it('should return correct multiplier for 3 month lock', () => {
    const threeMonthsSeconds = 90 * 24 * 3600;
    const mult = calculateVeVCoinMultiplier(threeMonthsSeconds);
    expect(mult).toBeCloseTo(1.3, 1);
  });

  it('should return correct multiplier for 1 year lock', () => {
    const oneYearSeconds = 365 * 24 * 3600;
    const mult = calculateVeVCoinMultiplier(oneYearSeconds);
    expect(mult).toBeCloseTo(2.0, 1);
  });

  it('should cap at 4 year maximum', () => {
    const fiveYearSeconds = 5 * 365 * 24 * 3600;
    const mult = calculateVeVCoinMultiplier(fiveYearSeconds);
    expect(mult).toBeLessThanOrEqual(4.0);
  });
});

describe('Core Module - Voting Power', () => {
  it('should return 0 for 0 veVCoin', () => {
    const power = calculateVotingPower(0, 5000, 'bronze');
    expect(power).toBe(0);
  });

  it('should apply quadratic formula (sqrt)', () => {
    const power = calculateVotingPower(10000, 0, 'none');
    // sqrt(10000) = 100
    expect(power).toBeCloseTo(100, 0);
  });

  it('should apply 5A boost', () => {
    const basepower = calculateVotingPower(10000, 0, 'none');
    const boostedPower = calculateVotingPower(10000, 10000, 'none'); // Max 5A score
    
    // 2.0x boost at max 5A
    expect(boostedPower).toBeCloseTo(basepower * 2, 0);
  });

  it('should apply tier multiplier', () => {
    const bronzePower = calculateVotingPower(10000, 0, 'bronze');
    const platinumPower = calculateVotingPower(10000, 0, 'platinum');
    
    // Platinum = 10x multiplier vs Bronze = 1x
    expect(platinumPower).toBeGreaterThan(bronzePower);
  });
});

describe('Core Module - Client Factory', () => {
  it('should create client with connection string', () => {
    const client = getViwoAppClient('https://api.devnet.solana.com');
    expect(client).toBeDefined();
  });

  it('should create client with Connection object', () => {
    const connection = new Connection('https://api.devnet.solana.com');
    const client = getViwoAppClient(connection);
    expect(client).toBeDefined();
  });
});

