<p align="center">
  <img src="https://salmon-calm-louse-860.mypinata.cloud/ipfs/bafkreifrxcxq54wr2se2gtzuhxedddaimijas2ca3ilrgxusnxvx4xvnd4" alt="ViWoApp Logo" width="120" height="120">
</p>

# ViWo Protocol Stack (VCoin)

Solana programs for staking, governance, private polls, rewards, reputation, identity, content
and social actions around the VCoin token, plus a TypeScript SDK and a Rust ZK SDK. MIT licensed.

> **Status:** v3 is the remediation of a 148-finding internal review. It is **not externally
> audited**: see [`AUDIT_STATUS.md`](AUDIT_STATUS.md) before relying on it.

## Programs

| Program | Id | Role |
|---|---|---|
| vcoin-token | `Gg1dtrjAfGYi6NLC31WaJjZNBoucvD98rK2h1u9qrUjn` | VCoin: Token-2022, 9 decimals, metadata only (no hook, delegate or freeze), 1B cap checked against real supply |
| vevcoin-token | `FB39ae9x53FxVL3pER9LqCPEx2TRnEnQP55i838Upnjx` | Non-transferable 1:1 receipt of staked VCoin, minted and burned only by staking |
| staking-protocol | `6EFcistyr2E81adLUcuBJRr8W2xzpt3D3dFYEcMewpWu` | Lockups of 1 week to 4 years with linear, decaying voting weight; Realms voter-weight plugin; DAO slashing of staked deposits |
| private-poll | `3R256kBN9iXozjypQFRAmegBhd6HJqXWqdNG7Th78HYe` | Secret-ballot polls (exponential ElGamal + CDS proofs, threshold decryption); `assert_poll_passed` gates Realms proposals |
| sscre-protocol | `6AJNcQSfoiE2UAeUDyJUBumS9SBwhAdSznoAeYpXrxXZ` | Per-epoch merkle reward distributor: declining emission from a reserve, oracle-quorum roots, clawback |
| gasless-protocol | `FcXJAjzJs8eVY2WTRFXynQBpC7WZUqKZppyp9xS6PaB3` | Session keys (≤ 24h, scoped) and user-capped VCoin fees for an off-chain relayer |
| five-a-protocol | `783PbtJw5cc7yatnr9fsvTGSnkKaV6iJe6E8VUPTYrT8` | 5A reputation scores applied by M-of-N oracle consensus; reputation-only vouching |
| identity-protocol | `3egAds3pFR5oog6iQCN42KPvgih8HQz2FGybNjiVWixG` | DID hash, unique username, attester-signed verification levels, USDC subscriptions |
| content-registry | `MJn1A4MPCBPJGWWuZrtq7bHSo2G289sUwW3ej2wcmLV` | Content proofs rate-limited by stake-tiered energy; engagement refunds |
| vilink-protocol | `CFGXTS2MueQwTYTMMTBQbRWzJtSTC2p4ZRuKPpLDmrv7` | Shareable tip, vouch and follow links |

Binding governance (proposals, thresholds, timelock, veto, delegation and treasury) runs on
**Realms (SPL Governance)**, with staking-protocol as the voter-weight plugin. Every program's
authority is handed to the DAO at launch (see the [deploy runbook](docs/offchain/deploy-runbook.md)).

## Repository

```
programs/               10 Anchor programs
crates/vcoin-common     shared authority timelock, upgrade-authority gate, mint validation
crates/zk-poll-core     ZK verifier shared by private-poll and the ZK SDK (SBF, host, wasm)
packages/zk-voting-sdk  Rust: Pedersen DKG, ballot encryption, decryption, tally (+ wasm)
packages/viwoapp-sdk    TypeScript SDK generated from the IDLs
integration-tests/      LiteSVM transaction tests against the built binaries
docs/                   architecture, per-program docs, generated API, off-chain components, audit
```

## Build and test

Toolchain: Rust 1.89 (`rust-toolchain.toml`), Anchor CLI 0.32, Agave 3.0, Node 22+.

```bash
anchor build                                   # programs + IDLs (must show no "Stack offset" warning)
cargo test --workspace                         # unit + LiteSVM integration tests
cd packages/viwoapp-sdk && npm ci && npm test  # SDK, including end-to-end runs on the binaries
```

CI (`.github/workflows/test.yml`) runs all of the above, plus fmt, `clippy -D warnings`, the wasm
build, IDL and API-doc drift checks, `cargo audit`, and the SDK lint, build and packaging smoke test.

## Documentation

- [Architecture](docs/ARCHITECTURE.md): components, flows, trust model.
- [Programs](docs/programs/): one page per program covering invariants and admin powers.
- [API reference](docs/API.md): instructions, accounts, events and errors, generated from the IDLs.
- [Integration](docs/INTEGRATION.md) and the [SDK README](packages/viwoapp-sdk/README.md).
- Off-chain components: [relayer](docs/offchain/relayer-kora.md),
  [activity indexer](docs/offchain/activity-indexer.md),
  [merkle format](docs/offchain/merkle-leaf-format.md),
  [deploy runbook](docs/offchain/deploy-runbook.md).
- [Security policy](SECURITY.md), [audit status](AUDIT_STATUS.md), [fix tracker](FIX-TRACKER.md),
  [changelog](CHANGELOG.md).

## License

MIT, see [LICENSE](LICENSE).
