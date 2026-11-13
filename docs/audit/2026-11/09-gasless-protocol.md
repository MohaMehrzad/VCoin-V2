# 09 — gasless-protocol

**Program:** `programs/gasless-protocol` · ID `FcXJAjzJs8eVY2WTRFXynQBpC7WZUqKZppyp9xS6PaB3`

**What it does:**
- Keeps session-key records (scope bitmask, limits, expiry) and counters.
- Has one instruction where the **main wallet** signs a VCoin transfer into a fee vault.
- Moves no SOL. No other program reads a session key.

**Verdict:** ❌ This is a ledger of counters, not a paymaster:
- The subsidized path bricks permanently after 3 idle days.
- Collected fees can never be withdrawn.
- The default fee can be set to any amount.

---

### GAS-02 · High · Confirmed · bug — Subsidized actions stop working permanently after 3 days without one
- **Where:**
  - `programs/gasless-protocol/src/instructions/session/execute_action.rs:40-55`
  - `programs/gasless-protocol/src/constants.rs:31`
- **Code:** `require!(new_day <= config.current_day.saturating_add(MAX_DAY_DELTA), GaslessError::ClockSkewDetected);` with `MAX_DAY_DELTA: u32 = 2`
- **Fails when:** three or more days pass with no subsidized action. That happens if:
  - launch is more than 2 days after `initialize`;
  - the protocol is paused for more than 2 days (paused calls return before the reset);
  - traffic is simply low.
- **Why:** `current_day` is written only at `admin/initialize.rs:32` and at `execute_action.rs:54`, after this check, and no setter exists. Every PlatformSubsidized action then reverts forever until a program upgrade. The C-AUDIT-15 "fix" is itself the cause.
- **Lead-verified.**
- **Fix:** delete the delta check (the Clock sysvar is consensus time), or always set `current_day = new_day`.

### GAS-03 · High · Confirmed · bug — Collected VCoin fees are locked forever
- **Where:**
  - `programs/gasless-protocol/src/contexts/initialize.rs:21-30`
  - `programs/gasless-protocol/src/instructions/fee/deduct_vcoin.rs:38-49`
- **Code:** `token::authority = config` on `fee_vault`. There is no withdraw instruction, and no `new_with_signer`/`invoke_signed` anywhere.
- **Fix:** add an authority-gated, timelocked `withdraw_fees` that signs with `[GASLESS_CONFIG_SEED, bump]`.

### GAS-01 · High · Confirmed · half-impl / doc-mismatch — Session keys authorize nothing, and "sponsoring" moves no SOL
- **Where:**
  - `programs/gasless-protocol/src/instructions/session/execute_action.rs:86-104`
  - `programs/gasless-protocol/src/contexts/create_session_key.rs:14-33`
  - `programs/gasless-protocol/src/contexts/deduct_vcoin_fee.rs:46-47`
  - `docs/programs/gasless-protocol.md:7,86-94,198-208`
- **Code:** `session.vcoin_spent = session.vcoin_spent.saturating_add(spend_amount);` is the only effect. Both inits use `payer = user`, and `DeductVCoinFee` requires `user: Signer`.
- **Fails when:**
  - No program consumes `SessionKey`; ViLink stores `gasless_program` but never reads it.
  - `spend_amount` is self-reported, so `max_spend` is unenforceable.
  - `fee_payer` is never read.
  - Users must hold SOL for rent, which contradicts "without holding SOL".
  - VCoinDeduction and SSCREDeduction sessions are charged nothing and skip all daily caps (MON-10).
  - Fee deduction needs the **main wallet's** signature, not the session key's.
- **Fix:**
  - Have target programs verify the session PDA and the session signer.
  - Make `fee_payer` the rent payer and signer.
  - Authorize deduction through a token delegate approved at session creation, and charge methods 1 and 2 inside `execute_session_action`.

### GAS-16 · Medium · Confirmed · security — The default fee path charges whatever the admin sets, with no cap (SEC-2)
- **Where:**
  - `programs/gasless-protocol/src/instructions/fee/deduct_vcoin.rs:17-18,26,29`
  - `programs/gasless-protocol/src/instructions/admin/update_fee_config.rs:12-13`
- **Code:** `let fee_to_deduct = if amount > 0 { amount } else { expected_vcoin_fee };` and `if amount > 0 && expected_vcoin_fee > 0 {`, which applies the slippage bound only when the user passes an amount.
- **Fails when:** a user calls `deduct_vcoin_fee(0)`, meaning "the default fee". The authority, or an honest units mistake, sets `vcoin_fee_multiplier` very large with no bounds, timelock or event.
  - The user is charged any fee up to their whole balance; above the balance, the transfer reverts.
  - The fee goes into a vault with no withdraw path (GAS-03).
- **Fix:**
  - Add a required `max_fee` argument and enforce it on both paths.
  - Bound the config values, emit an event, and delay fee changes.

### GAS-04 · Low · Confirmed · security — Scope checks can be bypassed (sessions authorize nothing today, so only the event field is wrong; GAS-01)
- **Where:**
  - `programs/gasless-protocol/src/state/session_key.rs:53-55`
  - `programs/gasless-protocol/src/instructions/session/execute_action.rs:23`
- **Code:** `(self.scope & action_type) != 0`
- **Fails when:** a tip-only session (0x01) passes `action_type = 0xFFFF`. It is accepted, and `SessionActionExecuted.action_type` lies to any integrator.
- **Fix:** `require!(action_type.count_ones() == 1 && scope & action_type == action_type)`.

### GAS-05 · Low · Confirmed · bug — Expired sessions count toward the 5-session cap, and rent is never returned
- **Where:**
  - `programs/gasless-protocol/src/instructions/session/create.rs:28-31,81`
  - `programs/gasless-protocol/src/instructions/session/revoke.rs:10-15`
- **Fails when:**
  - A user who opens one 24h session per day is blocked on day 6. Each revoke of an expired session, signed with the main wallet, frees one slot.
  - No account is ever closed, so about 0.00176 SOL per session is stranded.
  - A `session_pubkey` can't be reused after revoking it.
- **Fix:** count only live sessions, and add `close = user` on revoke and expiry.

### GAS-06 · Low · Confirmed · bug — Invalid session parameters are silently replaced with the maximums
- **Where:** `programs/gasless-protocol/src/instructions/session/create.rs:24,36-62`
- **Code:** `let spend = if max_spend > 0 && max_spend <= MAX_SESSION_SPEND { max_spend } else { MAX_SESSION_SPEND };` and `_ => FeeMethod::VCoinDeduction`
- **Fails when:**
  - `max_spend = 0` ("no spending") becomes 100,000 VCoin.
  - `max_actions = 0` becomes 1,000.
  - A 25h duration becomes 24h.
  - `fee_method = 7` becomes VCoinDeduction.
  - The `scope <= SCOPE_ALL` check at `:24` is always true, and clippy fails on it (TOOL-11).
- **Fix:** reject out-of-range values with errors.

### GAS-07 · Medium · Confirmed · security — Front-runnable `initialize` with an unvalidated mint
- **Where:** `programs/gasless-protocol/src/contexts/initialize.rs:9-33`
- **Fails when:** an attacker initializes first with a fake mint. The singleton config and `fee_vault` are bound to the wrong mint, and the attacker holds every admin setter. Only a redeploy fixes it.
- **Fix:** gate on the upgrade authority and pin the mint.

### GAS-08 · Medium · Possible · bug — Fee deduction reverts if VCoin has a transfer hook (latent; no hook is configured anywhere)
- **Where:** `programs/gasless-protocol/src/instructions/fee/deduct_vcoin.rs:38-49`
- **Problem:** `remaining_accounts` aren't forwarded to the transfer (same pattern as STK-01).

### GAS-09 · Medium · Confirmed · perf — Every action write-locks one global account
- **Where:**
  - `programs/gasless-protocol/src/contexts/execute_session_action.rs:10-15`
  - `programs/gasless-protocol/src/contexts/deduct_vcoin_fee.rs:10-15`
- **Problem:** `#[account(mut, …)] pub config` makes every gasless session action and fee deduction serialize on one account across the whole gasless program, including non-subsidized ones.

### GAS-10 · Low · Likely · security — Sybil wallets can exhaust the global subsidy budget
- **Where:** `programs/gasless-protocol/src/instructions/session/execute_action.rs:59-75`
- **Fails when:** throwaway wallets, at about 0.0036 SOL of rent each, consume 50 × `sol_fee_per_tx` per day each. Legitimate users then get `DailyBudgetExceeded`. This is a denial of service on a counter; no SOL is stolen.
- **Fix:** require a relayer co-signer, or gate on identity or 5A.

### GAS-15 · Medium · Confirmed · test-gap — No test covers the rollover, expiry, scope or budget paths
- **Where:** `programs/gasless-protocol/tests/common/mod.rs:101-190`, which asserts byte layouts with wrong account lists.
- **Problem:** the day rollover, time-based expiry, scope rejection and budget paths are untested. That gap is how GAS-02 shipped. The revoked-session path is tested (`tests/gasless-protocol.ts:360-375`).

### Low findings
- **GAS-11:** `deduct_vcoin_fee` creates stats without setting `user` (`deduct_vcoin.rs:51-56`), so `get_user_gasless_stats` then fails its seeds check for that user.
- **GAS-12 (fee math):**
  - A user-chosen `amount` has only an upper bound, so 1 base unit is accepted.
  - `max_slippage_bps.max(500)` can only loosen the bound, and the field is never written.
  - The "VCoin/SOL conversion" is a constant multiplier; there is no oracle.
  - `sol_fee_per_tx = 0` disables the budget and slippage checks.
  - `sscre_deduction_bps` can exceed 10000.
- **GAS-13 (dead code):**
  - `DailyBudget`, `DAILY_BUDGET_SEED`, `DAILY_SUBSIDY_BUDGET_SOL` and `DEFAULT_MAX_SLIPPAGE_BPS` are unused.
  - `fee_payer`, `sscre_program` and `sscre_deduction_bps` are written but never read.
  - `SSCREDeduction` is unimplemented.
  - Six errors are never raised.
  - `set_fee_payer` accepts the zero key.
  - `SessionActionExecuted.fee_amount` carries `spend_amount`.
  - The SDK's `isEligibleForSubsidy` is a stub (SDK-18).
- **GAS-14 (doc mismatches):**
  - The scope presets in `gasless-protocol.md:44-45` are wrong: SOCIAL = 0xCF also grants Governance and Claim.
  - `:64-78` give the wrong types.
  - `:104-147` describe structs that don't exist.
