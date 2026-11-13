# 10 — vilink-protocol

**Program:** `programs/vilink-protocol` · ID `CFGXTS2MueQwTYTMMTBQbRWzJtSTC2p4ZRuKPpLDmrv7`

**What it does:** stores shareable "action" links.
- Only **Tip** moves tokens: `transfer_checked` to the target, plus a fee to the treasury, rounded up and bounded to 0.1–10%.
- **Vouch** and **Follow** only update stats.
- Five other action types can be created but can never be executed.

**Verdict:** ⚠️ The tip path is mostly sound. Around it:
- Five types and batches are stubs.
- The per-type kill switch doesn't stop existing links.
- Vouch/Follow limits aren't enforced.
- The treasury is immutable.
- No account can ever be closed.

---

### VL-1 · Medium · Likely · security — Front-runnable `initialize`; the treasury and mint are then permanent
- **Where:** `programs/vilink-protocol/src/lib.rs:44-49,574-582`
- **Code:** `config.authority = ctx.accounts.authority.key(); config.vcoin_mint = …; config.treasury = treasury;`, with `pub authority: Signer<'info>` and no constraint.
- **Fails when:**
  - Anyone who initializes first becomes the authority and fixes the mint and treasury forever; there are no setters.
  - A legacy SPL mint is also accepted (`InterfaceAccount<Mint>`). Every tip CPI uses `Program<Token2022>`, so with a legacy mint all tips revert permanently.
- **Fix:** check the upgrade authority at init, and require a Token-2022-owned mint.

### VL-2 · Low · Confirmed · bug — Disabling an action type doesn't stop existing links (the global pause, which all three execute handlers check, still works as an emergency stop)
- **Where:** `programs/vilink-protocol/src/lib.rs:93` (the check runs on create only), versus the execute handlers at `:178-191`, `:281-287` and `:326-332`.
- **Fails when:** an admin disables Vouch during an incident. Every existing Vouch link still executes until it expires, up to 7 days later.
- **Fix:** check `is_action_enabled(action.action_type)` in all three execute handlers.

### VL-3 · Low · Confirmed · bug — Vouch and Follow links ignore `max_executions` (inflates ViLink-only stats that no other program reads)
- **Where:** `programs/vilink-protocol/src/lib.rs:286-292,331-337`, compared with the Tip check at `:193-195`.
- **Fails when:** a link has `one_time = false` and `max_executions = 1`. It can be executed any number of times, even by the same executor, which inflates `vouches_given`, `follows_given` and the global counters.
- **Fix:** copy Tip's `max_executions` guard, and add a per-executor receipt PDA `[b"exec", action, executor]`.

### VL-4 · Low · Confirmed · half-impl — The treasury can never be changed, and the zero key is accepted
- **Where:** `programs/vilink-protocol/src/lib.rs:49`. `update_programs` (`:443-461`) has no treasury parameter.
- **Fails when:** the treasury key is wrong, lost or compromised. Every tip keeps paying it fees forever, and fixing that needs a redeploy.
- **Fix:** add a timelocked `set_treasury` that emits an event, and reject `Pubkey::default()`.

### VL-5 · Medium · Possible · bug — Tips revert if VCoin has a transfer hook
- **Where:** `programs/vilink-protocol/src/lib.rs:212-237`
- **Fails when:** the VCoin mint has a transfer hook. No `remaining_accounts` are forwarded, so every tip reverts (same pattern as STK-01). This is latent today, because vcoin-token installs no hook.

### VL-6 · Medium · Confirmed · stub — Five action types can be created but never executed, and no action can be closed
- **Where:** `programs/vilink-protocol/src/lib.rs:55,92-93`
- **Code:** `config.enabled_actions = 0xFF;` and `require!(action_type < 8, ...)`
- **Fails when:**
  - Users create Challenge, Stake, ContentReact, Delegate or Vote links. Each one costs the rent for a 285-byte account, and no instruction can ever execute them.
  - No `ViLinkAction` or `ActionBatch` can be closed, even after expiry, so all rent is locked.
- **Fix:**
  - Initialize with `enabled_actions = 0b0000_0111` and reject types ≥ 3 until they are implemented.
  - Add `close_action` and `close_batch` (`close = creator`, allowed once expired or exhausted).

### VL-7 · Low · Confirmed · dead-code / doc-mismatch — The dApp registry is never enforced
- **Where:** `programs/vilink-protocol/src/lib.rs:139,366-401,640-650`. The error `DAppNotRegistered` is never raised.
- **Code:** `action.source_dapp = Pubkey::default();`
- **Fails when:** integrators rely on "only registered dApps can integrate" (`docs/programs/vilink-protocol.md:155`).
  - `RegisteredDApp` is written but never read.
  - `is_active`, `allowed_actions` and `fee_share_bps` are never checked, and `fee_share_bps` can exceed 10000.
  - There is no update, deactivate or close instruction.
- **Fix:** enforce the registry in `create_action`, or remove it.

### VL-16 · Low · Confirmed · bug — The fee rate is read at execution, not when the link is created (MON-9)
- **Where:** `programs/vilink-protocol/src/lib.rs:208`, which uses the live `config.platform_fee_bps`.
- **Fails when:** a link created at 2.5% executes after the admin raises the fee to 10%. The target silently receives less.
- **Fix:** store `fee_bps` in the action when it is created.

### Low findings
- **VL-8 · stub:** batches are pure storage.
  - The action IDs aren't validated, there is no `execute_batch`, and `executed_count` never changes.
  - `user_stats` isn't `init_if_needed`, so a fresh wallet can't create a batch.
  - `batch_id` is still derived from the timestamp (M-REMAINING-05 is not fixed).
- **VL-9 · bug:** a self-tip, where executor == target, puts the same stats PDA in two mutable slots (`lib.rs:606-609`).
  - The first time, it reverts on the discriminator.
  - Later, the executor's increments are overwritten.
  - Self-tips farm `tips_received` for just the fee.
  - Fix: `require!(executor != action.target)`.
- **VL-10 · bug:** `lib.rs:200-205` rejects any executor token account that has a delegate.
  - Users with an outstanding DEX approval can't tip, and users with CPI Guard can never tip.
  - The H-05 "race" premise isn't real: the owner signs and the transaction is atomic.
  - Fix: remove the check.
- **VL-11:** `action_id` omits the nonce (`state/utils.rs:19-36`), so two identical actions created in the same second share an ID that the URI scheme treats as unique.
- **VL-12:** admin instructions emit no events (`lib.rs:443-523`).
  - `accept_authority` has no 24h timelock.
  - The five program-ID fields in the config are never read.
- **VL-13 · perf:** a global `config` write lock on every create and execute (`lib.rs:588,602,628`), only to bump counters.
- **VL-14 · test-gap:** the tests assert bytes only.
  - `create_action_ix` omits `nonce` and `user_stats`.
  - The fee tests assert floor division, but the code rounds up.
  - Three size tests are stale.
  - Tip execution, expiry, `max_executions`, pause and authority transfer are all untested.

### Doc mismatches (`docs/programs/vilink-protocol.md`)
- **Parameters:**
  - `:47-52`: `create_action` is documented with `message`/`expires_at`. The real parameters are `action_type, amount, target, metadata_hash, expiry_seconds, one_time, max_executions, content_id, nonce` (`lib.rs:75-86`).
  - `:62-68`: tip is documented as taking `action_id`, updating 5A and emitting `TipExecuted`. It takes no arguments, makes no 5A call, and emits `ActionExecuted`.
  - `:82-85` and `:94-95`: `register_dapp` and `create_batch` parameters are wrong.
- **Behaviour:**
  - `:74,156`: the "60%+ 5A score" requirement to vouch isn't implemented.
  - `:25,27`: there is no staking on Vouch or Challenge.
- **Structs and URIs:**
  - `:99-135`: the account structs don't match `state/*.rs`.
  - `:143` and `lib.rs:18` give two different URI formats.
- **`lib.rs:14`:** "gasless execution via session keys" doesn't exist.
- **Correct:** the 10% fee cap and nonce-based PDAs.

### Checked and sound
- The fee is rounded up in u128 and capped at the amount (`lib.rs:208-210`).
- Owner, mint and treasury are checked on every token account (`:180-185`).
- The creator can't execute their own action (`:198`).
- The nonce must match exactly (`:100-103`).
- Concurrent batch creation is safe, because the second one fails the seeds check.
