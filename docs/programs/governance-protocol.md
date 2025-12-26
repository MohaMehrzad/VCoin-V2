# Governance Protocol

veVCoin-based governance with quadratic voting and 5A boost.

## Overview

The Governance Protocol enables decentralized decision-making using veVCoin voting power. Features include quadratic voting, 5A score boosts, delegation, and ZK private voting (currently disabled).

**Devnet Address:** `3R256kBN9iXozjypQFRAmegBhd6HJqXWqdNG7Th78HYe`

## Key Features

- **Quadratic voting** (anti-whale)
- **5A score boost** (1.0x - 2.0x)
- **Vote delegation** with expiry
- **Proposal threshold** enforcement
- **Execution timelock**
- **ZK private voting** (feature-flagged)

## Voting Power Formula

```
base_votes = sqrt(veVCoin_balance)
five_a_boost = 1.0 + (five_a_score / 10000)  // 1.0x to 2.0x
tier_multiplier = [1.0, 1.0, 2.0, 5.0, 10.0] // None to Platinum
effective_votes = base_votes × five_a_boost × tier_multiplier

Example (10,000 veVCoin, 80% 5A, Gold tier):
base = sqrt(10,000) = 100
boost = 1.0 + 0.8 = 1.8
tier = 5.0 (Gold)
power = 100 × 1.8 × 5.0 = 900 votes
```

## Governance Tiers

| Tier | veVCoin Required | Capabilities |
|------|------------------|--------------|
| Community | 1+ | Can vote |
| Delegate | 1,000+ | Can create proposals |
| Council | 10,000+ | Can fast-track proposals |

## Proposal Lifecycle

```
Create → Active (voting) → Passed/Failed → Queued → Executed
                                            ↑
                                    48h timelock
```

## Instructions

### initialize

Initializes governance protocol.

**Authority:** Admin only

### create_proposal

Create a new governance proposal.

**Authority:** veVCoin holder (1,000+ threshold)

**Parameters:**
- `title_hash: [u8; 32]` - Title hash
- `description_uri: String` - IPFS/Arweave URI (validated)
- `proposal_type: u8` - Type of proposal
- `voting_duration: i64` - Voting period in seconds

### cast_vote

Cast vote on active proposal.

**Authority:** veVCoin holder

**Parameters:**
- `proposal_id: u64` - Proposal to vote on
- `choice: VoteChoice` - For, Against, or Abstain

**Note:** Voting power is read from on-chain accounts (veVCoin, 5A score, staking tier).

### finalize_proposal

Determine if proposal passed or failed.

**Authority:** Anyone (after voting ends)

### execute_proposal

Execute a passed proposal after timelock.

**Authority:** Anyone (after 48h execution delay)

### delegate_votes

Delegate voting power to another user.

**Authority:** User

**Parameters:**
- `delegate: Pubkey` - Delegate address
- `amount: u64` - veVCoin amount to delegate
- `expires_at: i64` - Delegation expiry

### revoke_delegation

Revoke an existing delegation.

**Authority:** Delegator

## Account Structure

```rust
pub struct Proposal {
    pub id: u64,
    pub proposer: Pubkey,
    pub title_hash: [u8; 32],
    pub description_uri: [u8; 128],
    pub proposal_type: u8,
    pub votes_for: u128,
    pub votes_against: u128,
    pub votes_abstain: u128,
    pub status: u8,
    pub voting_ends_at: i64,
    pub execution_eta: i64,
    pub executed: bool,
    pub bump: u8,
}

pub struct VoteRecord {
    pub voter: Pubkey,
    pub proposal: Pubkey,
    pub vote_weight: u64,
    pub vote_choice: u8,
    pub bump: u8,
}

pub struct Delegation {
    pub delegator: Pubkey,
    pub delegate: Pubkey,
    pub delegated_amount: u64,
    pub expires_at: i64,
    pub revocable: bool,
    pub bump: u8,
}
```

## Security Features

- **On-chain voting power verification** - Reads from stake/5A accounts
- **Proposal threshold enforcement** - Verified against on-chain state
- **Delegation expiry checks** - Expired delegations cannot be used
- **Execution timelock** - 48 hours for passed proposals
- **URI validation** - Only ipfs://, https://, ar:// allowed
- **ZK voting disabled** - `ZK_VOTING_ENABLED = false` until ready

## Integration

```typescript
import { ViWoClient, ProposalStatus } from "@viwoapp/sdk";

// Get active proposals
const proposals = await client.governance.getActiveProposals();

// Get proposal details
const proposal = await client.governance.getProposal(proposalId);
console.log("Status:", client.governance.getStatusText(proposal.status));

// Get proposal progress
const progress = await client.governance.getProposalProgress(proposalId);
console.log("For:", progress.forPercentage, "%");
console.log("Quorum reached:", progress.quorumReached);

// Get voting power
const power = await client.governance.getVotingPower(wallet);

// Cast vote
const voteTx = await client.governance.buildVoteTransaction(proposalId, true);

// Create proposal (1000+ veVCoin required)
const createTx = await client.governance.buildCreateProposalTransaction({
  title: "Increase staking rewards",
  description: "ipfs://...",
  category: 1,
  durationDays: 7,
});
```

## Source Code

- [`programs/governance-protocol/src/`](../../programs/governance-protocol/src/)
- [`programs/governance-protocol/src/state/`](../../programs/governance-protocol/src/state/)

