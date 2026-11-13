import { Connection, Keypair, TransactionInstruction } from "@solana/web3.js";

import * as sdk from "../index";

const p = sdk.createPrograms(new Connection("http://127.0.0.1:8899"));
const key = () => Keypair.generate().publicKey;

interface IdlAccount {
  name: string;
  accounts?: IdlAccount[];
}
interface IdlIx {
  name: string;
  discriminator: number[];
  accounts: IdlAccount[];
}

function flatten(accounts: IdlAccount[]): number {
  return accounts.reduce((n, a) => n + (a.accounts ? flatten(a.accounts) : 1), 0);
}

/** The instruction carries the IDL discriminator and one meta per IDL account. */
function expectMatchesIdl(ix: TransactionInstruction, program: sdk.ProgramName, name: string) {
  const idl = sdk.IDLS[program] as unknown as { instructions: IdlIx[] };
  const def = idl.instructions.find((i) => i.name === name);
  if (!def) throw new Error(`${name} not in ${program}`);
  expect(ix.programId).toEqual(sdk.PROGRAM_IDS[program]);
  expect([...ix.data.subarray(0, 8)]).toEqual(def.discriminator);
  expect(ix.keys).toHaveLength(flatten(def.accounts));
}

describe("builders produce the IDL's instructions", () => {
  test("staking", async () => {
    const owner = key();
    expectMatchesIdl(await sdk.createVoterIx(p, owner), "staking", "create_voter");
    expectMatchesIdl(await sdk.deactivateExpiredIx(p, owner, 0), "staking", "deactivate_expired");
    const dep = await sdk.depositIx(p, { owner, index: 0, amount: 5n * sdk.VCOIN, lockSecs: sdk.MIN_LOCK_SECS });
    expectMatchesIdl(dep, "staking", "deposit");
    expect(dep.keys[1].pubkey).toEqual(sdk.pda.voter(owner));
    expect(dep.keys[7].pubkey).toEqual(sdk.receiptAta(owner));
    const wd = await sdk.withdrawIx(p, { owner, index: 0, amount: 1n, tokenOwnerRecord: key() });
    expectMatchesIdl(wd, "staking", "withdraw");
    const vwr = await sdk.updateVoterWeightRecordIx(p, owner, "castVote", key());
    expectMatchesIdl(vwr, "staking", "update_voter_weight_record");
    expectMatchesIdl(await sdk.updateMaxVoterWeightRecordIx(p), "staking", "update_max_voter_weight_record");
  });

  test("withVoterWeight orders plugin updates before the Realms instruction", async () => {
    const realmsIx = new TransactionInstruction({ programId: sdk.SPL_GOVERNANCE_PROGRAM_ID, keys: [], data: Buffer.alloc(1) });
    const ixs = await sdk.withVoterWeight(p, key(), "castVote", key(), realmsIx);
    expect(ixs.map((i) => i.programId.toBase58())).toEqual([
      sdk.PROGRAM_IDS.staking.toBase58(),
      sdk.PROGRAM_IDS.staking.toBase58(),
      sdk.SPL_GOVERNANCE_PROGRAM_ID.toBase58(),
    ]);
    expect(await sdk.withVoterWeight(p, key(), "createProposal", key(), realmsIx)).toHaveLength(2);
  });

  test("private poll", async () => {
    const poll = sdk.pda.poll(0n);
    const ix = await sdk.castVoteIx(p, { pollAddress: poll, owner: key(), ballot: new Uint8Array(192), proof: new Uint8Array(448) });
    expectMatchesIdl(ix, "privatePoll", "cast_vote");
    await expect(sdk.castVoteIx(p, { pollAddress: poll, owner: key(), ballot: new Uint8Array(96), proof: new Uint8Array(448) })).rejects.toThrow();
    expectMatchesIdl(await sdk.assertPollPassedIx(p, poll, key()), "privatePoll", "assert_poll_passed");
  });

  test("sscre", async () => {
    const claim = await sdk.claimIx(p, {
      epochIndex: 3n,
      claimant: key(),
      claimantToken: key(),
      amount: 10n,
      proof: [new Uint8Array(32)],
      payer: key(),
    });
    expectMatchesIdl(claim, "sscre", "claim");
    await expect(
      sdk.claimIx(p, { epochIndex: 0n, claimant: key(), claimantToken: key(), amount: 1n, proof: Array(33).fill(new Uint8Array(32)), payer: key() }),
    ).rejects.toThrow("proof too long");
    expect(claim.keys[6].pubkey).toEqual(sdk.pda.sscreFeeVault());
    expectMatchesIdl(await sdk.startEpochIx(p, 0n, key()), "sscre", "start_epoch");
    expectMatchesIdl(await sdk.withdrawFeesIx(p, key()), "sscre", "withdraw_fees");
  });

  test("content with and without a session", async () => {
    const author = key();
    const id = new Uint8Array(32).fill(4);
    const own = await sdk.createContentIx(p, { author, trackingId: id, contentHash: id, uri: "ipfs://x", contentType: 0 });
    expectMatchesIdl(own, "content", "create_content");
    expect(own.keys[3].pubkey).toEqual(sdk.PROGRAM_IDS.content); // optional session = None
    const session = key();
    const bySession = await sdk.createContentIx(p, { author, signer: session, trackingId: id, contentHash: id, uri: "u", contentType: 0 });
    expect(bySession.keys[3].pubkey).toEqual(sdk.pda.session(sdk.PROGRAM_IDS.content, session, author));
    expect(bySession.keys[5].pubkey).toEqual(sdk.pda.voter(author));
    expect(own.keys[5].pubkey).toEqual(sdk.pda.voter(author)); // always passed, never optional
    expectMatchesIdl(await sdk.editContentIx(p, { author, trackingId: id, contentHash: id, uri: "u" }), "content", "edit_content");
    expectMatchesIdl(await sdk.claimRefundIx(p, author, id), "content", "claim_refund");
    const sponsor = key();
    const del = await sdk.deleteContentIx(p, author, id, sponsor);
    expectMatchesIdl(del, "content", "delete_content");
    expect(del.keys[2].pubkey).toEqual(sponsor);
    expectMatchesIdl(await sdk.initUserEnergyIx(p, author, author), "content", "init_user_energy");
  });

  test("vilink, identity, five-a, gasless", async () => {
    const creator = key();
    expectMatchesIdl(
      await sdk.createActionIx(p, { creator, nonce: 0n, actionType: 0, amount: sdk.VCOIN, target: key(), expirySecs: 3600, maxExecutions: 1 }),
      "vilink",
      "create_action",
    );
    const action = sdk.pda.action(creator, 0n);
    expectMatchesIdl(await sdk.executeTipIx(p, { action, target: key(), executor: key(), treasury: key() }), "vilink", "execute_tip");
    expectMatchesIdl(await sdk.executeSocialIx(p, "follow", { action, target: key(), executor: key() }), "vilink", "execute_follow");
    expectMatchesIdl(await sdk.createIdentityIx(p, key(), new Uint8Array(32), "alice"), "identity", "create_identity");
    expect(() => sdk.createIdentityIx(p, key(), new Uint8Array(32), "Alice")).toThrow();
    expectMatchesIdl(await sdk.vouchForUserIx(p, key(), key()), "fiveA", "vouch_for_user");
    expectMatchesIdl(
      await sdk.createSessionIx(p, { authority: key(), sessionSigner: key(), payer: key(), targetProgram: sdk.PROGRAM_IDS.content, scope: sdk.SCOPE_CONTENT, validUntil: 1 }),
      "gasless",
      "create_session",
    );
  });

  test("empty transactions are refused", async () => {
    await expect(sdk.buildTransaction(new Connection("http://127.0.0.1:8899"), key(), [])).rejects.toThrow("no instructions");
  });
});
