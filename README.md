<p align="center">
  <img src="https://salmon-calm-louse-860.mypinata.cloud/ipfs/bafkreifrxcxq54wr2se2gtzuhxedddaimijas2ca3ilrgxusnxvx4xvnd4" alt="ViWoApp Logo" width="120" height="120">
</p>

# ViWo Protocol Stack

**Trust & Reputation Protocols for Consumer Crypto**

Powered by Solana | MIT Open Source | Contracts on Devnet

---

## Abstract

ViWoApp introduces a Solana-native protocol stack for trust, reputation, and sustainable value distribution in consumer crypto applications. The system addresses fundamental ecosystem problems that have blocked mainstream adoption: bot-infested engagement, weak on-chain identity, unsustainable token emissions, and prohibitive transaction costs.

The protocol stack includes four core innovations:

1. **The 5A Reputation Protocol** — Open-source anti-Sybil infrastructure that scores users across five dimensions, making bot farming economically irrational
2. **The SSCRE Protocol** — A Self-Sustaining Circular Reward Economy that solves the token death spiral through a 6-layer funding hierarchy
3. **Portable Decentralized Identity** — On-chain DIDs with reputation scores that travel across the Solana ecosystem
4. **Gasless UX Layer** — Account abstraction and session keys enabling mainstream-friendly interactions

Every protocol is **MIT licensed** and available as a **public good** for the entire Solana ecosystem.

---

## The Problems We Solve

### The Trust Problem
Consumer crypto is overrun by bots and fake engagement. LayerZero's 2024 Sybil Hunt identified over 1.2 million wallets as Sybil addresses. No reusable anti-bot infrastructure exists — every project builds from scratch.

### The Sustainability Problem
Previous token-incentivized platforms all faced the same death spiral. STEEM fell ~97% from ATH. Friend.tech's FRIEND dropped 90%+ in 2024. No proven model exists for perpetual, sustainable rewards.

### The UX Problem
Web3 remains inaccessible. Gas fees break social interactions. Crypto expertise is required. Transaction delays destroy real-time experiences.

---

## Protocol Architecture

| Layer | Components |
|-------|------------|
| **Application** | Reference Implementation (ViWoApp) |
| **Core Protocols** | 5A Protocol • SSCRE • Identity • Governance |
| **Infrastructure** | Staking • Transfer Hook • Gasless UX • ViLink |
| **Blockchain** | Solana — $0.00025/tx • 400ms blocks • 4,000+ TPS |

---

## Smart Contracts

This workspace contains **11 Solana programs** — all MIT licensed as ecosystem infrastructure:

### Core Protocols

| Program | Purpose | License |
|---------|---------|---------|
| **five-a-protocol** | Anti-Sybil reputation scoring across 5 dimensions | MIT |
| **sscre-protocol** | Self-Sustaining Circular Reward Economy with Merkle claims | MIT |
| **identity-protocol** | Portable DID with verification levels | MIT |
| **governance-protocol** | veVCoin voting with quadratic power and 5A boosts | MIT |

### Infrastructure Layer

| Program | Purpose | License |
|---------|---------|---------|
| **vcoin-token** | Token-2022 with Permanent Delegate & Metadata | MIT |
| **vevcoin-token** | Soulbound governance token (non-transferable) | MIT |
| **staking-protocol** | Lock VCoin → Earn veVCoin with tier-based rewards | MIT |
| **transfer-hook** | Auto-updates 5A scores, detects wash trading | MIT |
| **gasless-protocol** | Paymaster & Session Keys for zero-friction UX | MIT |
| **content-registry** | On-chain content tracking with energy system | MIT |
| **vilink-protocol** | Cross-dApp action deep links | MIT |

---

## The 5A Reputation Protocol

Open-source anti-Sybil infrastructure that makes bot farming economically irrational.

### The Five Dimensions

| Star | Name | What It Measures |
|------|------|------------------|
| A | **Authenticity** | KYC completion, profile verification, account age |
| A | **Accuracy** | Content quality, factual accuracy, community feedback |
| A | **Agility** | Response time, engagement speed, adaptability |
| A | **Activity** | Daily actions, posting frequency, consistency |
| A | **Approved** | Community standing, trust level, reputation history |

### Integration for Ecosystem

Other Solana applications can query 5A scores to:
- Weight rewards by reputation
- Gate access to features
- Detect and filter Sybil attacks
- Build trust graphs

---

## SSCRE Protocol

The **Self-Sustaining Circular Reward Economy** ensures rewards never run out:

| Phase | Years | Mechanism |
|-------|-------|-----------|
| **Emission** | 1-5 | 350M reward pool distributes ~5.83M/month |
| **Reserve** | 6-10 | ~84M saved reserves, zero new tokens |
| **Perpetual** | 11+ | Scheduled 250M minting every 5 years |

### 6-Layer Funding Hierarchy

| Layer | Source |
|-------|--------|
| L0 | Unused Allocation |
| L1 | Reserve Fund |
| L2 | Scheduled Minting |
| L3 | Buyback Recycling |
| L4 | Profit Buybacks |
| L5 | Fee Recycling |

This is the first proven model for perpetual token rewards without infinite inflation.

---

## VCoin Token

| Parameter | Value |
|-----------|-------|
| **Token** | VCoin (VIWO) |
| **Total Supply** | 1,000,000,000 (1B) |
| **Decimals** | 9 |
| **Standard** | Token-2022 with Extensions |
| **Network** | Solana |

### Token-2022 Extensions
- **Permanent Delegate** — Enables slashing bad actors without user signature
- **Metadata Extension** — On-chain metadata without Metaplex
- **Non-Transferable** (veVCoin) — True soulbound tokens

---

## Governance: veVCoin

Vote-Escrowed VCoin provides governance power with anti-whale mechanics:

### Voting Power Formula
```
base_votes = sqrt(vcoin_tokens)              // Quadratic (diminishing returns)
five_a_boost = 1.0 + (five_a_score / 100)   // 1.0x to 2.0x
tier_multiplier = [1.0, 1.0, 2.0, 5.0, 10.0] // None to Platinum
effective_votes = base_votes × five_a_boost × tier_multiplier
```

### Staking Tiers

| Tier | Minimum Stake | Fee Discount | veVCoin Boost |
|------|---------------|--------------|---------------|
| None | 0 | 0% | 1.0x |
| Bronze | 1,000 | 10% | 1.1x |
| Silver | 5,000 | 20% | 1.2x |
| Gold | 20,000 | 30% | 1.3x |
| Platinum | 100,000 | 50% | 1.4x |

---

## Solana Foundation Alignment

The ViWo Protocol Stack directly addresses the Solana Foundation's 2026 priorities:

| Foundation Priority | ViWo Protocol Implementation |
|---------------------|------------------------------|
| **Consumer Applications** | Reference implementation proving protocols work |
| **Identity and Social Proof** | Portable DID, 5A reputation system |
| **Safety Infrastructure** | Anti-Sybil scoring as ecosystem public good |
| **Open Source** | All protocols MIT licensed |

---

## Devnet Deployment

✅ **All 11 programs deployed to Solana Devnet**

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

## Development

### Prerequisites

```bash
# Install Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Install Solana CLI (v3.0+)
curl --proto '=https' --tlsv1.2 -sSfL https://solana-install.solana.workers.dev | bash

# Install Anchor CLI (v0.32.0)
cargo install --git https://github.com/coral-xyz/anchor avm --locked
avm install 0.32.0
avm use 0.32.0
```

### Build & Test

```bash
# Build all programs
anchor build

# Run Rust unit tests (279 tests)
cargo test --workspace

# Run BankRun integration tests
cd tests-bankrun && npm test

# Deploy to devnet
solana config set --url devnet
anchor deploy
```

### Test Coverage

| Test Layer | Count | Status |
|------------|-------|--------|
| Rust Unit Tests | 279 | ✅ Passing |
| Rust Integration | 55 files | ✅ Created |
| BankRun Tests | 98 | ✅ Passing |
| TypeScript E2E | 11 files | ✅ Ready |
| **Total** | **377+** | **✅ All Passing** |

---

## Project Structure

```
vcoin_workspace/
├── programs/
│   ├── five-a-protocol/      # Anti-Sybil reputation (Full Modular)
│   ├── sscre-protocol/       # Sustainable rewards (Streamlined)
│   ├── identity-protocol/    # Portable DID (Full Modular)
│   ├── governance-protocol/  # veVCoin voting (Streamlined)
│   ├── vcoin-token/          # Token-2022 (Full Modular)
│   ├── vevcoin-token/        # Soulbound token (Full Modular)
│   ├── staking-protocol/     # Tier-based staking (Full Modular)
│   ├── transfer-hook/        # Transfer automation (Full Modular)
│   ├── gasless-protocol/     # Session keys (Full Modular)
│   ├── content-registry/     # Content management (Full Modular)
│   └── vilink-protocol/      # Deep links (Streamlined)
├── tests/                    # TypeScript E2E tests
├── tests-bankrun/            # BankRun integration tests
├── packages/
│   └── viwoapp-sdk/          # TypeScript SDK
└── target/deploy/            # Compiled .so programs
```

---

## Security

- **Authority checks** on all admin functions
- **PDA seeds** prevent account collisions  
- **Lock duration validation** prevents gaming
- **Permanent delegate** enables slashing without user signature
- **Pausable** for emergency situations
- **Checked arithmetic** using `checked_add`, `checked_sub`
- **Modular architecture** for easier security audits
- **377+ tests** across multiple testing layers

---

## Public Good Commitment

All protocols are **MIT licensed** and publicly available:

| Protocol | Purpose | Status |
|----------|---------|--------|
| 5A Protocol | Anti-Sybil reputation scoring | ✅ Deployed |
| SSCRE Protocol | Sustainable reward economics | ✅ Deployed |
| Identity Protocol | Portable DID and verification | ✅ Deployed |
| Governance Protocol | veVCoin voting system | ✅ Deployed |
| Staking Protocol | Lock and earn mechanics | ✅ Deployed |
| Gasless UX | Session keys and paymaster | ✅ Deployed |

This infrastructure is designed for **ecosystem adoption**, not platform lock-in.

---

## Links

| Resource | URL |
|----------|-----|
| Whitepaper | [viwoapp.com/whitepaper](https://viwoapp.com/whitepaper) |
| Website | [viwoapp.com](https://viwoapp.com) |
| X/Twitter | [@ViWoApp](https://x.com/ViWoApp) |
| Telegram | [@ViWoApp](https://t.me/ViWoApp) |
| Discord | [discord.gg/viwoapp](https://discord.gg/viwoapp) |
| GitHub | [github.com/MohaMehrzad/VCoin-V2](https://github.com/MohaMehrzad/VCoin-V2) |

---

## License

**MIT License** — All protocols are open source and available as public goods for the Solana ecosystem.

---

**Version:** 2.1  
**Framework:** Anchor 0.32.0 | Solana Program 2.0 | Token-2022 6.0  
**Architecture:** Modular (2025-2026 Best Practices)  
**Network:** Solana Devnet (11/11 Deployed)  
**Token:** VCoin (VIWO) • Launch: Q1 2026
