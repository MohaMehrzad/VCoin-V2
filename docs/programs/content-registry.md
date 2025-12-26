# Content Registry

On-chain content tracking with energy system.

## Overview

The Content Registry tracks content hashes on-chain for proof of existence and engagement attribution. It uses an energy system to rate-limit content creation while rewarding quality engagement.

**Devnet Address:** `MJn1A4MPCBPJGWWuZrtq7bHSo2G289sUwW3ej2wcmLV`

## Key Features

- **Content hash registration** for proof of existence
- **Energy system** to prevent spam
- **Engagement tracking** for reward attribution
- **1-hour edit window** for corrections
- **Soft delete** (content hidden, not removed)
- **Energy refunds** for high-engagement content

## Energy System

Energy regenerates over time and is spent when creating content:

| Tier | Max Energy | Regen/Hour | Cost/Post |
|------|------------|------------|-----------|
| None | 200 | 20 | 10 |
| Bronze | 500 | 50 | 10 |
| Silver | 800 | 80 | 10 |
| Gold | 1,200 | 120 | 10 |
| Platinum | 2,000 | 200 | 10 |

## Content Types

| Type | Energy Cost | Description |
|------|-------------|-------------|
| Post | 10 | Standard text/media post |
| Comment | 5 | Comment on content |
| Reply | 5 | Reply to comment |
| Share | 3 | Share/repost |

## Instructions

### initialize

Initializes the content registry.

**Authority:** Admin only

### create_content

Register new content on-chain.

**Authority:** User

**Parameters:**
- `content_hash: [u8; 32]` - Hash of content data
- `content_type: u8` - Type of content
- `ipfs_cid: String` - IPFS content identifier

**Effects:**
- Deducts energy from user
- Creates content record
- Starts 1-hour edit window

### edit_content

Update content (within 1-hour window).

**Authority:** Author

**Parameters:**
- `content_id: Pubkey` - Content to edit
- `new_hash: [u8; 32]` - Updated content hash
- `new_ipfs_cid: String` - Updated IPFS CID

### delete_content

Soft delete content (hides from feeds).

**Authority:** Author

### update_engagement

Update engagement metrics (backend oracle).

**Authority:** Engagement oracle

**Parameters:**
- `content_id: Pubkey` - Content account
- `likes: u32` - Like count
- `comments: u32` - Comment count
- `shares: u32` - Share count

### claim_energy_refund

Claim energy refund for high-engagement content.

**Authority:** Author

**Eligibility:** Content with 100+ engagements gets 50% energy refund

## Account Structure

```rust
pub struct ContentConfig {
    pub authority: Pubkey,
    pub engagement_oracle: Pubkey,
    pub total_content: u64,
    pub paused: bool,
    pub bump: u8,
}

pub struct ContentRecord {
    pub author: Pubkey,
    pub content_hash: [u8; 32],
    pub ipfs_cid: [u8; 64],
    pub content_type: u8,
    pub created_at: i64,
    pub edited_at: i64,
    pub edit_window_ends: i64,
    pub deleted: bool,
    pub engagement_count: u32,
    pub energy_refunded: bool,
    pub bump: u8,
}

pub struct UserEnergy {
    pub owner: Pubkey,
    pub current_energy: u32,
    pub max_energy: u32,
    pub regen_per_hour: u32,
    pub last_regen_at: i64,
    pub tier: u8,
    pub bump: u8,
}
```

## Security Features

- **Energy rate limiting** - Prevents spam
- **Hash verification** - Content integrity
- **Edit window** - 1 hour for corrections
- **Soft delete** - Audit trail preserved
- **Oracle authorization** - Only verified oracles update engagement

## Integration

```typescript
import { ViWoClient, ContentType } from "@viwoapp/sdk";

// Get user energy
const energy = await client.content.getEnergy(wallet);
console.log("Current Energy:", energy.current, "/", energy.max);
console.log("Can post:", energy.current >= 10);

// Calculate energy regen
const regenSince = client.content.calculateRegen(energy);
const currentEnergy = energy.current + regenSince;

// Create content
const createTx = await client.content.buildCreateTransaction({
  contentHash: sha256(contentData),
  contentType: ContentType.Post,
  ipfsCid: "QmYwAPJzv...",
});

// Get content record
const content = await client.content.getContent(contentId);
console.log("Engagement:", content.engagementCount);

// Check energy refund eligibility
const eligible = content.engagementCount >= 100 && !content.energyRefunded;
```

## Source Code

- [`programs/content-registry/src/`](../../programs/content-registry/src/)
- [`programs/content-registry/src/state/`](../../programs/content-registry/src/state/)

