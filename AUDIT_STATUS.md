# Audit status

**This code has not had an external audit. Do not deploy it to mainnet with real value until one is
done.**

## History

- **v2 (up to February 2026):** three internal documents claimed 100%, 96% and 78% "audit
  completion" at the same time. An end-to-end internal review on 2026-11-13 found otherwise:
  - private votes could never verify;
  - one oracle could drain rewards;
  - governance could be captured with about 100 VCoin;
  - no test executed any instruction.

  Those documents have been removed. The review is kept in
  [`docs/audit/2026-11/`](docs/audit/2026-11/README.md): 148 verified findings (2 Critical,
  32 High, 63 Medium, 51 Low), with proof-of-concept tests.
- **v3 (branch `audit-remediation`):** addresses every finding. [`FIX-TRACKER.md`](FIX-TRACKER.md)
  maps each finding ID to the commit that fixed or removed it and to the test that proves it. The
  design changes, and the reasons for them, are in [`CHANGELOG.md`](CHANGELOG.md).

## What v3 verifies

- **Rust tests:** `cargo test --workspace` runs unit tests and LiteSVM transaction tests.
  - The tests execute the real program binaries.
  - Realms tests run against the deployed mainnet SPL Governance binary.
  - The ZK prover and verifier are round-trip tested.
- **TypeScript SDK tests:** they execute SDK-built transactions on the program binaries.
- **CI:** fails on formatting, clippy warnings, SBF stack-frame overruns, IDL drift, RustSec
  advisories, and SDK typecheck, lint, test or packaging errors.

## What remains before mainnet

- **External audit:** an independent audit of the v3 programs, `crates/zk-poll-core` and
  `packages/zk-voting-sdk`.
- **Off-chain components:** these hold trust the programs cannot remove, and need their own
  review:
  - 5A oracles;
  - SSCRE oracles;
  - the engagement oracle;
  - identity attesters;
  - the decryption committee's DKG ceremony.
- **Deployment:** execute the [deploy runbook](docs/offchain/deploy-runbook.md), including
  handing every program authority to the Realms DAO.
