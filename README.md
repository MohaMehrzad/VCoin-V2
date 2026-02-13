<p align="center">
  <img src="https://salmon-calm-louse-860.mypinata.cloud/ipfs/bafkreifrxcxq54wr2se2gtzuhxedddaimijas2ca3ilrgxusnxvx4xvnd4" alt="ViWoApp Logo" width="120" height="120">
</p>

# ViWo Protocol Stack

**Trust & Reputation Protocols for Consumer Crypto**

[![npm version](https://img.shields.io/npm/v/@viwoapp/sdk.svg)](https://www.npmjs.com/package/@viwoapp/sdk)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](LICENSE)
[![Solana](https://img.shields.io/badge/Solana-Devnet-9945FF.svg)](https://solana.com)
[![Tests](https://img.shields.io/badge/tests-397%20passing-brightgreen.svg)](https://github.com/MohaMehrzad/VCoin-V2)

Powered by Solana | MIT Open Source | All 11 Contracts on Devnet

---

## Overview

ViWoApp introduces a Solana-native protocol stack for trust, reputation, and sustainable value distribution in consumer crypto applications. The system addresses fundamental ecosystem problems: bot-infested engagement, weak on-chain identity, unsustainable token emissions, and prohibitive transaction costs.

**Core Innovations:**

1. **ZK Private Voting** — Production-ready encrypted governance with twisted ElGamal on Ristretto255
2. **5A Reputation Protocol** — Anti-Sybil infrastructure scoring users across five dimensions
3. **SSCRE Protocol** — Self-Sustaining Circular Reward Economy with 6-layer funding
4. **Portable DID** — On-chain identity with reputation that travels across Solana
5. **Gasless UX** — Account abstraction and session keys for mainstream-friendly interactions

All protocols are **MIT licensed** as a **public good** for the Solana ecosystem.

### Latest Updates (v2.0.1)

**100% Security Audit Complete** — All 68 findings from comprehensive deep audit resolved:
- 12 Critical severity fixes (100%)
- 8 High severity fixes (100%) including new delegation balance validation
- 40 Medium severity fixes (100%) including voting power precision and tier spam prevention
- 8 Low severity fixes (100%)

**H-NEW-03: Delegation Balance Validation** — New in v2.0.1:
- Validates delegation amount against actual veVCoin balance on-chain
- Cross-program PDA verification from staking protocol
- Prevents users from claiming more voting power than they possess

**SDK v2.0.1 Released** — Adds delegation transaction builders, PDA helpers, and new types.

### v2.0.0 Highlights

**Production ZK Private Voting** — Full encrypted governance implementation:
- Twisted ElGamal encryption with compressed sigma proofs
- On-chain vote verification without revealing vote content
- Homomorphic tallying for private vote accumulation
- Threshold decryption with DLEQ proof verification
- Trustless tally verification using cryptographic proofs

---

## 📚 Documentation

| Document | Description |
|----------|-------------|
| [**SECURITY.md**](SECURITY.md) | Security policy, vulnerability reporting, audit status |
| [**CONTRIBUTING.md**](CONTRIBUTING.md) | Contribution guidelines, code style, PR process |
| [**CHANGELOG.md**](CHANGELOG.md) | Version history and release notes |
| [**Architecture**](docs/ARCHITECTURE.md) | System design, protocol interactions, PDA structure |
| [**Integration Guide**](docs/INTEGRATION.md) | Quick start, SDK usage, CPI examples |
| [**API Reference**](docs/API.md) | Program instructions, accounts, error codes |

### Per-Program Documentation

| Program | Documentation |
|---------|---------------|
| vcoin-token | [docs/programs/vcoin-token.md](docs/programs/vcoin-token.md) |
| vevcoin-token | [docs/programs/vevcoin-token.md](docs/programs/vevcoin-token.md) |
| staking-protocol | [docs/programs/staking-protocol.md](docs/programs/staking-protocol.md) |
| five-a-protocol | [docs/programs/five-a-protocol.md](docs/programs/five-a-protocol.md) |
| governance-protocol | [docs/programs/governance-protocol.md](docs/programs/governance-protocol.md) |
| sscre-protocol | [docs/programs/sscre-protocol.md](docs/programs/sscre-protocol.md) |
| identity-protocol | [docs/programs/identity-protocol.md](docs/programs/identity-protocol.md) |
| content-registry | [docs/programs/content-registry.md](docs/programs/content-registry.md) |
| transfer-hook | [docs/programs/transfer-hook.md](docs/programs/transfer-hook.md) |
| vilink-protocol | [docs/programs/vilink-protocol.md](docs/programs/vilink-protocol.md) |
| gasless-protocol | [docs/programs/gasless-protocol.md](docs/programs/gasless-protocol.md) |

---

## Quick Start

### Prerequisites

```bash
# Rust, Solana CLI (2.0+), Anchor (0.32.0)
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
curl --proto '=https' --tlsv1.2 -sSfL https://solana-install.solana.workers.dev | bash
cargo install --git https://github.com/coral-xyz/anchor avm --locked
avm install 0.32.0 && avm use 0.32.0
```

### Build & Test

```bash
anchor build                    # Build all 11 programs
cargo test --workspace          # Run 279 Rust tests (includes ZK crypto tests)
cd tests-bankrun && npm test    # Run 118 BankRun tests (includes ZK voting)
```

### SDK Installation

```bash
npm install @viwoapp/sdk@2.0.1
```

```typescript
import { ViWoClient, parseVCoin, VoteChoice } from "@viwoapp/sdk";

const client = new ViWoClient({ connection, wallet });

// Get 5A reputation score
const score = await client.fivea.getScore(wallet);
console.log("5A Score:", score.composite / 100, "%");

// Cast private vote with ZK encryption
const voteTx = await client.governance.buildCastPrivateVoteTransaction({
  proposalId: new BN(1),
  voteChoice: VoteChoice.For,
  votingPower: new BN(1000),
});
```

---

## Devnet Deployment

All 11 programs deployed to Solana Devnet:

| Program | Address |
|---------|---------|
| vcoin-token | `Gg1dtrjAfGYi6NLC31WaJjZNBoucvD98rK2h1u9qrUjn` |
| vevcoin-token | `FB39ae9x53FxVL3pER9LqCPEx2TRnEnQP55i838Upnjx` |
| staking-protocol | `6EFcistyr2E81adLUcuBJRr8W2xzpt3D3dFYEcMewpWu` |
| transfer-hook | `9K14FcDRrBeHKD9FPNYeVJaEqJQTac2xspJyb1mM6m48` |
| identity-protocol | `3egAds3pFR5oog6iQCN42KPvgih8HQz2FGybNjiVWixG` |
| five-a-protocol | `783PbtJw5cc7yatnr9fsvTGSnkKaV6iJe6E8VUPTYrT8` |
| content-registry | `MJn1A4MPCBPJGWWuZrtq7bHSo2G289sUwW3ej2wcmLV` |
| governance-protocol | `3R256kBN9iXozjypQFRAmegBhd6HJqXWqdNG7Th78HYe` |
| sscre-protocol | `6AJNcQSfoiE2UAeUDyJUBumS9SBwhAdSznoAeYpXrxXZ` |
| vilink-protocol | `CFGXTS2MueQwTYTMMTBQbRWzJtSTC2p4ZRuKPpLDmrv7` |
| gasless-protocol | `FcXJAjzJs8eVY2WTRFXynQBpC7WZUqKZppyp9xS6PaB3` |

---

## Smart Contracts

### Core Protocols

| Program | Purpose |
|---------|---------|
| **governance-protocol** | ZK private voting with twisted ElGamal + quadratic power + 5A boost |
| **five-a-protocol** | Anti-Sybil reputation scoring across 5 dimensions |
| **sscre-protocol** | Self-Sustaining Circular Reward Economy |
| **identity-protocol** | Portable DID with verification levels |

### Infrastructure

| Program | Purpose |
|---------|---------|
| **vcoin-token** | Token-2022 with Permanent Delegate & Transfer Hook |
| **vevcoin-token** | Soulbound governance token (non-transferable) |
| **staking-protocol** | Lock VCoin → Earn veVCoin with tier rewards |
| **transfer-hook** | Auto-updates 5A scores, detects wash trading |
| **gasless-protocol** | Paymaster & Session Keys for zero-friction UX |
| **content-registry** | On-chain content tracking with energy system |
| **vilink-protocol** | Cross-dApp action deep links |

---

## Security

All 68 security findings have been resolved (v2.0.1):

| Severity | Fixed | Status |
|----------|-------|--------|
| Critical | 12/12 | ✅ |
| High | 8/8 | ✅ |
| Medium | 40/40 | ✅ |
| Low | 8/8 | ✅ |

See [SECURITY.md](SECURITY.md) for vulnerability reporting and security features.

---

## Links

### Documentation

| | |
|--|--|
| [Whitepaper](https://viwoapp.com/whitepaper) | [Pitch Deck](https://viwoapp.com/pitch) |
| [Token Economy](https://viwoapp.com/docs/economy) | [Roadmap](https://viwoapp.com/docs/roadmap) |

### Resources

| Resource | URL |
|----------|-----|
| npm SDK | [@viwoapp/sdk](https://www.npmjs.com/package/@viwoapp/sdk) |
| Website | [viwoapp.com](https://viwoapp.com) |
| GitHub | [github.com/MohaMehrzad/VCoin-V2](https://github.com/MohaMehrzad/VCoin-V2) |

### Community

| Platform | URL |
|----------|-----|
| X/Twitter | [@ViWoApp](https://x.com/ViWoApp) |
| Telegram | [@ViWoApp](https://t.me/ViWoApp) |
| Discord | [discord.gg/viwoapp](https://discord.gg/viwoapp) |

---

## License

**MIT License** — All protocols are open source and available as public goods.

See [LICENSE](LICENSE) for details.

---

**Version:** 2.0.1 | **Framework:** Anchor 0.32.0 | **Network:** Solana Devnet | **SDK:** @viwoapp/sdk v2.0.1

---

## What's New in v2.0.0

### ZK Private Voting

Production-ready encrypted governance with full cryptographic verification:

- **Twisted ElGamal Encryption**: Vote privacy on Ristretto255 curve
- **Compressed Sigma Proofs**: On-chain vote verification (352 bytes)
- **Homomorphic Tallying**: Encrypted vote accumulation
- **Threshold Decryption**: Committee-based trustless reveal with DLEQ proofs
- **Verifiable Results**: Anyone can verify tally correctness

**Off-chain SDK**: `packages/zk-voting-sdk` for vote encryption/decryption operations

### Security Audit Remediation

All 33 findings from comprehensive security audit have been fixed:

**Critical (6/6 Fixed):**
- C-03: Abstains excluded from quorum calculation
- C-05/C-06: Ceiling division for all fee calculations
- C-08: Mutual vouch prevention in 5A protocol
- C-AUDIT-10: Monotonic engagement enforcement
- C-AUDIT-17: SAS attestation required for upgrades
- C-AUDIT-22: USDC payment enforcement

**High (10/10 Fixed):**
- H-02: Two-step authority transfer with 24h timelock (all protocols)
- H-AUDIT-12: Per-user session limit (MAX=5)
- H-AUDIT-13: Verification required before DID changes
- H-NEW-02: Merkle proof size limited to 32 levels
- + 6 additional high severity fixes

**Medium (9/9 Fixed):**
- M-02: Platform fee bounds validation (0.1%-10%)
- M-04: Nonce-based deterministic PDA derivation
- M-05: TierUnchanged error for no-op updates
- M-18: Vouch expiry (MAX_AGE=1 year)
- + 5 additional medium severity fixes

**Low (8/8 Fixed):**
- Enhanced error handling, edge case validation, and security checks

### SDK v2.0.0

Major TypeScript SDK release with breaking changes:

**New Features:**
- Full ZK voting API (`buildCastPrivateVoteTransaction`, `buildSubmitDecryptionShareTransaction`)
- `VoteChoice` enum (Against, For, Abstain) replaces boolean votes
- Enhanced type safety with security-related state fields
- New security constants and validation

**Breaking Changes:**
- Updated state account structures (all configs include `pending_authority`)
- New instruction parameters for security validation
- Updated PDA derivation for deterministic nonces

See [packages/viwoapp-sdk/README.md](packages/viwoapp-sdk/README.md) for full migration guide.
