# Staking Protocol

Stake VCoin to receive veVCoin governance power.

## Overview

The Staking Protocol allows users to lock VCoin for a period of time to receive veVCoin (vote-escrowed VCoin). Longer locks and higher stakes provide more voting power and platform benefits.

**Devnet Address:** `6EFcistyr2E81adLUcuBJRr8W2xzpt3D3dFYEcMewpWu`

## Key Features

- **Tier-based staking** (Bronze → Platinum)
- **Time-weighted veVCoin** minting
- **Lock periods** from 1 week to 4 years
- **Fee discounts** based on tier
- **CPI integration** with veVCoin program

## Staking Tiers

| Tier | Min Stake | Fee Discount | veVCoin Boost |
|------|-----------|--------------|---------------|
| None | 0 | 0% | 1.0x |
| Bronze | 1,000 VCoin | 10% | 1.1x |
| Silver | 5,000 VCoin | 20% | 1.2x |
| Gold | 20,000 VCoin | 30% | 1.3x |
| Platinum | 100,000 VCoin | 50% | 1.4x |

## veVCoin Formula

```
veVCoin = staked_amount × (lock_duration / 4_years) × tier_boost

Example (1 year lock, Gold tier):
veVCoin = 20,000 × (1/4) × 1.3 = 6,500 veVCoin
```

## Instructions

### initialize_pool

Creates the staking pool.

**Authority:** Admin only

### stake

Stakes VCoin with a lock duration.

**Authority:** User

**Parameters:**
- `amount: u64` - VCoin amount to stake
- `lock_duration: i64` - Lock duration in seconds (min: 604,800 = 1 week)

**Effects:**
- Transfers VCoin to pool vault
- Mints veVCoin via CPI
- Updates user tier

### extend_lock

Extends lock duration to receive more veVCoin.

**Authority:** User (existing staker)

**Parameters:**
- `new_lock_duration: i64` - New lock duration (must be longer)

### unstake

Withdraws staked VCoin after lock expires.

**Authority:** User

**Effects:**
- Returns VCoin from pool vault
- Burns veVCoin via CPI
- Resets user stake

### update_tier

Recalculates user tier based on current stake.

**Authority:** Anyone

### get_stake_info

Query stake details.

**Authority:** Anyone

## Account Structure

```rust
pub struct StakingPool {
    pub authority: Pubkey,
    pub vcoin_mint: Pubkey,
    pub vevcoin_mint: Pubkey,
    pub vevcoin_program: Pubkey,
    pub pool_vault: Pubkey,
    pub total_staked: u64,
    pub total_stakers: u64,
    pub paused: bool,
    pub reentrancy_guard: ReentrancyGuard,
    pub bump: u8,
    pub vault_bump: u8,
}

pub struct UserStake {
    pub owner: Pubkey,
    pub staked_amount: u64,
    pub lock_duration: i64,
    pub lock_end: i64,
    pub stake_start: i64,
    pub tier: u8,
    pub ve_vcoin_amount: u64,
    pub bump: u8,
}
```

## Security Features

- **Reentrancy guards** for CPI calls
- **Lock duration validation** (min 1 week, max 4 years)
- **Pausable** for emergencies
- **Two-step authority transfer**

## Integration

```typescript
import { ViWoClient, parseVCoin, LOCK_DURATIONS, STAKING_TIERS } from "@viwoapp/sdk";

// Get staking pool info
const pool = await client.staking.getPool();
console.log("Total Staked:", pool.totalStaked);

// Get user stake
const stake = await client.staking.getUserStake(wallet);
console.log("Your Tier:", client.staking.getTierName(stake.tier));

// Calculate tier for amount
const tier = client.staking.calculateTier(50000); // Gold

// Stake VCoin
const stakeTx = await client.staking.buildStakeTransaction({
  amount: parseVCoin("10000"),
  lockDuration: LOCK_DURATIONS.oneYear,
});

// Check unstake eligibility
const { canUnstake, reason } = await client.staking.canUnstake(wallet);
```

## Source Code

- [`programs/staking-protocol/src/`](../../programs/staking-protocol/src/)
- [`programs/staking-protocol/src/state/`](../../programs/staking-protocol/src/state/)
- [`programs/staking-protocol/src/instructions/`](../../programs/staking-protocol/src/instructions/)

