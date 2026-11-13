/**
 * End to end: instructions built by this SDK, executed by the real programs (litesvm,
 * binaries from `target/deploy`). Proves the builders, PDAs, account order and the merkle
 * format against the chain, not just against the IDL.
 */
import { BN } from "@coral-xyz/anchor";
import { Connection, Keypair, PublicKey, SystemProgram, TransactionInstruction } from "@solana/web3.js";

import * as sdk from "../index";
import { Harness } from "./harness";

const p = sdk.createPrograms(new Connection("http://127.0.0.1:8899"));
const DAY = 86_400;

function transferCheckedIx(source: PublicKey, mint: PublicKey, dest: PublicKey, owner: PublicKey, amount: bigint): TransactionInstruction {
  const data = Buffer.alloc(10);
  data[0] = 12; // TransferChecked
  data.writeBigUInt64LE(amount, 1);
  data[9] = sdk.VCOIN_DECIMALS;
  return new TransactionInstruction({
    programId: sdk.TOKEN_2022_PROGRAM_ID,
    keys: [
      { pubkey: source, isSigner: false, isWritable: true },
      { pubkey: mint, isSigner: false, isWritable: false },
      { pubkey: dest, isSigner: false, isWritable: true },
      { pubkey: owner, isSigner: true, isWritable: false },
    ],
    data,
  });
}

/** VCoin, veVCoin and staking deployed and initialized; the harness admin is mint authority. */
async function stack(h: Harness): Promise<void> {
  const a = h.admin.publicKey;
  for (const [id, name] of [
    [sdk.PROGRAM_IDS.vcoinToken, "vcoin_token"],
    [sdk.PROGRAM_IDS.vevcoinToken, "vevcoin_token"],
    [sdk.PROGRAM_IDS.staking, "staking_protocol"],
  ] as const) {
    h.deployUpgradeable(id, name, a);
  }
  const common = { authority: a, tokenProgram: sdk.TOKEN_2022_PROGRAM_ID, systemProgram: SystemProgram.programId };
  const vcoinInit = await p.vcoinToken.methods
    .initialize("VCoin", "VIWO", "")
    .accountsStrict({
      ...common,
      config: sdk.pda.vcoinConfig(),
      mint: sdk.vcoinMint(),
      program: sdk.PROGRAM_IDS.vcoinToken,
      programData: Harness.programData(sdk.PROGRAM_IDS.vcoinToken),
    })
    .instruction();
  const vevcoinInit = await p.vevcoinToken.methods
    .initialize()
    .accountsStrict({
      ...common,
      config: sdk.pda.vevcoinConfig(),
      mint: sdk.vevcoinMint(),
      program: sdk.PROGRAM_IDS.vevcoinToken,
      programData: Harness.programData(sdk.PROGRAM_IDS.vevcoinToken),
    })
    .instruction();
  h.send([vcoinInit, vevcoinInit], [h.admin]);
  const registrar = await p.staking.methods
    .createRegistrar()
    .accountsStrict({
      ...common,
      registrar: sdk.pda.registrar(),
      vcoinMint: sdk.vcoinMint(),
      vevcoinMint: sdk.vevcoinMint(),
      vault: sdk.pda.vault(),
      program: sdk.PROGRAM_IDS.staking,
      programData: Harness.programData(sdk.PROGRAM_IDS.staking),
    })
    .instruction();
  h.send([registrar], [h.admin]);
}

/** Mints `amount` VCoin into `owner`'s ATA (created if needed). */
async function fundVcoin(h: Harness, owner: PublicKey, amount: bigint): Promise<PublicKey> {
  const mint = sdk.vcoinMint();
  const mintIx = await p.vcoinToken.methods
    .mintTokens(new BN(amount.toString()))
    .accountsStrict({
      config: sdk.pda.vcoinConfig(),
      mint,
      destination: sdk.associatedTokenAddress(owner, mint),
      authority: h.admin.publicKey,
      tokenProgram: sdk.TOKEN_2022_PROGRAM_ID,
    })
    .instruction();
  const createAta = sdk.createAtaIdempotentIx(h.admin.publicKey, owner, mint);
  h.send(amount > 0n ? [createAta, mintIx] : [createAta], [h.admin]);
  return sdk.associatedTokenAddress(owner, mint);
}

describe("SDK instructions on the real programs", () => {
  test("stake, compute weight locally, withdraw", async () => {
    const h = new Harness();
    await stack(h);
    const user = h.funded();
    const vcoin = await fundVcoin(h, user.publicKey, 1_000n * sdk.VCOIN);

    h.send(
      [
        await sdk.createVoterIx(p, user.publicKey),
        sdk.createAtaIdempotentIx(user.publicKey, user.publicKey, sdk.vevcoinMint()),
        await sdk.depositIx(p, { owner: user.publicKey, index: 0, amount: 1_000n * sdk.VCOIN, lockSecs: sdk.MAX_LOCK_SECS }),
      ],
      [user],
    );
    expect(h.tokenBalance(vcoin)).toBe(0n);
    expect(h.tokenBalance(sdk.receiptAta(user.publicKey))).toBe(1_000n * sdk.VCOIN);

    const voter = p.staking.coder.accounts.decode("voter", h.accountData(sdk.pda.voter(user.publicKey))!);
    const weight = sdk.voterWeight(sdk.toDeposits(voter), sdk.DEFAULT_WEIGHT_PARAMS, h.now());
    expect(weight).toBe(2_000n * sdk.VCOIN);

    // Locked: refused. After the lock ends (governance not wired yet): returned.
    const wd = await sdk.withdrawIx(p, { owner: user.publicKey, index: 0, amount: 1_000n * sdk.VCOIN, tokenOwnerRecord: Keypair.generate().publicKey });
    expect(h.sendFailing([wd], [user])).toMatch(/StillLocked|0x177/);
    h.warp(sdk.MAX_LOCK_SECS);
    h.send([wd], [user]);
    expect(h.tokenBalance(vcoin)).toBe(1_000n * sdk.VCOIN);
    expect(h.tokenBalance(sdk.receiptAta(user.publicKey))).toBe(0n);
  });

  test("an SSCRE epoch pays claims built from the TypeScript merkle tree", async () => {
    const h = new Harness();
    await stack(h);
    const a = h.admin;
    h.deployUpgradeable(sdk.PROGRAM_IDS.sscre, "sscre_protocol", a.publicKey);
    const feeDestination = await fundVcoin(h, a.publicKey, 0n);
    const init = await p.sscre.methods
      .initializePool(
        { epochDuration: new BN(30 * DAY), claimWindow: new BN(90 * DAY), disputeDelay: new BN(DAY), decayBps: 200 },
        new BN((1_000_000n * sdk.VCOIN).toString()),
        100,
      )
      .accountsStrict({
        authority: a.publicKey,
        config: sdk.pda.poolConfig(),
        vcoinMint: sdk.vcoinMint(),
        reserveVault: sdk.pda.reserveVault(),
        feeVault: sdk.pda.sscreFeeVault(),
        feeDestination,
        program: sdk.PROGRAM_IDS.sscre,
        programData: Harness.programData(sdk.PROGRAM_IDS.sscre),
        tokenProgram: sdk.TOKEN_2022_PROGRAM_ID,
        systemProgram: SystemProgram.programId,
      })
      .instruction();
    h.send([init], [a]);
    const source = await fundVcoin(h, a.publicKey, 5_000_000n * sdk.VCOIN);
    h.send([transferCheckedIx(source, sdk.vcoinMint(), sdk.pda.reserveVault(), a.publicKey, 5_000_000n * sdk.VCOIN)], [a]);
    const oracles = [h.funded(), h.funded()];
    for (const o of oracles) {
      h.send([await p.sscre.methods.registerOracle(o.publicKey).accountsStrict({ config: sdk.pda.poolConfig(), authority: a.publicKey }).instruction()], [a]);
    }
    h.warp(DAY);

    h.send([await sdk.startEpochIx(p, 0n, a.publicKey)], [a]);
    const claimants = [Keypair.generate(), Keypair.generate(), Keypair.generate()];
    const tree = new sdk.MerkleTree(
      0n,
      claimants.map((c, i) => ({ claimant: c.publicKey, amount: BigInt(i + 1) * 100n * sdk.VCOIN })),
      1_000_000n * sdk.VCOIN,
    );
    h.warp(30 * DAY);
    for (const o of oracles) h.send([await sdk.voteRootIx(p, o.publicKey, 0n, tree.root(), tree.total)], [o]);
    h.warp(DAY);
    h.send([await sdk.activateRootIx(p, 0n)], [a]);

    const relayer = h.funded();
    for (const c of claimants) {
      const token = await fundVcoin(h, c.publicKey, 0n);
      const { amount, proof } = tree.proofFor(c.publicKey);
      const ix = await sdk.claimIx(p, { epochIndex: 0n, claimant: c.publicKey, claimantToken: token, amount, proof, payer: relayer.publicKey });
      h.send([ix], [relayer, c]);
      expect(h.tokenBalance(token)).toBe(amount - sdk.claimFee(amount, 100));
    }
    const epoch = p.sscre.coder.accounts.decode("epoch", h.accountData(sdk.pda.epoch(0n))!);
    expect(BigInt(epoch.totalClaimed.toString())).toBe(tree.total);

    // Fees collect in the program's vault; anyone sweeps them to the fee account.
    const fees = sdk.claimFee(tree.total, 100);
    expect(h.tokenBalance(sdk.pda.sscreFeeVault())).toBe(fees);
    h.send([await sdk.withdrawFeesIx(p, feeDestination)], [relayer]);
    expect(h.tokenBalance(feeDestination)).toBe(fees);
  });
});
