# VCoin-V2 — Full Code Audit Report

**Repository:** github.com/MohaMehrzad/VCoin-V2 @ `81959fb` (2026-02-13, "release: SDK v2.0.1 … 100% audit completion")
**Audited:** 2026-11-13 · all 11 Anchor programs, `@viwoapp/sdk` (TypeScript), `zk-voting-sdk` (Rust), all test suites, CI and docs
**Toolchain:** Rust 1.89.0 · Anchor CLI 0.32.0 (anchor-lang 0.32.1) · Solana CLI 3.0.13 · Node 24

## Verdict: ❌ Not ready for mainnet

The repo claims "100% audit complete — ready for mainnet". The code does not support that claim.

**Broken or capturable flows:**
- Governance can be captured for about 100 VCoin, and anyone can block execution of any proposal forever.
- Private (ZK) voting cannot accept a single vote. Reproduced: 20 of 20 honest SDK votes are rejected.
- The SSCRE reward vault has no on-chain spending cap per epoch.
- Unstaking depends on an undocumented mint setup and can lock funds permanently.
- The transfer hook cannot run under Token-2022.
- Gasless subsidies stop working permanently after 3 idle days.
- The published SDK sends empty transactions and misreads most accounts.

**Never verified end to end:**
- CI has failed on every push.
- No test executes a value-moving instruction.

**What is good:** the programs compile for SBF. Program IDs match. Most arithmetic is checked. One vote per wallet per proposal is enforced. SSCRE leaves are domain-separated. Unstake is not pausable by the admin. The 4 programs that have a 24h authority timelock implement it correctly.

## Findings by severity (after independent verification)

| Severity | Count | Findings |
|---|---|---|
| **Critical** | 2 | GOV-02 (governance capture via base-unit voting power), ZK-01 (private voting can never verify) |
| **High** | 32 | GOV-01, GOV-03, GOV-04 · SSC-01, SSC-02 · VEV-01, VEV-02, TOK-02, TOK-04, STK-02 · 5A-01 · ID-01 · TH-01, TH-02, TH-04 · GAS-01, GAS-02, GAS-03 · X-1 · SDK-01, SDK-03…06, SDK-08…10 · TOOL-1…4, TOOL-10 |
| **Medium** | 63 | see the sections |
| **Low** | 51 headed + ~45 grouped | see the sections |

- Duplicates (ZK-04/05 = GOV-07/08; SDK-02 = TOOL-10) are counted once.
- The stubs, TODOs, half-implemented features and dead code are inventoried in `15-stubs-todos-dead-code.md`.
- Every finding is pre-existing. This was a full audit with no diff.

## Top 10 fixes, in order
1. **Governance power and lifecycle** (GOV-02, GOV-01, GOV-03, GOV-04):
   - Compute power on whole tokens and add Sybil resistance (an identity or 5A gate, or linear ve).
   - Make `finalize` one-shot.
   - Require `lock_end ≥ end_time` and snapshot power at `start_time`.
   - Bound and timelock `update_config`.
2. **SSCRE vault safety** (SSC-02, SSC-01):
   - Cap claims at `total_allocation` and `max_epoch_emission`, and auto-trip the breaker.
   - Require M-of-N oracles or a timelock before a root is claimable.
   - `Box<>` the `ClaimRewards` accounts to get under the 4,096-byte stack limit.
3. **Unstake can't lock funds** (VEV-01, VEV-02, STK-03, STK-02):
   - Validate the veVCoin mint at init (PermanentDelegate and mint authority = config PDA; NonTransferable).
   - Burn `min(recorded, balance)`.
   - Timelock `update_staking_protocol`.
   - Burn whenever ve decreases, and add decay.
4. **Initialization and authority** (X-1, X-2): gate every `initialize` on the program's upgrade authority, and add the 24h authority timelock to the 7 programs that lack it.
5. **ZK voting** (ZK-01…06): either gate it behind a real flag, or do all of the following:
   - Rewrite the OR proof (AOS-chained, or CDS with c1).
   - Use one shared hash-to-scalar function.
   - Bind voter, proposal, key and weight into the transcripts.
   - Verify committee keys with Feldman commitments.
   - Require `reveal_completed` before finalize.
6. **Gasless** (GAS-02, GAS-03, GAS-16): delete the day-delta check, add `withdraw_fees`, and add a `max_fee` argument.
7. **Reputation and identity** (5A-01, ID-01): add a bounded consensus setter and oracle removal, and make the admin verification path reachable.
8. **Transfer hook** (TH-01…04): implement the SPL interface (discriminator, account order, signed meta-list PDA, `transferring` check), or remove the hook claims.
9. **SDK** (TOOL-3 first, then SDK-*):
   - Fix the IDL build.
   - Generate the builders and decoders from the IDL.
   - Fix seeds, enums and ESM/subpath exports.
   - Make `sendTransaction` reject empty transactions.
10. **Make the tests real** (TOOL-1…4): get CI green (fmt, clippy, IDL), replace byte-assertion tests with `litesvm` transaction tests, and remove `|| true` from CI.

## Report index

| File | Contents |
|---|---|
| `01-tokens-and-staking.md` | vcoin-token, vevcoin-token, staking-protocol |
| `02-governance-protocol.md` | proposals, voting power, delegation, admin |
| `03-zk-private-voting.md` | on-chain crypto plus `zk-voting-sdk` |
| `04-sscre-protocol.md` | rewards epochs, Merkle claims, circuit breaker |
| `05-content-registry.md` | content records, energy, refunds |
| `06-five-a-protocol.md` | oracles, consensus, vouching |
| `07-identity-protocol.md` | DID, attestations, subscriptions |
| `08-transfer-hook.md` | Token-2022 hook |
| `09-gasless-protocol.md` | session keys, subsidies, fees |
| `10-vilink-protocol.md` | action links, tips, batches |
| `11-typescript-sdk.md` | `@viwoapp/sdk` plus a method-by-method wiring table |
| `12-tests-ci-tooling.md` | build, test and CI results; packaging |
| `13-cross-cutting-security.md` | front-runnable inits, admin powers, permissionless instructions |
| `14-money-time-edge-cases.md` | edge-case analysis (P0–P3), invariants that hold |
| `15-stubs-todos-dead-code.md` | half-implemented features, stubs, dead code inventory |
| `16-docs-vs-code.md` | false doc claims; audit items marked FIXED that aren't |
| `17-qa-flow-traces.md` | 10 user flows traced through SDK → program |
| `18-tests-to-add.md` | regression tests for P0/P1 findings |
| `poc/` | runnable proof-of-concept tests (8, all passing) |

Each finding uses this format:

```
### <ID> · <Severity> · <Confidence> · <category> — title
- **Where:** path:line
- **Code:** the quoted lines
- **Fails when:** concrete scenario → consequence
- **Fix:** the concrete change
```

## Reproduce the key bugs

```bash
cd docs/audit/2026-11/poc
cargo test -- --nocapture --test-threads=1
```

The crate depends on the repo's own program crates by path. Each test asserts the current buggy behaviour, so passing means reproduced.

| Test | Finding | Observed |
|---|---|---|
| `bug_zk01_honest_sdk_vote_is_rejected_on_chain` | ZK-01 | 20/20 honest SDK votes rejected by `verify_vote_validity_proof` |
| `bug_gov02_ten_vcoin_wallets_outvote_entire_supply` | GOV-02 | 10 VCoin → 100,000 votes; 1B VCoin → 253,831 votes |
| `bug_gov09_proposal_threshold_is_a_millionth_of_a_vevcoin` | GOV-09 | threshold = 1,000 base units |
| `bug_stk03_topup_with_short_lock_lowers_recorded_ve` | STK-03 | 139,440 veVCoin orphaned, 0 burned |
| `bug_stk08_one_week_lock_is_under_credited` | STK-08 | 4.40e9 vs 5.27e9 (−16.6%) |
| `bug_ssc03_documented_leaf_differs_from_onchain_leaf` | SSC-03 | documented leaf ≠ on-chain leaf |
| `bug_ssc10_epoch_1024_can_never_be_claimed` | SSC-10 | `mark_epoch_claimed(1024)` → Err |
| `bug_id04_no_documents_classified_as_kyc` | ID-04 | email+phone+social → level 2 (KYC) |

## Methodology
- **Skills applied:** code-qualifier (what's wrong), edge-case-finder (what's missing), qa (flow tracing), code-bloodhound (every reference backed by a tool result).
- **Review:**
  - 10 parallel read-only reviewers: 7 per-program slices, the SDK wiring, and two cross-cutting passes (money/time; security/abuse).
  - A first-pass reading by 6 readers preceded them.
- **Verification:**
  - 3 independent adversarial verifiers re-checked **117 Critical/High/Medium findings**: **69 confirmed, 48 downgraded, 0 rejected**. Their corrections to wording and line numbers are applied.
  - About 15 headline findings were also re-verified by the lead directly in code ("Lead-verified" in the text).
- **Tools run:**
  - `anchor build --no-idl` ✅; `anchor build` ❌ (IDL)
  - `cargo test --workspace` ❌ (compile); `--lib` 265 passed / 14 failed
  - `cargo clippy` ❌; `cargo fmt --check` ❌ (2,413 diffs)
  - zk-voting-sdk `cargo test` ✅ (11/11, none verifies a proof)
  - SDK `npm ci` / `tsc` / `jest` ✅ (120/120); `npm run build` ✅
  - `npm pack @viwoapp/sdk@2.0.1` (export paths broken)
  - `gh run list` (CI red on every push)
  - Grep sweeps for markers and dead code (`(local audit scratch file, not included)`)
- **Not done:**
  - `cargo audit` (not installed).
  - The litesvm transaction tests in `18-tests-to-add.md` are sketches only; they need a new harness.
  - Compute units were not measured.

## Limits: not verified
- **Deployed devnet state:** we did not check whether the deployed programs match this commit, or how the VCoin and veVCoin mints were actually created. That decides whether VEV-01, TOK-01, STK-01, SSC-06, GAS-08 and VL-5 are live or latent.
- **Off-chain components:** the oracles, the Merkle tree builder, the relayer and the ViWoApp backend are not in this repo.
- **Token-2022 runtime:** behaviour with real extensions (TransferHook, TransferFee, CPI Guard) was traced through library source, not executed.
