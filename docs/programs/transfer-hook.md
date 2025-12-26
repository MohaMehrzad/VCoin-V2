# Transfer Hook

Automatic 5A updates and wash trading detection.

## Overview

The Transfer Hook program is invoked on every VCoin transfer via Token-2022's transfer hook extension. It automatically updates 5A Activity scores and detects potential wash trading patterns.

**Devnet Address:** `9K14FcDRrBeHKD9FPNYeVJaEqJQTac2xspJyb1mM6m48`

## Key Features

- **Auto 5A Activity updates** on transfer
- **Wash trading detection** between related accounts
- **Tip recording** for ViLink actions
- **Engagement trust tracking**
- **No user action required** - runs automatically

## How It Works

1. User initiates VCoin transfer
2. Token-2022 program calls transfer hook
3. Hook executes before transfer completes:
   - Updates sender's 5A Activity score
   - Checks for wash trading patterns
   - Records tip if from ViLink
4. Transfer completes (or is blocked)

## Instructions

### initialize

Creates the hook configuration.

**Authority:** Admin only

### execute

Called automatically on every VCoin transfer.

**Authority:** Token-2022 program only

**Effects:**
- Updates sender 5A Activity score
- Checks wash trading patterns
- Records tips for ViLink
- May block suspicious transfers

### initialize_extra_account_meta_list

Setup extra accounts needed for hook execution.

**Authority:** Admin only

### update_config

Update hook parameters.

**Authority:** Protocol authority

**Parameters:**
- `block_wash_trading: bool` - Enable/disable blocking
- `min_transfer_for_update: u64` - Minimum for 5A update
- `wash_trading_window: i64` - Detection window

## Account Structure

```rust
pub struct TransferHookConfig {
    pub authority: Pubkey,
    pub vcoin_mint: Pubkey,
    pub five_a_program: Pubkey,
    pub block_wash_trading: bool,
    pub min_transfer_for_update: u64,
    pub wash_trading_window: i64,
    pub total_transfers: u64,
    pub blocked_transfers: u64,
    pub paused: bool,
    pub bump: u8,
}

pub struct UserTransferHistory {
    pub user: Pubkey,
    pub recent_recipients: [Pubkey; 10],
    pub recent_timestamps: [i64; 10],
    pub recent_index: u8,
    pub total_sent: u64,
    pub total_received: u64,
    pub last_transfer_at: i64,
    pub bump: u8,
}
```

## Wash Trading Detection

The hook detects circular transfer patterns:

1. Tracks last 10 transfer recipients per user
2. Checks if current recipient recently sent to sender
3. If pattern detected within window (default: 1 hour):
   - If `block_wash_trading = true`: Transfer blocked
   - If `block_wash_trading = false`: 5A update skipped

```
A → B → A (within 1 hour) = Wash trading detected
```

## Security Features

- **Token-2022 authorization** - Only callable by token program
- **Wash trading prevention** - Configurable blocking
- **Rate limiting** - Minimum transfer amount for 5A updates
- **Pausable** - Emergency stop capability

## Integration

The transfer hook is automatic - no explicit integration needed. When transferring VCoin:

```typescript
import { ViWoClient, parseVCoin } from "@viwoapp/sdk";

// Normal transfer - hook runs automatically
const transferTx = await client.buildTransferTransaction({
  to: recipient,
  amount: parseVCoin("100"),
});
await client.sendTransaction(transferTx);

// User's 5A Activity score automatically updated
// Wash trading automatically checked
```

## Configuration

Hook behavior is controlled by the protocol authority:

```typescript
// Update hook config (admin only)
const updateTx = await client.transferHook.buildUpdateConfigTransaction({
  blockWashTrading: true,
  minTransferForUpdate: parseVCoin("1"),
  washTradingWindow: 3600, // 1 hour
});
```

## Extra Account Meta List

The hook requires extra accounts for execution:

1. Transfer Hook Config
2. User Transfer History (sender)
3. 5A Protocol Program
4. User Score (sender)

These are automatically resolved by Token-2022 using the extra account meta list.

## Source Code

- [`programs/transfer-hook/src/`](../../programs/transfer-hook/src/)
- [`programs/transfer-hook/src/state/`](../../programs/transfer-hook/src/state/)

