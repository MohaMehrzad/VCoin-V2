/** Transaction assembly. Uses the caller's connection and signers only. */
import {
  Connection,
  PublicKey,
  type Signer,
  TransactionInstruction,
  TransactionMessage,
  VersionedTransaction,
} from "@solana/web3.js";

/**
 * Builds a v0 transaction. Refuses an empty instruction list, which v2 silently sent as an
 * empty transaction (audit SDK-01).
 */
export async function buildTransaction(
  connection: Connection,
  payer: PublicKey,
  instructions: TransactionInstruction[],
): Promise<VersionedTransaction> {
  if (instructions.length === 0) throw new Error("refusing to build a transaction with no instructions");
  const { blockhash } = await connection.getLatestBlockhash();
  const message = new TransactionMessage({ payerKey: payer, recentBlockhash: blockhash, instructions }).compileToV0Message();
  return new VersionedTransaction(message);
}

/** Builds, signs with `signers` (the first pays), sends and confirms. */
export async function sendInstructions(
  connection: Connection,
  instructions: TransactionInstruction[],
  signers: Signer[],
): Promise<string> {
  if (signers.length === 0) throw new Error("at least one signer (the fee payer) is required");
  const tx = await buildTransaction(connection, signers[0].publicKey, instructions);
  tx.sign(signers);
  const signature = await connection.sendTransaction(tx);
  const { blockhash, lastValidBlockHeight } = await connection.getLatestBlockhash();
  const result = await connection.confirmTransaction({ signature, blockhash, lastValidBlockHeight });
  if (result.value.err) throw new Error(`transaction ${signature} failed: ${JSON.stringify(result.value.err)}`);
  return signature;
}
