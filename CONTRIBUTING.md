# Contributing

## Setup

- Rust 1.89 (pinned by `rust-toolchain.toml`, with the `wasm32-unknown-unknown` target for the ZK
  SDK's wasm build).
- Anchor CLI 0.32 (`avm install 0.32.0`).
- Agave (Solana) CLI 3.0.
- Node 22+.

## Before every commit

```bash
cargo fmt --all
cargo clippy --workspace --all-targets -- -D warnings
anchor build                      # must not print "Stack offset"
cargo test --workspace            # LiteSVM tests load target/deploy/*.so: rebuild after program changes
node scripts/sync-idl.mjs && node scripts/gen-api-docs.mjs   # when an IDL changed
(cd packages/viwoapp-sdk && npm run typecheck && npm run lint && npm test)
```

CI runs the same commands and fails on any difference, including IDL or `docs/API.md` drift.

## Conventions

- **Admin and init.** Use `vcoin-common`:
  - `AdminState` for the 24h two-step authority;
  - `require_upgrade_authority` with a `Program` + `ProgramData` constraint in every `initialize`;
  - `validate_vcoin_mint` wherever a VCoin mint is accepted.
- **Stack.** Box every `Account` / `InterfaceAccount` in contexts with more than a few accounts.
  SBF frames are 4 KiB.
- **Foreign accounts.** Read them as typed accounts with `seeds::program`, or check owner, PDA
  and type tag explicitly.
- **Fees and other admin-settable money parameters.** Bound them on-chain, timelock changes, and
  snapshot them where users act on a quoted value.
- **Pause.** Pause covers mutating user instructions, never withdrawals.
- **Tests.**
  - Every instruction path, including its failure modes, gets a LiteSVM test in
    `integration-tests/tests/`, using the Anchor-generated `accounts::*` / `instruction::*` types.
  - Bug fixes start with a failing test.
  - Changes to a finding from the audit update `FIX-TRACKER.md`.
- **ZK changes.** Any change to a transcript or proof format changes `crates/zk-poll-core` and the
  SDK together. The round-trip tests in `packages/zk-voting-sdk/tests` must stay green.

## Commits

One logical change per commit, with a message that says what changed and why. Never commit keys.
Program keypairs stay out of the repo; see the deploy runbook.
