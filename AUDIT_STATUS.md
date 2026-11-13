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
- **Re-review of v3 (2026-11-13):** because v3 is a rewrite, it was reviewed again from scratch
  by three independent passes. They found 22 new issues in the rewritten code: no Critical,
  2 High (a rewards-root veto that could never land in time, and a rogue-key attack on the
  committee's key generation), 6 Medium and 14 Low. All are fixed and tested; they are listed
  at the end of `FIX-TRACKER.md`.
- **Confirmation review (2026-11-13):** the commit that fixed those 22 issues was itself
  reviewed by two more independent passes. They found 10 further issues in the new code
  (1 High in a new key-generation helper, 2 Medium, 7 Low), also fixed and tested. Each round
  of review has found less, but none has found nothing. That is exactly why the external
  audit below is not optional.

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
  - the decryption committee's DKG ceremony (the programs cannot see it: its checks, including
    the proof that rules out a rogue election key, run in `zk-voting-sdk`);
  - the guardian key of each program (it can pause and veto a rewards root at once).
- **Deployment:** execute the [deploy runbook](docs/offchain/deploy-runbook.md), including
  handing every program authority to the Realms DAO.
