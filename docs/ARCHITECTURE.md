# Architecture Documentation

This document describes the technical architecture of the ViWo Protocol Stack.

## Table of Contents

- [System Overview](#system-overview)
- [Protocol Stack](#protocol-stack)
- [On-Chain vs Off-Chain](#on-chain-vs-off-chain)
- [Token Architecture](#token-architecture)
- [Security Architecture](#security-architecture)
- [Program Interactions](#program-interactions)

## System Overview

ViWo is a Solana-native protocol stack for trust, reputation, and sustainable value distribution. The system consists of 11 on-chain programs working together to provide:

- **Anti-Sybil Protection** via 5A reputation scoring
- **Sustainable Rewards** via the SSCRE protocol
- **Gasless UX** via session keys and paymaster
- **Decentralized Governance** via veVCoin voting

```
┌─────────────────────────────────────────────────────────────────┐
│                    VIWO PROTOCOL STACK                           │
├─────────────────────────────────────────────────────────────────┤
│  ┌─────────────────────────────────────────────────────────┐    │
│  │                  APPLICATION LAYER                       │    │
│  │           Reference Implementation (ViWoApp)            │    │
│  └─────────────────────────────────────────────────────────┘    │
│                              │                                   │
│  ┌─────────────────────────────────────────────────────────┐    │
│  │                   CORE PROTOCOLS                         │    │
│  │  ┌──────────┐ ┌──────────┐ ┌──────────┐ ┌──────────┐   │    │
│  │  │   5A     │ │  SSCRE   │ │ Identity │ │Governance│   │    │
│  │  │ Protocol │ │ Protocol │ │ Protocol │ │ Protocol │   │    │
│  │  └──────────┘ └──────────┘ └──────────┘ └──────────┘   │    │
│  └─────────────────────────────────────────────────────────┘    │
│                              │                                   │
│  ┌─────────────────────────────────────────────────────────┐    │
│  │                 INFRASTRUCTURE LAYER                     │    │
│  │  ┌───────┐ ┌───────┐ ┌───────┐ ┌───────┐ ┌───────┐     │    │
│  │  │VCoin  │ │veVCoin│ │Staking│ │Transfer│ │Gasless│    │    │
│  │  │Token  │ │Token  │ │       │ │Hook   │ │       │     │    │
│  │  └───────┘ └───────┘ └───────┘ └───────┘ └───────┘     │    │
│  │  ┌───────┐ ┌───────┐                                    │    │
│  │  │Content│ │ViLink │                                    │    │
│  │  │Registry│ │       │                                   │    │
│  │  └───────┘ └───────┘                                    │    │
│  └─────────────────────────────────────────────────────────┘    │
│                              │                                   │
│  ┌─────────────────────────────────────────────────────────┐    │
│  │                   BLOCKCHAIN LAYER                       │    │
│  │     Solana • $0.00025/tx • 400ms blocks • 4,000+ TPS    │    │
│  └─────────────────────────────────────────────────────────┘    │
└─────────────────────────────────────────────────────────────────┘
```

## Protocol Stack

### Core Protocols (4)

| Protocol | Purpose | Key Features |
|----------|---------|--------------|
| **5A Protocol** | Anti-Sybil reputation | 5 dimension scoring, oracle consensus, vouch system |
| **SSCRE Protocol** | Sustainable rewards | Merkle claims, 6-layer funding, circuit breaker |
| **Identity Protocol** | Portable DID | SAS integration, verification levels, subscriptions |
| **Governance Protocol** | veVCoin voting | Quadratic voting, 5A boost, delegation, ZK support |

### Infrastructure Protocols (7)

| Protocol | Purpose | Key Features |
|----------|---------|--------------|
| **VCoin Token** | Utility token | Token-2022, permanent delegate, metadata |
| **veVCoin Token** | Governance token | Soulbound (non-transferable), staking-only minting |
| **Staking Protocol** | Lock VCoin → veVCoin | Tier-based, lock durations, CPI to veVCoin |
| **Transfer Hook** | Auto-updates | 5A activity scoring, wash trading detection |
| **Gasless Protocol** | Zero-friction UX | Session keys, paymaster, fee conversion |
| **Content Registry** | Content tracking | Energy system, engagement refunds |
| **ViLink Protocol** | Cross-dApp actions | Deep links, batch actions, tip protocol |

## On-Chain vs Off-Chain

The protocol uses a hybrid architecture to balance decentralization with performance:

| Component | Location | Rationale |
|-----------|----------|-----------|
| **Identity (SAS attestation)** | On-chain | Portable, verifiable identity |
| **5A Scores** | On-chain | Verifiable via oracle consensus |
| **Content hash/state** | On-chain | Proof of existence |
| **Content data** | Off-chain (IPFS) | Large data, high volume |
| **Engagement tracking** | Off-chain | High frequency, aggregated on-chain |
| **Staking/Governance** | On-chain | Trustless, financial |
| **Rewards claims** | On-chain (Merkle) | Verifiable, batch efficient |
| **Session keys** | On-chain | Security-critical, scoped |
| **Transaction fees** | On-chain (Gasless) | Paymaster model |

## Token Architecture

### VCoin (VIWO)

**Standard:** Token-2022 with extensions

```
┌─────────────────────────────────────────────┐
│               VCOIN TOKEN                    │
├─────────────────────────────────────────────┤
│ Total Supply: 1,000,000,000 (1B)            │
│ Decimals: 9                                  │
│ Standard: Token-2022                         │
├─────────────────────────────────────────────┤
│ Extensions:                                  │
│ • Metadata Extension (on-chain metadata)    │
│ • Permanent Delegate (governance slashing)  │
│ • Transfer Hook (auto 5A updates)           │
└─────────────────────────────────────────────┘
```

### veVCoin (Vote-Escrowed VCoin)

**Standard:** Token-2022 with non-transferable extension

```
┌─────────────────────────────────────────────┐
│              VEVCOIN TOKEN                   │
├─────────────────────────────────────────────┤
│ Non-Transferable (Soulbound)                │
│ Minted by: Staking Protocol only            │
│ Burned by: Staking Protocol only            │
├─────────────────────────────────────────────┤
│ veVCoin Formula:                            │
│ ve = staked × (lock_duration/4_years) × tier│
├─────────────────────────────────────────────┤
│ Lock Durations:                             │
│ • Minimum: 1 week                           │
│ • Maximum: 4 years                          │
└─────────────────────────────────────────────┘
```

### Staking Tiers

| Tier | Min Stake | Fee Discount | veVCoin Boost |
|------|-----------|--------------|---------------|
| None | 0 | 0% | 1.0x |
| Bronze | 1,000 | 10% | 1.1x |
| Silver | 5,000 | 20% | 1.2x |
| Gold | 20,000 | 30% | 1.3x |
| Platinum | 100,000 | 50% | 1.4x |

## Security Architecture

### Multi-Layer Security

```
┌─────────────────────────────────────────────────────────────┐
│                   SECURITY LAYERS                            │
├─────────────────────────────────────────────────────────────┤
│  Layer 1: Access Control                                     │
│  • Authority checks on all admin functions                   │
│  • PDA derivation verification                               │
│  • Account ownership validation                              │
├─────────────────────────────────────────────────────────────┤
│  Layer 2: Authority Transfer                                 │
│  • Two-step pattern (propose → accept)                       │
│  • 24-hour timelock                                          │
│  • Cancellation capability                                   │
├─────────────────────────────────────────────────────────────┤
│  Layer 3: Financial Security                                 │
│  • Checked arithmetic (overflow protection)                  │
│  • Reentrancy guards for CPI                                 │
│  • Slippage protection (5% max)                              │
├─────────────────────────────────────────────────────────────┤
│  Layer 4: Governance Security                                │
│  • 48-hour timelock for slashing                             │
│  • Proposal threshold enforcement                            │
│  • Quadratic voting (anti-whale)                             │
├─────────────────────────────────────────────────────────────┤
│  Layer 5: Oracle Security                                    │
│  • 3-of-N consensus for 5A scores                            │
│  • Rate limiting (1-hour cooldown)                           │
│  • Pending state before application                          │
├─────────────────────────────────────────────────────────────┤
│  Layer 6: Emergency Controls                                 │
│  • Pausable protocols                                        │
│  • Circuit breaker with 6-hour cooldown                      │
│  • ZK voting disabled until ready                            │
└─────────────────────────────────────────────────────────────┘
```

### Authority Roles

| Role | Responsibilities | Security |
|------|-----------------|----------|
| Protocol Authority | Config updates, oracle registration | Multisig, 24h timelock |
| Governance Authority | Proposal execution, parameter changes | veVCoin voting |
| Oracle Authority | 5A score submissions | 3-of-N consensus |
| Permanent Delegate | Token slashing | Governance approval + 48h |

## Program Interactions

### Staking Flow

```
User                    Staking               veVCoin              VCoin
  │                     Protocol              Token                Token
  │                         │                    │                    │
  │  stake(amount, lock)    │                    │                    │
  │────────────────────────>│                    │                    │
  │                         │                    │                    │
  │                         │  transfer VCoin    │                    │
  │                         │───────────────────────────────────────>│
  │                         │                    │                    │
  │                         │  CPI: mint_vevcoin │                    │
  │                         │───────────────────>│                    │
  │                         │                    │                    │
  │                         │   veVCoin minted   │                    │
  │<────────────────────────│<───────────────────│                    │
  │                         │                    │                    │
```

### 5A Score Update Flow

```
Oracle                  5A Protocol           Pending Score        User Score
  │                         │                    │                    │
  │  submit_score(user, scores)                  │                    │
  │────────────────────────>│                    │                    │
  │                         │                    │                    │
  │                         │  Check if pending exists               │
  │                         │───────────────────>│                    │
  │                         │                    │                    │
  │                         │  Record oracle vote                    │
  │                         │───────────────────>│                    │
  │                         │                    │                    │
  │                         │  3-of-N consensus reached?             │
  │                         │<───────────────────│                    │
  │                         │                    │                    │
  │                         │  Apply score       │                    │
  │                         │───────────────────────────────────────>│
  │                         │                    │                    │
```

### Governance Voting Flow

```
User                    Governance            User Stake           5A Score
  │                     Protocol                  │                    │
  │  cast_vote(proposal, choice)                  │                    │
  │────────────────────────>│                    │                    │
  │                         │                    │                    │
  │                         │  Read veVCoin balance                  │
  │                         │───────────────────>│                    │
  │                         │                    │                    │
  │                         │  Read 5A score     │                    │
  │                         │───────────────────────────────────────>│
  │                         │                    │                    │
  │                         │  Calculate voting power:               │
  │                         │  base = sqrt(veVCoin)                  │
  │                         │  boost = 1 + (5A_score / 100)          │
  │                         │  power = base × boost × tier           │
  │                         │                    │                    │
  │  Vote recorded          │                    │                    │
  │<────────────────────────│                    │                    │
```

### Transfer Hook Flow

```
Sender                  Token-2022            Transfer Hook        5A Protocol
  │                         │                    │                    │
  │  transfer(to, amount)   │                    │                    │
  │────────────────────────>│                    │                    │
  │                         │                    │                    │
  │                         │  execute_hook()    │                    │
  │                         │───────────────────>│                    │
  │                         │                    │                    │
  │                         │                    │  Update activity   │
  │                         │                    │─────────────────>  │
  │                         │                    │                    │
  │                         │                    │  Check wash trading│
  │                         │                    │─────────────────>  │
  │                         │                    │                    │
  │  Transfer complete      │                    │                    │
  │<────────────────────────│<───────────────────│                    │
```

## Modular Code Structure

All programs follow a consistent modular architecture:

```
programs/<protocol>/src/
├── lib.rs              # Entry point, instruction dispatch
├── constants.rs        # Protocol constants
├── errors.rs           # Error definitions
├── events.rs           # Event definitions
├── tests.rs            # Unit tests
├── state/              # Account structures
│   ├── mod.rs          # Re-exports
│   └── *.rs            # Individual account types
├── contexts/           # Anchor account contexts
│   ├── mod.rs          # Re-exports
│   └── *.rs            # Per-instruction contexts
└── instructions/       # Handler logic
    ├── mod.rs          # Re-exports
    ├── admin/          # Admin-only instructions
    └── user/           # User instructions
```

### Benefits

- **Auditability:** Isolated components for focused review
- **Maintainability:** Changes isolated to specific files
- **Testing:** Unit test individual modules
- **Readability:** Clear separation of concerns
- **Scalability:** Easy to add new instructions

## PDA Architecture

### Singleton PDAs (One per program)

| PDA | Seeds | Purpose |
|-----|-------|---------|
| VCoinConfig | `["vcoin_config"]` | Token configuration |
| StakingPool | `["staking_pool"]` | Pool state |
| GaslessConfig | `["gasless_config"]` | Paymaster config |

### Per-User PDAs

| PDA | Seeds | Purpose |
|-----|-------|---------|
| UserStake | `["user_stake", user]` | Stake position |
| UserScore | `["user_score", user]` | 5A reputation |
| UserClaim | `["user_claim", user]` | Reward claims |
| Identity | `["identity", user]` | DID anchor |
| SessionKey | `["session", user, session_pubkey]` | Active sessions |

### Per-Entity PDAs

| PDA | Seeds | Purpose |
|-----|-------|---------|
| Proposal | `["proposal", proposal_id]` | Governance proposal |
| VoteRecord | `["vote", voter, proposal]` | Vote record |
| SlashRequest | `["slash_request", timestamp]` | Pending slash |
| EpochDistribution | `["epoch", epoch_number]` | Rewards epoch |

---

*See also:*
- [Integration Guide](INTEGRATION.md)
- [API Reference](API.md)
- [Security Policy](../SECURITY.md)

