# 5A Protocol

Anti-Sybil reputation scoring with oracle consensus.

## Overview

The 5A Protocol provides reputation scoring across five dimensions, making bot farming economically irrational. Scores are determined by a multi-oracle consensus mechanism.

**Devnet Address:** `783PbtJw5cc7yatnr9fsvTGSnkKaV6iJe6E8VUPTYrT8`

## Key Features

- **Five-dimensional scoring** (Authenticity, Accuracy, Agility, Activity, Approved)
- **Oracle consensus** (3-of-N required)
- **Vouch system** for new user onboarding
- **Rate limiting** (1-hour cooldown per user)
- **Privacy mode** option

## The Five Stars

| Star | Name | Weight | Question |
|------|------|--------|----------|
| A1 | Authenticity | 25% | Are you a real person? |
| A2 | Accuracy | 20% | Is your content quality? |
| A3 | Agility | 15% | Are you responsive? |
| A4 | Activity | 25% | Do you show up daily? |
| A5 | Approved | 15% | Does the community trust you? |

## Score Calculation

```
composite_score = (authenticity × 25 + accuracy × 20 + agility × 15 
                 + activity × 25 + approved × 15) / 100

Score range: 0-10000 (representing 0-100.00%)
```

## Vouch System

New users can bootstrap their reputation through vouches:

1. Need 3 vouches from users with 60%+ 5A score
2. Vouchers stake 5 VCoin per vouch
3. After 90 days:
   - Successful vouches → voucher earns 10 VCoin bonus
   - Failed vouches (vouchee banned) → voucher loses stake

## Instructions

### initialize

Initializes the 5A protocol.

**Authority:** Admin only

### register_oracle

Registers a score submission oracle.

**Authority:** Protocol authority

### submit_score

Submit user scores (requires 3-of-N consensus).

**Authority:** Registered oracle

**Parameters:**
- `user: Pubkey` - Target user
- `authenticity: u16` - Score (0-10000)
- `accuracy: u16`
- `agility: u16`
- `activity: u16`
- `approved: u16`

### vouch_for_user

Vouch for a new user (stake 5 VCoin).

**Authority:** User with 60%+ score

**Parameters:**
- `vouchee: Pubkey` - User to vouch for

### evaluate_vouch

Evaluate vouch outcome after 90 days.

**Authority:** Anyone

### enable_private_score

Enable private score mode.

**Authority:** User

## Account Structure

```rust
pub struct UserScore {
    pub user: Pubkey,
    pub authenticity: u16,    // 0-10000
    pub accuracy: u16,
    pub agility: u16,
    pub activity: u16,
    pub approved: u16,
    pub composite_score: u16,
    pub last_updated: i64,
    pub is_private: bool,
    pub bump: u8,
}

pub struct PendingScoreUpdate {
    pub user: Pubkey,
    pub oracle_submissions: [Option<Pubkey>; 5],
    pub submission_count: u8,
    pub authenticity_sum: u32,
    pub accuracy_sum: u32,
    pub agility_sum: u32,
    pub activity_sum: u32,
    pub approved_sum: u32,
    pub created_at: i64,
    pub bump: u8,
}
```

## Security Features

- **3-of-N oracle consensus** - No single oracle can manipulate scores
- **Rate limiting** - 1-hour cooldown between updates per user
- **Pending state** - Scores not applied until consensus reached
- **Vouch stake slashing** - Economic consequences for bad vouches

## Integration

```typescript
import { ViWoClient, FIVE_A_CONSTANTS } from "@viwoapp/sdk";

// Get user's 5A score
const score = await client.fivea.getScore(wallet);
console.log("Composite:", score.composite / 100, "%");

// Get score breakdown
const breakdown = client.fivea.getScoreBreakdown(score);
console.log("Authenticity:", breakdown.authenticity, "%");
console.log("Activity:", breakdown.activity, "%");

// Get reward multiplier (1.0x - 2.0x based on score)
const multiplier = client.fivea.getRewardMultiplier(score.composite);

// Check vouch capability
const { canVouch, reason } = await client.fivea.canVouchFor(targetWallet);
```

## Use Cases

- **Airdrop eligibility** - Require minimum 5A score
- **Reward weighting** - Higher scores get more rewards
- **Content quality** - Use Accuracy score for content ranking
- **Trust gating** - Gate features by reputation level

## Source Code

- [`programs/five-a-protocol/src/`](../../programs/five-a-protocol/src/)
- [`programs/five-a-protocol/src/state/`](../../programs/five-a-protocol/src/state/)

