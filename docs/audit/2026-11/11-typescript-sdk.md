# 11 — TypeScript SDK (`packages/viwoapp-sdk`, published as `@viwoapp/sdk@2.0.1`)

**Tools run:** `npm ci --ignore-scripts`, `tsc --noEmit`, `jest` (120/120 pass), `npm run build`, and `npm pack @viwoapp/sdk@2.0.1`. The tests pass because they assert the wrong values.

**Verdict:** ❌
- The SDK cannot send any instruction.
- Most account decoders read the wrong byte offsets.
- Three PDA seeds and two enums are wrong.
- ESM and subpath imports of the published package are broken.

An app built on it would show wrong balances, wrong vote choices and wrong statuses, and report "success" for transactions that do nothing.

**Offsets:** byte offsets include the 8-byte discriminator, computed from the Rust structs.

---

### SDK-01 · High · Confirmed · stub — Every `build*Transaction` returns an empty transaction
- **Where:** all 23 public builders; the 26 instruction calls are commented out.
  - Example: `packages/viwoapp-sdk/src/staking/index.ts:209-216`.
  - Also governance `:300-305,329-335,351-356,419-437,456-465,511-520,557-567,601-612,641-650`; rewards `:312-317`; vilink `:314-357,390-396`; gasless `:384-437`; identity `:117-169`; fivea `:242-248`; content `:262-319`.
- **Code:** `const tx = new Transaction(); // tx.add(await this.program.methods.stake(...)...); return tx;`
- **Fails when:** an app follows the README (`README.md:338-342`) and calls `client.sendTransaction(stakeTx)` (`client.ts:145-168`).
  - web3.js only warns `No instructions provided`, then signs and sends.
  - The user pays a fee and gets back a success signature, but nothing was staked, voted, claimed or tipped.
  - The success itself is Likely, not run.
- **If the calls were uncommented, the signatures would still be wrong:**
  - `unstake` needs an `amount`.
  - `create_proposal` takes `(title_hash, description_uri, proposal_type, enable_private_voting)`.
  - `create_identity` needs a `username`.
  - `create_content` needs a `tracking_id`.
  - `subscribe` allows tiers 0–3 only.
- **Lead-verified.**
- **Fix:**
  - Build instructions from the Anchor IDL. This first needs TOOL-3 fixed.
  - Until then, make every builder throw, and make `sendTransaction` reject transactions with 0 instructions.

### SDK-02 · High · Confirmed · bug — Published ESM and subpath exports point to files that don't exist
- See **TOOL-10**. Lead-verified with `npm pack`.

### SDK-03 · High · Confirmed · bug — The ViLinkAction decoder makes every action look expired
- **Where:** `src/vilink/index.ts:108-111`, used at `:207` and `:380-383`.
- **Code:** `expiresAt: new BN(data.slice(145, 153), "le"), executed: data[153] !== 0, executionCount: data.readUInt32LE(193),`
- **On-chain layout:**
  - `created_at` 145, `expires_at` 153, `executed` 161, `executor` 162, `executed_at` 194.
  - `content_id: Option<[u8;32]>` is at 202, so `execution_count` is at 236 or 268.
- **Fails when:** always. `expiresAt` is really `created_at`, which is never in the future, so `isActionValid` always says "expired" and the tip builder always throws. `one_time` isn't decoded at all.
- **Fix:** correct the offsets, branch on the Option tag, or use the IDL coder.

### SDK-04 · High · Confirmed · bug — The UserStake decoder is wrong, and `canUnstake` throws for about 255 in 256 users
- **Where:**
  - `src/staking/index.ts:78-82,163`
  - `src/client.ts:216-217`
- **Code:** `vevcoinBalance: …slice(48, 56)…, tier: data[56], lockEndTime: …slice(57, 65)…`
- **On-chain layout:** `lock_duration` 48, `lock_end` 56, `stake_start` 64, `tier` 72, `ve_vcoin_amount` 73. Governance reads the same offsets.
- **Fails when:**
  - `vevcoinBalance` is really `lock_duration`.
  - `tier` is really the low byte of `lock_end`.
  - `lockEndTime` straddles two fields, so `.toNumber()` throws (more than 53 bits) about 255 times in 256. In the remaining case, a locked stake shows as unstakable.
- **Fix:** use offsets 56, 64, 72 and 73.

### SDK-05 · High · Confirmed · bug — The StakingPool decoder is wrong
- **Where:** `src/staking/index.ts:46-51`
- **Code:** `vcoinMint …slice(40, 72)… totalStaked …slice(104, 112)… paused: data[120]`
- **On-chain layout:** `pending_authority` 40, `vcoin_mint` 80, `total_staked` 208, `total_stakers` 216, `paused` 224.
- **Fails when:** always. `vcoinMint` comes back as the pending authority, TVL is garbage, and `paused` is a random byte.

### SDK-06 · High · Confirmed · bug — The rewards decoders and claim-status logic are wrong
- **Where:** `src/rewards/index.ts:47,76,132-135,155-156,184`
- **Wrong reads:**
  - `currentEpoch` is read from bytes of the mint pubkey (the correct offset is 329), so `getUnclaimedEpochs` and `getStats` throw on `.toNumber()`.
  - `isFinalized` is read from the low byte of `start_time` (the correct offset is 96).
  - `claimExpiry` is computed as if epochs started in 1970.
- **Wrong claim status:** `hasClaimedEpoch` uses `lastClaimedEpoch >= epoch` instead of the bitmap. A user who claimed epoch 5 is told they can't claim epoch 3, even though the chain allows it.
- **Fix:** correct the offsets, read `claim_expiry` from the account, and mirror `is_epoch_claimed`.

### SDK-07 · Medium · Confirmed · stub — The Merkle helpers can never match the chain (no SDK path calls them, and they are marked as placeholders)
- **Where:**
  - `src/rewards/index.ts:256-263,274-281`
  - `src/constants.ts:274-276`
- **Code:** `hash[i % 32] ^= data[i];`, an XOR-fold placeholder that collides trivially. The leaf has no `SSCRE_CLAIM_V1` prefix and uses a 48-byte layout.
- **Fails when:** `verifyMerkleProof` rejects every valid proof, and any tree built with `computeLeaf` produces roots that can't be claimed on-chain.
- **Fix:** keccak256 (`@noble/hashes`), `prefix‖user‖amount_le‖epoch_le`, sorted pairs, and a test vector generated from Rust's `compute_leaf`.

### SDK-08 · High · Confirmed · bug — Three PDA seeds are wrong
- **Where:**
  - `src/constants.ts:44,46,77`
  - `src/core/index.ts:121`
- **Code:** `governanceConfig: "governance-config"`; `voteRecord: "vote"` in order `(user, proposal)`; `content: "content"`. On-chain these are `gov-config`, `vote-record` in order `(proposal, voter)`, and `content-record`.
- **Fails when:** always.
  - `governance.getConfig` returns null.
  - `getActiveProposals` returns `[]`.
  - `hasVoted` is always false.
  - Every content lookup reports "not found".
- **Lead-verified.**

### SDK-09 · High · Confirmed · bug — The VoteChoice and ProposalStatus enums are wrong
- **Where:**
  - `src/types.ts:80-96`
  - `src/governance/index.ts:128,204,222,332`
- **Code:** `Against = 0, For = 1, Abstain = 2` and `Active = 0, Passed = 1, …`. On-chain the values are Abstain = 0, For = 1, Against = 2, and Pending = 0, Active = 1, Passed = 2, Rejected = 3, Executed = 4, Cancelled = 5.
- **Fails when:**
  - **A vote "Against" is recorded as Abstain**, because `cast_vote` takes the raw u8.
  - `getActiveProposals` filters on status 0, which is Pending and is never set.
  - `canExecute` treats Active proposals as Passed.

### SDK-10 · High · Confirmed · bug — The Proposal decoder and proposal-ID loop are wrong
- **Where:** `src/governance/index.ts:96-104,123-127`
- **Problems:**
  - It reads a 64-byte title where the account has a 32-byte `title_hash`.
  - It ignores the 128-byte `description_uri`.
  - It reads votes as u64 where they are u128.
  - The real `status` is at 274.
  - The loop asks for ID 0, which never exists, and skips the newest proposal.
  - `canExecute` uses `endTime + 2d` instead of `execution_time`.

### SDK-11 · Medium · Confirmed · bug — The GovernanceConfig and VoteRecord decoders are wrong
- **Where:** `src/governance/index.ts:68-72,159-164`. This is currently masked by SDK-08, because the config is never found.
- **Problems:**
  - `proposalCount` is read from `pending_authority`; the real offset is 168.
  - An Against vote (2) decodes as `support = true`.
  - The ciphertexts are shifted by 1 byte, because `is_private` is skipped.

### SDK-12 · Medium · Confirmed · bug — Voting power, threshold and quorum disagree with the program
- **Where:** `src/governance/index.ts:195-197,267-268,296`
- **Code:** `const fiveAMultiplier = 1.0; // Placeholder`, linear power, quorum `400 × 10000`, and threshold 100.
- **Fails when:** the SDK reports about 1e12 votes where the chain counts about 1e6. Proposals the chain passes show as "quorum not reached".
- **Fix:** port `calculate_voting_power` using BN, and read threshold and quorum from the config account.

### SDK-13 · Medium · Confirmed · bug — The content client is wrong
- **Where:**
  - `src/content/index.ts:44-48,78,101`
  - `src/types.ts:371-376`
  - `src/constants.ts:188-194`
- **Problems:**
  - `state` and `createdAt` are read from URI bytes. The real offsets are 234 and 237.
  - `tier` is really the regen rate.
  - The enums, regen rates and costs don't match the Rust code.
  - `canCreateContent` rejects posts the chain would allow.

### SDK-14 · Medium · Confirmed · bug — The identity client is wrong
- **Where:** `src/identity/index.ts:47-48,67,113`
- **Problems:**
  - `createdAt` is read from the verification hash, so it shows 1970 when the hash is zero and garbage otherwise. The real offset is 138.
  - The level names are wrong: KYC shows as "Standard" and Enhanced as "Premium".
  - Tier 4 is accepted, though the chain allows 0–3.

### SDK-15 · Medium · Confirmed · bug — The default VCoin mint is the System Program
- **Where:**
  - `src/constants.ts:26`
  - `src/client.ts:185-202`
- **Fails when:** an app uses the Quick Start configuration. `getVCoinBalance` silently returns 0 for everyone.
- **Fix:** ship the real mint, or throw while the placeholder is still in use.

### SDK-16 · Medium · Confirmed · test-gap — The SDK tests lock in the wrong values
- **Where:** `src/__tests__/constants.test.ts:61-63,194-201`, `src/__tests__/governance.test.ts:17-29,42-73`, `src/__tests__/types.test.ts:53-69`
- **Example:** `expect(SEEDS.governanceConfig).toBe('governance-config');`
- **Problems:**
  - Fixing SDK-08, 09, 12 or 13 will break these tests.
  - No test decodes real Borsh-encoded account data.
  - `core.test.ts:88` passes a BN where a PublicKey is expected, yet jest passed in our run.

### SDK-17 · Medium · Confirmed · doc-mismatch — The README examples don't work
- **Where:** `README.md:108-124,212,258,508-515`; JSDoc at `src/client.ts:37-40`, `src/vilink/index.ts:17`
- **Problems:**
  - The ZK example parameters cause TypeErrors.
  - `buildVoteTransaction(id, VoteChoice.Abstain)` passes 2, which is truthy, so it becomes a For vote.
  - `buildUpdateConfigTransaction` (README:212) doesn't exist.
  - JSDoc advertises `staking.stake` and `vilink.createTipAction`, which don't exist.
  - `score.composite` is undefined, so the UI shows "NaN%".
  - 14 `catch { return null; }` blocks remain, contrary to README:258. RPC errors therefore look like "account absent".

### Low findings
- **SDK-18 · stub:** `isEligibleForSubsidy` hardcodes `const usedToday = 0; // Would track per-day usage` (`gasless/index.ts:284`).
- **SDK-19 · bug:** drift from the on-chain values.
  - `.toNumber()` overflows above about 9M VCoin.
  - The float lock ratio is about 2.7% high.
  - The tier is computed from the new amount alone.
  - `LOCK_DURATIONS.none = 0` is rejected on-chain.
  - The ViLink fee is fixed at 250 bps.
  - 5A `isPrivate` is read at offset 60; the real offset is 72.
  - `oracleConsensusRequired: 3` doesn't match the on-chain value of 1.
- **SDK-20 · bug:** `new ViWoClient(connection)` rebuilds the Connection (`client.ts:64-68`). Headers, a custom fetch and the commitment are dropped, so authenticated RPCs return 401 and the reads come back null.
- **SDK-21 · dead code:**
  - The `lint` script has no eslint installed or configured.
  - `SEEDS.vouch` is wrong and unused; `MERKLE_CONSTANTS` and `VALID_URI_PREFIXES` are unused.
  - There are no decoders for PrivateVotingConfig, DecryptionShare, SlashRequest, HookConfig or VeVCoinConfig.
  - `Program`, `AnchorProvider` and `Wallet` imports are unused.

### Wiring table, method by method
- **ViWoClient**
  - `getVCoinBalance` — WRONG (placeholder mint)
  - `getVeVCoinBalance` — WRONG
  - `sendTransaction` — sends empty transactions
  - `healthCheck`, `getProvider`, `setWallet` — OK
  - constructor with a Connection — WRONG
- **PDAs**
  - All correct except `getGovernanceConfig`, `getVoteRecord` and `getContentRecord`.
  - Encodings are correct: u64-LE proposal, epoch and nonce; session, delegation and delegate-stats seed order.
- **Staking**
  - Reads: `getPool`, `getUserStake`, `canUnstake`, `getStats` — WRONG
  - Calculations: `calculateTier` and `calculateVeVCoin` — edge cases wrong
  - `getTierName`, `getTierInfo` — OK
  - 3 builders — STUB
- **Governance**
  - Config and proposals: `getConfig`, `getProposal`, `getActiveProposals`, `canAcceptAuthority` — WRONG
  - Votes: `getVoteRecord`, `hasVoted`, `getVotingPower` — WRONG
  - Status helpers: `getStatusText`, `canExecute`, `getProposalProgress` — WRONG
  - `getDelegation` — OK
  - 9 builders — STUB
- **Rewards**
  - Reads: `getPoolConfig`, `getEpochDistribution`, `getCurrentEpoch` — WRONG
  - Claim status: `hasClaimedEpoch`, `getUnclaimedEpochs`, `getStats` — WRONG
  - `getUserClaim`, `calculateGaslessFee`, `calculateNetClaim` — OK
  - Merkle helpers — STUB
  - Builder — STUB
- **ViLink**
  - `getConfig`, `getUserStats`, `getNextNonce`, type/URI/QR helpers — OK
  - `getAction`, `getActionByTimestamp`, `isActionValid` — WRONG
  - `calculateFee` — WRONG (fixed bps)
  - 4 builders — STUB
- **Gasless**
  - `getConfig`, `getSessionKey`, `getUserStats`, session/scope helpers, `calculateVCoinFee` — OK
  - `isEligibleForSubsidy` — STUB
  - 3 builders — STUB
- **Identity**
  - `getIdentity`, level names — WRONG
  - `hasIdentity` — OK
  - 3 builders — STUB
- **FiveA**
  - `getScore` — OK except `isPrivate`
  - Helpers — OK
  - Builder — STUB
- **Content**
  - Every read and helper — WRONG
  - 3 builders — STUB
- **Constants**
  - Program IDs match `declare_id!`.
  - Scopes, FeeMethod, ActionType, StakingTier, VerificationLevel, SSCRE fee and minimum claim, ViLink limits, ZK sizes and 5A weights — OK
