# 08 — transfer-hook

**Program:** `programs/transfer-hook` · ID `9K14FcDRrBeHKD9FPNYeVJaEqJQTac2xspJyb1mM6m48`

**What it claims to do:** "auto-updates 5A scores, detects wash trading" on every VCoin transfer.

**What it does:**
- Token-2022 can't call it at all. Any wallet can call it directly to write its state.
- It never touches 5A, and nothing attaches it to the VCoin mint.

**Verdict:** ❌ Not a working Token-2022 hook. If it were attached as written, every VCoin transfer would fail, and the program's state could be forged even if it worked.

---

### TH-01 · High · Confirmed · bug — Wrong instruction discriminator and account order, so Token-2022 can never invoke it
- **Where:**
  - `programs/transfer-hook/src/lib.rs:40`
  - `programs/transfer-hook/src/contexts/execute.rs:10-14,17-18,61-62`
- **Code:** `pub fn execute(ctx: Context<Execute>, amount: u64)`. It has no `#[instruction(discriminator = …)]`, no `#[interface]` and no fallback. The context order is `hook_config, sender, receiver, sender_activity, receiver_activity, pair_tracking, payer, system_program`.
- **Fails when:**
  - **Discriminator:** Token-2022 sends `sha256("spl-transfer-hook-interface:execute")[..8]`. Anchor only routes `sha256("global:execute")`, so the call fails with `InstructionFallbackNotFound`.
  - **Account order:** even with the right discriminator, the interface passes `source, mint, destination, authority, validation, extras…`. Anchor would deserialize the source token account as `HookConfig`.
  - **Impact:** once registered on the mint, every VCoin transfer fails. It is latent today, because vcoin-token configures no extensions.
  - Lead-verified (discriminator).
- **Severity note:** this would be Critical if it were deployed on the mint.
- **Fix:**
  - Add `#[instruction(discriminator = ExecuteInstruction::SPL_DISCRIMINATOR_SLICE)]`.
  - Reorder the context to `source_token, mint, destination_token, owner, extra_account_meta_list (seeds ["extra-account-metas", mint]), extras…`.
  - Add a real Token-2022 `transfer_checked` test.

### TH-02 · High · Confirmed · bug — The ExtraAccountMetaList can never be created, and its entries are wrong
- **Where:**
  - `programs/transfer-hook/src/instructions/hook/initialize_extra_accounts.rs:45-49,55-63,67-72,82-99,102-114`
  - `programs/transfer-hook/src/contexts/initialize_extra_accounts.rs:13-16`
- **Code:** `Seed::AccountData { account_index: 1, data_index: 32, length: 32 }`; `ExtraAccountMeta::new_external_pda_with_seeds(4, &[], false, true)`; `system_program::create_account(CpiContext::new(...))`, with no signer seeds.
- **Fails when:**
  1. **PDA can't be created.** The validation PDA `["extra-account-metas", mint]` can't sign without `new_with_signer`, so creation always fails. The account is an unconstrained `UncheckedAccount`, so a caller can only "initialize" a random account that Token-2022 never reads.
  2. **Wrong account index.** Index 1 is the **mint**, not the destination. The "receiver" and "pair" PDAs come out the same for every receiver and change whenever supply changes.
  3. **Payer can never sign.** The payer meta resolves to an off-curve PDA nobody can sign for, and it is flagged non-signer, but `Execute` requires `payer: Signer` for `init_if_needed`. Token-2022 calls the hook with plain `invoke`, so the first transfer to any new counterparty would fail.
  4. **Wrong comment.** The H-AUDIT-20 comment says `init` validates and is idempotent. `ExtraAccountMetaList::init` only allocates and fails on a second call, and there is no update instruction.
  5. **Not admin-only.** The docs say it is; any signer can call it.
- **Fix:**
  - Constrain the account with `seeds = [b"extra-account-metas", mint]` and create it with signer seeds (allocate/assign).
  - Gate it on `hook_config.authority`.
  - Use index 2 for the destination.
  - Drop the payer and `init_if_needed` from `Execute`, pre-create the PDAs elsewhere, and add an update instruction.

### TH-03 · Medium · Confirmed · security — Anyone can call `execute` directly to forge activity, poison pair stats and block real transfers
*Downgraded from High by the verifier: forging works today, but nothing reads this state (TH-04). Blocking real transfers needs a working attached hook (impossible today because of TH-01/02) and `block_wash_trading`, which defaults to false.*
- **Where:**
  - `programs/transfer-hook/src/instructions/hook/execute.rs:10-45`
  - `programs/transfer-hook/src/utils.rs:110-130`
  - `docs/programs/transfer-hook.md:41,110`
- **Code:** there is no `transferring` check; grep finds no `transferring` or `TransferHookAccount` in `src`. `let is_rapid_transfer = time_since_last < WASH_TRADING_COOLDOWN_SECONDS && …`
- **Fails when:** a wallet calls `execute` with a victim's token account as `sender`, a merchant's as `receiver`, itself as payer, and any `amount`.
  - It can push any wallet's activity score to 5000.
  - It can set the pair's `last_transfer_time`, so the victim's real transfer within the hour is rejected once `block_wash_trading = true`.
  - About 6 calls a day push `transfers_24h > 5` and trip `wash_flags`. About 10 flagged calls (−500 each) drive `trust_score` from 5000 to 0, though only while blocking is off; with blocking on, flagged calls revert and the flags don't accumulate.
  - Repeating this keeps any chosen sender → receiver path blocked indefinitely.
  - The doc claims "only callable by token program", which is false.
- **Fix:** read the source account's `TransferHookAccount` extension and require `transferring == true`. Also check the mint and that the token accounts are owned by Token-2022.

### TH-04 · High · Confirmed · half-impl / doc-mismatch — There is no 5A update, no tip recording, and no circular wash detection
- **Where:**
  - `programs/transfer-hook/src/instructions/admin/initialize.rs:15-17`
  - `programs/transfer-hook/src/instructions/hook/execute.rs:65-71`
  - `programs/transfer-hook/src/utils.rs:30-63`
  - `docs/programs/transfer-hook.md:13-15,24-26,94-103`
  - `README.md:170`
- **Code:** `config.five_a_program = five_a_program;` is written but never read, as is `min_activity_amount`. `is_tip: false, // Tip detection requires additional context`.
- **Fails when:**
  - The program makes no CPI to 5A, and no other program reads `user-activity`.
  - `TransferRecord` is dead.
  - The doc's A→B→A detection over the "last 10 recipients" doesn't exist; `contexts/execute.rs:42-51` admits A→B→A is two independent pairs.
  - The score accrues before the wash check runs.
- **Fix:** implement the 5A CPI and circular detection, or remove the claims from all docs.

### TH-05 · Medium · Confirmed · bug — The wash-trading heuristic blocks normal payments
- **Where:** `programs/transfer-hook/src/utils.rs:110-113,123`
- **Code:** `let is_wash_trading = is_rapid_transfer || is_high_frequency;` with `transfers_24h > 5`.
- **Fails when:** `block_wash_trading` is on. Then any of these is rejected:
  - paying the same merchant twice in an hour;
  - a second swap into the same pool within an hour;
  - a 7th payment to the same counterparty in a day.
- **Fix:** flag rather than block, combine signals with amount and round-trip checks, and allow-list owners.

### TH-06 · Low · Confirmed · bug — The query instructions always fail because `bump` is never stored (log-only views; the data can still be read over RPC)
- **Where:**
  - `programs/transfer-hook/src/contexts/get_user_activity.rs:11`
  - `programs/transfer-hook/src/contexts/get_pair_stats.rs:10`
  - `programs/transfer-hook/src/contexts/execute.rs:29,38,57`
- **Code:** `bump = user_activity.bump`. Nothing assigns `.bump`, and `init_if_needed … bump` does not store it.
- **Fails when:** every call. Anchor derives the address with `create_program_address(seeds + [0])`, which is not the canonical PDA, so the call fails with `ConstraintSeeds`.
- **Fix:** set `.bump = ctx.bumps.*` on init, or use a bare `bump` in the query contexts.

### TH-07 · Low · Confirmed · bug — The "daily" activity cap is lifetime, and dust earns full credit (nothing consumes the score)
- **Where:**
  - `programs/transfer-hook/src/utils.rs:41-63`
  - `programs/transfer-hook/src/constants.rs:9-11,21-25`
- **Fails when:** the cap is hit after 50 qualifying events, and the score then freezes forever because nothing resets it. A transfer of 1 base unit earns the full +100, because `MIN_ACTIVITY_THRESHOLD` is used only in tests.
- **Fix:** add a daily reset and enforce `min_activity_amount`.

### TH-08 · Medium · Likely · bug — Self-transfers between your own token accounts fail or lose data
- **Where:**
  - `programs/transfer-hook/src/contexts/execute.rs:24-40`
  - `programs/transfer-hook/src/instructions/hook/execute.rs:30-35`
- **Code:** both `UserActivity` accounts use `init_if_needed` with seeds `[USER_ACTIVITY_SEED, *.owner]`.
- **Fails when:** the sender and receiver have the same owner.
  - **First time:** the second `init_if_needed` hits a discriminator mismatch and the transaction reverts.
  - **Later:** the second copy overwrites the first, so the sender's update is lost.
- **Fix:** special-case same-owner transfers, updating a single account.

### TH-09 · Medium · Confirmed · security — Front-runnable `initialize`, and one key can freeze every VCoin transfer
- **Where:**
  - `programs/transfer-hook/src/contexts/initialize.rs:9-22`
  - `programs/transfer-hook/src/instructions/admin/set_paused.rs:7`
  - `programs/transfer-hook/src/instructions/hook/execute.rs:21`
- **Fails when:**
  - **Front-running:** anyone initializes first and binds any mint.
  - **Global freeze (SEC-4):** once the hook is live, the authority's one-step `set_paused` blocks every VCoin transfer, including unstakes, SSCRE claims and gasless fees. There is no timelock or auto-expiry, and authority transfer has no timelock either.
- **Fix:**
  - Check the upgrade authority at init, and assert the mint's TransferHook `program_id == crate::ID`.
  - Exempt protocol vault owners from the pause, or add `paused_until`.
  - Require a multisig to pause.

### TH-10 · Low · Confirmed · bug — Activity is recorded under token-account addresses, not wallets
- **Where:** `programs/transfer-hook/src/instructions/hook/execute.rs:26-27,31,39-45`
- **Problem:** the PDAs are keyed by owner, but `UserActivity.user`, the pair fields and the events record token-account addresses.

### TH-11 · Medium · Confirmed · test-gap — No test exercises `execute` or a Token-2022 transfer
- **Where:**
  - `programs/transfer-hook/tests/execute.rs`: checks bytes only.
  - `programs/transfer-hook/tests/common/mod.rs:88-110`: initializes without `vcoin_mint`, with accounts in the wrong order.
  - `programs/transfer-hook/tests/common/mod.rs:163`: calls a non-existent `global:update_authority`.
  - `tests/transfer-hook.ts:33-55`: initializes with a non-mint keypair and swallows the error.

### Dead code and doc mismatches
- **Dead code:**
  - `HookConfig.total_transfers` and `wash_trading_flags` are never incremented, because `hook_config` is not `mut` in `Execute`.
  - `TransferRecord` and `TRANSFER_RECORD_SEED` are unused.
  - Errors `InvalidProgramOwner`, `Overflow` and `InvalidExtraAccountMetas` are never raised.
  - `bump` is never set.
- **Doc mismatches:**
  - `transfer-hook.md:53` says "Admin only"; anyone can call it.
  - `:62-64` lists parameters and `:68-91` lists structs that don't exist.
  - The meta list in the doc (5A program, user score) differs from the code.
