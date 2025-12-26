<p align="center">
  <img src="https://salmon-calm-louse-860.mypinata.cloud/ipfs/bafkreifrxcxq54wr2se2gtzuhxedddaimijas2ca3ilrgxusnxvx4xvnd4" alt="ViWoApp Logo" width="120" height="120">
</p>

# ViWo Protocol Stack

**Trust & Reputation Protocols for Consumer Crypto**

[![npm version](https://img.shields.io/npm/v/@viwoapp/sdk.svg)](https://www.npmjs.com/package/@viwoapp/sdk)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](LICENSE)
[![Solana](https://img.shields.io/badge/Solana-Devnet-9945FF.svg)](https://solana.com)
[![Tests](https://img.shields.io/badge/tests-377%20passing-brightgreen.svg)](https://github.com/MohaMehrzad/VCoin-V2)

Powered by Solana | MIT Open Source | All 11 Contracts on Devnet

---

## Overview

ViWoApp introduces a Solana-native protocol stack for trust, reputation, and sustainable value distribution in consumer crypto applications. The system addresses fundamental ecosystem problems: bot-infested engagement, weak on-chain identity, unsustainable token emissions, and prohibitive transaction costs.

**Core Innovations:**

1. **5A Reputation Protocol** — Anti-Sybil infrastructure scoring users across five dimensions
2. **SSCRE Protocol** — Self-Sustaining Circular Reward Economy with 6-layer funding
3. **Portable DID** — On-chain identity with reputation that travels across Solana
4. **Gasless UX** — Account abstraction and session keys for mainstream-friendly interactions

All protocols are **MIT licensed** as a **public good** for the Solana ecosystem.

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
cargo test --workspace          # Run 279 Rust tests
cd tests-bankrun && npm test    # Run 98 BankRun tests
```

### SDK Installation

```bash
npm install @viwoapp/sdk
```

```typescript
import { ViWoClient, parseVCoin } from "@viwoapp/sdk";

const client = new ViWoClient({ connection, wallet });
const score = await client.fivea.getScore(wallet);
console.log("5A Score:", score.composite / 100, "%");
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
| **five-a-protocol** | Anti-Sybil reputation scoring across 5 dimensions |
| **sscre-protocol** | Self-Sustaining Circular Reward Economy |
| **identity-protocol** | Portable DID with verification levels |
| **governance-protocol** | veVCoin voting with quadratic power + 5A boost |

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

All security issues have been addressed in v2.8.4:

| Severity | Fixed | Status |
|----------|-------|--------|
| Critical | 6/6 | ✅ |
| High | 10/10 | ✅ |
| Medium | 9/9 | ✅ |
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

**Version:** 2.8.4 | **Framework:** Anchor 0.32.0 | **Network:** Solana Devnet | **SDK:** @viwoapp/sdk v0.1.8
