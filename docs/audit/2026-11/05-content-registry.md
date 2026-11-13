# 05 — content-registry

**Program:** `programs/content-registry` · ID `MJn1A4MPCBPJGWWuZrtq7bHSo2G289sUwW3ej2wcmLV`

**What it does:**
- Records a content hash and URI under a caller-chosen `tracking_id`.
- Rate-limits posting with a tier-based "energy" budget plus daily caps.
- Refunds energy for posts with high engagement. No tokens move.

**Verdict:** ⚠️ The anti-spam model is bypassable, because users pick their own tier. Authorship can be squatted, and regeneration math loses progress.

---

### CR-01 · Medium · Confirmed · security — Users choose their own tier, which bypasses the anti-spam model
- **Where:**
  - `programs/content-registry/src/instructions/energy/initialize_user.rs:5-13`
  - `programs/content-registry/src/instructions/energy/update_tier.rs:8`
- **Code:** `user_energy.tier = tier; user_energy.max_energy = UserEnergy::max_energy_for_tier(tier);` and `require!(new_tier > 0 && new_tier <= 4, ContentError::InvalidTier);`
- **Fails when:**
  - Any user calls `initialize_user_energy(4)` and gets 2,000 energy, 200 per hour regen and 1,000 posts per day. A tier-0 user gets 200, 20 per hour and 50 posts per day.
  - Values above 4 are stored as-is, but they map to None-tier limits, which is harmless.
  - The admin can fix this only one user at a time, and can never set tier 0 (None). A cheater or a fully unstaked user therefore keeps at least Bronze.
- **Fix:** derive the tier from the staking `UserStake` PDA (check its owner and seeds) at init and on refresh. Allow tier 0, and reject values above 4.

### CR-02 · Low · Confirmed · security — Content records can be squatted by front-running a predictable `tracking_id` (denial of one ID; the victim can retry with a random ID)
- **Where:** `programs/content-registry/src/contexts/create_content.rs:15-20`
- **Code:** `seeds = [CONTENT_RECORD_SEED, tracking_id.as_ref()],`
- **Fails when:** `tracking_id` is predictable, for example a content or IPFS hash, or visible in a pending transaction. An attacker creates the record first and becomes `author`. The real author's create then fails with "already in use" forever.
- **Fix:** add the author to the seeds: `[CONTENT_RECORD_SEED, author.key().as_ref(), tracking_id.as_ref()]`.

### CR-03 · Low · Confirmed · bug — Energy regeneration throws away partial progress (less than 1 energy lost per successful action)
- **Where:** `programs/content-registry/src/utils.rs:12-19`
- **Code:** `let regen_amount = ((elapsed_seconds as u32).saturating_mul(energy.regen_rate as u32) / 3600)…; … energy.last_regen_time = current_time;`
- **Fails when:** a user acts more often than every `3600 / rate` seconds (180 s at tier 0, 18 s at Platinum). Each call rounds regeneration down to 0 and still resets the clock. A tier-0 user posting every 2 minutes never regenerates, though they should gain 20 per hour.
- **Fix:** carry the remainder. Advance `last_regen_time` by `regen_amount * 3600 / rate` instead of setting it to `now`, except at max energy.

### CR-04 · Medium · Confirmed · security — Both config initializers are front-runnable
- **Where:**
  - `programs/content-registry/src/contexts/initialize.rs:6-19`
  - `programs/content-registry/src/contexts/initialize_energy.rs:6-19`
- **Code:** `#[account(init, payer = authority, … seeds = [REGISTRY_CONFIG_SEED], bump)]`. `InitializeEnergy` has no link to `registry_config`.
- **Fails when:** anyone initializes first and becomes the registry authority. That authority controls `update_engagement` (and therefore all refunds), `update_user_tier` and pause.
- **Fix:** gate `initialize` on the program's upgrade authority, and add `has_one = authority` on `registry_config` in `InitializeEnergy`.

### CR-05 · Low · Likely · perf — Every post write-locks one global account, which caps throughput
- **Where:**
  - `programs/content-registry/src/contexts/create_content.rs:8-13`
  - `programs/content-registry/src/instructions/content/create.rs:68-70`
  - `programs/content-registry/src/contexts/delete_content.rs:7-12`
- **Code:** `#[account(mut, seeds = [REGISTRY_CONFIG_SEED], …)] pub registry_config` → `config.total_content_count = …saturating_add(1);`
- **Fails when:** under platform load, all creates and deletes serialise on one writable account. Solana caps per-account compute per block, so posts fail or queue platform-wide.
- **Fix:** drop the global counters (derive them from events or an indexer), or shard them. Make `registry_config` read-only in create and delete.

### CR-06 · Low · Confirmed · bug — An early refund claim burns the refund forever; deleted content still earns refunds (impact limited to rate-limit energy)
- **Where:** `programs/content-registry/src/instructions/energy/claim_refund.rs:10-31,58`
- **Code:** `} else { 0 // No refund };` … `content.refund_claimed = true;`
- **Fails when:**
  - The author claims at hour 24, before the backend has posted engagement. `refund_claimed` becomes true with a 0% refund, so later high engagement can never be redeemed.
  - Neither `claim_refund` nor `update_engagement` checks the content's state, so deleted posts still earn refunds.
- **Fix:**
  - Don't set `refund_claimed` when the percentage is 0; return `RefundNotReady` instead.
  - Add a claim deadline.
  - Reject `Deleted` content in both instructions.

### CR-07 · Medium · Confirmed · security / doc-mismatch — The engagement "oracle" is the admin key, with no timelock on authority transfer
- **Where:**
  - `programs/content-registry/src/contexts/update_engagement.rs:15-22`
  - `programs/content-registry/src/instructions/admin/accept_authority.rs` (no timelock field)
- **Fails when:**
  - The backend hot key that posts engagement is the same key that can pause, change any user's tier, and transfer authority instantly.
  - Docs promise a separate "Engagement oracle" reporting likes, comments and shares (`docs/programs/content-registry.md:84-92,144`). The code takes one `u32` from the admin.
- **Fix:** add a separate `engagement_oracle` key to the config, and a 24h authority-transfer timelock.

### CR-08 · Low · Confirmed · half-impl — Pause covers only `create_content`
- **Where:** `programs/content-registry/src/instructions/content/create.rs:15`, the only `paused` check.
- **Problem:** edit, delete, `claim_energy_refund`, `update_engagement`, `update_user_tier` and `initialize_user_energy` all ignore pause.

### CR-09 · Low · Confirmed · bug — A tier change applies the new regeneration rate retroactively
- **Where:** `programs/content-registry/src/instructions/energy/update_tier.rs:12-14`
- **Problem:** `regen_rate` changes without first settling regeneration at the old rate.
- **Fix:** call `regenerate_energy(user_energy, now)` before changing the rate.

### CR-10 · Low · Confirmed · half-impl — The edit rules aren't enforced
- **Where:**
  - `programs/content-registry/src/utils.rs:45-47`: `edits_this_hour` is reset but never incremented.
  - `programs/content-registry/src/instructions/content/edit.rs:26`
- **Problem:**
  - Edits are unlimited and free in the first hour, then allowed forever at 5 energy. The doc says edits are allowed only "within 1-hour window" (`docs/programs/content-registry.md:67`).
  - Only one `previous_hash` is kept, so "proof of existence" can be rewritten.
  - Edits aren't counted in `energy_spent_today` and emit no `EnergySpent` event.

### CR-11 · Low · Confirmed · design — Rent is locked forever
- **Where:** `programs/content-registry/src/instructions/content/delete.rs:19-21`
- **Problem:** a comment refers to an "account rent reclamation feature" that doesn't exist. `ContentRecord` (~293 B, about 0.003 SOL) and `UserRateLimit` can never be closed.

### CR-12 · Low · Confirmed · test-gap — Test helpers use the wrong account order and count
- **Where:** `programs/content-registry/tests/common/mod.rs:101-105`, `:132-137`, `:154-158`
- **Code:** `AccountMeta::new(*authority, true), AccountMeta::new(*config, false),`
- **Problem:**
  - The order is reversed against `Initialize`.
  - The `create_content` helper passes 4 accounts; the context needs 6.
  - `delete` has the wrong order.
- **Not tested:** refunds, regeneration, rate limits, tier abuse and paid edits.

### CR-13 · Low · Confirmed · doc-mismatch — The docs describe a different program
- **Content types:** `docs/programs/content-registry.md:34-39` lists Post, Comment, Reply and Share costing 10/5/5/3. The code has Post, Article, Media, NFT and Thread costing 10/20/50/20/40 (`src/state/content_type.rs:27-34`).
- **Refunds:** the doc says 100+ engagements earn 50% (`:100`). The code gives 25/50/100/150% at 10/50/100/1,000 engagements, after a 24h delay.
- **Parameters and structs:**
  - The documented parameters (`:56-58,72-74`) omit `tracking_id`.
  - The struct layouts (`:105-135`) are wrong.
- **lib.rs:97** says "Update user tier (from staking program)". Nothing calls it from staking; it is admin-signed.

### Dead code and write-only state
- **Orphan file:** `src/state/config.rs` is not in `state/mod.rs`. It is an older `RegistryConfig` layout.
- **Never read:**
  - The `EnergyConfig` account and `RegistryConfig.identity_program` / `staking_program`.
  - `UserEnergy.energy_spent_today` / `energy_refunded_today` and `UserRateLimit.last_post_time`.
- **Never written:** `UserRateLimit.user` and `.bump`.
- **Unused:**
  - `ContentState::Archived` and `from_u8`, although `lib.rs:17` advertises an "→ Archived" state.
  - `ENERGY_COST_REPLY`/`ENERGY_COST_REPOST` (tests only).
  - Errors `ContentNotFound` and `CooldownNotElapsed`.
- **Missing events:** engagement updates, tier changes and authority transfer emit none.
