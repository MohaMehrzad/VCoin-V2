/** VCoin / veVCoin mints and Token-2022 associated token accounts. */
import { PublicKey, SystemProgram, TransactionInstruction } from "@solana/web3.js";

import { pda } from "./pda";

export const VCOIN_DECIMALS = 9;
/** One VCoin in base units. */
export const VCOIN = 1_000_000_000n;

export const TOKEN_2022_PROGRAM_ID = new PublicKey("TokenzQdBNbLqP5VEhdkAS6EPFLC1PHnBqCXEpPxuEb");
export const TOKEN_PROGRAM_ID = new PublicKey("TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA");
export const ASSOCIATED_TOKEN_PROGRAM_ID = new PublicKey("ATokenGPvbdGVxr1b2hvZbsiqW5xWH25efTNsLJA8knL");

/** The VCoin mint is a PDA of vcoin-token, never a configurable address (audit SDK-15). */
export function vcoinMint(): PublicKey {
  return pda.vcoinMint();
}

export function vevcoinMint(): PublicKey {
  return pda.vevcoinMint();
}

export function associatedTokenAddress(
  owner: PublicKey,
  mint: PublicKey,
  tokenProgram: PublicKey = TOKEN_2022_PROGRAM_ID,
): PublicKey {
  return PublicKey.findProgramAddressSync(
    [owner.toBuffer(), tokenProgram.toBuffer(), mint.toBuffer()],
    ASSOCIATED_TOKEN_PROGRAM_ID,
  )[0];
}

/** The owner's veVCoin receipt account: staking mints receipts only into this ATA. */
export function receiptAta(owner: PublicKey): PublicKey {
  return associatedTokenAddress(owner, vevcoinMint());
}

/** `CreateIdempotent`: creates the ATA if missing, no-op otherwise. */
export function createAtaIdempotentIx(
  payer: PublicKey,
  owner: PublicKey,
  mint: PublicKey,
  tokenProgram: PublicKey = TOKEN_2022_PROGRAM_ID,
): TransactionInstruction {
  return new TransactionInstruction({
    programId: ASSOCIATED_TOKEN_PROGRAM_ID,
    keys: [
      { pubkey: payer, isSigner: true, isWritable: true },
      { pubkey: associatedTokenAddress(owner, mint, tokenProgram), isSigner: false, isWritable: true },
      { pubkey: owner, isSigner: false, isWritable: false },
      { pubkey: mint, isSigner: false, isWritable: false },
      { pubkey: SystemProgram.programId, isSigner: false, isWritable: false },
      { pubkey: tokenProgram, isSigner: false, isWritable: false },
    ],
    data: Buffer.from([1]),
  });
}

/** Decimal string → base units, exact (no floating point). */
export function parseVcoin(amount: string): bigint {
  const m = /^(\d+)(?:\.(\d{0,9}))?$/.exec(amount.trim());
  if (!m) throw new Error(`invalid VCoin amount: ${amount}`);
  return BigInt(m[1]) * VCOIN + BigInt((m[2] ?? "").padEnd(9, "0"));
}

export function formatVcoin(baseUnits: bigint): string {
  const whole = baseUnits / VCOIN;
  const frac = (baseUnits % VCOIN).toString().padStart(9, "0").replace(/0+$/, "");
  return frac ? `${whole}.${frac}` : `${whole}`;
}
