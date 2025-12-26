# SSCRE Protocol

Self-Sustaining Circular Reward Economy with Merkle-based claims.

## Overview

The SSCRE Protocol ensures rewards never run out through a 6-layer funding hierarchy. Users claim rewards via gas-efficient Merkle proofs.

**Devnet Address:** `6AJNcQSfoiE2UAeUDyJUBumS9SBwhAdSznoAeYpXrxXZ`

## Key Features

- **Merkle tree claims** for gas efficiency
- **5A score-weighted** rewards
- **6-layer funding** hierarchy
- **Circuit breaker** for emergency stops
- **90-day claim window** per epoch

## Sustainability Model

| Phase | Years | Mechanism |
|-------|-------|-----------|
| Emission | 1-5 | 350M reward pool (~5.83M/month) |
| Reserve | 6-10 | ~84M saved reserves |
| Perpetual | 11+ | 250M minting every 5 years |

## 6-Layer Funding Hierarchy

| Layer | Source | Priority |
|-------|--------|----------|
| L0 | Unused Allocation | First |
| L1 | Reserve Fund | |
| L2 | Scheduled Minting | |
| L3 | Buyback Recycling | |
| L4 | Profit Buybacks | |
| L5 | Fee Recycling | Last |

## Instructions

### initialize_pool

Creates the rewards pool with initial allocation.

**Authority:** Admin only

### start_epoch

Begins a new monthly epoch.

**Authority:** Protocol authority

**Parameters:**
- `epoch: u32` - Epoch number
- `total_allocation: u64` - Total rewards for epoch

### update_merkle_root

Oracle pushes the finalized distribution.

**Authority:** Registered oracle

**Parameters:**
- `epoch: u32` - Epoch number
- `merkle_root: [u8; 32]` - Root of claims tree

### claim_rewards

User claims rewards with Merkle proof.

**Authority:** User

**Parameters:**
- `epoch: u32` - Epoch to claim
- `amount: u64` - Claim amount
- `merkle_proof: Vec<[u8; 32]>` - Proof (max 32 levels)

**Fee:** 1% deducted for gasless UX

### trigger_circuit_breaker

Emergency stop for the protocol.

**Authority:** Protocol authority

### reset_circuit_breaker

Resume operations after 6-hour cooldown.

**Authority:** Protocol authority

## Account Structure

```rust
pub struct RewardsPoolConfig {
    pub authority: Pubkey,
    pub vcoin_mint: Pubkey,
    pub pool_vault: Pubkey,
    pub five_a_program: Pubkey,
    pub oracles: [Pubkey; 5],
    pub oracle_count: u8,
    pub current_epoch: u32,
    pub total_distributed: u64,
    pub remaining_reserves: u64,
    pub paused: bool,
    pub circuit_breaker_active: bool,
    pub circuit_breaker_triggered_at: i64,
    pub bump: u8,
}

pub struct EpochDistribution {
    pub epoch: u32,
    pub merkle_root: [u8; 32],
    pub total_allocation: u64,
    pub claims_count: u32,
    pub claimed_amount: u64,
    pub start_timestamp: i64,
    pub end_timestamp: i64,
    pub finalized: bool,
    pub bump: u8,
}

pub struct UserClaim {
    pub user: Pubkey,
    pub total_claimed: u64,
    pub last_claim_epoch: u32,
    pub claimed_epochs_bitmap: [u64; 4],      // Epochs 0-255
    pub claimed_epochs_bitmap_ext: [u64; 4],  // Epochs 256-511
    pub high_epochs_bitmap: [u64; 8],         // Epochs 512-1023
    pub bump: u8,
}
```

## Security Features

- **Merkle proof size limit** - Maximum 32 levels (DoS prevention)
- **Domain separation** - `SSCRE_CLAIM_V1` prefix for leaf hashes
- **Double-claim prevention** - Bitmap tracking for 1024 epochs (85+ years)
- **Circuit breaker** - 6-hour cooldown before reset
- **90-day claim window** - Unclaimed funds return to pool

## Integration

```typescript
import { ViWoClient } from "@viwoapp/sdk";

// Get pool stats
const stats = await client.rewards.getStats();
console.log("Current Epoch:", stats.currentEpoch);
console.log("Total Distributed:", stats.totalDistributed);

// Get user claim history
const claim = await client.rewards.getUserClaim(wallet);
console.log("Total Claimed:", claim.totalClaimed);

// Get unclaimed epochs
const unclaimed = await client.rewards.getUnclaimedEpochs(wallet);

// Claim rewards (1% fee deducted)
const claimTx = await client.rewards.buildClaimTransaction({
  epoch,
  amount,
  merkleProof,
});
```

## Merkle Proof Generation (Off-chain)

```typescript
import { MerkleTree } from "merkletreejs";
import { keccak256 } from "@ethersproject/keccak256";

// Build tree from user allocations
const leaves = allocations.map(a => 
  keccak256(Buffer.concat([
    Buffer.from("SSCRE_CLAIM_V1"),
    a.user.toBuffer(),
    Buffer.from(a.amount.toString(16).padStart(16, "0"), "hex")
  ]))
);

const tree = new MerkleTree(leaves, keccak256, { sortPairs: true });
const root = tree.getRoot();
const proof = tree.getProof(leaves[0]);
```

## Source Code

- [`programs/sscre-protocol/src/`](../../programs/sscre-protocol/src/)
- [`programs/sscre-protocol/src/state/`](../../programs/sscre-protocol/src/state/)

