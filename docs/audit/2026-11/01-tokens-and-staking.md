# 01 — vcoin-token, vevcoin-token, staking-protocol

**Programs:**
- `vcoin-token` (`Gg1dtrjAfGYi6NLC31WaJjZNBoucvD98rK2h1u9qrUjn`): admin layer over an externally created Token-2022 mint. Supply capped at 1B (9 decimals). Slashing flow: propose → approve → wait 48h → execute.
- `vevcoin-token` (`FB39ae9x53FxVL3pER9LqCPEx2TRnEnQP55i838Upnjx`): mints and burns veVCoin for the staking pool PDA.
- `staking-protocol` (`6EFcistyr2E81adLUcuBJRr8W2xzpt3D3dFYEcMewpWu`): locks VCoin for 1 week to 4 years. `ve = staked × ⌊d·1000/4y⌋ × tier boost(1.0–1.4) / 1e6`. CPIs veVCoin mint and burn, signed by the `staking-pool` PDA.

**Verdict:** ❌ Unstaking depends on an undocumented mint configuration and can be blocked permanently. "Governance-approved" slashing is one admin key, and veVCoin never decays. Neither token program creates or validates its mint, and the advertised Token-2022 extensions would break staking if they were enabled.

---

### VEV-01 · High · Likely · bug — Every unstake reverts unless the veVCoin mint's permanent delegate is the vevcoin config PDA, and a user who burns their own veVCoin is locked out forever
- **Where:**
  - `programs/vevcoin-token/src/instructions/token/burn.rs:22,37-48`
  - `programs/staking-protocol/src/instructions/user/unstake.rs:41-49,69-86`
- **Code:**
  - `token_2022::burn(… from: ctx.accounts.user_token_account …, authority: ctx.accounts.config.to_account_info() …)`
  - `require!(ctx.accounts.user_account.balance >= amount, VeVCoinError::InsufficientBalance);`
- **Fails when, case 1 (mint setup):** Token-2022 only lets the owner, an approved delegate, or the mint's PermanentDelegate burn from a user-owned account (`spl-token-2022 processor.rs` `process_burn`).
  - Nothing in the repo makes the config PDA the veVCoin mint's PermanentDelegate. `initialize_mint` takes an `UncheckedAccount` mint and never reads it.
  - The docs and tests create plain mints (`tests/vevcoin-token.ts:62`, `tests/staking-protocol.ts:88`; `docs/ARCHITECTURE.md:121-129` mentions only NonTransferable).
  - Result: every `unstake` with ve > 0 reverts and the VCoin stays in `pool-vault`. There is no admin recovery path.
- **Fails when, case 2 (self-burn, MON-2):** even with the mint set up correctly, the owner of a token account can always burn from it, NonTransferable or not, and wallet "burn spam" tools do exactly that.
  - Unstake then burns the recorded `ve_vcoin_amount`. That passes the program's own record check at `burn.rs:22`, but the Token-2022 burn fails for lack of funds, so the user's VCoin is permanently locked.
  - Lead-verified.
- **Fix:**
  - In `initialize_mint`, deserialize the mint and require: PermanentDelegate == config PDA, NonTransferable present, mint_authority == config PDA, supply == 0 and no freeze authority.
  - In unstake, burn `min(vevcoin_to_burn, user_vevcoin_account.amount)` and reconcile the record. Alternatively, hold ve in a program-owned escrow PDA so users can't burn it.
  - Add an end-to-end stake → unstake test with the real mint configuration.

### TOK-01 · Medium · Likely · security — The "permanent delegate" means two different things, which breaks slashing or hands one key unlimited burn power
*Downgraded from High by the verifier: the harm requires the deployer to misconfigure the mint, and the extra power goes to a key that already controls slashing (TOK-02).*
- **Where:**
  - `programs/vcoin-token/src/instructions/token/execute_slash.rs:37-47`
  - `programs/vcoin-token/src/contexts/execute_slash.rs:44-47`
  - `programs/vcoin-token/src/instructions/admin/initialize.rs:24`
  - `programs/vcoin-token/src/lib.rs:19`
- **Code:** `authority: config.to_account_info()`. The burn is signed by the config PDA, while the caller check is `executor.key() == config.permanent_delegate`, and the docs say "Permanent Delegate: Slashing authority for governance multisig".
- **Fails when:** deployers follow the naming and docs and set the multisig as the mint's Token-2022 PermanentDelegate.
  - `execute_slash` then always fails, because the config PDA has no burn rights.
  - The multisig can also `burn` or `transfer_checked` any holder's VCoin directly through Token-2022 at any time, skipping propose/approve and the 48h timelock entirely.
  - Slashing only works if the PermanentDelegate is the config PDA, and nothing verifies that.
- **Fix:** rename the config field to `slash_proposer`. Assert at init that the mint's PermanentDelegate is the config PDA, and document that no external key may hold it.

### TOK-02 · High · Confirmed · security — "Governance-approved" slashing is controlled by a single admin key
- **Where:**
  - `programs/vcoin-token/src/contexts/approve_slash.rs:15`
  - `programs/vcoin-token/src/instructions/admin/update_delegate.rs:21`
  - `programs/vcoin-token/src/instructions/token/approve_slash.rs:14`
- **Code:** `constraint = authority.key() == config.authority @ VCoinError::GovernanceApprovalRequired`; `config.permanent_delegate = new_delegate;`; `slash_request.proposal_id = proposal_id;`, which is never validated.
- **Fails when:** the admin calls `update_permanent_delegate(self)`. That change takes effect instantly, with no timelock, and the `PermanentDelegateUpdated` event is never emitted.
  - The admin then proposes, approves, waits 48h and executes, burning anyone's VCoin.
  - No governance program is ever consulted; `proposal_id` is free-form.
- **Fix:**
  - Require approval through CPI from the governance program (a governance PDA signer) and verify the proposal account.
  - Timelock `update_permanent_delegate` and emit its event.
  - Reject proposer == approver.

### TOK-03 · Medium · Confirmed · half-impl — Slashes are easy to dodge and can't be cancelled; approvals never expire; staked VCoin can't be slashed fairly
- **Where:**
  - `programs/vcoin-token/src/instructions/token/execute_slash.rs:25-28`
  - `programs/vcoin-token/src/constants.rs:27-28`
  - `programs/vcoin-token/src/state/slash_request.rs:66-68`
- **Code:** `require!(ctx.accounts.target_account.amount >= slash_request.amount, VCoinError::SlashingExceedsBalance);`. `SLASH_STATUS_REJECTED`/`CANCELLED` are never assigned.
- **Fails when:**
  - **Dodging:** the target sees `SlashProposed` and moves `balance − amount + 1` base units out during the ≥48h window. That is a single unit when the slash equals the whole balance. The slash is all-or-nothing, so it fails.
  - **No veto:** there is no reject or cancel instruction, so a wrong approval can't be stopped.
  - **No expiry:** a request approved years ago can still run against a refilled account, or against an account whose owner has since been reassigned (MON-6).
  - **Vault targets:** any VCoin token account is accepted as a target, including the staking `pool-vault`. Burning from the vault makes staking insolvent, and the last unstakers can't withdraw (MON-7).
  - **Rent:** SlashRequest accounts are never closed.
- **Fix:**
  - Burn `min(amount, balance)`, or freeze the account at proposal time.
  - Add reject/cancel instructions and an expiry.
  - Store the target's owner and re-check it at execution.
  - Reject protocol-owned vaults as targets.
  - Slash staked positions in staking-protocol instead.

### TOK-04 · High · Likely · security — `initialize_mint` is front-runnable and never validates the mint; supply-cap claims don't hold
- **Where:**
  - `programs/vcoin-token/src/contexts/initialize_mint.rs:7-24`
  - `programs/vcoin-token/src/instructions/admin/initialize.rs:6-27`
- **Code:**
  - `/// CHECK: Validated by Token-2022 program  pub mint: UncheckedAccount<'info>`. The handler never reads the mint.
  - The comment claims "This creates the mint with: Metadata extension, Permanent delegate extension".
- **Fails when:**
  - **Front-run:** the repo's own flow (`tests/vcoin-token.ts:52-91`) first creates the mint with mint authority = config PDA, then initializes in a separate transaction. An attacker who initializes in between becomes `authority` and can call `mint_tokens` for the full 1B.
  - **No mint checks:** nothing verifies the mint's owner, decimals, mint or freeze authority, existing supply or extensions.
    - Supply minted before the authority moved bypasses `TOTAL_SUPPLY` (`mint.rs:28-31`).
    - A leftover freeze authority can freeze the staking, SSCRE and gasless vaults (MON-8).
  - **Comments overclaim:** no extension is ever created. `TOKEN_NAME/SYMBOL/URI` and `VCOIN_DECIMALS` are used only in tests.
- **Fix:**
  - Gate `initialize` on the program's upgrade authority (check ProgramData).
  - Create the mint inside the instruction, or assert decimals = 9, mint_authority = PDA, freeze_authority = None, supply = 0, and the expected extension set.

### VEV-02 · High · Confirmed · security — The veVCoin admin can freeze every unstake instantly, or brick chosen users
- **Where:**
  - `programs/vevcoin-token/src/instructions/admin/update_staking_protocol.rs:11-23`
  - `programs/vevcoin-token/src/instructions/token/burn.rs:22`
- **Code:** `config.staking_protocol = new_staking_protocol;` is single-step with no timelock.
- **Fails when:**
  - **Freeze everyone:** pointing `staking_protocol` anywhere other than the staking pool PDA makes every unstake with ve > 0 fail `Unauthorized`.
  - **Brick one user:** pointing it at the admin's own key lets them call `burn_vevcoin` on a victim. The victim's `UserVeVCoin.balance` drops below `UserStake.ve_vcoin_amount`, so their unstake fails at `burn.rs:22` until the admin re-mints to restore it.
- **Fix:** use propose/accept with a timelock, or derive the expected pool PDA from the staking program ID and pin it.

### STK-01 · Medium · Likely · bug — If VCoin ships the advertised transfer hook, stake and unstake revert; a transfer fee would make the vault insolvent
*Latent: no mint in the repo has TransferHook or TransferFee. If the hook is set when the mint is created, the first stake fails, so nothing gets locked. Funds lock only if a hook program is set later. Downgraded from High by the verifier.*
- **Where:**
  - `programs/staking-protocol/src/instructions/user/stake.rs:67-79`
  - `programs/staking-protocol/src/instructions/user/unstake.rs:90-103`
- **Code:** `token_2022::transfer_checked(CpiContext::new(...), amount, ctx.accounts.vcoin_mint.decimals)?;`. anchor-spl 0.32.1 passes only from, mint, to and authority.
- **Fails when:**
  - **Transfer hook:** the VCoin mint carries the TransferHook that README.md:167, ARCHITECTURE.md:115 and vcoin `lib.rs:20` advertise. The hook program and extra-account-metas are not forwarded, so every stake and unstake reverts.
  - **Transfer fee:** if a fee is added, the vault receives `amount − fee` while `staked_amount` is credited the full `amount`, and the last unstakers can't withdraw.
- **Fix:**
  - Use `spl_token_2022::onchain::invoke_transfer_checked` with `remaining_accounts`.
  - Credit the stake from the vault balance difference, or reject mints with a TransferFeeConfig.

### STK-02 · High · Confirmed · bug — veVCoin never decays, so expired locks keep full voting power
- **Where:**
  - `programs/staking-protocol/src/instructions/user/unstake.rs:41-46,113`
  - `programs/staking-protocol/src/state/user_stake.rs:20`
  - Governance reads it at `programs/governance-protocol/src/instructions/vote/cast.rs:80-84`.
- **Code:** `((ve_vcoin_amount as u128) * (new_staked_amount as u128) / (staked_amount as u128)) as u64`. Nothing changes at `lock_end`.
- **Fails when:** a 4-year lock ends. The holder keeps the full 1.4× 4-year voting power on tokens they can withdraw at any time, so they can vote and unstake in the same slot (see GOV-03). This contradicts "Time-weighted voting power" (`docs/programs/vevcoin-token.md:17`).
- **Fix:** derive power from the remaining lock, `amount × max(0, lock_end − now)/4y × boost`, either on read or via checkpoints. At minimum, treat ve as 0 once `now ≥ lock_end`.

### STK-03 · Medium · Confirmed · bug — Top-ups and extends can lower recorded ve without burning, leaving orphaned veVCoin that can be minted repeatedly
**Reproduced by PoC:** `poc/tests/poc.rs::bug_stk03_topup_with_short_lock_lowers_recorded_ve` shows old ve 140,000 VCoin, new recorded ve 560, minted 0, and 139,440 orphaned.
- **Where:**
  - `programs/staking-protocol/src/instructions/user/stake.rs:52-53,116`
  - `programs/staking-protocol/src/instructions/user/extend_lock.rs:33-34,72`
  - `programs/staking-protocol/src/instructions/user/update_tier.rs:26-31,65`
- **Code:** `let vevcoin_to_mint = new_vevcoin.checked_sub(old_vevcoin).unwrap_or(0);` … `user_stake.ve_vcoin_amount = new_vevcoin;`
- **Fails when, case 1 (top-up):** a user stakes 100k for 4 years (ve 140k). After the lock ends, they top up 1 base unit with a 1-week lock, which is allowed because the end time doesn't move back.
  - The recorded ve drops to about 560 with no burn, and a full unstake later burns only 560.
  - The user keeps about 139k veVCoin tokens and `UserVeVCoin.balance` with zero stake.
- **Fails when, case 2 (repeatable mint, MON-3):** `stake(1, MAX−1)` lowers the record by 0.1% because the factor rounds down to 999. A same-transaction `extend_lock(MAX)` then mints the difference.
  - Repeating this mints about 0.1% of ve per second as untracked tokens.
  - Governance reads `UserStake`, so votes are not inflated. veVCoin supply, `UserVeVCoin`, holder counts and the SDK's `getVeVCoinBalance` are.
  - Lead-verified.
- **Fix:** when `new < old`, CPI `burn_vevcoin(old − new)`, or reject the operation.

### STK-04 · Medium · Confirmed · bug — Partial unstake keeps the old tier's boost permanently
- **Where:** `programs/staking-protocol/src/instructions/user/unstake.rs:41-46,112`
- **Code:** `user_stake.tier = new_tier.as_u8();`, while ve is scaled proportionally from the old boosted value.
- **Fails when:** a Platinum staker (100k, 1.4×) unstakes down to 500 VCoin, which is tier None. They keep ve = 700 instead of 500 (+40%). `update_tier` then returns TierUnchanged, so the boost can never be corrected.
- **Fix:** recompute with `calculate_vevcoin(new_staked, lock_duration, new_tier)` and burn the difference.

### STK-05 · Low · Confirmed · dead-code — `update_tier` can never succeed
- **Where:** `programs/staking-protocol/src/instructions/user/update_tier.rs:17-21`
- **Code:** `require!(new_tier.as_u8() != old_tier, StakingError::TierUnchanged);`
- **Fails when:** always. `stake.rs:115` and `unstake.rs:112` always store `from_amount(staked)`, so the tier is never stale.
  - The "M-04 fix" therefore created a dead instruction. If tier thresholds were ever upgraded, it would mint using the original duration and never burn.
  - The docs say "Authority: Anyone" (`staking-protocol.md:85`); the code requires the owner.
- **Fix:** remove it, or recompute from the remaining lock with a burn on downgrade.

### TOK-05 · Low · Confirmed · bug — `propose_slash` requires `request_id` to equal the exact on-chain second (admin-only and retryable, so a usability issue)
- **Where:**
  - `programs/vcoin-token/src/instructions/token/propose_slash.rs:26-29`
  - `programs/vcoin-token/src/contexts/propose_slash.rs:24`
- **Code:** `require!(request_id == clock.unix_timestamp as u64, VCoinError::InvalidRequestId);`
- **Fails when:** the client must guess the cluster clock of the slot where its transaction lands, and also derive the PDA from that guess. Many submissions fail, and only one request per target per second is possible.
- **Fix:** seed with a `slash_nonce` counter kept in the config.

### TOK-06 · Medium · Confirmed · half-impl — Pause covers almost nothing
- **Where:**
  - `programs/vcoin-token/src/instructions/token/mint.rs:25`, the only read of the vcoin `paused` flag.
  - `programs/staking-protocol/src/instructions/user/stake.rs:27`, the only read of the staking `paused` flag.
- **Fails when:**
  - VCoin's pause only blocks admin-only minting, while the slash instructions keep working.
  - Staking's pause blocks `stake` but not `extend_lock` or `update_tier`, both of which mint veVCoin.
  - vevcoin has no pause at all.
  - The docs promise "Pausable for emergency situations" (`vcoin-token.md:96`, `staking-protocol.md:126`).
- **Fix:** add the pause check where it matters, and document the coverage.

### VEV-03 · Low · Likely · doc-mismatch — Docs say "authorized staking program"; the code needs the staking-pool PDA (fails on the first stake and the admin can fix it)
- **Where:**
  - `programs/vevcoin-token/src/instructions/token/mint.rs:17-20`
  - `docs/API.md:84`
  - `docs/programs/vevcoin-token.md:64-66`
- **Code:** `ctx.accounts.staking_protocol.key() == ctx.accounts.config.staking_protocol`. Staking signs as the pool PDA (`stake.rs:90`).
- **Fails when:** an operator stores the staking program ID as documented. A program ID can't sign, so every stake fails `Unauthorized`.
  - `tests/staking-protocol.ts:91` also makes the pool PDA the veVCoin mint authority, while vevcoin mints as its own config PDA (`mint.rs:46`).
- **Fix:** document `PDA(["staking-pool"], staking_program_id)`, or derive and check it on-chain.

### STK-06 · Low · Confirmed · half-impl — The tier fee discounts don't exist anywhere
- **Where:**
  - `programs/staking-protocol/src/instructions/query/get_stake_info.rs:28`
  - `programs/staking-protocol/src/state/tier.rs:41-49`
- **Fails when:** users expect the advertised 10–50% discounts (`staking-protocol.md:16,21-27`; `lib.rs:26-30`). No program reads the tier or the discount; they exist only in a log-only view.
- **Fix:** implement the discount in gasless and vilink fees, or remove the claim.

### STK-07 / VEV-04 · Medium · Confirmed · security — Staking and veVCoin initializers are front-runnable
- **Where:**
  - `programs/staking-protocol/src/contexts/initialize_pool.rs:9-38`
  - `programs/vevcoin-token/src/contexts/initialize_mint.rs:7-24`
- **Fails when:**
  - **Staking:** an attacker initializes with fake mints. The singleton pool and vault PDAs are taken, forcing a redeploy.
  - **veVCoin:** an attacker sets `staking_protocol` to their own key. If the real ve mint's authority is already the config PDA, they control veVCoin issuance. The veVCoin mint is completely unchecked; staking's mints are at least owner-checked as `InterfaceAccount`.
- **Fix:** check the upgrade authority, validate the mint, and cross-check `vcoin_mint`/`vevcoin_mint` against the token programs' configs.

### Test gap · Low · Confirmed — The money paths have never executed (aggregated as TOOL-4, High)
- **Where:**
  - `programs/staking-protocol/tests/unstake.rs:25-28` and `programs/vcoin-token/tests/slash.rs:61-65` check bytes only.
  - `programs/vcoin-token/tests/admin.rs:224-232` calls a non-existent `update_authority` and expects proposing yourself to succeed.
  - `programs/vcoin-token/src/tests.rs:122-126` and `programs/vevcoin-token/src/tests.rs:219-229` are tautologies.
  - The TS staking tests omit the veVCoin CPI accounts.
- **Consequence:** VEV-01, TOK-01 and STK-01 have never been exercised.

### Low findings
- **STK-08 · Truncating ve formula (reproduced by PoC: 4.40e9 vs 5.27e9):** `programs/staking-protocol/src/utils.rs:14`, `let duration_factor = (lock_duration as u128) * 1000 / (FOUR_YEARS_SECONDS as u128);`, truncates before multiplying. A 1-week lock gets 4‰ instead of 4.79‰ (−16.6%). Use `amount × lock_duration × boost / (4y × 1000)` in u128.
- **STK-09 · Reentrancy guard is a no-op:**
  - `reentrancy_guard` (`state/pool.rs:33`; `stake.rs:24,63,128`; `unstake.rs:65,125`) is set and cleared in memory within one instruction, so on-chain it is always false.
  - The Solana runtime already blocks A→B→A reentrancy.
  - The docs list it as a security feature. Remove it.
- **TOK-07 · Panics instead of errors:** vcoin `mint.rs:29,54` uses `checked_add(amount).unwrap()`, which panics instead of returning `ExceedsMaxSupply`. `mint_tokens` also accepts 0.
  - The "vevcoin burn panics" claim from the first pass is refuted: `burn.rs:28` is guarded by the check at `:22`.
- **TOK-08 · Wrong error code:** `contexts/execute_slash.rs:38` returns `InvalidMint` for a wrong target; `InvalidTarget` exists.
- **Stale pending timestamp:** `cancel_authority_transfer` leaves `pending_authority_activated_at` stale. Harmless today.
- **Perf:** `bump` without the stored value re-derives PDAs on every call (`stake.rs:17,26,54`; `unstake.rs:17,24,45`; vevcoin contexts), even though `pool.bump`, `vault_bump` and `config.bump` are stored.

### Dead code, stubs and write-only state
- **vcoin:**
  - Errors never raised: `MintAlreadyInitialized`, `InvalidMintAuthority`, `SlashRequestNotFound`.
  - Events never emitted: `TokensSlashed`, `PermanentDelegateUpdated`. `SlashExecuted` is emitted at `execute_slash.rs:55`.
  - Constants: `SLASH_STATUS_REJECTED/CANCELLED` are unused; `TOKEN_NAME/SYMBOL/URI` and `VCOIN_DECIMALS` are tests-only.
  - State: `config.treasury` is written but never read.
  - Stub: `slash_tokens` plus its context is a permanent `require!(false)` (`instructions/token/slash.rs:22`) that still ships in the program.
- **vevcoin:**
  - Errors: `MintAlreadyInitialized`, `TransferNotAllowed`.
  - Constants: `TOKEN_*`, `VEVCOIN_DECIMALS` and `FOUR_YEARS_SECONDS` are tests-only.
  - State: `first_mint_at`/`last_update_at` are write-only, and `first_mint_at` resets whenever the balance returns from 0.
- **staking:**
  - Errors: `VeVCoinMintFailed`, `VeVCoinBurnFailed`.
  - State: `pool.vevcoin_program`, `pool.pool_vault` and `vault_bump` are write-only.
  - Dependencies: the `vcoin-token` crate (`Cargo.toml:26`) is unused.
  - Constants: `FOUR_YEARS_SECONDS` duplicates `MAX_LOCK_DURATION`.

### Doc mismatches
- **vcoin-token.md:**
  - `:33` says "Creates the VCoin mint with Token-2022 extensions"; nothing is created.
  - `:54` says the target is a "wallet"; it is a token account, and the required `request_id` is undocumented.
  - `:68` says execute is open to "Anyone"; only the permanent delegate can call it.
  - `:60` says approval is by "Governance authority"; it is the admin key.
  - `:86-87` describe `Option` fields; they are plain fields.
- **vevcoin-token.md:**
  - `:78-86` omit the pending-authority fields.
  - `:17` calls it "time-weighted"; there is no decay.
  - `:52,:62` treat `user` as a parameter; it is an account.
- **staking-protocol.md:**
  - `:68` says a new lock "must be longer"; only the end time must be later, which enables STK-03.
  - `:79` says unstake "resets user stake"; that is only true for a full unstake.
  - `:96-108` show the wrong struct.
- **vcoin `lib.rs:17-20` and `Cargo.toml:4`** advertise Transfer Hook, Permanent Delegate and Metadata. None of these is configured.
