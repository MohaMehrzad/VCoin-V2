# 04 — sscre-protocol (Self-Sustaining Circular Reward Economy)

**Program:** `programs/sscre-protocol` · ID `6AJNcQSfoiE2UAeUDyJUBumS9SBwhAdSznoAeYpXrxXZ`

**What it does:**
- The authority starts an epoch with an allocation.
- Any one registered oracle posts the epoch's Merkle root.
- Users claim VCoin from a single pre-funded `pool-vault`, minus a 1% fee rounded up.
- Leaf = `keccak("SSCRE_CLAIM_V1" ‖ user ‖ amount u64 LE ‖ epoch u64 LE)` (`src/state/utils.rs:24-35`).

**Verdict:** ❌ Nothing on-chain bounds an epoch's payout. The claim path exceeds the SBF stack limit. The documented leaf format produces unclaimable epochs. "6-layer funding" and the 5A multipliers exist only in docs.

---

### SSC-01 · High · Likely · bug — The `claim_rewards` account-validation stack frame is 4,544 bytes, over the 4,096-byte SBF limit
- **Where:** `programs/sscre-protocol/src/lib.rs:531-552`, the `ClaimRewards` accounts struct.
- **Evidence:** our `anchor build --no-idl` printed: `Error: Function …ClaimRewards…try_accounts… Stack offset of 4192 exceeded max offset of 4096 by 96 bytes … Estimated function frame size: 4544 bytes. Exceeding the maximum stack offset may cause undefined behavior during execution.`
- **Code:** `pub pool_config: Account<'info, RewardsPoolConfig>,` … `pub user_claim: Account<'info, UserClaim>,` … three `InterfaceAccount<'info, TokenAccount>`, no `Box<…>` anywhere in the repo.
- **Fails when:** every `claim_rewards`, the only instruction that moves user funds.
  - Overrunning the frame either causes an access violation, so every claim reverts, or silently corrupts memory.
  - No test ever executes this path (SSC-09).
  - The large items: 3 × `InterfaceAccount<TokenAccount>` (~624 B), `RewardsPoolConfig` (~392 B, it holds `oracles: [Pubkey;5]`), `UserClaim` (~216 B), plus the `init_if_needed` create-account temporaries.
- **Fix:**
  - Box them: `Box<Account<'info, RewardsPoolConfig>>`, `Box<Account<'info, UserClaim>>`, `Box<Account<'info, EpochDistribution>>`, `Box<InterfaceAccount<'info, Mint>>`, and `Box<InterfaceAccount<'info, TokenAccount>>` ×3.
  - Use `bump = pool_config.vault_bump` on `pool_vault` (`:542`). `vault_bump` is stored but never used.
  - Rebuild and confirm the warning is gone.

### SSC-02 · High · Confirmed · security — Claims are never capped by the epoch's allocation or the emission limits, so one root can drain the vault
- **Where:** `programs/sscre-protocol/src/lib.rs:226-227`, `:292-298`, `:132-149`
- **Code:**
  - `require!(amount <= MAX_SINGLE_CLAIM, …)` is the only bound.
  - `epoch_dist.total_claimed = epoch_dist.total_claimed.saturating_add(amount);` is never compared with `total_allocation`.
  - `cb.current_epoch_emission = …saturating_add(amount)` is never compared with `cb.max_epoch_emission`.
  - `config.remaining_reserves = …saturating_sub(amount);`
- **Fails when:** one oracle posts a root containing N leaves of 100K VCoin each for fresh wallets. The oracle is any of up to 5, first to call `update_merkle_root`; alternatively, the authority registers itself as an oracle with no timelock.
  - Those wallets drain the entire shared `pool_vault`, which holds every epoch's funds (up to 350M).
  - `start_epoch` resets `current_epoch_emission` while up to 4 earlier epochs remain claimable.
  - `max_epoch_emission` and `max_single_claim` (`:99-100`) are written and never read.
  - The advertised circuit breaker never trips automatically.
- **Fix:**
  - In `claim_rewards`: `require!(epoch_dist.total_claimed.checked_add(amount)? <= epoch_dist.total_allocation)` and `require!(cb.current_epoch_emission + amount <= cb.max_epoch_emission)`. Auto-trip the breaker on a breach, and use `cb.max_single_claim`.
  - Reserve the allocation at `start_epoch` (`remaining_reserves -= total_allocation`).
  - Require M-of-N oracle agreement, or a timelock, before a root becomes claimable.
  - Use checked math.

### SSC-03 · Medium · Confirmed · doc-mismatch — Following the documented leaf recipe makes an epoch unclaimable
- **Where:** `docs/programs/sscre-protocol.md:172-177` vs `programs/sscre-protocol/src/state/utils.rs:29-32`
- **Code:**
  - The doc's recipe: `"SSCRE_CLAIM_V1" ‖ user ‖ Buffer.from(a.amount.toString(16).padStart(16, "0"), "hex")`. The amount is big-endian and there is no epoch.
  - The program: `… data.extend_from_slice(&amount.to_le_bytes()); data.extend_from_slice(&epoch.to_le_bytes());`. The amount is little-endian and the epoch is included.
- **Fails when:** an oracle builds the tree using the only documented recipe. No leaf verifies, and the root can't be replaced (SSC-04). The allocation isn't reserved, so the tokens stay in the vault and can be re-issued in a later epoch: the cost is a delay for users, not lost funds.
  - Nothing in the repo builds leaves correctly. The SDK's `computeLeaf` is an XOR placeholder (`packages/viwoapp-sdk/src/rewards/index.ts:256-281`), and `tests/sscre-protocol.ts:380-386` omits the domain prefix.
  - **Reproduced by PoC:** `bug_ssc03_documented_leaf_differs_from_onchain_leaf`.
  - Downgraded from High by the verifier.
- **Fix:**
  - Correct the doc to `"SSCRE_CLAIM_V1" ‖ user(32) ‖ amount u64 LE ‖ epoch u64 LE`, hashed with keccak256, with nodes as sorted pairs.
  - Ship one reference tree builder, with a Rust test that cross-checks it against `compute_leaf`.

### SSC-04 · Medium · Confirmed · bug — A wrong or malicious root can never be replaced, and roots bypass the circuit breaker
- **Where:** `programs/sscre-protocol/src/lib.rs:185-186`, `:522-528` (`UpdateMerkleRoot` has no `circuit_breaker` account)
- **Code:** `require!(!config.paused, …); require!(!epoch_dist.is_finalized, SSCREError::EpochAlreadyExists);`
- **Fails when:**
  - A wrong root can't be replaced and the epoch can't be invalidated, so users lose that epoch.
  - The circuit breaker doesn't block root posting.
  - A root can be posted right after `start_epoch`; there is no `now >= end_time` check.
  - The only remedy is a permanent pause, which also blocks every legitimate epoch.
- **Fix:**
  - Add an authority-gated `invalidate_epoch` / re-root, allowed only while `total_claimed == 0` or behind a timelock.
  - Require `!circuit_breaker_active && now >= end_time` in `update_merkle_root`.

### SSC-05 · Low · Confirmed · bug — Pauses, circuit-breaker trips and late roots forfeit users' rewards (the tokens stay in the vault)
- **Where:** `programs/sscre-protocol/src/lib.rs:158`, `:223-224`, `:235`
- **Code:** `epoch_dist.claim_expiry = clock.unix_timestamp + EPOCH_DURATION + CLAIM_WINDOW;` … `require!(clock.unix_timestamp <= epoch_dist.claim_expiry, …)`
- **Fails when:**
  - A pause or breaker trip (minimum 6h) overlaps the end of a claim window. Claims are blocked, the expiry isn't extended, and those rewards are lost.
  - The window counts from `start_epoch`, not from root posting, so a late root shortens it further.
- **Fix:** start the window when the root is posted, and extend `claim_expiry` by the accumulated downtime.

### SSC-06 · Low · Likely · half-impl — Claims would fail if VCoin were minted with its documented transfer hook (latent; the extension can only be set when a mint is created)
- **Where:** `programs/sscre-protocol/src/lib.rs:251-263`, `:265-279`
- **Code:** `token_2022::transfer_checked(CpiContext::new_with_signer(… TransferChecked { from, to, authority, mint }, signer_seeds), …)`, with no `remaining_accounts` forwarded.
- **Fails when:** the docs promise VCoin has a transfer hook (`docs/programs/vcoin-token.md:15`). If it is enabled, every claim reverts for missing extra accounts. Today this is latent, because vcoin-token configures no extensions (TOK findings).
- **Fix:** `anchor_spl::token_2022::transfer_checked` never forwards remaining accounts, so `.with_remaining_accounts` doesn't help. Use `spl_token_2022::onchain::invoke_transfer_checked` with `ctx.remaining_accounts`, or build the instruction manually and add the extras with `spl_transfer_hook_interface::onchain::add_extra_accounts_for_execute_cpi`.

### SSC-07 · Low · Confirmed · half-impl — "Gasless" claims still need SOL for rent (a relayer can work around it by funding the user earlier in the same transaction)
- **Where:**
  - `programs/sscre-protocol/src/lib.rs:536`, `:548-549`
  - `programs/sscre-protocol/src/constants.rs:14-15`
- **Code:** `#[account(init_if_needed, payer = user, …)] pub user_claim` and `GASLESS_FEE_BPS: u16 = 100; // 1% deducted for gas`
- **Fails when:**
  - A user with 0 SOL makes their first claim through a relayer. The relayer can pay the transaction fee, but `UserClaim` rent (~0.0023 SOL) is hard-wired to `user`, so the claim fails.
  - The 1% "gas" fee goes to a fixed `fee_recipient`, not to whoever paid the gas.
- **Fix:** add a separate `payer: Signer` for `init_if_needed`, and optionally route the fee to the payer.

### SSC-08 · Medium · Confirmed · security — `initialize_pool` is front-runnable, with an arbitrary mint
- **Where:** `programs/sscre-protocol/src/lib.rs:467-477`
- **Code:** `#[account(init, payer = authority, … seeds = [POOL_CONFIG_SEED], bump)] pub pool_config` … `pub vcoin_mint: InterfaceAccount<'info, Mint>,`
- **Fails when:** anyone initializes first. They become the authority, with their chosen mint and `fee_recipient`, and the singleton PDA forces a redeploy.
- **Fix:** check the upgrade authority through the `ProgramData` account, and pin the VCoin mint.

### SSC-09 · Medium · Confirmed · test-gap — No test ever executes `claim_rewards`
- **Where:**
  - `programs/sscre-protocol/tests/common/mod.rs:163-175`: 7 accounts; `ClaimRewards` needs 11.
  - `tests/sscre-protocol.ts:348-356`: reset is called immediately after the trigger, so it must fail with `CircuitBreakerCooldown` (6h).
  - `programs/sscre-protocol/src/tests.rs:116,128,261-271,278-293`: stale size tests, stale epoch semantics, and floor-rounded fees while the program rounds up.
- **Fix:** add an end-to-end claim test with a real mint, a funded vault and a 2-leaf tree. Cover:
  - a double claim and an expired claim
  - proof lengths 0, 32 and 33
  - epochs 255/256, 511/512 and 1023/1024
  - an allocation overrun (SSC-02)

### SSC-10 · Low · Confirmed · bug — Epochs from 1024 on can never be claimed, and that is reachable quickly
- **Where:** `programs/sscre-protocol/src/state/user_claim.rs:96-98`, `programs/sscre-protocol/src/lib.rs:132`
- **Code:** `require!(false, SSCREError::TooManyEpochsClaimed);`
- **Fails when:** `start_epoch` has no spacing check, so epoch 1024 can be reached in 1,024 transactions. From then on every claim reverts.
- **Fix:** enforce `EPOCH_DURATION` spacing, or use per-epoch claim PDAs `[user, epoch]`.

### SSC-11 · Low · Confirmed · half-impl — Oracle and fee-recipient management is missing
- **Where:** `programs/sscre-protocol/src/lib.rs:113-125`, `:59`
- **Problem:**
  - There is no `remove_oracle`.
  - `Pubkey::default()` is accepted as an oracle, wasting one of the 5 slots forever.
  - `fee_recipient` has no setter.
- **Fix:** add `remove_oracle`, reject the zero key, and add a timelocked `set_fee_recipient`.

### SSC-12 · Low · Confirmed · bug — Out-of-range leaves are silently unclaimable
- **Where:** `programs/sscre-protocol/src/lib.rs:226-227`
- **Problem:** leaves below `MIN_CLAIM_AMOUNT` (1 VCoin) or above `MAX_SINGLE_CLAIM` (100K) can never be claimed. Nothing tells the tree builder, and the funds become untracked vault balance.
- **Fix:** validate leaf amounts in the reference tree builder and document the limits.

### SSC-13 · Low · Confirmed · bug — Misleading error, and inconsistent accounting units
- **Where:** `programs/sscre-protocol/src/lib.rs:224`, `:138`, `:283`, `:292-296`
- **Problem:**
  - A tripped breaker reports `CircuitBreakerEpochMax`.
  - `user_claim.total_claimed` adds the net amount, while the epoch and config totals add the gross amount.
- **Fix:** add a `CircuitBreakerActive` error, and use one unit consistently (gross or net) across all three totals.

### Partly refuted earlier claim
"Every claim fails if the fee_recipient's token account doesn't exist" is only a one-time inconvenience. `ClaimRewards` checks only `owner == fee_recipient` and the mint (`lib.rs:546`), so anyone can create that account permissionlessly. The missing setter (SSC-11) is real.

### Dead code, write-only state and doc mismatches
- **Never read:**
  - `RewardsPoolConfig.five_a_program`, set only by `set_five_a_program` (`lib.rs:437-438`).
  - `circuit_breaker_active` and `vault_bump`.
  - `CircuitBreaker.authority`, `max_epoch_emission`, `max_single_claim` and `largest_claim_this_epoch`.
  - `epoch_dist.avg_five_a_score` and `eligible_users`.
  - The whole `FundingLayerConfig` account (`lib.rs:74-91`), written once and never read.
- **Unused:**
  - `get_five_a_multiplier` and the `SCORE_MULT_*` constants (tests only).
  - Errors `InvalidEpoch` and `FundingLayerInactive`, and the `FundingLayerSwitch` event (never emitted).
  - `UserClaim._reserved` ("for epochs 1024+") is 8 bytes and has no realloc path.
- **Doc mismatches:**
  - `lib.rs:9-19` advertises "6-layer funding" and an on-chain `base × five_a × streak × vouch` formula. Neither exists, and the struct has 5 layers.
  - `docs/programs/sscre-protocol.md:52-54,62-64,72-74` list `epoch: u32` parameters that don't exist.
  - The struct layouts at `:94-130` are wrong.
  - The L0–L5 "scheduled minting / 250M every 5 years" model (`:25-36`) has no mint path.
  - "Unclaimed funds return to pool" (`:139`) is false: there is no sweep instruction.
  - The "monthly epoch" cadence is not enforced.
