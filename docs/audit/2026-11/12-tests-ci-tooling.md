# 12 — Tests, CI and Tooling

**Scope:** `.github/workflows/*.yml`, root `package.json`, `Anchor.toml`, `programs/*/tests/`, `programs/*/src/tests.rs`, `tests/`, `tests-bankrun/`, `trident-tests/`, and the SDK packaging in `packages/viwoapp-sdk/package.json`.

**What was run on 2026-11-13** (Rust 1.89.0, Anchor CLI 0.32.0, Solana CLI 3.0.13, Node 24):

| Command | Result |
|---|---|
| `anchor build --no-idl` | ✅ All 11 `.so` files built. One stack-frame warning (see SSC findings). |
| `anchor build` (with IDL) | ❌ Exit 1: `Building IDL failed`. It compiles the broken integration tests. |
| `cargo test --workspace` | ❌ Integration-test targets fail to compile. |
| `cargo test --workspace --lib --no-fail-fast` | ⚠️ 265 passed, 14 failed (279 total). |
| `cargo clippy --workspace --lib` | ❌ Exit 101: one deny-level lint and about 300 warnings. |
| `cargo fmt --all -- --check` | ❌ 2,413 diffs. |
| SDK `npm ci && tsc --noEmit && jest && npm run build` | ✅ Typechecks, 120/120 jest tests pass, build succeeds. The jest tests assert the wrong values (see SDK report). |
| `npm pack @viwoapp/sdk@2.0.1` (published tarball) | ❌ It contains only `dist/index.{js,mjs,d.ts,d.mts}`, but `package.json` exports `dist/index.esm.js` and 6 subpath folders. |
| `gh run list` (GitHub Actions) | ❌ "ViWoApp Test Suite" failed on **every** push, including the "100% audit completion" release. "Fuzz Testing" is green only because it ignores failures. |

---

### TOOL-1 · High · Confirmed · tooling — The main CI suite has been red on every push, including the release
- **Where:** `.github/workflows/test.yml:63-73`
- **Evidence:** `gh run list -R MohaMehrzad/VCoin-V2 --workflow "ViWoApp Test Suite"`:
  - Runs 20528460783 (2025-12-26), 21967128189, 21967542919, 21967991378 and 21969219695 (2026-02-13) all show `failure`.
  - Run 21969219695 is the "release: SDK v2.0.1 … 100% audit completion" commit. Its "Build Programs" job failed with exit code 101, and Rust tests, TypeScript tests and coverage were skipped.
- **Code:** `anchor build` · `cargo fmt --all -- --check` · `cargo clippy --all-targets --all-features -- -D warnings`
- **Fails when:** every push. Three independent causes, each reproduced locally:
  - `anchor build` with IDL fails (TOOL-3).
  - `cargo fmt --check` reports 2,413 diffs.
  - `clippy --all-targets` compiles the broken integration tests. It also hits a deny-level lint and roughly 300 warnings that `-D warnings` turns into errors.
- **Consequence:** no merged code has ever passed CI. The "397 tests passing" badge (`README.md:12`) is static and not backed by any green run.
- **Fix:**
  - Run `cargo fmt --all`.
  - Fix the integration tests (TOOL-2) or exclude them from the IDL build.
  - Fix the clippy error (TOOL-12).
  - Make the branch protection require the suite.

### TOOL-2 · High · Confirmed · test-gap — None of the 209 Rust integration tests in `programs/*/tests` compile
- **Where:**
  - `programs/vilink-protocol/tests/common/mod.rs:32`, `:65-78`
  - `programs/gasless-protocol/tests/common/mod.rs:71`, `:187`
  - Similar `common/mod.rs` harnesses in all 11 programs
- **Code:** `processor!(vilink_protocol::entry),` · `) -> Result<(), BanksClientError> {`
- **Fails when:** `cargo test --workspace` or `cargo test -p <program> --test <name>`. Three causes:
  - `E0308`: Anchor 0.32's `entry` takes `&'info [AccountInfo<'info>]`, which doesn't match `processor!`'s higher-ranked fn type in solana-program-test 2.3.
  - `E0107`: Anchor's prelude `Result<T>` alias shadows `std::result::Result<T, E>`.
  - `E0433`: `solana_program::hash` no longer exists.

  The gasless harness also uses `spl_token_2022`, which is not a dev-dependency.
- **Consequence:** even when compiled, most of these tests only build an `Instruction` and assert bytes, for example `programs/vilink-protocol/tests/admin.rs:21`: `assert_eq!(ix.data[8], 1);`. Several use account lists that don't match the contexts (see CR/SSC findings). They give no on-chain assurance.
- **Fix:**
  - Rebuild the harness on `litesvm` or `mollusk-svm`, loading `target/deploy/*.so`. Alternatively wrap `entry` in a shim fn with the expected signature.
  - Fully qualify `std::result::Result`.
  - Replace byte-assertion tests with tests that execute the transaction and assert resulting state.

### TOOL-3 · High · Confirmed · tooling — IDL generation is broken, so the TypeScript tests and an IDL-based SDK are impossible
- **Where:** `anchor build` → `Error: Building IDL failed`. The IDL step compiles every test target (`could not compile vilink-protocol (test "action")…`, `E0433 could not find hash in solana_program`).
- **Fails when:** a fresh clone runs `anchor build` or `anchor test`.
- **Consequence:**
  - `target/idl/` and `target/types/` are never produced.
  - The mocha tests import from `target/types`, e.g. `tests/staking-protocol.ts:3`, so they cannot run.
  - No IDL is published, so integrators can't generate clients. This is likely why the SDK hand-rolls byte offsets, many of them wrong.
- **Fix:** fix TOOL-2, or move the integration tests behind a feature flag that the IDL build doesn't enable. Commit or publish IDLs per release.

### TOOL-4 · High · Confirmed · test-gap — No test executes any program instruction end to end
- **Where:**
  - `tests-bankrun/tests/*.ts`: no file calls `startAnchor(`. `staking.test.ts:7-8` imports `BankrunProvider` and `startAnchor` but never uses them.
  - `trident-tests/`
  - `tests/*.ts`
- **Fails when:** you rely on any of the 118 bankrun "tests":
  - They re-implement program logic inline and assert against it, e.g. `getTier` at `tests-bankrun/tests/staking.test.ts:23-38`.
  - Some assert facts that contradict the chain. `governance.test.ts:89` says "On-chain enum: Against=0, For=1, Abstain=2"; the program has Abstain=0, For=1, Against=2 (`programs/governance-protocol/src/state/vote_choice.rs:7-9`). `sscre.test.ts:21,26` hashes with sha256 where the program uses keccak.
  - The trident "fuzz" targets print `is_valid()` for hard-coded inputs (`trident-tests/src/fuzz_staking.rs:7-31`). The trident dependencies are unused.
  - The Anchor mocha tests are stale against the current interfaces:
    - `castVote(choice, balance, score, tier)` against an on-chain `cast_vote(choice)`.
    - `stake` omits the 4–5 veVCoin accounts.
    - Several assertions sit inside `try { … expect.fail() } catch {}` blocks, so they can never fail.
- **Consequence:** these value-moving paths have zero executed coverage:
  - stake → veVCoin CPI, unstake
  - SSCRE claim
  - ViLink tip
  - gasless fee deduction
  - identity subscribe
  - slash execute
  - every ZK instruction
- **Fix:** one `litesvm` integration test per instruction, covering happy path, wrong signer, wrong account and repeated call (see `18-tests-to-add.md`).

### TOOL-5 · Medium · Confirmed · tooling — The fuzz workflow reports success while doing nothing
- **Where:** `.github/workflows/fuzz.yml:62-63` (and `:98-99`, `:134-135`)
- **Code:** `cd trident-tests && cargo run --bin fuzz_staking || true` + `continue-on-error: true`
- **Fails when:** every Sunday. `trident-tests` is not a workspace member, and its `{ workspace = true }` dependencies (`trident-tests/Cargo.toml:14-22`) have nothing to resolve against, so the binary can't build. The failure is swallowed, and runs 22535867971 … 24621318539 all show ✅.
- **Fix:** remove the `|| true` and `continue-on-error`, make trident a real crate with a real `fuzz_instructions` implementation, or delete the workflow.

### TOOL-6 · Medium · Confirmed · tooling — The CI environment cannot match the project's toolchain
- **Where:** `.github/workflows/test.yml:27-30,52-55,159-162`; `fuzz.yml:24-26`; `Anchor.toml:36-38`
- **Problems:**
  - CI pins Rust `1.79.0`, but `rust-toolchain.toml:2` pins `1.89.0`.
  - Solana is installed from `https://release.solana.com/...`; a TLS connection to that host fails today (`curl: (35) SSL_ERROR_SYSCALL`).
  - The TypeScript job runs `solana-keygen new` and then `anchor test`, but `Anchor.toml` sets `cluster = "devnet"`. That deploy would need a funded devnet wallet, and the program keypairs aren't in the repo (TOOL-9).
  - CI runs `yarn install` under Yarn 1, while `yarn.lock` is Yarn Berry v8 (`__metadata: version: 8`).
- **Fix:**
  - Use the toolchain file.
  - Install Agave from `release.anza.xyz`.
  - Run `anchor test` against localnet: set `--provider.cluster localnet` in CI.
  - Pin `packageManager` and add `.yarnrc.yml`, or regenerate a v1 lockfile.

### TOOL-7 · Medium · Confirmed · tooling — Root `package.json` scripts point at things that don't exist
- **Where:** `package.json:9-15,22-28`
- **Problems:**
  - `test:rust*`, `test:quick`, `coverage` and `test:all` use `-p tests-rust`, but there is no `tests-rust` crate or workspace member.
  - `test:fuzz*` use `-p trident-tests`, which is not a workspace member.
- **Fails when:** a contributor follows `CONTRIBUTING.md` or runs `yarn test:all`. It fails before the Anchor tests start.
- **Fix:** point the scripts at `cargo test --workspace --lib` and real crates, or delete them.

### TOOL-8 · Medium · Confirmed · test-gap — 14 unit tests fail; 13 of them are stale size assertions
- **Where:**
  - `programs/gasless-protocol/src/tests.rs:62,74`
  - `programs/governance-protocol/src/tests.rs:71`
  - `programs/identity-protocol/src/tests.rs:41`
  - `programs/sscre-protocol/src/tests.rs:116,128,261-271`
  - `programs/transfer-hook/src/tests.rs:60,72`
  - `programs/vcoin-token/src/tests.rs:69`
  - `programs/vevcoin-token/src/tests.rs:78`
  - `programs/vilink-protocol/src/tests.rs:55,61,67`
- **Code:** e.g. `let expected = 8 + 32 + 32 + 32 + 32 + 32 + 8 + 8 + 8 + 2 + 4 + 8 + 8 + 8 + 1 + 4 + 8 + 1; assert_eq!(GaslessConfig::LEN, expected` gives 236 vs the real 270.
- **Fails when:** every run. The LEN constants are correct. For example, `GaslessConfig::LEN` = 270 matches the struct at `programs/gasless-protocol/src/state/gasless_config.rs:47-67`. The tests predate the `pending_authority*` and `max_slippage_bps` fields.
  - `test_user_claim_large_epoch` expects "epoch ≤ last_claimed counts as claimed", but the bitmap replaced that rule.
- **Consequence:** a permanently red suite hides real regressions; a genuinely wrong LEN would look like more of the same noise.
- **Fix:** derive `InitSpace` and assert `LEN == 8 + T::INIT_SPACE`, or compute expected sizes from `borsh::to_vec(&T::default())`. Update the epoch test to the bitmap semantics.

### TOOL-9 · Medium · Confirmed · tooling — Program keypairs are not in the repo, so a fresh build deploys to the wrong addresses
- **Where:** `.gitignore:7` (`target/`). `git ls-files | grep keypair` finds 0 files.
- **Code:** a fresh `anchor build` generated:
  - `target/deploy/staking_protocol-keypair.json` → `2trLGZzLRrWQP6qYLzB3nGj1suhckkNY45MZXcfV62xu`, but `declare_id!("6EFcistyr2E81adLUcuBJRr8W2xzpt3D3dFYEcMewpWu")`.
  - The same mismatch for `governance_protocol` (`EdreEV…` vs `3R25…`) and `vcoin_token` (`5KNrP…` vs `Gg1d…`).
- **Fails when:** anyone other than the original deployer runs `anchor deploy` or `anchor test`. The programs deploy at new addresses and every instruction fails `DeclaredProgramIdMismatch`.
- **Fix:** document `anchor keys sync` for local and test runs. Keep the real upgrade-authority keys out of git, and record the deploy procedure.

### TOOL-10 · High · Confirmed · tooling — The published `@viwoapp/sdk@2.0.1` has broken ESM and subpath exports
- **Where:** `packages/viwoapp-sdk/package.json:6,45-79`
- **Code:** `"module": "dist/index.esm.js"`, `"import": "./dist/index.esm.js"`, `"./staking": { "import": "./dist/staking/index.esm.js", "require": "./dist/staking/index.js" … }`
- **Fails when:** any ESM consumer (Vite, Next.js app router, Node ESM) runs `import { ViWoClient } from "@viwoapp/sdk"`. It resolves to `dist/index.esm.js`, which does not exist in the published tarball; the tarball has only `dist/index.{js,mjs,d.ts,d.mts}`, verified with `npm pack @viwoapp/sdk@2.0.1`. All 6 subpath imports (`@viwoapp/sdk/staking`, etc.) fail for both CJS and ESM.
- **Fix:** `"module": "dist/index.mjs"`, `"import": "./dist/index.mjs"`. Either drop the subpath exports or build multiple tsup entries (`tsup src/index.ts src/staking/index.ts … --format cjs,esm --dts`).

### TOOL-11 · Medium · Confirmed · bug — Clippy deny-level error: an always-true scope check
- **Where:** `programs/gasless-protocol/src/instructions/session/create.rs:24`
- **Code:** `scope <= SCOPE_ALL` with `scope: u16` and `SCOPE_ALL = 0xFFFF`
- **Fails when:** always. The validation is a no-op, and clippy's `absurd_extreme_comparisons` lint is deny-by-default, so `cargo clippy` exits 101.
- **Fix:** validate that `scope & !VALID_SCOPE_MASK == 0`, where `VALID_SCOPE_MASK` is the OR of the defined SCOPE_* bits.

### TOOL-12 · Low · Confirmed · tooling — Smaller tooling problems
- **Formatting:** `cargo fmt --check` reports 2,413 diffs, which breaks CI.
- **Clippy warnings:** 109 unused imports, 36 ambiguous glob re-exports (`pub use x::*` re-exporting many `handler`s), 127 unexpected `cfg` warnings, and 4 `manual saturating arithmetic` hits. All 4 sit on the staking ve-delta lines that implement STK finding "top-up lowers ve without burn".
- **CI header:** `test.yml:13-14` advertises "Property Tests" and "Security Tests" jobs that don't exist. `security-audit` (`test.yml:226-235`) uses `continue-on-error`, and the last run's audit job reported exit 5 / exit 1 while the job still showed ✓.
- **SDK lint:** `lint` (`packages/viwoapp-sdk/package.json:14`) calls `eslint`, which is not installed and has no config.
- **README test counts** (`README.md:12,103-104`):
  - 279 matches the library unit tests, 14 of which fail.
  - `zk-voting-sdk` is a separate workspace, so `cargo test --workspace` doesn't run its tests.
  - The 118 bankrun tests never touch a program.
  - "includes ZK crypto tests" is false, because `programs/governance-protocol/src/crypto/` has no tests.
