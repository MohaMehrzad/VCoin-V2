/**
 * @viwoapp/sdk Core Module Tests
 */

import { PublicKey, Keypair, Connection } from '@solana/web3.js';
import { BN } from '@coral-xyz/anchor';
import {
  ViWoConnection,
  PDAs,
  formatVCoin,
  parseVCoin,
  getCurrentTimestamp,
  timestampToDate,
  dateToTimestamp,
} from '../core';
import { PROGRAM_IDS, VCOIN_DECIMALS } from '../constants';

describe('Core Module - ViWoConnection', () => {
  it('should create connection with endpoint', () => {
    const conn = new ViWoConnection({
      endpoint: 'https://api.devnet.solana.com',
    });

    expect(conn.connection).toBeInstanceOf(Connection);
    expect(conn.commitment).toBe('confirmed');
  });

  it('should create connection with custom commitment', () => {
    const conn = new ViWoConnection({
      endpoint: 'https://api.devnet.solana.com',
      commitment: 'finalized',
    });

    expect(conn.commitment).toBe('finalized');
  });
});

describe('Core Module - PDAs', () => {
  let pdas: PDAs;

  beforeAll(() => {
    pdas = new PDAs();
  });

  describe('Staking PDAs', () => {
    it('should derive staking pool PDA', () => {
      const pda = pdas.getStakingPool();
      expect(pda).toBeInstanceOf(PublicKey);
    });

    it('should derive user stake PDA', () => {
      const user = Keypair.generate().publicKey;
      const pda = pdas.getUserStake(user);
      expect(pda).toBeInstanceOf(PublicKey);
    });

    it('should derive different PDAs for different users', () => {
      const user1 = Keypair.generate().publicKey;
      const user2 = Keypair.generate().publicKey;

      const pda1 = pdas.getUserStake(user1);
      const pda2 = pdas.getUserStake(user2);

      expect(pda1.toBase58()).not.toBe(pda2.toBase58());
    });
  });

  describe('Governance PDAs', () => {
    it('should derive governance config PDA', () => {
      const pda = pdas.getGovernanceConfig();
      expect(pda).toBeInstanceOf(PublicKey);
    });

    it('should derive proposal PDA', () => {
      const pda = pdas.getProposal(new BN(1));
      expect(pda).toBeInstanceOf(PublicKey);
    });

    it('should derive different PDAs for different proposals', () => {
      const pda1 = pdas.getProposal(new BN(1));
      const pda2 = pdas.getProposal(new BN(2));

      expect(pda1.toBase58()).not.toBe(pda2.toBase58());
    });

    it('should derive vote record PDA', () => {
      const voter = Keypair.generate().publicKey;
      const pda = pdas.getVoteRecord(voter, new BN(1));
      expect(pda).toBeInstanceOf(PublicKey);
    });
  });

  describe('SSCRE PDAs', () => {
    it('should derive rewards pool config PDA', () => {
      const pda = pdas.getRewardsPoolConfig();
      expect(pda).toBeInstanceOf(PublicKey);
    });

    it('should derive user claim PDA', () => {
      const user = Keypair.generate().publicKey;
      const pda = pdas.getUserClaim(user);
      expect(pda).toBeInstanceOf(PublicKey);
    });
  });

  describe('ViLink PDAs', () => {
    it('should derive ViLink config PDA', () => {
      const pda = pdas.getViLinkConfig();
      expect(pda).toBeInstanceOf(PublicKey);
    });
  });

  describe('Gasless PDAs', () => {
    it('should derive gasless config PDA', () => {
      const pda = pdas.getGaslessConfig();
      expect(pda).toBeInstanceOf(PublicKey);
    });

    it('should derive session key PDA', () => {
      const user = Keypair.generate().publicKey;
      const session = Keypair.generate().publicKey;
      const pda = pdas.getSessionKey(user, session);
      expect(pda).toBeInstanceOf(PublicKey);
    });
  });
});

describe('Core Module - Formatting', () => {
  describe('VCoin Formatting', () => {
    it('should format VCoin from BN', () => {
      const amount = new BN('1000000000'); // 1 VCoin
      const formatted = formatVCoin(amount);
      expect(formatted).toBe('1.000000000');
    });

    it('should format zero correctly', () => {
      const formatted = formatVCoin(new BN(0));
      expect(formatted).toBe('0.000000000');
    });

    it('should format fractional amounts', () => {
      const amount = new BN('1500000000'); // 1.5 VCoin
      const formatted = formatVCoin(amount);
      expect(formatted).toBe('1.500000000');
    });
  });

  describe('VCoin Parsing', () => {
    it('should parse whole VCoin to base units', () => {
      const parsed = parseVCoin('1');
      expect(parsed.toString()).toBe('1000000000');
    });

    it('should parse fractional VCoin', () => {
      const parsed = parseVCoin('1.5');
      expect(parsed.toString()).toBe('1500000000');
    });

    it('should parse zero', () => {
      const parsed = parseVCoin('0');
      expect(parsed.toString()).toBe('0');
    });

    it('should be inverse of format', () => {
      const original = new BN('12345678901');
      const formatted = formatVCoin(original);
      const parsed = parseVCoin(formatted);
      expect(parsed.toString()).toBe(original.toString());
    });
  });
});

describe('Core Module - Timestamps', () => {
  it('should get current timestamp', () => {
    const ts = getCurrentTimestamp();
    const now = Math.floor(Date.now() / 1000);
    expect(Math.abs(ts - now)).toBeLessThan(2);
  });

  it('should convert timestamp to date', () => {
    const ts = 1700000000;
    const date = timestampToDate(ts);
    expect(date).toBeInstanceOf(Date);
    expect(Math.floor(date.getTime() / 1000)).toBe(ts);
  });

  it('should convert BN timestamp to date', () => {
    const ts = new BN(1700000000);
    const date = timestampToDate(ts);
    expect(date).toBeInstanceOf(Date);
  });

  it('should convert date to timestamp', () => {
    const date = new Date(1700000000000);
    const ts = dateToTimestamp(date);
    expect(ts).toBe(1700000000);
  });
});
