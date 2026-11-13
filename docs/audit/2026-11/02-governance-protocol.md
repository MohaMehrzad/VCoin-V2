# 02 — governance-protocol (non-cryptographic logic)

**Program:** `programs/governance-protocol` · ID `3R256kBN9iXozjypQFRAmegBhd6HJqXWqdNG7Th78HYe`

**What it does:**
- Proposals are voted on with power = `isqrt(ve) × 5A boost (1.0–2.0×) × tier multiplier (1–10×)`, then soft-capped at 100,000.
- The program reads staking `UserStake` and 5A `UserScore` as raw bytes.
- Lifecycle: Active → (finalize) Passed/Rejected → (execute) Executed.
- The ZK cryptography is covered in `03-zk-private-voting.md`.

**Verdict:** ❌ Governance can be captured for about 100 VCoin, and any outcome can be blocked or reversed by anyone. "Execution" changes nothing on-chain.

---

### GOV-02 · Critical · Confirmed · security — Voting power is computed on 9-decimal base units, so 10 wallets of 10 VCoin capture governance
- **Where:**
  - `programs/governance-protocol/src/state/utils.rs:10,27,30-31`
  - `programs/governance-protocol/src/constants.rs:39,52`
  - `programs/governance-protocol/src/instructions/vote/cast.rs:134`
- **Code:**
  - `let base_votes = integer_sqrt(vevcoin_balance);`
  - `if raw_votes > DIMINISHING_THRESHOLD { DIMINISHING_THRESHOLD + integer_sqrt(raw_votes - DIMINISHING_THRESHOLD) }`
  - The constants are `DIMINISHING_THRESHOLD: u64 = 100_000` and `DEFAULT_QUORUM: u64 = 1_000_000`.
- **Fails when:** `ve_vcoin_amount` is stored in base units (VEVCOIN_DECIMALS = 9), so the diminishing knee sits at only 10 veVCoin (`sqrt(1e10) = 100,000`).
  - 10 VCoin locked for 4 years gives **100,000 votes**.
  - 1B VCoin (the entire supply) at Platinum, a 4-year lock and the maximum 5A score gives **about 253,900 votes**.
  - So 3 wallets holding 30 VCoin in total outvote the entire supply.
  - 10 such wallets reach quorum by themselves, which no single wallet can ever do.
  - With a 1-week lock, about 2,500 VCoin per wallet does the same and the capital comes back after a week.
  - Votes are not tied to identity or 5A (5A only boosts; it never gates).
  - Found independently by three reviewers (GOV, SEC-1, MON-1). Lead-verified.
  - **Reproduced by PoC:** `poc/tests/poc.rs::bug_gov02_ten_vcoin_wallets_outvote_entire_supply`. A 10-VCoin wallet gets 100,000 votes; the entire 1B supply in one wallet gets 253,831.
- **Repro:**
  1. Fund 10 wallets with 10 VCoin each.
  2. Each wallet calls `stake(10_000_000_000, 126_144_000)`.
  3. One wallet calls `create_proposal`.
  4. All 10 call `cast_vote(1)`, so `votes_for = 1,000,000`.
  5. After `end_time`, `finalize_proposal` returns Passed.
- **Fix:**
  - Normalise to whole tokens before the square root (`ve / 1e9`) and re-derive the threshold and quorum in those units.
  - Express quorum as basis points of total locked ve.
  - Quadratic voting needs Sybil resistance: require a verified identity level or a minimum 5A score to vote, or use linear ve.

### GOV-01 · High · Confirmed · bug — `finalize_proposal` can be re-run forever: execution is blocked and outcomes can be flipped
- **Where:**
  - `programs/governance-protocol/src/instructions/proposal/finalize.rs:11-34`
  - `programs/governance-protocol/src/contexts/finalize_proposal.rs:6-21`
- **Code:** the only precondition is `clock.unix_timestamp > proposal.end_time`. Then `proposal.status = ProposalStatus::Passed as u8; proposal.execution_time = clock.unix_timestamp + config.timelock_delay;`
- **Fails when:**
  - Any signer re-finalizes a Passed proposal every ~47h, pushing `execution_time` to now + 48h each time, so `execute_proposal` (`execute.rs:17-20`) never becomes callable.
  - Re-finalizing an Executed proposal flips it back to Passed.
  - Combined with GOV-04, re-finalizing after a quorum change flips Rejected ↔ Passed.
  - Lead-verified.
- **Severity note:** this would be Critical if execution did anything on-chain (see GOV-06).
- **Fix:** require `proposal.status == Active`, set `execution_time` once, and emit `ProposalFinalized`. For private proposals, also require `reveal_completed` (GOV-08).

### GOV-03 · High · Confirmed · security — No snapshot and no `lock_end` check, so the same tokens vote twice and unlocked stake votes
- **Where:**
  - `programs/governance-protocol/src/instructions/vote/cast.rs:71-85`
  - `programs/governance-protocol/src/instructions/vote/cast_private.rs:126-136`
  - `programs/governance-protocol/src/instructions/proposal/create.rs:84`
- **Code:** `let tier = stake_data[72]; let ve_vcoin_amount = u64::from_le_bytes(stake_data[73..81]…)`. `lock_end` (offset 56) is never read. `proposal.start_time = clock.unix_timestamp;` gives no voting delay.
- **Fails when:**
  1. Wallet A's lock has expired, but its ve stays full because there is no decay.
  2. A votes, then in the same transaction unstakes (`unstake.rs:30` only needs `now >= lock_end`) and sends the VCoin to wallet B.
  3. B stakes with a 1-week lock and votes again.
  - Stake bought after a proposal opens also counts.
- **Fix:**
  - Require `lock_end >= proposal.end_time`.
  - Snapshot voting power at `start_time` (checkpoints in staking).
  - Add a voting delay.

### GOV-04 · High · Confirmed · security — One admin key can rewrite any vote outcome after the fact, or brick governance
- **Where:**
  - `programs/governance-protocol/src/instructions/admin/update_config.rs:13-24`
  - `finalize.rs:20,29`
  - `create.rs:85`
- **Code:** `if let Some(q) = quorum { config.quorum = q; }`. There are no bounds and no timelock, and `finalize` reads `quorum` and `timelock_delay` live.
- **Fails when:**
  - **Overturn outcomes:** raise quorum to `u64::MAX` and re-finalize (Passed → Rejected), or lower it (Rejected → Passed).
  - **Skip the timelock:** `timelock_delay ≤ 0` allows immediate execution.
  - **Block voting:** `voting_period ≤ 0` makes `end_time < start_time`, so nobody can vote.
  - **Brick the program:** values near `i64::MAX` overflow at `create.rs:85` and `finalize.rs:29`, and with `overflow-checks = true` that panics in every create and finalize. The same admin can undo this with another `update_config`.
  - This contradicts `constants.rs:25-26`: "Any changes … MUST go through governance".
- **Fix:**
  - Bound every parameter (timelock ≥ 24h, a voting period range, quorum > 0).
  - Snapshot quorum and timelock into the Proposal at creation.
  - Use `checked_add`.
  - Gate `update_config` behind a timelock or a passed proposal.

### GOV-05 · Medium · Confirmed · half-impl — Delegation transfers no voting power (an advertised feature that does nothing; no power is gained or lost)
- **Where:**
  - `programs/governance-protocol/src/instructions/vote/cast.rs:110-128,134`
  - `programs/governance-protocol/src/contexts/cast_vote.rs:45`
  - `programs/governance-protocol/src/instructions/delegation/delegate.rs:29-37`
  - `programs/governance-protocol/src/instructions/delegation/revoke.rs:4-16`
- **Code:** `require!(vevcoin_balance <= delegation.delegated_amount, …ExceedsDelegatedAmount);`. The weight is always calculated from the voter's own stake.
- **Fails when:**
  - A delegate votes but gains no delegated power, while the delegator still votes with full power.
  - The optional `delegation` account can simply be omitted.
  - The balance check compares the delegate's own balance with `delegated_amount`, which is meaningless.
  - `revocable`, `delegation_type` and `categories` are never read, and `expires_at` is not validated (negative values mean "permanent").
  - `delegated_amount` is a stale snapshot, and `delegate_votes` ignores pause.
  - Docs advertise working delegation (`docs/programs/governance-protocol.md:15,97`).
- **Fix:** implement it (add delegated power to the delegate, block the delegator while delegated, PDA-constrain the Delegation account, honour `revocable`/`expires_at`), or remove the instructions and the doc claims.

### GOV-06 · Medium · Confirmed · half-impl / doc-mismatch — `execute_proposal` executes nothing
- **Where:**
  - `programs/governance-protocol/src/instructions/proposal/execute.rs:22-23`
  - `programs/governance-protocol/src/lib.rs:7`
- **Code:** `proposal.executed = true; proposal.status = ProposalStatus::Executed as u8;`. There is no CPI and no stored instruction, and `proposal_type` is not validated (`create.rs:83`).
- **Fails when:**
  - A proposal passes and "executes", but no on-chain state changes, while every real parameter stays under one admin key (GOV-04).
  - `lib.rs:7` claims "Full on-chain governance".
  - The tiers in `lib.rs:16-19` and docs `:37-41` (Community 1+, Council fast-track) aren't enforced: `COMMUNITY_THRESHOLD`/`COUNCIL_THRESHOLD` are unused, and zero-weight votes are accepted.
- **Fix:** store instructions in the proposal and invoke them through a governance-PDA signer, or document governance as advisory.

### GOV-07 · Medium · Likely · security — A proposer can wipe a public vote through the ZK reveal path; bad committee parameters panic or brick
- **Where:**
  - `programs/governance-protocol/src/instructions/zk_voting/enable_private_voting.rs:17-38`
  - `programs/governance-protocol/src/contexts/enable_private_voting.rs:7-11`
  - `programs/governance-protocol/src/instructions/zk_voting/initiate_reveal.rs:10-16`
  - `programs/governance-protocol/src/instructions/zk_voting/aggregate_revealed_votes.rs:31-36,174-182`
- **Code:** `private_config.decryption_threshold = decryption_threshold;` has no bounds. The handler never checks `proposal.is_private_voting` or the time.
- **Fails when:**
  1. On a **public** proposal, the proposer calls `enable_private_voting(committee_size=0, threshold=0)`.
  2. After the vote ends, anyone calls `initiate_reveal` and then `aggregate_revealed_votes(0,0,0,[])`.
  3. The identity accumulators satisfy `0·H == 0 − 0`, so `votes_*` are overwritten with 0.
  4. A re-finalize turns Passed or Executed into Rejected.

  Other bad parameters:
  - `threshold > committee_size` means the reveal never completes.
  - `committee_size > 5` panics with an index out of bounds at `:25`. That only reverts the one transaction.
  - The wipe can only zero the tally; it can't set other values.
  - Duplicate committee keys are accepted.
  - The identity point is accepted as `encryption_pubkey`. The TS test enables with all-zero keys.
- **Fix:**
  - Require `proposal.is_private_voting` and `now < start_time`.
  - Require `MIN_DECRYPTION_THRESHOLD ≤ threshold ≤ committee_size ≤ MAX_COMMITTEE_SIZE`, distinct members, and a non-identity key.
  - `aggregate` must require `proposal.is_private_voting` and `status == Active`.

### GOV-08 · Medium · Confirmed · bug — Private proposals can be finalized before the reveal and always end Rejected
- **Where:**
  - `programs/governance-protocol/src/instructions/proposal/finalize.rs:11-24`
  - `programs/governance-protocol/src/contexts/finalize_proposal.rs:6-21`: the context has no PrivateVotingConfig.
- **Fails when:** anyone finalizes right after `end_time`, before `aggregate_revealed_votes`. `votes_*` are still 0, so the result is Rejected.
  - Today only the GOV-01 bug lets the result be re-finalized after the reveal.
  - Fixing GOV-01 naively would let any griefer permanently reject every private proposal.
- **Fix:** for private proposals, pass PrivateVotingConfig and require `reveal_completed`.

### GOV-09 · Medium · Confirmed · bug — The proposal threshold is 0.000001 veVCoin, so anyone can spam proposals
**Reproduced by PoC** (`bug_gov09_…`). About 0.00025 VCoin locked for 1 week is enough to propose.
- **Where:**
  - `programs/governance-protocol/src/instructions/proposal/create.rs:56-65`
  - `programs/governance-protocol/src/constants.rs:40`
- **Code:** `DEFAULT_PROPOSAL_THRESHOLD: u64 = 1_000; // 1000 veVCoin`, compared against 9-decimal base units.
- **Fix:** `1_000 * 10u64.pow(9)`, or whole-token units consistent with the GOV-02 fix.

### GOV-10 · Medium · Confirmed · security — Front-runnable `initialize` lets an attacker choose which programs voting power is read from
- **Where:**
  - `programs/governance-protocol/src/contexts/initialize.rs:7-17`
  - `programs/governance-protocol/src/instructions/admin/initialize.rs:12-14`
- **Code:** `config.staking_program = staking_program;` has no setter afterwards.
- **Fails when:**
  - An attacker initializes first, pointing `staking_program` and `five_a_program` at their own program. They can then forge any ve, tier or 5A data, because `cast.rs:46-63` trusts only these IDs.
  - Even with a correct initialization, a redeployed staking or 5A program can never be re-pointed.
- **Fix:** check the upgrade authority at init, and add timelocked setters.

### GOV-11 · Medium · Confirmed · reliability — Pause stops only new proposals
- **Where:** `programs/governance-protocol/src/instructions/proposal/create.rs:26` is the only `paused` check.
- **Fails when:** in an incident, voting, finalize, execute, delegation and every ZK instruction keep running. `ExecuteProposal` doesn't even load the config.
- **Fix:** check pause in every state-changing instruction.

### GOV-12 · Low · Confirmed · doc-mismatch — The governance docs are wrong in security-relevant ways
- **Where:** `docs/programs/governance-protocol.md`
- **Mismatches:**
  - The `:24-32` example says 900 votes; the code gives 105,325, and the diminishing cap isn't documented.
  - `:46` lists a "Queued" status, which doesn't exist.
  - `:57` says initialize is "Admin only"; anyone can call it.
  - `:69,:78` list wrong parameters.
  - `:114-146` describe the wrong struct.
  - `:156` documents `ZK_VOTING_ENABLED = false`, which doesn't exist anywhere in code. ZK voting is live for any proposer.
  - `:154` says the 48h timelock is fixed; it is admin-mutable without bounds.
  - `lib.rs:12` has a "five_a_score / 100" comment that is wrong.

### GOV-13 · Low · Confirmed · test-gap — Tests use stale interfaces and lock in wrong values
- **Where:**
  - `programs/governance-protocol/tests/common/mod.rs:144-161`
  - `tests/governance-protocol.ts:134-147,492-510`
  - `programs/governance-protocol/src/tests.rs`: `:71` (stale size test), `:137-186` (whole-token voting tests), `:281` (quorum), `:417` (seed order), `:367-386` (tautology)
- **Problems:**
  - `create_cast_vote_ix(…, choice, vevcoin_balance, five_a_score, tier)` uses the pre-C-NEW-01 signature.
  - The TS test expects a private vote with all-zero ciphertexts and proofs to succeed.
  - The unit tests:
    - include abstain in the quorum, but the program excludes it;
    - use PDA seed order `voter, proposal`, but the program uses `proposal, voter`;
    - use whole-token numbers, which hides GOV-02;
    - contain a tautological state-machine test.
  - Nothing tests finalize, execute, re-finalize, reveal, aggregate, delegation or pause.

### Low findings, stubs and dead code
- **Hard-coded event field:** `submit_decryption_share.rs:81` always emits `proposal_id: 0, // Proposal ID not stored…`.
- **Misused errors:**
  - `execute.rs:12` returns `ProposalNotFound` for "not Passed".
  - `initiate_reveal.rs:14` returns `RevealAlreadyComplete` for "already started".
  - `create.rs:27` returns `Overflow` for a long URI.
- **Errors never raised:** AlreadyVoted, QuorumNotReached, DelegationExists, DelegationNotFound, DelegationNotActive.
- **Unused types:** `ProposalStatus::Pending`/`Cancelled` are never set, and there is no cancel instruction. `ProposalType::from_u8` and `DelegationType` are unused.
- **Constants used only in tests:** COMMUNITY_THRESHOLD, DELEGATE_THRESHOLD, COUNCIL_THRESHOLD, MIN_DECRYPTION_THRESHOLD, MAX_COMMITTEE_SIZE.
- **Dead state:**
  - `GovernanceConfig.treasury_balance` (there is no treasury).
  - Ten never-written `DelegateStats` fields (`league_tier`, `voting_accuracy`, `participation_rate`, …) and an uncalled `max_delegation_pct()`.
  - `VoteRecord.revealed` is never updated.
  - `PrivateVotingConfig.is_enabled` is always true.
  - `VoteCast.is_private` is always false.
  - `Proposal.executed` duplicates the Executed status.
- **Missing events:** finalize, revoke_delegation, update_config, set_paused, the authority transfers, enable_private_voting and initiate_reveal.
- **Locked rent:** VoteRecord, Proposal, PrivateVotingConfig and DecryptionShare can never be closed.
- **No expiry:** a Passed proposal stays executable forever.
- **Unused dependencies:** `anchor-spl` and `solana-hash` (`Cargo.toml:21,23`).
- **Raw-offset reads** of foreign accounts (`cast.rs:79-83,101-103`) are correct today: tier at 72, ve at 73..81, composite at 50..52. But any staking or 5A layout change silently corrupts voting power; use the CPI crate account types.
- **`integer_sqrt`** (`state/utils.rs:43`): `(x + 1) / 2` overflows at `u64::MAX`, which real data can't reach.

### Checked and sound
- There is one vote per (proposal, voter) across public and private voting, because both `init` the same `[vote-record, proposal, voter]` PDA.
- Proposal IDs use `checked_add`.
- All `LEN`s match their structs.
- The two-step authority transfer with a 24h timelock is correct here.
- Window boundaries are consistent: voting runs while `≤ end_time`, and finalize/reveal require `> end_time`.
