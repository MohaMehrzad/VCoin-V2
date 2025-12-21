# VCoin Smart Contracts

Solana smart contracts for VCoin Token, veVCoin (Soulbound), and Staking Protocol using Anchor framework with Token-2022.

## Overview

This workspace contains three core programs:

### 1. VCoin Token (`vcoin-token`)
The main utility and rewards token for ViWoApp.

**Features:**
- **Token-2022** with extensions
- **Total Supply:** 1,000,000,000 (1B)
- **Decimals:** 9
- **Permanent Delegate:** For slashing bad actors
- **Metadata Extension:** On-chain token metadata

**Instructions:**
- `initialize_mint` - Create VCoin mint with extensions
- `mint_tokens` - Mint to treasury (authority-controlled)
- `slash_tokens` - Slash tokens from bad actors (permanent delegate)
- `set_paused` - Pause/unpause token operations
- `update_authority` - Transfer authority

### 2. veVCoin Token (`vevcoin-token`)
Vote-Escrowed VCoin - Soulbound governance token.

**Features:**
- **Non-Transferable** (Soulbound via Token-2022)
- **Mint Authority:** Staking Protocol only
- **Burn Authority:** Staking Protocol only
- Prevents governance power markets

**Instructions:**
- `initialize_mint` - Create soulbound veVCoin mint
- `mint_vevcoin` - Called by staking program on stake
- `burn_vevcoin` - Called by staking program on unstake
- `update_staking_protocol` - Update authorized staking program

### 3. Staking Protocol (`staking-protocol`)
Stake VCoin to receive veVCoin voting power.

**Staking Tiers:**

| Tier | Minimum Stake | Fee Discount | veVCoin Boost |
|------|---------------|--------------|---------------|
| None | 0 | 0% | 1.0x |
| Bronze | 1,000 | 10% | 1.1x |
| Silver | 5,000 | 20% | 1.2x |
| Gold | 20,000 | 30% | 1.3x |
| Platinum | 100,000 | 50% | 1.4x |

**veVCoin Formula:**
```
ve_vcoin = staked_amount × (lock_duration / 4_years) × tier_boost
```

**Lock Duration:**
- Minimum: 1 week
- Maximum: 4 years
- Longer locks = More veVCoin

**Instructions:**
- `initialize_pool` - Create staking pool
- `stake` - Stake VCoin with lock duration
- `extend_lock` - Extend lock to increase veVCoin
- `unstake` - Withdraw after lock expires
- `update_tier` - Recalculate tier
- `set_paused` - Pause/unpause pool

## Development Setup

### Prerequisites

Install Solana development tools:

```bash
curl --proto '=https' --tlsv1.2 -sSfL https://solana-install.solana.workers.dev | bash
```

This installs:
- Rust
- Solana CLI
- Anchor CLI
- Node.js & Yarn

### Configure Solana

```bash
# Set to devnet
solana config set --url devnet

# Create wallet (if needed)
solana-keygen new

# Get test SOL
solana airdrop 5
```

### Build

```bash
cd VCoinContract/vcoin_workspace
anchor build
```

### Test

```bash
# Install dependencies
yarn install

# Run tests
anchor test
```

### Deploy to Devnet

```bash
anchor deploy
```

## Project Structure

```
vcoin_workspace/
├── Anchor.toml              # Anchor configuration
├── Cargo.toml               # Workspace configuration
├── programs/
│   ├── vcoin-token/         # VCoin Token-2022
│   │   ├── Cargo.toml
│   │   └── src/lib.rs
│   ├── vevcoin-token/       # Soulbound veVCoin
│   │   ├── Cargo.toml
│   │   └── src/lib.rs
│   └── staking-protocol/    # Staking with tiers
│       ├── Cargo.toml
│       └── src/lib.rs
├── tests/
│   ├── vcoin-token.ts
│   ├── vevcoin-token.ts
│   └── staking-protocol.ts
└── target/
    └── deploy/              # Compiled programs
```

## Program IDs

| Program | ID |
|---------|-----|
| vcoin-token | `VCNtkM3xg8ihH3JY8bQbqjUWCNEAVCiqUGmPjAqPNwP` |
| vevcoin-token | `VEVCnmRk9hYxBGhH3JY8bQbqjUWCNEAVCiqUGmPjBqQ` |
| staking-protocol | `STKGnmRk9hYxBGhH3JY8bQbqjUWCNEAVCiqUGmPjCrR` |

> **Note:** These are placeholder IDs. After first build, real program IDs will be generated.

## Architecture

```
┌──────────────────────────────────────────────────┐
│                  User Wallet                      │
└─────────────────────┬────────────────────────────┘
                      │
          ┌───────────┴───────────┐
          ▼                       ▼
┌─────────────────┐     ┌─────────────────┐
│   VCoin Token   │     │  Staking Pool   │
│   (Token-2022)  │◄────┤                 │
└─────────────────┘     │  stake/unstake  │
                        │                 │
                        └────────┬────────┘
                                 │
                                 ▼
                        ┌─────────────────┐
                        │  veVCoin Token  │
                        │  (Soulbound)    │
                        └─────────────────┘
```

## Security

- **Authority checks** on all admin functions
- **PDA seeds** prevent account collisions
- **Lock duration validation** prevents gaming
- **Permanent delegate** enables slashing without user signature
- **Pausable** for emergency situations

## License

MIT License - See main repository for details.

