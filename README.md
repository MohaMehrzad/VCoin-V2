<p align="center">
  <img src="https://salmon-calm-louse-860.mypinata.cloud/ipfs/bafkreifrxcxq54wr2se2gtzuhxedddaimijas2ca3ilrgxusnxvx4xvnd4" alt="ViWoApp Logo" width="120" height="120">
</p>

# ViWoApp Smart Contracts

**Censorship Proof Social Media for Crypto Community**

Built on Solana | 100% Creator Earnings | MIT Open Source | Modular Architecture

---

## Abstract

ViWoApp is an open-source SocialFi platform built natively on Solana, designed to fundamentally reshape how crypto users interact, earn, and build reputation in the digital economy. Unlike traditional social platforms that extract value from users, ViWoApp returns ownership to both creators and consumers — monetizing every engagement action and rewarding genuine participation.

The platform introduces three core innovations:

1. **The 5A Reputation Protocol** — A transparent, gamified system that rewards genuine engagement and penalizes bots and spam
2. **The SSCRE Protocol** — A Self-Sustaining Circular Reward Economy that ensures rewards never run out
3. **Portable Decentralized Identity** — Your reputation travels with you across the Solana ecosystem

Every protocol is MIT licensed and available as a public good for the entire Solana ecosystem.

---

## What's New in v2.0

### Modular Architecture (December 2025)

All 11 programs have been restructured into a **modular architecture** following 2025-2026 Solana best practices:

| Feature | Benefit |
|---------|---------|
| **Separated Concerns** | Constants, errors, events, state, contexts, instructions in dedicated files |
| **Enhanced Auditability** | Auditors can review isolated components |
| **Better Maintainability** | Changes isolated to specific files |
| **Improved Testing** | Unit test individual modules |
| **Clear Organization** | ~196 files organized by purpose |

---

## Why ViWoApp?

| Problem | ViWoApp Solution |
|---------|------------------|
| Platforms take 30-55% of creator earnings | **Creators keep 100%** of tips and content sales |
| Users earn nothing for engagement | **Every action is monetized** — like, comment, share |
| Bots and fake engagement pollute platforms | **5A Protocol** makes bot farming economically unviable |
| Your identity is locked in platforms | **Portable DID** travels with you across Solana |
| Web3 transactions cost $5-50 | Solana transactions cost **$0.00025** |
| Inflationary tokens destroy value | VCoin becomes **deflationary by Year 5** |

---

## Smart Contracts

This workspace contains **11 Solana programs** powering the entire ViWoApp ecosystem:

### Core Token Layer

| Program | Description | Structure |
|---------|-------------|-----------|
| **vcoin-token** | VCoin Token-2022 with Permanent Delegate & Metadata extensions | Full Modular |
| **vevcoin-token** | Vote-Escrowed VCoin — Soulbound governance token (non-transferable) | Full Modular |

### Protocol Layer

| Program | Description | Structure |
|---------|-------------|-----------|
| **staking-protocol** | Stake VCoin → Earn veVCoin with tier-based rewards | Full Modular |
| **transfer-hook** | Auto-updates 5A scores, detects wash trading on transfers | Full Modular |
| **identity-protocol** | On-chain DID anchor with verification levels | Full Modular |
| **five-a-protocol** | Anti-bot reputation scoring with oracle model | Full Modular |
| **content-registry** | On-chain content tracking with state management | Full Modular |
| **gasless-protocol** | Paymaster & Session Keys for zero-friction UX | Full Modular |
| **governance-protocol** | Quadratic voting with 5A boosts and delegation | Streamlined |
| **sscre-protocol** | Self-Sustaining Circular Reward Economy — Merkle claims | Streamlined |
| **vilink-protocol** | Cross-dApp action deep links | Streamlined |

---

## VCoin Token

The native utility token powering the ViWoApp ecosystem.

| Parameter | Value |
|-----------|-------|
| **Total Supply** | 1,000,000,000 (1B) |
| **Decimals** | 9 |
| **Standard** | Token-2022 with Extensions |
| **Permanent Delegate** | Enables slashing bad actors |

### Token Utility

- **Rewards** — Earn VCoin for quality engagement
- **Staking** — Lock VCoin for yield and fee discounts
- **Governance** — Vote on protocol changes with veVCoin
- **Tips** — Direct creator-to-fan value transfer
- **Commerce** — Buy and sell in the marketplace

---

## Staking Tiers

Lock VCoin to earn veVCoin governance power with tier-based multipliers:

| Tier | Minimum Stake | Fee Discount | veVCoin Boost |
|------|---------------|--------------|---------------|
| None | 0 | 0% | 1.0x |
| Bronze | 1,000 | 10% | 1.1x |
| Silver | 5,000 | 20% | 1.2x |
| Gold | 20,000 | 30% | 1.3x |
| Platinum | 100,000 | 50% | 1.4x |

**veVCoin Formula:**
```
veVCoin = staked_amount × (lock_duration / 4_years) × tier_boost
```

---

## The 5A Reputation Protocol

Every user is scored 0-100% on five dimensions:

| Star | Name | What It Measures |
|------|------|------------------|
| A | **Authenticity** | KYC completion, profile verification, account age |
| A | **Accuracy** | Content quality, factual accuracy, community feedback |
| A | **Agility** | Response time, engagement speed, adaptability |
| A | **Activity** | Daily actions, posting frequency, consistency |
| A | **Approved** | Community standing, trust level, reputation history |

### Reward Multipliers

- **50% average** = 1.0x (baseline)
- **100% average** = 2.0x (maximum)
- **0% average** = 0.0x (earn nothing)

Power users earn nearly **2x rewards** while bots earn almost nothing.

---

## SSCRE Protocol

The **Self-Sustaining Circular Reward Economy** ensures rewards never run out:

| Phase | Years | Mechanism |
|-------|-------|-----------|
| Emission | 1-5 | 350M reward pool distributes ~5.83M/month |
| Reserve | 6-10 | ~84M saved reserves, zero new tokens |
| Perpetual | 11+ | Scheduled 250M minting every 5 years |

### 6-Layer Funding Hierarchy

| Layer | Source |
|-------|--------|
| L0 | Unused Allocation |
| L1 | Reserve Fund |
| L2 | Scheduled Minting |
| L3 | Buyback Recycling |
| L4 | Profit Buybacks |
| L5 | Fee Recycling |

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

### Build

```bash
anchor build
```

### Verify Build (Modular Structure)

```bash
cargo check --all
# Expected: Finished `dev` profile [unoptimized + debuginfo] target(s)
```

### Test

```bash
yarn install
anchor test
```

### Deploy to Devnet

```bash
solana config set --url devnet
anchor deploy
```

---

## Project Structure (Modular Architecture)

```
vcoin_workspace/
├── Anchor.toml                 # Anchor configuration
├── Cargo.toml                  # Workspace configuration
├── programs/
│   ├── vcoin-token/            # VCoin Token-2022
│   │   └── src/
│   │       ├── lib.rs          # Program entry point
│   │       ├── constants.rs    # Protocol constants
│   │       ├── errors.rs       # Custom error types
│   │       ├── events.rs       # Event definitions
│   │       ├── state/          # Account state definitions
│   │       │   ├── mod.rs
│   │       │   └── *.rs
│   │       ├── contexts/       # Anchor account contexts
│   │       │   ├── mod.rs
│   │       │   └── *.rs
│   │       └── instructions/   # Instruction handlers
│   │           ├── mod.rs
│   │           ├── admin/
│   │           ├── token/
│   │           └── query/
│   ├── vevcoin-token/          # Soulbound veVCoin (same structure)
│   ├── staking-protocol/       # Tier-based staking
│   ├── transfer-hook/          # Transfer automation
│   ├── identity-protocol/      # Decentralized identity
│   ├── five-a-protocol/        # Reputation scoring
│   ├── content-registry/       # Content management
│   ├── gasless-protocol/       # Session keys
│   ├── governance-protocol/    # On-chain governance (streamlined)
│   │   └── src/
│   │       ├── lib.rs          # Entry + contexts + handlers
│   │       ├── constants.rs
│   │       ├── errors.rs
│   │       ├── events.rs
│   │       └── state/
│   ├── sscre-protocol/         # Reward economics (streamlined)
│   └── vilink-protocol/        # Deep links (streamlined)
├── tests/                      # TypeScript integration tests
├── packages/
│   └── viwoapp-sdk/            # TypeScript SDK
│       ├── src/
│       │   ├── index.ts
│       │   ├── client.ts
│       │   ├── staking/
│       │   ├── governance/
│       │   ├── rewards/
│       │   └── ...
│       └── dist/               # Built SDK (CJS/ESM/DTS)
└── target/
    └── deploy/                 # Compiled .so programs
```

### Module Structure Types

**Full Modular** (8 programs):
- Complete separation: constants, errors, events, state, contexts, instructions
- Instruction handlers in `instructions/admin/`, `instructions/user/`, etc.
- Best for complex programs with many instructions

**Streamlined** (3 programs):
- State modules extracted (constants, errors, events, state)
- Contexts and handlers remain in `lib.rs`
- Better for very large programs to reduce file count

---

## TypeScript SDK

The `@viwoapp/sdk` package provides full TypeScript integration:

```typescript
import { ViWoAppClient } from '@viwoapp/sdk';

const client = new ViWoAppClient(connection, wallet);

// Stake VCoin
await client.staking.stake(amount, lockDuration);

// Check 5A score
const score = await client.fiveA.getScore(userPubkey);

// Claim SSCRE rewards
await client.rewards.claimRewards(merkleProof);

// Create ViLink action
const actionLink = await client.vilink.createAction({
  type: 'tip',
  target: creatorPubkey,
  amount: 100_000_000, // 0.1 VCoin
});

// Use gasless session
const session = await client.gasless.createSession({
  scope: ['tip', 'vouch', 'content'],
  duration: 24 * 60 * 60, // 24 hours
});
```

---

## Devnet Deployment

✅ **All 11 programs deployed to Solana Devnet**

| Program | Status | Address |
|---------|--------|---------|
| vcoin-token | ✅ Deployed | `Gg1dtrjAfGYi6NLC31WaJjZNBoucvD98rK2h1u9qrUjn` |
| vevcoin-token | ✅ Deployed | `FB39ae9x53FxVL3pER9LqCPEx2TRnEnQP55i838Upnjx` |
| staking-protocol | ✅ Deployed | `6EFcistyr2E81adLUcuBJRr8W2xzpt3D3dFYEcMewpWu` |
| transfer-hook | ✅ Deployed | `9K14FcDRrBeHKD9FPNYeVJaEqJQTac2xspJyb1mM6m48` |
| identity-protocol | ✅ Deployed | `3egAds3pFR5oog6iQCN42KPvgih8HQz2FGybNjiVWixG` |
| five-a-protocol | ✅ Deployed | `783PbtJw5cc7yatnr9fsvTGSnkKaV6iJe6E8VUPTYrT8` |
| content-registry | ✅ Deployed | `MJn1A4MPCBPJGWWuZrtq7bHSo2G289sUwW3ej2wcmLV` |
| governance-protocol | ✅ Deployed | `3R256kBN9iXozjypQFRAmegBhd6HJqXWqdNG7Th78HYe` |
| sscre-protocol | ✅ Deployed | `6AJNcQSfoiE2UAeUDyJUBumS9SBwhAdSznoAeYpXrxXZ` |
| vilink-protocol | ✅ Deployed | `CFGXTS2MueQwTYTMMTBQbRWzJtSTC2p4ZRuKPpLDmrv7` |
| gasless-protocol | ✅ Deployed | `FcXJAjzJs8eVY2WTRFXynQBpC7WZUqKZppyp9xS6PaB3` |

**Devnet Explorer:** [explorer.solana.com/?cluster=devnet](https://explorer.solana.com/?cluster=devnet)

---

## Architecture

```
┌─────────────────────────────────────────────────────────────┐
│                      ViWoApp Platform                        │
├─────────────────────────────────────────────────────────────┤
│  Identity │ Content │ 5A Policy │ Governance │ Rewards      │
├─────────────────────────────────────────────────────────────┤
│              Gasless Layer (Paymaster + Sessions)            │
├─────────────────────────────────────────────────────────────┤
│         VCoin (Token-2022) ◄──► Staking ──► veVCoin         │
├─────────────────────────────────────────────────────────────┤
│                    Solana Blockchain                         │
└─────────────────────────────────────────────────────────────┘
```

---

## Security

- **Authority checks** on all admin functions
- **PDA seeds** prevent account collisions  
- **Lock duration validation** prevents gaming
- **Permanent delegate** enables slashing without user signature
- **Pausable** for emergency situations
- **Checked arithmetic** using `checked_add`, `checked_sub`, etc.
- **Event emission** for all state changes
- **Modular architecture** for easier security audits
- **Planned audits** with Neodyme and OtterSec

---

## Build Status

| Metric | Value |
|--------|-------|
| Programs | 11 |
| Total Rust Files | ~196 |
| Total Lines of Code | ~18,500 |
| Build Status | ✅ All Passing |
| Deployed to Devnet | ✅ 11/11 |

---

## Links

| Resource | URL |
|----------|-----|
| Website | [viwoapp.com](https://viwoapp.com) |
| X/Twitter | [@ViWoApp](https://x.com/ViWoApp) |
| Telegram | [@ViWoApp](https://t.me/ViWoApp) |
| Discord | [discord.gg/viwoapp](https://discord.gg/viwoapp) |
| GitHub | [github.com/viwoapp](https://github.com/viwoapp) |

---

## License

MIT License — All protocols are open source and available as public goods.

---

**Version:** 2.1  
**Framework:** Anchor 0.32.0 | Solana Program 2.0 | Token-2022 6.0  
**Architecture:** Modular (2025-2026 Best Practices)  
**Network:** Solana Devnet (11/11 Deployed)
