# ViLink Protocol

Cross-dApp action links for social interactions.

## Overview

ViLink enables shareable, cross-dApp action links for social interactions on Solana. Users can create tip links, vouch requests, follow actions, and more that work across any ViLink-integrated application.

**Devnet Address:** `CFGXTS2MueQwTYTMMTBQbRWzJtSTC2p4ZRuKPpLDmrv7`

## Key Features

- **Shareable action links** for tips, vouches, follows
- **Cross-dApp compatibility** - works in any ViLink app
- **2.5% platform fee** on tips
- **Batch actions** for efficiency
- **Deep link URIs** for mobile apps
- **Nonce-based PDAs** for deterministic actions

## Action Types

| Type | Value | Fee | Description |
|------|-------|-----|-------------|
| Tip | 0 | 2.5% | Send VCoin with message |
| Vouch | 1 | Stake | Vouch for user's reputation |
| Follow | 2 | None | Follow user |
| Challenge | 3 | Stake | Create user challenge |
| Stake | 4 | None | Stake call-to-action |
| ContentReact | 5 | None | React to content |
| Delegate | 6 | None | Delegate governance votes |
| Vote | 7 | None | Vote on proposal |

## Instructions

### initialize

Initializes ViLink protocol.

**Authority:** Admin only

### create_action

Create a shareable action link.

**Authority:** User

**Parameters:**
- `action_type: u8` - Type of action
- `target: Pubkey` - Target user/content
- `amount: u64` - Amount (for tips)
- `message: [u8; 128]` - Optional message
- `expires_at: i64` - Expiration timestamp

**Returns:** Action PDA (used in link)

### execute_tip_action

Execute a tip action.

**Authority:** User

**Parameters:**
- `action_id: Pubkey` - Action to execute

**Effects:**
- Transfers VCoin (minus 2.5% fee)
- Updates 5A scores
- Emits TipExecuted event

### execute_vouch_action

Execute a vouch action.

**Authority:** User with 60%+ 5A score

### register_dapp

Authorize an external dApp for ViLink.

**Authority:** Protocol authority

**Parameters:**
- `dapp_id: Pubkey` - dApp identifier
- `name: String` - dApp name
- `webhook_url: String` - Callback URL

### create_batch

Create a batch of actions.

**Authority:** User

**Parameters:**
- `actions: Vec<ActionParams>` - Actions to batch
- `batch_nonce: u64` - Deterministic nonce

## Account Structure

```rust
pub struct ViLinkConfig {
    pub authority: Pubkey,
    pub vcoin_mint: Pubkey,
    pub five_a_program: Pubkey,
    pub fee_vault: Pubkey,
    pub tip_fee_bps: u16,        // 250 = 2.5%
    pub total_actions: u64,
    pub total_tips: u64,
    pub paused: bool,
    pub bump: u8,
}

pub struct ViLinkAction {
    pub creator: Pubkey,
    pub action_type: u8,
    pub target: Pubkey,
    pub amount: u64,
    pub message: [u8; 128],
    pub created_at: i64,
    pub expires_at: i64,
    pub executed: bool,
    pub executed_by: Option<Pubkey>,
    pub executed_at: Option<i64>,
    pub nonce: u64,
    pub bump: u8,
}

pub struct UserActionStats {
    pub user: Pubkey,
    pub actions_created: u64,
    pub actions_executed: u64,
    pub tips_sent: u64,
    pub tips_received: u64,
    pub batch_nonce: u64,
    pub bump: u8,
}
```

## URI Format

ViLink actions use a standard URI format:

```
viwo://action/{action_type}/{action_id}

Examples:
viwo://action/tip/7rK3...xYz
viwo://action/vouch/9aB2...mNp
viwo://action/follow/2dF5...qRs
```

## Security Features

- **Nonce-based PDAs** - Deterministic, no collision
- **Expiration** - Actions can expire
- **dApp authorization** - Only registered dApps can integrate
- **5A requirement** - 60%+ score required for vouching
- **Fee cap** - Maximum 10% platform fee

## Integration

```typescript
import { ViWoClient, ActionType, parseVCoin } from "@viwoapp/sdk";

// Create tip action
const { actionId, uri } = await client.vilink.createAction({
  type: ActionType.Tip,
  target: creatorWallet,
  amount: parseVCoin("50"),
  message: "Great content!",
  expiresIn: 86400 * 7, // 7 days
});

console.log("Share this link:", uri);
// viwo://action/tip/7rK3...xYz

// Execute action (recipient or anyone)
const executeTx = await client.vilink.buildExecuteActionTransaction(actionId);

// Create batch of tips
const batchTx = await client.vilink.buildBatchTransaction([
  { type: ActionType.Tip, target: user1, amount: parseVCoin("10") },
  { type: ActionType.Tip, target: user2, amount: parseVCoin("20") },
  { type: ActionType.Follow, target: user3 },
]);
```

## Deep Linking

Mobile apps should register for the `viwo://` scheme:

**iOS:**
```xml
<key>CFBundleURLTypes</key>
<array>
  <dict>
    <key>CFBundleURLSchemes</key>
    <array><string>viwo</string></array>
  </dict>
</array>
```

**Android:**
```xml
<intent-filter>
  <action android:name="android.intent.action.VIEW" />
  <data android:scheme="viwo" android:host="action" />
</intent-filter>
```

## Source Code

- [`programs/vilink-protocol/src/`](../../programs/vilink-protocol/src/)
- [`programs/vilink-protocol/src/state/`](../../programs/vilink-protocol/src/state/)

