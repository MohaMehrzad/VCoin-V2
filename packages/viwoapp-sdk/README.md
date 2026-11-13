# @viwoapp/sdk v3

TypeScript client for the VCoin programs on Solana. v3 is a rewrite: every account decoder and
instruction encoder comes from the programs' Anchor IDLs, which are committed in `src/idl` and
checked against `anchor build` in CI. The v2 SDK's hand-written byte offsets, wrong seeds and
empty transactions are gone.

```bash
npm install @viwoapp/sdk @coral-xyz/anchor@^0.32.1 @solana/web3.js@^1
```

`@coral-xyz/anchor` and `@solana/web3.js` are peer dependencies. The package ships CommonJS
and ESM builds.

## Programs and addresses

```ts
import { Connection } from "@solana/web3.js";
import { createPrograms, pda, PROGRAM_IDS, vcoinMint } from "@viwoapp/sdk";

const connection = new Connection("https://api.devnet.solana.com");
const programs = createPrograms(connection); // typed Program<IDL> per program, no wallet needed

const registrar = await programs.staking.account.registrar.fetch(pda.registrar());
const mint = vcoinMint(); // a PDA of vcoin-token, never a configurable address
```

`createPrograms` exposes every instruction, including admin ones (which are executed through
Realms proposals once authority is handed to the DAO). The builders below cover user flows.

## Staking and Realms voting

```ts
import {
  createAtaIdempotentIx, createVoterIx, depositIx, sendInstructions,
  vevcoinMint, withVoterWeight, withdrawIx, tokenOwnerRecordAddress, VCOIN, MAX_LOCK_SECS,
} from "@viwoapp/sdk";

await sendInstructions(connection, [
  await createVoterIx(programs, owner.publicKey),
  createAtaIdempotentIx(owner.publicKey, owner.publicKey, vevcoinMint()), // receipt account
  await depositIx(programs, { owner: owner.publicKey, index: 0, amount: 1_000n * VCOIN, lockSecs: MAX_LOCK_SECS }),
], [owner]);
```

- Voting weight = amount × (1 + remaining lock / 4 years). It decays to 0 when fewer than
  4 days of lock remain, and is never multiplied by reputation.
  `ViWoClient#votingPower` computes it locally with the same rounding as the program.
- Votes happen in Realms. Build the Realms instruction with `@solana/spl-governance` and
  prepend the plugin update in the same transaction:

```ts
const castVote = /* spl-governance CastVote for `proposal` */;
const ixs = await withVoterWeight(programs, owner.publicKey, "castVote", proposal, castVote);
```

- Withdrawals pass the owner's TokenOwnerRecord, and are refused while that record has
  unrelinquished votes. Relinquish first with `@solana/spl-governance`:

```ts
const tor = tokenOwnerRecordAddress(realm, vcoinMint(), owner.publicKey);
await withdrawIx(programs, { owner: owner.publicKey, index: 0, amount, tokenOwnerRecord: tor });
```

## Private polls

Ballots are encrypted and proven by the Rust `zk-voting-sdk`. Build its wasm with:

```bash
wasm-pack build packages/zk-voting-sdk --features wasm
```

Then pass its `encryptBallot` export:

```ts
import { encryptAndCastIx } from "@viwoapp/sdk";
import { encryptBallot } from "zk-voting-sdk"; // the wasm package

const ix = await encryptAndCastIx(programs, encryptBallot, pollAddress, owner.publicKey, 0 /* For */);
```

The weight is read from your staking deposits at the poll's end time, in whole VCoin;
`expectedPollWeight` shows it in advance. A Realms proposal can require a passed poll by
including `assertPollPassedIx(programs, poll, governancePda)`.

## Reward claims (SSCRE)

```ts
import { MerkleTree, claimIx } from "@viwoapp/sdk";

// Oracles: build the epoch's tree (same format as the program; see docs/offchain/merkle-leaf-format.md)
const tree = new MerkleTree(epochIndex, claims, maxTotalClaim);
// Users: claim with the published proof; a relayer may pay the rent.
const { amount, proof } = tree.proofFor(user);
const ix = await claimIx(programs, { epochIndex, claimant: user, claimantToken, amount, proof, payer: relayer });
```

Claim fees collect in the program's fee vault; anyone can sweep them to the pool's fee account
with `withdrawFeesIx`. `deactivateExpiredIx` (staking) is a permissionless crank that stops an
expired deposit from counting toward the max voter weight.

## Session keys, content, ViLink, identity, 5A

- `createSessionIx` opens a ≤ 24h session key, scoped to one program and to content, vouch
  or follow actions.
- `createContentIx` / `editContentIx` accept an optional session `signer`. The energy tier
  comes from your staked VCoin; the builders always pass the staking account for you.
- `deleteContentIx(programs, author, trackingId, rentPayer)` returns the record's rent to
  whoever paid it. `rentPayer` is required: read it from `ContentRecord.rentPayer`.
- `createActionIx`, `executeTipIx` (wallet only) and `executeSocialIx` (wallet or session) cover
  ViLink.
- `createIdentityIx`, `linkAttestationIx` (co-signed by a trusted attester) and `subscribeIx`
  (with a `maxPayment` cap) cover identity.
- `vouchForUserIx` covers 5A vouching.

## Transactions

`buildTransaction` and `sendInstructions` use your connection and signers, and refuse empty
instruction lists.

## Development

```bash
anchor build                       # from the repo root; tests run the real programs
node scripts/sync-idl.mjs          # refresh src/idl after program changes
npm run typecheck && npm run lint && npm test && npm run build && npm run smoke
```

`npm test` includes end-to-end tests: SDK-built instructions are executed by the program binaries
in-process (litesvm). These cover staking with local weight, and an SSCRE epoch claimed with
TypeScript merkle proofs.
