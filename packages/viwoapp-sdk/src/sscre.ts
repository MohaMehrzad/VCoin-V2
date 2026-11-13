/**
 * sscre-protocol: per-epoch merkle reward claims. The leaf format matches
 * `programs/sscre-protocol/src/merkle.rs` and `docs/offchain/merkle-leaf-format.md`; the
 * frozen test vector is checked in this package's tests (audit SDK-07, SSC-03).
 */
import { BN } from "./anchor";
import { keccak_256 } from "@noble/hashes/sha3";
import { PublicKey, SystemProgram, TransactionInstruction } from "@solana/web3.js";

import { pda } from "./pda";
import type { Programs } from "./programs";
import { TOKEN_2022_PROGRAM_ID, vcoinMint } from "./tokens";

export const LEAF_DOMAIN = Buffer.from("SSCRE_CLAIM_V1");
export const MAX_PROOF_LEN = 32;

function u64le(n: bigint): Buffer {
  const b = Buffer.alloc(8);
  b.writeBigUInt64LE(n);
  return b;
}

/** keccak256("SSCRE_CLAIM_V1" ‖ claimant ‖ amount u64 LE ‖ epoch u64 LE). */
export function computeLeaf(claimant: PublicKey, amount: bigint, epoch: bigint): Buffer {
  return Buffer.from(keccak_256(Buffer.concat([LEAF_DOMAIN, claimant.toBuffer(), u64le(amount), u64le(epoch)])));
}

/** keccak256(min ‖ max), byte-wise ordering. */
export function hashPair(a: Uint8Array, b: Uint8Array): Buffer {
  const [lo, hi] = Buffer.compare(Buffer.from(a), Buffer.from(b)) <= 0 ? [a, b] : [b, a];
  return Buffer.from(keccak_256(Buffer.concat([lo, hi])));
}

export function verifyProof(proof: Uint8Array[], root: Uint8Array, leaf: Uint8Array): boolean {
  return Buffer.from(proof.reduce<Uint8Array>((node, sib) => hashPair(node, sib), leaf)).equals(Buffer.from(root));
}

export interface Claim {
  claimant: PublicKey;
  /** Gross amount in base units (the fee is taken on claim). */
  amount: bigint;
}

/**
 * Reference tree, identical to `sscre_protocol::merkle::builder::Tree`: leaves in input order,
 * an odd node is promoted unchanged. Rejects what the program could not pay (audit SSC-12).
 */
export class MerkleTree {
  readonly total: bigint;
  private readonly layers: Buffer[][];

  constructor(readonly epoch: bigint, readonly claims: Claim[], maxTotalClaim: bigint) {
    if (claims.length === 0) throw new Error("empty claim list");
    const seen = new Set<string>();
    let total = 0n;
    for (const c of claims) {
      if (c.amount <= 0n) throw new Error(`zero amount for ${c.claimant.toBase58()}`);
      if (seen.has(c.claimant.toBase58())) throw new Error(`duplicate claimant ${c.claimant.toBase58()}`);
      seen.add(c.claimant.toBase58());
      total += c.amount;
    }
    if (total > maxTotalClaim) throw new Error(`claims total ${total} exceed max_total_claim ${maxTotalClaim}`);
    this.total = total;
    this.layers = [claims.map((c) => computeLeaf(c.claimant, c.amount, epoch))];
    while (this.layers[this.layers.length - 1].length > 1) {
      const prev = this.layers[this.layers.length - 1];
      const next: Buffer[] = [];
      for (let i = 0; i < prev.length; i += 2) next.push(i + 1 < prev.length ? hashPair(prev[i], prev[i + 1]) : prev[i]);
      this.layers.push(next);
    }
  }

  root(): Buffer {
    return this.layers[this.layers.length - 1][0];
  }

  proof(index: number): Buffer[] {
    const proof: Buffer[] = [];
    let i = index;
    for (const layer of this.layers.slice(0, -1)) {
      const sibling = i ^ 1;
      if (sibling < layer.length) proof.push(layer[sibling]);
      i = Math.floor(i / 2);
    }
    return proof;
  }

  proofFor(claimant: PublicKey): { amount: bigint; proof: Buffer[] } {
    const i = this.claims.findIndex((c) => c.claimant.equals(claimant));
    if (i < 0) throw new Error(`${claimant.toBase58()} has no claim in epoch ${this.epoch}`);
    return { amount: this.claims[i].amount, proof: this.proof(i) };
  }
}

/** Fee the program deducts (rounded up). */
export function claimFee(amount: bigint, feeBps: number): bigint {
  return (amount * BigInt(feeBps) + 9_999n) / 10_000n;
}

/**
 * Claims for `claimant` (who signs); `payer` (e.g. a relayer) funds the claim marker. The fee
 * goes to the program's own fee vault, so a claim depends on no outside account.
 */
export async function claimIx(
  p: Programs,
  args: {
    epochIndex: bigint;
    claimant: PublicKey;
    claimantToken: PublicKey;
    amount: bigint;
    proof: Uint8Array[];
    payer: PublicKey;
  },
): Promise<TransactionInstruction> {
  if (args.proof.length > MAX_PROOF_LEN) throw new Error("proof too long");
  const epoch = pda.epoch(args.epochIndex);
  return p.sscre.methods
    .claim(new BN(args.amount.toString()), args.proof.map((x) => Array.from(x)))
    .accountsStrict({
      config: pda.poolConfig(),
      epoch,
      vault: pda.epochVault(args.epochIndex),
      vcoinMint: vcoinMint(),
      claimant: args.claimant,
      claimantToken: args.claimantToken,
      feeVault: pda.sscreFeeVault(),
      claimStatus: pda.claimStatus(epoch, args.claimant),
      payer: args.payer,
      tokenProgram: TOKEN_2022_PROGRAM_ID,
      systemProgram: SystemProgram.programId,
    })
    .instruction();
}

/** Permissionless: sweeps collected claim fees to the pool's configured fee account. */
export function withdrawFeesIx(p: Programs, feeDestination: PublicKey): Promise<TransactionInstruction> {
  return p.sscre.methods
    .withdrawFees()
    .accountsStrict({
      config: pda.poolConfig(),
      feeVault: pda.sscreFeeVault(),
      feeDestination,
      vcoinMint: vcoinMint(),
      tokenProgram: TOKEN_2022_PROGRAM_ID,
    })
    .instruction();
}

/**
 * Funds epoch `index` from the reserve. Anyone may call it when the reserve covers the
 * scheduled budget. A smaller epoch needs the pool authority to sign as `authority` (it can be
 * a governance PDA; `payer` only funds the rent).
 */
export function startEpochIx(
  p: Programs,
  index: bigint,
  payer: PublicKey,
  authority: PublicKey | null = null,
): Promise<TransactionInstruction> {
  return p.sscre.methods
    .startEpoch()
    .accountsStrict({
      config: pda.poolConfig(),
      reserveVault: pda.reserveVault(),
      vcoinMint: vcoinMint(),
      epoch: pda.epoch(index),
      epochVault: pda.epochVault(index),
      payer,
      authority,
      tokenProgram: TOKEN_2022_PROGRAM_ID,
      systemProgram: SystemProgram.programId,
    })
    .instruction();
}

export function voteRootIx(p: Programs, oracle: PublicKey, index: bigint, root: Uint8Array, maxTotalClaim: bigint): Promise<TransactionInstruction> {
  return p.sscre.methods
    .voteRoot(Array.from(root), new BN(maxTotalClaim.toString()))
    .accountsStrict({ config: pda.poolConfig(), epoch: pda.epoch(index), oracle })
    .instruction();
}

export function activateRootIx(p: Programs, index: bigint): Promise<TransactionInstruction> {
  return p.sscre.methods.activateRoot().accountsStrict({ config: pda.poolConfig(), epoch: pda.epoch(index) }).instruction();
}

export function clawbackIx(p: Programs, index: bigint, rentPayer: PublicKey): Promise<TransactionInstruction> {
  return p.sscre.methods
    .clawback()
    .accountsStrict({
      config: pda.poolConfig(),
      epoch: pda.epoch(index),
      vault: pda.epochVault(index),
      reserveVault: pda.reserveVault(),
      vcoinMint: vcoinMint(),
      rentPayer,
      tokenProgram: TOKEN_2022_PROGRAM_ID,
    })
    .instruction();
}
