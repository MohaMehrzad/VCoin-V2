# Identity Protocol

Portable decentralized identity with SAS integration.

## Overview

The Identity Protocol provides a portable DID (Decentralized Identifier) anchored on Solana. It integrates with the Solana Attestation Service (SAS) for verified credentials and supports subscription tiers for platform features.

**Devnet Address:** `3egAds3pFR5oog6iQCN42KPvgih8HQz2FGybNjiVWixG`

## Key Features

- **Portable DID** anchored on Solana
- **SAS integration** for attestations
- **Verification levels** (0-4)
- **Subscription tiers** for premium features
- **Profile metadata** linking

## Verification Levels

| Level | Name | Requirements | Benefits |
|-------|------|--------------|----------|
| 0 | None | Wallet only | Basic access |
| 1 | Basic | Email + phone | Full features |
| 2 | KYC | Identity documents | Higher limits |
| 3 | Full | KYC + biometric | Max limits |
| 4 | Enhanced | Full + UniqueHuman | Creator status |

## Subscription Tiers

| Tier | Features |
|------|----------|
| Free | Standard access |
| Creator | Monetization features |
| Premium | Priority support, analytics |
| Business | API access, custom integration |

## Instructions

### initialize

Initializes the identity protocol.

**Authority:** Admin only

### create_identity

Creates a DID anchor for a user.

**Authority:** User

**Parameters:**
- `metadata_uri: String` - Profile metadata location (IPFS)

### update_verification

Updates user verification level.

**Authority:** Protocol authority (off-chain verification)

**Parameters:**
- `user: Pubkey` - Target user
- `level: u8` - New verification level (0-4)

### link_sas_attestation

Links a SAS attestation to identity.

**Authority:** User

**Parameters:**
- `attestation_id: Pubkey` - SAS attestation account

### subscribe

Subscribe to a tier.

**Authority:** User

**Parameters:**
- `tier: u8` - Subscription tier
- `duration_months: u8` - Subscription duration

### update_profile

Update profile metadata URI.

**Authority:** User

## Account Structure

```rust
pub struct IdentityConfig {
    pub authority: Pubkey,
    pub sas_program: Pubkey,
    pub vcoin_mint: Pubkey,
    pub subscription_vault: Pubkey,
    pub total_identities: u64,
    pub paused: bool,
    pub bump: u8,
}

pub struct UserIdentity {
    pub owner: Pubkey,
    pub did_uri: [u8; 128],
    pub verification_level: u8,
    pub verified_at: i64,
    pub created_at: i64,
    pub sas_attestations: Vec<Pubkey>,
    pub subscription_tier: u8,
    pub subscription_expires_at: i64,
    pub metadata_uri: [u8; 128],
    pub bump: u8,
}
```

## Security Features

- **SAS attestation verification** - Checks attestation validity
- **Level progression** - Cannot skip verification levels
- **Subscription validation** - Tier features gated properly
- **Two-step authority transfer**

## Integration

```typescript
import { ViWoClient, VerificationLevel } from "@viwoapp/sdk";

// Get user identity
const identity = await client.identity.getIdentity(wallet);
console.log("Level:", identity.verificationLevel);

// Check verification level
const levelName = client.identity.getVerificationLevelName(identity.verificationLevel);

// Get subscription status
const isCreator = identity.subscriptionTier >= 1;
const isExpired = identity.subscriptionExpiresAt < Date.now() / 1000;

// Create identity
const createTx = await client.identity.buildCreateIdentityTransaction({
  metadataUri: "ipfs://...",
});

// Subscribe to tier
const subscribeTx = await client.identity.buildSubscribeTransaction({
  tier: 2,
  durationMonths: 12,
});
```

## SAS Integration

The protocol integrates with Solana Attestation Service for:

- **Email verification** - Email ownership proof
- **Phone verification** - Phone number ownership
- **Identity documents** - Government ID verification
- **Biometric verification** - Liveness check
- **UniqueHuman attestation** - Sybil resistance proof

## Source Code

- [`programs/identity-protocol/src/`](../../programs/identity-protocol/src/)
- [`programs/identity-protocol/src/state/`](../../programs/identity-protocol/src/state/)

