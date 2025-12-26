# Gasless Protocol

Session keys and paymaster for zero-friction UX.

## Overview

The Gasless Protocol enables users to interact with Solana without holding SOL. It provides session keys for popup-free transactions and a paymaster system for fee abstraction.

**Devnet Address:** `FcXJAjzJs8eVY2WTRFXynQBpC7WZUqKZppyp9xS6PaB3`

## Key Features

- **Session keys** for popup-free transactions
- **VCoin fee deduction** instead of SOL
- **Platform subsidization** for key actions
- **Scoped permissions** per session
- **24-hour session duration**
- **Per-user daily budget limits**

## Fee Methods

| Method | Description | Use Case |
|--------|-------------|----------|
| PlatformSubsidized | Platform pays 100% | Onboarding, governance |
| VCoinDeduction | User pays in VCoin | General usage |
| SSCRE | Deducted from rewards | Reward claims |

## Session Key Scopes

Session keys have limited permissions (bitmap):

| Scope | Bit | Value | Description |
|-------|-----|-------|-------------|
| Tip | 0 | 0x01 | Send VCoin tips |
| Vouch | 1 | 0x02 | Vouch for users |
| Content | 2 | 0x04 | Create/edit content |
| Governance | 3 | 0x08 | Vote on proposals |
| Transfer | 4 | 0x10 | Transfer VCoin |
| Stake | 5 | 0x20 | Stake VCoin |
| Claim | 6 | 0x40 | Claim rewards |
| Follow | 7 | 0x80 | Follow users |

**Common Presets:**
- `SOCIAL = 0xCF` (Tip, Vouch, Content, Follow)
- `CREATOR = 0x6F` (Content, Claim, Follow, Tip, Vouch)
- `FULL = 0xFF` (All permissions - use with caution)

## Instructions

### initialize

Initialize gasless infrastructure.

**Authority:** Admin only

### create_session_key

Create a temporary signing key for popup-free transactions.

**Authority:** User (requires biometric)

**Parameters:**
- `session_pubkey: Pubkey` - Temporary key
- `allowed_scopes: u8` - Permission bitmap
- `max_spend: u64` - Maximum VCoin spend
- `max_actions: u16` - Maximum action count
- `expires_in: i64` - Duration in seconds (max 24h)
- `fee_method: u8` - How to pay fees

### execute_session_action

Execute an action using session key.

**Authority:** Session key holder

**Parameters:**
- `action_type: u8` - Action to execute
- `spend_amount: u64` - VCoin spent (if applicable)

**Checks:**
- Session not expired
- Action within allowed scopes
- Spend within limits
- Actions count within limits

### deduct_vcoin_fee

Deduct VCoin for fee payment.

**Authority:** Backend/relay

**Parameters:**
- `user: Pubkey` - User to charge
- `amount: u64` - VCoin amount

### revoke_session_key

Invalidate a session early.

**Authority:** User

## Account Structure

```rust
pub struct GaslessConfig {
    pub authority: Pubkey,
    pub pending_authority: Option<Pubkey>,
    pub fee_payer: Pubkey,          // Platform fee payer
    pub vcoin_mint: Pubkey,
    pub fee_vault: Pubkey,          // VCoin fee collection
    pub sscre_program: Pubkey,
    pub daily_subsidy_budget: u64,
    pub daily_subsidy_used: u64,
    pub per_user_daily_limit: u64,
    pub sol_fee_per_tx: u64,
    pub vcoin_fee_rate_bps: u16,
    pub max_slippage_bps: u16,      // 500 = 5%
    pub total_vcoin_collected: u64,
    pub total_sessions: u64,
    pub paused: bool,
    pub bump: u8,
}

pub struct SessionKey {
    pub owner: Pubkey,
    pub session_pubkey: Pubkey,
    pub created_at: i64,
    pub expires_at: i64,
    pub allowed_scopes: u8,
    pub max_spend: u64,
    pub max_actions: u16,
    pub vcoin_spent: u64,
    pub actions_used: u16,
    pub fee_method: u8,
    pub is_active: bool,
    pub revoked_at: Option<i64>,
    pub bump: u8,
}

pub struct UserDailyBudget {
    pub user: Pubkey,
    pub date: u32,              // Days since epoch
    pub subsidized_used: u64,
    pub vcoin_spent: u64,
    pub actions_count: u16,
    pub bump: u8,
}
```

## Security Features

- **Session key signature verification** - Cryptographic validation
- **Scoped permissions** - Limited action types
- **Spend limits** - Per-session maximums
- **Action limits** - Per-session maximums
- **Daily budgets** - Per-user daily limits
- **5% slippage protection** - VCoin/SOL conversion cap
- **24-hour max duration** - Sessions auto-expire

## Integration

```typescript
import { ViWoClient, SessionScope, FeeMethod } from "@viwoapp/sdk";

// Create session key (user approves once)
const sessionKeypair = Keypair.generate();
const createTx = await client.gasless.buildCreateSessionTransaction({
  sessionPubkey: sessionKeypair.publicKey,
  scopes: SessionScope.SOCIAL,     // Tip, Vouch, Content, Follow
  maxSpend: parseVCoin("100"),     // Max 100 VCoin
  maxActions: 50,                   // Max 50 actions
  durationHours: 24,
  feeMethod: FeeMethod.VCoinDeduction,
});
await client.sendTransaction(createTx);

// Execute actions using session key (no popups)
const tipTx = await client.gasless.buildSessionActionTransaction({
  sessionKey: sessionKeypair,
  actionType: ActionType.Tip,
  target: creatorWallet,
  amount: parseVCoin("5"),
});
// Sign with session key, not main wallet
tipTx.sign([sessionKeypair]);
await connection.sendTransaction(tipTx);

// Check session validity
const session = await client.gasless.getSession(sessionKeypair.publicKey);
const isValid = client.gasless.isSessionValid(session);

// Revoke session early
const revokeTx = await client.gasless.buildRevokeSessionTransaction(
  sessionKeypair.publicKey
);
```

## Gasless Flow

```
1. User creates session (one wallet popup)
2. Session key stored locally
3. Subsequent actions:
   - Build transaction
   - Sign with session key (no popup)
   - Relay submits with platform fee payer
   - VCoin deducted for fees
4. Session expires or is revoked
```

## Source Code

- [`programs/gasless-protocol/src/`](../../programs/gasless-protocol/src/)
- [`programs/gasless-protocol/src/state/`](../../programs/gasless-protocol/src/state/)

