# 16 — Documentation and audit claims vs code

**Scope:**
- `README.md`, `SECURITY.md`, `CHANGELOG.md`
- `docs/ARCHITECTURE.md`, `docs/API.md`, `docs/INTEGRATION.md`
- the three audit-status files
- the per-program docs. Their mismatches are listed at the end of each program section.

## The audit-status documents contradict each other

| Document | Claim |
|---|---|
| `AUDIT_COMPLETE.md:18-20` | 68 of 68 fixed, 0 remaining |
| `AUDIT_PROGRESS_REPORT.md` | 65 of 68 (96%) fixed, 3 remaining; High 7/8 |
| `REMAINING_AUDIT_ITEMS.md:5-6` | 53 of 68 (78%) fixed, 15 remaining (2 Critical, 3 High, 10 Medium) |
| `README.md:34` | "All 68 findings resolved" |
| `README.md:247` | "All 33 findings … fixed" |
| `SECURITY.md:55-57` | 33/37 |

All three audit files landed in one commit, `81959fb`, on 2026-02-13. Commit `973dde2` says "external audit pending". These are internal reviews.

## Spot-check of items `AUDIT_COMPLETE.md` marks FIXED

- **Present as described:** C-01, C-02, C-03, C-05, C-06, C-07, C-08, C-09/C-16, C-10, C-22, H-02, H-NEW-02, H-NEW-04.
  - C-06's ceiling division is in the code, but the vilink tests still assert floor division.
- **Not fixed, or fixed incorrectly:**

| ID | Claimed fix | Actual state |
|---|---|---|
| H-NEW-01 | 24h authority timelock "across all programs" | Present in 4 of 11 only (X-2) |
| H-01 | Full ZK private voting | Verifier can never accept (ZK-01, PoC 20/20 rejected) |
| H-NEW-05 | On-chain proposal threshold | Unit mismatch, effectively 0.000001 veVCoin (GOV-09, PoC) |
| C-11 | `require!(new_tier > 0 && new_tier <= 4)` | Rejects the valid tier 0, so users can never be demoted (CR-01) |
| M-04 | TierUnchanged spam guard | Makes `update_tier` permanently uncallable (STK-05) |
| C-AUDIT-15 | Clock-skew check in gasless | Causes the permanent subsidy brick (GAS-02) |
| C-AUDIT-17 | SAS attestation required for admin verification | Makes `update_verification` impossible (ID-01) |
| H-03 | Expiry re-check in consensus | Re-checks the same clock in the same instruction; moot with consensus = 1 (5A-01) |
| H-05 / H-NEW-03 | Delegation validated against balance | Point-in-time only, and delegation transfers no power (GOV-05) |
| H-AUDIT-20 | Extra-account-meta validation | `init` validates nothing; the PDA can't be created (TH-02) |
| H-05 (vilink) | Delegate check prevents a race | The race isn't real, and the check blocks legitimate users (VL-10) |
| M-REMAINING-05 | Batch nonce vs batch ID | `batch_id` is still timestamp-based (VL-8) |
| M-06 | Vault PDA includes the pool key (CHANGELOG:152-153) | Seeds are still `[POOL_VAULT_SEED]` |
| Reentrancy guards (CHANGELOG:135-137) | Present | No-op (STK-09) |

## False technical claims in the docs

| # | Claim | Where | Reality |
|---|---|---|---|
| DOC-1 | 24h authority timelock in all 11 programs | `SECURITY.md:72,94,144`; `README.md:258`; `ARCHITECTURE.md:165,194`; `CHANGELOG.md:64,168-169` | 4 of 11 (X-2) |
| DOC-2 | 3-of-N oracle consensus | `SECURITY.md:74,122,147`; `ARCHITECTURE.md:179,196,235`; `API.md:154`; `CHANGELOG.md:181` | 1-of-N (5A-01) |
| DOC-3 | `ZK_VOTING_ENABLED = false` blocks private voting | `SECURITY.md:61,63,116,132`; `ARCHITECTURE.md:186`; `CHANGELOG.md:193-194,201` | Constant doesn't exist; instructions are live (ZK-07) |
| DOC-4 | "Production-ready" ZK voting | `README.md:24,49` | Can't cast a vote (ZK-01) |
| DOC-5 | PDA seed tables and the CPI example | `INTEGRATION.md:291,354-359`; `ARCHITECTURE.md:324-345` | Real seeds use hyphens (`user-stake`, `vcoin-config`, …). VoteRecord is `["vote-record", proposal, voter]`; SlashRequest is `["slash-request", target, request_id]`. The CPI example at `INTEGRATION.md:291` fails its PDA check. |
| DOC-6 | VCoin mint has Metadata, PermanentDelegate and TransferHook | `ARCHITECTURE.md:112-115`; `API.md:47`; `README.md:167` | Mint never created or checked (TOK-04) |
| DOC-7 | Hook updates 5A and records tips | `API.md:395-397`; `ARCHITECTURE.md:77,277`; `README.md:170` | No (TH-04) |
| DOC-8 | Proposal threshold "1000 veVCoin" | `INTEGRATION.md:128-130` | 0.000001 veVCoin (GOV-09) |
| DOC-9 | Proposal struct and status codes | `API.md:209-223` | Fields are `start_time`/`end_time`/`execution_time`; statuses are 0 = Pending … 5 = Cancelled |
| DOC-10 | UserClaim layout | `API.md:255-263` | Different fields and offsets |
| DOC-11 | `VCoinConfig.pending_authority: Option<Pubkey>` | `API.md:66` | Plain `Pubkey` plus a timestamp |
| DOC-12 | `update_authority` instruction | `API.md:53,85` | It's `propose_authority` |
| DOC-13 | Slashing is "governance approved"; execute open to "Anyone" | `API.md:50-51`; `SECURITY.md:70,112,145`; `ARCHITECTURE.md:197` | One admin key approves; only the delegate can execute (TOK-02) |
| DOC-14 | Voting power = √ve × (1 + score/100) × tier | `API.md:228-233`; `ARCHITECTURE.md:258-260` | Boost is 1 + score/10000, and an undocumented diminishing cap is the dominant term (GOV-02) |
| DOC-15 | Error codes `InsufficientOracleConsensus`, `InvalidPDA`, `Paused` | `API.md:408-412,442` | Not defined in any program |
| DOC-16 | "6-layer funding" | `README.md:26`; `ARCHITECTURE.md:66` | 5 layers, never read |
| DOC-17 | "Reentrancy guards for CPI" | `SECURITY.md:105`; `ARCHITECTURE.md:170` | No-op (STK-09) |
| DOC-18 | "397 tests passing" / "279 Rust tests" | `README.md:12,103` | CI red on every push; 14 unit tests fail; integration tests don't compile (TOOL-1, TOOL-2) |
| DOC-19 | 48h timelock for slashing via governance | `ARCHITECTURE.md:174-176` | The timelock exists, but there's no governance link and no veto |
| DOC-20 | Pausable protocols as an emergency control | `ARCHITECTURE.md:184-185` | Pause covers one or two instructions per program (GOV-11, TOK-06, ID-08, CR-08, 5A-07) |
