# veVCoin Token

Vote-Escrowed VCoin - the soulbound governance token.

## Overview

veVCoin is a non-transferable (soulbound) token representing locked VCoin. It's used for governance voting power and cannot be bought, sold, or transferred.

**Devnet Address:** `FB39ae9x53FxVL3pER9LqCPEx2TRnEnQP55i838Upnjx`

## Key Features

- **Non-Transferable** (soulbound via Token-2022)
- **Minted only by Staking Protocol**
- **Burned on unstake**
- **Prevents governance power markets**
- **Time-weighted voting power**

## How It Works

1. User stakes VCoin with a lock duration
2. Staking Protocol mints veVCoin proportional to stake × lock time
3. veVCoin provides governance voting power
4. On unstake, veVCoin is burned

## veVCoin Formula

```
veVCoin = staked_amount × (lock_duration / 4_years) × tier_boost

Where:
- lock_duration: 1 week to 4 years
- tier_boost: 1.0x (None) to 1.4x (Platinum)
```

## Instructions

### initialize_mint

Creates the soulbound veVCoin mint.

**Authority:** Admin only

### mint_vevcoin

Mints veVCoin to user. Only callable by the staking protocol.

**Authority:** Staking Protocol only

**Parameters:**
- `amount: u64` - Amount to mint
- `user: Pubkey` - Recipient

### burn_vevcoin

Burns veVCoin on unstake. Only callable by the staking protocol.

**Authority:** Staking Protocol only

**Parameters:**
- `amount: u64` - Amount to burn
- `user: Pubkey` - Token owner

### update_staking_protocol

Updates the authorized staking program.

**Authority:** Protocol authority

### get_balance

Query user veVCoin balance.

**Authority:** Anyone

## Account Structure

```rust
pub struct VeVCoinConfig {
    pub authority: Pubkey,
    pub mint: Pubkey,
    pub staking_protocol: Pubkey,  // Only this can mint/burn
    pub total_supply: u64,
    pub total_holders: u64,
    pub bump: u8,
}

pub struct UserVeVCoin {
    pub owner: Pubkey,
    pub balance: u64,
    pub first_mint_at: i64,
    pub last_update_at: i64,
    pub bump: u8,
}
```

## Security Features

- **Soulbound** - Cannot be transferred
- **Single minter** - Only staking protocol can mint
- **Proportional burns** - Burns match unstake amounts
- **Two-step authority transfer**

## Integration

```typescript
import { ViWoClient, formatVCoin } from "@viwoapp/sdk";

// Get veVCoin balance
const veBalance = await client.getVeVCoinBalance(wallet);
console.log("veVCoin:", formatVCoin(veBalance));

// veVCoin is minted automatically when staking
const stakeTx = await client.staking.buildStakeTransaction({
  amount: parseVCoin("1000"),
  lockDuration: LOCK_DURATIONS.oneYear,
});
// User receives veVCoin proportional to stake × lock time
```

## Source Code

- [`programs/vevcoin-token/src/`](../../programs/vevcoin-token/src/)
- [`programs/vevcoin-token/src/state/`](../../programs/vevcoin-token/src/state/)

