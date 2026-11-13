# 06 — five-a-protocol (5A reputation)

**Program:** `programs/five-a-protocol` · ID `783PbtJw5cc7yatnr9fsvTGSnkKaV6iJe6E8VUPTYrT8`

**What it does:** registered oracles write five reputation dimensions per wallet. Composite = 25/20/15/25/15 weighted (`state/user_score.rs:51-60`). The program also keeps a vouch ledger. Governance reads `composite_score` (byte offset 50) as a 1.0–2.0× vote boost (`programs/governance-protocol/src/state/utils.rs:14`).

**Verdict:** ❌ Score integrity rests on one oracle key, and the "consensus" and "vouch staking" features are not implemented.

---

### 5A-01 · High · Confirmed · security — "3-of-N consensus" is actually 1-of-N, and oracles can't be removed
- **Where:**
  - `programs/five-a-protocol/src/instructions/admin/initialize.rs:15`
  - `programs/five-a-protocol/src/instructions/oracle/submit_score.rs:44-48,55-59,104`
  - `programs/five-a-protocol/src/lib.rs:39-42`
- **Code:** `config.required_consensus = 1;`. This value has no setter; the only other write copies it into each pending update (`submit_score.rs:74`).
- **Fails when:** any one registered oracle calls `submit_score`. The score is applied in the same instruction, so the confirmation branch (`submit_score.rs:78-95`) can never run.
  - A compromised oracle can set a user's score to any value. After each apply, every oracle hits `ScoreUpdateTooFrequent` (`:44-48`) for 1h; after that, the attacker and honest oracles race.
  - Oracles can only be appended. `Oracle.is_active` is written at registration and never read.
- **Impact:** one oracle key can double any wallet's governance weight and decide who may vouch.
- **Docs claim the opposite:** `docs/programs/five-a-protocol.md:14,64,129` ("3-of-N … No single oracle can manipulate scores") and `SECURITY.md:74,122,147`.
- **Fix:**
  - Default `required_consensus >= 3`.
  - Add an authority setter bounded to `2..=min(oracle_count, MAX_CONFIRMING_ORACLES)`.
  - Add `remove_oracle`/`deactivate_oracle` and check `oracle_account.is_active` in `submit_score`.

### 5A-02 · Medium · Confirmed · security — Front-runnable `initialize`; unvalidated, immutable config addresses
- **Where:**
  - `programs/five-a-protocol/src/contexts/initialize.rs:17-21`
  - `programs/five-a-protocol/src/instructions/admin/initialize.rs:10-12`
- **Code:** `pub authority: Signer<'info>,` with no upgrade-authority check
- **Fails when:** anyone calls `initialize` before the deployer and becomes authority, which lets them register oracles (see 5A-01).
  - `identity_program`, `vcoin_mint` and `vouch_vault` are stored unvalidated and have no setter.
- **Fix:** require the signer to equal the program's upgrade authority (check the `ProgramData` account), or deploy and initialize in one transaction.

### 5A-03 · Medium · Confirmed · security / doc-mismatch — No 24h timelock on authority transfer
- **Where:**
  - `programs/five-a-protocol/src/instructions/admin/accept_authority.rs:12`
  - `programs/five-a-protocol/src/state/config.rs:10`: there is no `pending_authority_activated_at` field.
- **Code:** `config.authority = new_authority;`, with no time check
- **Fails when:** a compromised authority key proposes and accepts a new authority in a single transaction. `SECURITY.md:71,75` and `README.md:258` promise a "24-hour timelock across all 11 protocols".
- **Fix:** store the proposal timestamp and require `now >= set_at + 86_400` in accept. The vcoin, vevcoin, staking and governance programs already do this.

### 5A-04 · Low · Confirmed · bug (latent; unreachable while consensus = 1) — The consensus design lets the first oracle block honest updates
- **Where:**
  - `programs/five-a-protocol/src/instructions/oracle/submit_score.rs:55-59,85-88`
  - `programs/five-a-protocol/src/state/pending_score.rs:65-70`
- **Code:** `require!(pending_score.scores_match(...), FiveAError::ScoreMismatch);` and `if (self.confirmation_count as usize) < 5 { ... }`, which is a silent no-op past 5
- **Fails when:** once `required_consensus >= 2` (after 5A-01 is fixed), one oracle submits bad values first.
  - Every honest submission reverts with `ScoreMismatch` until `expires_at`. At `expires_at + 1` the same oracle can open a new pending entry, blocking that user's updates indefinitely for the cost of rent.
  - If consensus is ever set above 5, `add_confirming_oracle` silently stops counting, so consensus is never reached. `MAX_CONFIRMING_ORACLES` is unused.
- **Fix:**
  - Tally competing value sets, or let a differing submission start a new tally.
  - Bar the same oracle from re-opening an expired pending entry.
  - Bound `required_consensus <= MAX_CONFIRMING_ORACLES`.

### 5A-05 · Low · Confirmed · half-impl — Vouching has no stake, no slashing, and a gameable evaluation (no program reads the vouch stats, so the gaming has no on-chain effect today)
- **Where:**
  - `programs/five-a-protocol/src/contexts/vouch_for_user.rs:7-63` (no token accounts)
  - `programs/five-a-protocol/src/contexts/evaluate_vouch.rs:30`
  - `programs/five-a-protocol/src/instructions/vouch/evaluate_vouch.rs:16-24`
  - `programs/five-a-protocol/src/instructions/vouch/vouch_for_user.rs:15-18,41-46`
- **Code:** `pub evaluator: Signer<'info>,` / `let is_successful = vouchee_score.composite_score >= 5000;`
- **Fails when:**
  - **Stake and slashing:** `VOUCH_STAKE_AMOUNT` and `VOUCH_REWARD` are recorded numbers only, and no VCoin moves. The docs (`docs/programs/five-a-protocol.md:43-46,132`) promise a 5 VCoin stake and slashing, and define failure as "vouchee banned".
  - **Evaluation:** anyone may evaluate at any moment in the [90d, 365d] window, and the result samples the vouchee's score at that single instant. The voucher evaluates on a good day; a griefer evaluates on a bad one.
  - **Free success:** a ≥60% voucher can vouch for established users who already score ≥50% and collect guaranteed "successful" vouches, raising `vouch_accuracy` and `total_rewards_earned`.
- **Fix:**
  - Either implement escrow (a token vault PDA, transfer on vouch, release or slash on evaluate) or remove the claims.
  - Require the vouchee to be new or low-scored at vouch time.
  - Restrict evaluation to an oracle, or to the 90-day point using a snapshot score.

### 5A-06 · Medium · Confirmed · bug — Vouch slots can be locked permanently
- **Where:**
  - `programs/five-a-protocol/src/contexts/evaluate_vouch.rs:17-21`: `vouchee_score: Account<UserScore>` is required.
  - `programs/five-a-protocol/src/instructions/vouch/evaluate_vouch.rs:20` (`VouchExpired` after 365d) and `:29` (the only decrement of `vouches_active`)
- **Fails when:**
  - A vouchee who is never scored cannot be evaluated, because the `UserScore` account doesn't exist.
  - After day 365 evaluation reverts with `VouchExpired`.
  - In both cases `vouches_active` never decrements. A voucher who hits their 3–10 slot cap through Sybil vouchees, or through evaluations nobody called in time, can never vouch again.
- **Fix:** allow evaluation with a missing `UserScore` (count it as failed), and add an `expire_vouch` path after 365 days that frees the slot.

### 5A-07 · Low · Confirmed · reliability — Pause doesn't cover all mutating instructions
- **Where:**
  - `programs/five-a-protocol/src/contexts/evaluate_vouch.rs:7-31`: the context has no config account at all.
  - Pause is checked only at `submit_score.rs:24`, `create_snapshot.rs:15` and `vouch_for_user.rs:12`.
- **Fails when:** during an incident the admin pauses, but `evaluate_vouch`, `enable_private_score`, `disable_private_score` and `register_oracle` keep working.
- **Fix:** add the config account to those contexts and check `!paused` in each.

### 5A-08 · Medium · Confirmed · test-gap — No test exercises consensus or vouching
- **Where:**
  - `programs/five-a-protocol/src/tests.rs`: constants and `calculate_composite` only.
  - `programs/five-a-protocol/tests/common/mod.rs:100-107` (initialize: 3 of 4 accounts), `:159-166` (submit_score: 3 of 7), `:170-190` (vouch: 4 of 9)
- **Fails when:** even after the compile issue (TOOL-2) is fixed, these Rust integration tests fail on account validation.
  - The only executed coverage is `tests/five-a-protocol.ts:119-136`, which runs `submit_score` once on the 1-oracle path.
  - Consensus with more than one oracle, mismatched scores, expiry, and the whole vouch/evaluate lifecycle are untested.
- **Fix:** build instructions from Anchor's generated `five_a_protocol::accounts::*` and `instruction::*` types, and add tests for consensus, mismatch, expiry and the vouch lifecycle.

### 5A-09 · Low · Confirmed · doc-mismatch — Docs describe a different program
- **Where:** `docs/programs/five-a-protocol.md:17,43,64-69,82-83,113-124,148`
- **Mismatches:**
  - `PendingScoreUpdate` is documented as `oracle_submissions: [Option<Pubkey>; 5] … authenticity_sum: u32` with averaging. The code (`state/pending_score.rs:23`) requires every oracle to submit the exact same values.
  - `user` and `vouchee` are documented as Pubkey parameters; in the code they are accounts.
  - "Privacy mode" does nothing: `is_private` is never read, and `get_score` prints everything.
- **Fix:** rewrite the doc to match the code, or implement the documented features.

### Low findings
- **5A-L1 · perf.** Score submissions are serialised, and rent is locked forever.
  - `contexts/submit_score.rs:11-16` marks `five_a_config` as `mut` on every submit, only to bump `total_users` (`submit_score.rs:117`). That write lock serialises all score submissions in a block.
  - `PendingScoreUpdate` and `UserScore` accounts are oracle-paid and can never be closed (no `close =` anywhere), roughly 3,900 SOL of rent per 1M users.
- **5A-L2 · bug.** `create_snapshot` has no interval or validation.
  - Any single oracle can create a snapshot at any time (`instructions/oracle/create_snapshot.rs:14-37`; `SNAPSHOT_INTERVAL` is unused).
  - `avg_score` is not bounded to 10000.
  - No program reads `ScoreSnapshot`, and `UserScore.last_snapshot_epoch` is never written.
- **5A-L3 · bug.** Wrong error code: `submit_score.rs:80` returns `FiveAError::Overflow` for "already applied". The H-21 re-check at `:106-109` is always true on both branches.
- **5A-L4.** `get_score` only calls `msg!`. It never calls `set_return_data`, so CPI callers can't use it.
- **5A-L5.** The mutual-vouch check (`contexts/vouch_for_user.rs:55-60`) only blocks direct pairs; an A→B→C→A ring is allowed.

### Dead code and write-only state (each confirmed by grep)
- **Constants:** `SNAPSHOT_INTERVAL` is used only in tests. `MAX_CONFIRMING_ORACLES` is unused.
- **Enum variants:** `VouchStatus::Revoked` is never set.
- **Errors never raised:** `UserNotFound`, `InvalidStakeAmount`.
- **Oracle:** `Oracle.is_active` is never read. `Oracle.accuracy_rate` never changes after init.
- **FiveAConfig:** `identity_program`, `vcoin_mint`, `vouch_vault` and `last_snapshot_time` are written but never read.
- **UserScore:** `is_private` and `last_snapshot_epoch` have no reader.
- **UserVouchStatus:** `reward_multiplier`, `is_fully_vouched` and `vouch_completed_at` have no reader in any program.
- **Dependencies:** the `anchor-spl` dependency (`programs/five-a-protocol/Cargo.toml:21`) is unused.
