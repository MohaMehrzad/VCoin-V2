import { Keypair, PublicKey } from "@solana/web3.js";

import {
  DEFAULT_WEIGHT_PARAMS,
  MAX_LOCK_SECS,
  MerkleTree,
  claimFee,
  computeLeaf,
  depositWeight,
  formatVcoin,
  levelForClaims,
  lockedAmount,
  parseVcoin,
  pollWeight,
  tierForLocked,
  tipFee,
  verifyProof,
  isValidUsername,
  CLAIMS,
  VCOIN,
} from "../index";

describe("merkle leaves match the program", () => {
  test("frozen vector (docs/offchain/merkle-leaf-format.md)", () => {
    const claimant = new PublicKey(new Uint8Array(32).fill(7));
    expect(computeLeaf(claimant, 1_234_567_890n, 3n).toString("hex")).toBe(
      "5867436a6f21c025885194461701203778b53c261bd3f449af326ca8b9cf0083",
    );
  });

  test("every leaf of a built tree verifies, nothing else does", () => {
    for (const n of [1, 2, 3, 5, 8, 13, 64]) {
      const claims = Array.from({ length: n }, (_, i) => ({ claimant: Keypair.generate().publicKey, amount: BigInt(i + 1) * 1000n }));
      const tree = new MerkleTree(9n, claims, 1n << 60n);
      claims.forEach((c, i) => {
        const proof = tree.proof(i);
        expect(verifyProof(proof, tree.root(), computeLeaf(c.claimant, c.amount, 9n))).toBe(true);
        expect(verifyProof(proof, tree.root(), computeLeaf(c.claimant, c.amount + 1n, 9n))).toBe(false);
        expect(verifyProof(proof, tree.root(), computeLeaf(c.claimant, c.amount, 10n))).toBe(false);
      });
    }
  });

  test("the builder rejects what the program could not pay", () => {
    const a = Keypair.generate().publicKey;
    expect(() => new MerkleTree(0n, [], 10n)).toThrow("empty");
    expect(() => new MerkleTree(0n, [{ claimant: a, amount: 0n }], 10n)).toThrow("zero");
    expect(() => new MerkleTree(0n, [{ claimant: a, amount: 1n }, { claimant: a, amount: 1n }], 10n)).toThrow("duplicate");
    expect(() => new MerkleTree(0n, [{ claimant: a, amount: 11n }], 10n)).toThrow("exceed");
  });
});

describe("weight math matches weight.rs", () => {
  const t0 = 1_700_000_000n;

  test("a 4-year lock is worth 2x, linear in amount", () => {
    const d = { amount: 1_000n * VCOIN, lockEnd: t0 + BigInt(MAX_LOCK_SECS), isUsed: true };
    expect(depositWeight(d, DEFAULT_WEIGHT_PARAMS, t0)).toBe(2_000n * VCOIN);
    // one second later: the Rust test's decayed value
    const ten = { amount: 10n * VCOIN, lockEnd: t0 + BigInt(MAX_LOCK_SECS), isUsed: true };
    expect(depositWeight(ten, DEFAULT_WEIGHT_PARAMS, t0 + 1n)).toBe(19_999_999_920n);
  });

  test("weight is zero once the lock ends within 4 days", () => {
    const d = { amount: VCOIN, lockEnd: t0 + 4n * 86_400n - 1n, isUsed: true };
    expect(depositWeight(d, DEFAULT_WEIGHT_PARAMS, t0)).toBe(0n);
    expect(lockedAmount([d], t0)).toBe(VCOIN);
    expect(lockedAmount([d], d.lockEnd)).toBe(0n);
  });

  test("poll weight is whole VCoin", () => {
    expect(pollWeight(2_999_999_999n)).toBe(2n);
  });
});

describe("fees, tiers, levels and parsing match the programs", () => {
  test("fees round up", () => {
    expect(claimFee(101n, 100)).toBe(2n);
    expect(claimFee(1_000n * VCOIN, 100)).toBe(10n * VCOIN);
    expect(tipFee(101n, 250)).toBe(3n);
    expect(tipFee(1n, 1000)).toBe(1n);
  });

  test("content tiers come from locked stake", () => {
    expect(tierForLocked(999n * VCOIN)).toBe(0);
    expect(tierForLocked(1_000n * VCOIN)).toBe(1);
    expect(tierForLocked(100_000n * VCOIN)).toBe(4);
  });

  test("KYC needs the KYC claim; usernames", () => {
    expect(levelForClaims(CLAIMS.email | CLAIMS.phone | CLAIMS.social)).toBe(1);
    expect(levelForClaims(CLAIMS.kyc)).toBe(2);
    expect(isValidUsername("alice_1")).toBe(true);
    expect(isValidUsername("Al")).toBe(false);
  });

  test("VCoin amounts parse exactly", () => {
    expect(parseVcoin("1.5")).toBe(1_500_000_000n);
    expect(parseVcoin("0.000000001")).toBe(1n);
    expect(formatVcoin(1_500_000_000n)).toBe("1.5");
    expect(() => parseVcoin("1.0000000001")).toThrow();
  });
});
