/**
 * In-process Solana (litesvm) running the real program binaries from `target/deploy`, driven
 * with web3.js transactions built by this SDK. Run `anchor build` first.
 */
import { readFileSync } from "fs";
import { join } from "path";

import { address, getTransactionDecoder, lamports } from "@solana/kit";
import { Keypair, PublicKey, Transaction, TransactionInstruction } from "@solana/web3.js";
import { FailedTransactionMetadata, LiteSVM, type TransactionMetadata } from "litesvm";

const DEPLOY_DIR = join(__dirname, "../../../../target/deploy");
const BPF_UPGRADEABLE_LOADER = new PublicKey("BPFLoaderUpgradeab1e11111111111111111111111");

export const SOL = 1_000_000_000n;

export class Harness {
  readonly svm = new LiteSVM();
  readonly admin = Keypair.generate();

  constructor() {
    this.airdrop(this.admin.publicKey, 100n * SOL);
  }

  airdrop(to: PublicKey, amount: bigint): void {
    this.svm.airdrop(address(to.toBase58()), lamports(amount));
  }

  funded(sol = 10n): Keypair {
    const kp = Keypair.generate();
    this.airdrop(kp.publicKey, sol * SOL);
    return kp;
  }

  setAccount(key: PublicKey, data: Uint8Array, owner: PublicKey, executable: boolean): void {
    this.svm.setAccount({
      address: address(key.toBase58()),
      data,
      executable,
      lamports: lamports(this.svm.minimumBalanceForRentExemption(BigInt(data.length))),
      programAddress: address(owner.toBase58()),
      space: BigInt(data.length),
    });
  }

  /** Deploys under the upgradeable loader with `authority` as upgrade authority (ProgramData first). */
  deployUpgradeable(programId: PublicKey, name: string, authority: PublicKey): void {
    const so = readFileSync(join(DEPLOY_DIR, `${name}.so`));
    const [programData] = PublicKey.findProgramAddressSync([programId.toBuffer()], BPF_UPGRADEABLE_LOADER);
    const header = Buffer.alloc(45); // enum tag u32 = 3, slot u64, Option<Pubkey>
    header.writeUInt32LE(3, 0);
    header[12] = 1;
    authority.toBuffer().copy(header, 13);
    this.setAccount(programData, Buffer.concat([header, so]), BPF_UPGRADEABLE_LOADER, false);
    const program = Buffer.alloc(36); // enum tag u32 = 2, programdata address
    program.writeUInt32LE(2, 0);
    programData.toBuffer().copy(program, 4);
    this.setAccount(programId, program, BPF_UPGRADEABLE_LOADER, true);
  }

  static programData(programId: PublicKey): PublicKey {
    return PublicKey.findProgramAddressSync([programId.toBuffer()], BPF_UPGRADEABLE_LOADER)[0];
  }

  /** Sends; throws with the logs on failure. The first signer pays. */
  send(instructions: TransactionInstruction[], signers: Keypair[]): TransactionMetadata {
    const tx = new Transaction().add(...instructions);
    tx.feePayer = signers[0].publicKey;
    tx.recentBlockhash = this.svm.latestBlockhash();
    tx.sign(...signers);
    const result = this.svm.sendTransaction(getTransactionDecoder().decode(tx.serialize()));
    this.svm.expireBlockhash();
    if (result instanceof FailedTransactionMetadata) {
      throw new Error(`transaction failed: ${result.err().toString()}\n${result.meta().logs().join("\n")}`);
    }
    return result;
  }

  /** Expects failure; returns the logs for assertions. */
  sendFailing(instructions: TransactionInstruction[], signers: Keypair[]): string {
    try {
      this.send(instructions, signers);
    } catch (e) {
      return (e as Error).message;
    }
    throw new Error("expected the transaction to fail");
  }

  accountData(key: PublicKey): Buffer | null {
    const a = this.svm.getAccount(address(key.toBase58()));
    return a.exists ? Buffer.from(a.data) : null;
  }

  tokenBalance(account: PublicKey): bigint {
    const data = this.accountData(account);
    return data ? data.readBigUInt64LE(64) : 0n;
  }

  now(): bigint {
    return this.svm.getClock().unixTimestamp;
  }

  warp(seconds: number | bigint): void {
    const clock = this.svm.getClock();
    clock.unixTimestamp = clock.unixTimestamp + BigInt(seconds);
    clock.slot = clock.slot + 1n;
    this.svm.setClock(clock);
  }
}
