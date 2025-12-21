<p align="center">
  <img src="https://salmon-calm-louse-860.mypinata.cloud/ipfs/bafkreifrxcxq54wr2se2gtzuhxedddaimijas2ca3ilrgxusnxvx4xvnd4" alt="ViWoApp Logo" width="120" height="120">
</p>

# ViWoApp Smart Contracts

**Censorship Proof Social Media for Crypto Community**

Built on Solana | 100% Creator Earnings | MIT Open Source

---

## Abstract

ViWoApp is an open-source SocialFi platform built natively on Solana, designed to fundamentally reshape how crypto users interact, earn, and build reputation in the digital economy. Unlike traditional social platforms that extract value from users, ViWoApp returns ownership to both creators and consumers — monetizing every engagement action and rewarding genuine participation.

The platform introduces three core innovations:

1. **The 5A Reputation Protocol** — A transparent, gamified system that rewards genuine engagement and penalizes bots and spam
2. **The SSCRE Protocol** — A Self-Sustaining Circular Reward Economy that ensures rewards never run out
3. **Portable Decentralized Identity** — Your reputation travels with you across the Solana ecosystem

Every protocol is MIT licensed and available as a public good for the entire Solana ecosystem.

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

| Program | Description |
|---------|-------------|
| **vcoin-token** | VCoin Token-2022 with Permanent Delegate & Metadata extensions |
| **vevcoin-token** | Vote-Escrowed VCoin — Soulbound governance token (non-transferable) |

### Protocol Layer

| Program | Description |
|---------|-------------|
| **staking-protocol** | Stake VCoin → Earn veVCoin with tier-based rewards |
| **transfer-hook** | Auto-updates 5A scores, detects wash trading on transfers |
| **identity-protocol** | On-chain DID anchor with verification levels |
| **five-a-protocol** | Anti-bot reputation scoring with oracle model |
| **content-registry** | On-chain content tracking with state management |
| **governance-protocol** | Quadratic voting with 5A boosts and delegation |
| **sscre-protocol** | Self-Sustaining Circular Reward Economy — Merkle claims |
| **vilink-protocol** | Cross-dApp action deep links |
| **gasless-protocol** | Paymaster & Session Keys for zero-friction UX |

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
# Install Solana development tools
curl --proto '=https' --tlsv1.2 -sSfL https://solana-install.solana.workers.dev | bash
```

### Build

```bash
anchor build
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

## Project Structure

```
vcoin_workspace/
├── Anchor.toml                 # Anchor configuration
├── Cargo.toml                  # Workspace configuration
├── programs/
│   ├── vcoin-token/            # VCoin Token-2022
│   ├── vevcoin-token/          # Soulbound veVCoin
│   ├── staking-protocol/       # Tier-based staking
│   ├── transfer-hook/          # Transfer automation
│   ├── identity-protocol/      # Decentralized identity
│   ├── five-a-protocol/        # Reputation scoring
│   ├── content-registry/       # Content management
│   ├── governance-protocol/    # On-chain governance
│   ├── sscre-protocol/         # Reward economics
│   ├── vilink-protocol/        # Deep links
│   └── gasless-protocol/       # Session keys
├── tests/                      # TypeScript integration tests
├── packages/
│   └── viwoapp-sdk/            # TypeScript SDK
└── target/
    └── deploy/                 # Compiled .so programs
```

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
```

---

## Devnet Deployment

| Program | Status | Address |
|---------|--------|---------|
| vcoin-token | ✅ Deployed | `Fg6...` |
| vevcoin-token | ✅ Deployed | `9xQ...` |
| staking-protocol | ✅ Deployed | `4vS...` |
| transfer-hook | ✅ Deployed | `Gh7...` |
| identity-protocol | ✅ Deployed | `Bx8...` |
| five-a-protocol | ✅ Deployed | `Ck9...` |
| content-registry | ✅ Deployed | `Dm2...` |
| governance-protocol | ✅ Deployed | `En3...` |
| sscre-protocol | 🔄 Pending | — |
| vilink-protocol | 🔄 Pending | — |
| gasless-protocol | 🔄 Pending | — |

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
- **Planned audits** with Neodyme and OtterSec

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

**Version:** 1.0  
**Framework:** Anchor 0.32.0  
**Network:** Solana (Devnet/Mainnet)
