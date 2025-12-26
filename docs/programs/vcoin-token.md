# VCoin Token

The main utility and rewards token for the ViWo ecosystem.

## Overview

VCoin (VIWO) is a Token-2022 token with advanced extensions enabling governance-controlled slashing and automatic 5A score updates via transfer hooks.

**Devnet Address:** `Gg1dtrjAfGYi6NLC31WaJjZNBoucvD98rK2h1u9qrUjn`

## Key Features

- **Token-2022 Standard** with metadata extension
- **Permanent Delegate** for governance-approved slashing
- **Transfer Hook** for auto 5A updates
- **1 Billion Total Supply** with 9 decimals
- **Governance Slashing** with 48-hour timelock

## Token Specifications

| Property | Value |
|----------|-------|
| Name | VCoin |
| Symbol | VIWO |
| Total Supply | 1,000,000,000 |
| Decimals | 9 |
| Standard | Token-2022 |

## Instructions

### initialize_mint

Creates the VCoin mint with Token-2022 extensions.

**Authority:** Admin only

### mint_tokens

Mints VCoin tokens to a destination account.

**Authority:** Protocol authority

**Parameters:**
- `amount: u64` - Amount to mint (with 9 decimals)
- `destination: Pubkey` - Recipient token account

### propose_slash

Proposes a token slash request for governance approval.

**Authority:** Permanent delegate

**Parameters:**
- `target: Pubkey` - Target wallet
- `amount: u64` - Amount to slash
- `reason: [u8; 32]` - Reason hash

### approve_slash

Approves a pending slash request (governance vote).

**Authority:** Governance authority

### execute_slash

Executes an approved slash after 48-hour timelock.

**Authority:** Anyone (after timelock)

### set_paused

Emergency pause/unpause functionality.

**Authority:** Protocol authority

## Account Structure

```rust
pub struct VCoinConfig {
    pub authority: Pubkey,           // Admin authority
    pub mint: Pubkey,                // Token mint address
    pub treasury: Pubkey,            // Treasury account
    pub permanent_delegate: Pubkey,  // Slashing authority
    pub total_minted: u64,           // Total tokens minted
    pub paused: bool,                // Emergency pause flag
    pub pending_authority: Option<Pubkey>,
    pub pending_authority_activated_at: Option<i64>,
    pub bump: u8,
}
```

## Security Features

- **Two-step authority transfer** with 24-hour timelock
- **Governance-controlled slashing** with 48-hour execution delay
- **Pausable** for emergency situations
- **Legacy slash disabled** - must use propose/approve/execute flow

## Integration

```typescript
import { ViWoClient, parseVCoin, formatVCoin } from "@viwoapp/sdk";

// Get VCoin balance
const balance = await client.getVCoinBalance(wallet);
console.log("Balance:", formatVCoin(balance));

// Mint tokens (admin only)
const mintTx = await client.buildMintTransaction({
  destination: userWallet,
  amount: parseVCoin("1000"),
});
```

## Source Code

- [`programs/vcoin-token/src/`](../../programs/vcoin-token/src/)
- [`programs/vcoin-token/src/state/`](../../programs/vcoin-token/src/state/)
- [`programs/vcoin-token/src/instructions/`](../../programs/vcoin-token/src/instructions/)

