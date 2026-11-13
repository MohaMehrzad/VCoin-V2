# 15 — Stubs, TODOs, half-implemented features and dead code

**How this was found:**
- A marker grep: `(local audit scratch file, not included)`, 41 hits.
- A dead-code scanner: `(local audit scratch file, not included)`, reporting error codes never raised, events never emitted, and constants that are unused or used only in tests.
- Every program review (sections 01–11), which looked for state written but never read.

**Key observation:** the programs contain almost no TODO comments. Their unfinished work is invisible in grep. It looks like:
- config fields and accounts that are written once and never read,
- error variants that are never raised,
- features described in the docs that have no code path.

## A. Advertised features that are missing or broken

| Feature (where it's claimed) | Reality in code | Finding |
|---|---|---|
| "Production-ready ZK private voting" (README:24) | Verifier can never accept; 20/20 honest votes rejected (PoC) | ZK-01 |
| `ZK_VOTING_ENABLED = false` gate (SECURITY.md:132) | Constant doesn't exist; ZK instructions are live | ZK-07 |
| Governance execution ("full on-chain governance", lib.rs:7) | `execute_proposal` only sets a flag | GOV-06 |
| Vote delegation | No power is transferred | GOV-05 |
| Council fast-track and Community tiers | Constants are test-only | GOV-06 |
| 3-of-N oracle consensus | `required_consensus = 1`, no setter | 5A-01 |
| Vouch staking, rewards and slashing | Numbers recorded only, no token moves | 5A-05 |
| 5A privacy mode | `is_private` is never read | 5A-09 |
| SAS attestation verification | SAS program never read; args are caller-supplied | ID-03 |
| Admin verification path | `update_verification` can never succeed | ID-01 |
| Subscription tier gating | Nothing reads `Subscription` | ID-05 |
| "6-layer funding" | `FundingLayerConfig` written once, never read; it has 5 layers | SSC dead code |
| On-chain 5A × streak × vouch reward multipliers | `get_five_a_multiplier` used only in tests | SSC dead code |
| Automatic circuit breaker | Manual only; emission counters never compared | SSC-02 |
| Unclaimed rewards return to the pool | No sweep instruction | SSC dead code |
| Transfer hook updates 5A scores | No CPI; `five_a_program` never read | TH-04 |
| Transfer hook records tips | `is_tip: false` is hard-coded | TH-04 |
| Circular A→B→A wash detection | Not implemented | TH-04 |
| Working Token-2022 hook | Wrong discriminator, account order, meta list and PDA creation | TH-01, TH-02 |
| Gasless paymaster sponsoring SOL | Counters only; `fee_payer` never read | GAS-01 |
| Session keys that authorize actions | No program consumes them | GAS-01 |
| SSCRE fee deduction method | Unimplemented | GAS-13 |
| Fee withdrawal from the gasless vault | No instruction exists | GAS-03 |
| ViLink Challenge / Stake / ContentReact / Delegate / Vote actions | Can be created, can never be executed | VL-6 |
| ViLink batches | Storage only | VL-8 |
| ViLink dApp registry | Never enforced | VL-7 |
| ViLink 5A gating | Not implemented | VL docs |
| VCoin Token-2022 extensions (metadata, permanent delegate, hook) | Mint never created or checked | TOK-04 |
| veVCoin "non-transferable, time-weighted" | No extension check; no decay | VEV-01, STK-02 |
| Staking tier fee discounts (10–50%) | Exist only in a log-only view | STK-06 |
| Reentrancy guards | In-memory flag that does nothing | STK-09 |
| Content "Archived" state and the 1-hour edit window | Never set or enforced | CR-10, CR dead code |
| Content rent reclamation (delete.rs comment) | No account can be closed anywhere | X-6 |
| SDK transaction builders (README Quick Start) | All 23 return empty transactions | SDK-01 |

## B. Explicit stubs and placeholders in code

| Where | Code |
|---|---|
| `programs/vcoin-token/src/instructions/token/slash.rs:22` | `require!(false, VCoinError::DeprecatedSlashFunction);` — an instruction that always fails, still shipped in the program |
| `programs/governance-protocol/src/instructions/zk_voting/submit_decryption_share.rs:81` | `proposal_id: 0, // Proposal ID not stored…` |
| `programs/transfer-hook/src/instructions/hook/execute.rs:70` | `is_tip: false, // Tip detection requires additional context` |
| `programs/governance-protocol/src/instructions/vote/cast_private.rs:73,79` | `vote_record.vote_choice = 0;` — private votes decode as Abstain |
| `packages/viwoapp-sdk/src/staking/index.ts:209-210` (and 22 other builders) | `// For now, return placeholder transaction` |
| `packages/viwoapp-sdk/src/constants.ts:26` | `vcoinMint: new PublicKey("11111111111111111111111111111111"), // Placeholder` |
| `packages/viwoapp-sdk/src/governance/index.ts:194-195` | `const fiveAMultiplier = 1.0; // Placeholder` |
| `packages/viwoapp-sdk/src/governance/index.ts:91,115,266` | "simplified" parsing / "In production, use getProgramAccounts" / "quorum (simplified)" |
| `packages/viwoapp-sdk/src/rewards/index.ts:131,246,275` | "Simplified check", "In production, use proper keccak256", XOR "Placeholder" hash |
| `packages/viwoapp-sdk/src/staking/index.ts:44` | "In production, use proper IDL deserialization" |
| `packages/viwoapp-sdk/src/gasless/index.ts:280-284` | `const usedToday = 0; // Would track per-day usage` |
| `programs/sscre-protocol/src/state/user_claim.rs:30,71,96-98` | "Reserved for future epochs 1024+"; claims at epoch 1024 or later always fail (SSC-10, PoC) |
| `programs/*/src/constants.rs` (five-a, governance, staking, transfer-hook) | "L-02 … Option 2: Config Account Migration (Future Enhancement)" — parameters are hard-coded and need a program upgrade to change |

## C. Dead code, per program (from the scanner)

| Program | Errors never raised | Events never emitted | Constants unused / test-only |
|---|---|---|---|
| content-registry | ContentNotFound, CooldownNotElapsed | — | ENERGY_COST_REPLY, ENERGY_COST_REPOST (test-only) |
| five-a-protocol | InvalidStakeAmount, UserNotFound | — | MAX_CONFIRMING_ORACLES (unused); SNAPSHOT_INTERVAL (test-only) |
| gasless-protocol | InsufficientVCoinBalance, InvalidSessionKey, SessionAlreadyExists, FeeMethodNotAllowed, InvalidActionType, Overflow | — | DEFAULT_MAX_SLIPPAGE_BPS (unused); DAILY_BUDGET_SEED, DAILY_SUBSIDY_BUDGET_SOL, all 8 SCOPE_* (test-only) |
| governance-protocol | AlreadyVoted, QuorumNotReached, DelegationExists, DelegationNotFound, DelegationNotActive | — | COMMUNITY/DELEGATE/COUNCIL_THRESHOLD, MIN_DECRYPTION_THRESHOLD, MAX_COMMITTEE_SIZE (test-only) |
| identity-protocol | IdentityAlreadyExists, SubscriptionExpired, InsufficientPayment | — | — |
| sscre-protocol | InvalidEpoch, FundingLayerInactive | FundingLayerSwitch | — |
| staking-protocol | VeVCoinMintFailed, VeVCoinBurnFailed | — | — |
| transfer-hook | InvalidProgramOwner, Overflow, InvalidExtraAccountMetas | — | MIN_ACTIVITY_THRESHOLD, TRANSFER_RECORD_SEED (test-only) |
| vcoin-token | MintAlreadyInitialized, InvalidMintAuthority, SlashRequestNotFound | TokensSlashed, PermanentDelegateUpdated | SLASH_STATUS_REJECTED/CANCELLED (unused); VCOIN_DECIMALS, TOKEN_NAME/SYMBOL/URI (test-only) |
| vevcoin-token | MintAlreadyInitialized, TransferNotAllowed | — | VEVCOIN_DECIMALS, TOKEN_*, FOUR_YEARS_SECONDS (test-only) |
| vilink-protocol | DAppNotRegistered, ActionNotFound, InsufficientBalance, Overflow | — | ACTION_CHALLENGE/STAKE/CONTENT_REACT/DELEGATE/VOTE (test-only) |

Errors that are never raised often point to a missing check rather than harmless leftovers. Examples:
- `SubscriptionExpired`, `InsufficientPayment`: nothing checks subscription expiry or payment amount.
- `DelegationNotActive`, `DelegationNotFound`: delegation is never enforced.
- `DAppNotRegistered`: the dApp registry is never enforced.
- `InvalidExtraAccountMetas`: the hook's meta list is never validated.

*Scanner note: the first scan also listed vcoin's `SlashExecuted` as never emitted. That was a false positive. It is emitted through a path-qualified `emit!(crate::events::SlashExecuted …)` at `execute_slash.rs:55`, which the scanner's pattern missed. It is the only path-qualified `emit!` in the repo.*

## D. State that is written but never read
- **vcoin:** `config.treasury`.
- **vevcoin:** `UserVeVCoin.first_mint_at`, `last_update_at`.
- **staking:** `pool.vevcoin_program`, `pool.pool_vault`, `vault_bump`.
- **governance:** `GovernanceConfig.treasury_balance`; 10 `DelegateStats` fields that are never written; `VoteRecord.revealed`; `PrivateVotingConfig.is_enabled` (always true).
- **five-a:** `FiveAConfig.identity_program`, `vcoin_mint`, `vouch_vault`, `last_snapshot_time`; `Oracle.is_active`; `UserScore.is_private`, `last_snapshot_epoch`; `UserVouchStatus.reward_multiplier`, `is_fully_vouched`, `vouch_completed_at`; the `ScoreSnapshot` account.
- **identity:** `IdentityConfig.sas_program`; `Identity.is_active`; `Subscription.auto_renew`; `UserSASAttestation.portable_score`, `attestation_type`.
- **sscre:** `five_a_program`; `circuit_breaker_active`; `vault_bump`; `CircuitBreaker.authority`, `max_epoch_emission`, `max_single_claim`, `largest_claim_this_epoch`; the whole `FundingLayerConfig`; `EpochDistribution.avg_five_a_score`, `eligible_users`.
- **content-registry:** the whole `EnergyConfig`; `RegistryConfig.identity_program`, `staking_program`; `UserEnergy.energy_spent_today`, `energy_refunded_today`; `UserRateLimit.last_post_time`. `UserRateLimit.user` and `.bump` are never even written. `state/config.rs` is an orphan file.
- **transfer-hook:** `five_a_program`, `min_activity_amount`; `HookConfig.total_transfers`, `wash_trading_flags`; the `TransferRecord` account; `bump` (never written, which breaks the queries, TH-06).
- **gasless:** `fee_payer`, `sscre_program`, `sscre_deduction_bps`, `max_slippage_bps` (never written); the `DailyBudget` account.
- **vilink:** the five program-ID config fields; `RegisteredDApp`; `ActionBatch.executed_count`.

## E. Unused dependencies
- `programs/staking-protocol/Cargo.toml:26`: the `vcoin-token` crate.
- `programs/five-a-protocol/Cargo.toml:21`: `anchor-spl`.
- `programs/governance-protocol/Cargo.toml:21,23`: `anchor-spl`, `solana-hash`.
- `trident-tests`: every trident dependency.
