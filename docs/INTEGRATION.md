# Integration Guide

This guide explains how to integrate ViWo Protocol Stack into your Solana application.

## Table of Contents

- [Quick Start](#quick-start)
- [Use Cases](#use-cases)
- [SDK Integration](#sdk-integration)
- [CPI Integration](#cpi-integration)
- [Testing Your Integration](#testing-your-integration)

## Quick Start

### 5-Minute Integration: Query 5A Scores

The simplest integration is querying 5A reputation scores for anti-Sybil protection.

**1. Install the SDK:**

```bash
npm install @viwoapp/sdk @coral-xyz/anchor @solana/web3.js
```

**2. Query a User's Score:**

```typescript
import { ViWoClient } from "@viwoapp/sdk";
import { Connection, PublicKey } from "@solana/web3.js";

const connection = new Connection("https://api.devnet.solana.com");
const client = new ViWoClient({ connection });

// Get 5A score for any wallet
const userWallet = new PublicKey("...");
const score = await client.fivea.getScore(userWallet);

console.log("Composite Score:", score.composite / 100, "%");
console.log("Authenticity:", score.authenticity / 100, "%");
console.log("Activity:", score.activity / 100, "%");

// Check if user is trusted (score > 60%)
if (score.composite >= 6000) {
  console.log("User is trusted");
}
```

**3. Weight Rewards by Reputation:**

```typescript
// Calculate reward multiplier based on 5A score
const multiplier = client.fivea.getRewardMultiplier(score.composite);
const baseReward = 100; // tokens
const adjustedReward = baseReward * multiplier;
```

## Use Cases

### Anti-Sybil for Airdrops

Prevent bots from claiming airdrops by requiring minimum 5A scores:

```typescript
import { ViWoClient, FIVE_A_CONSTANTS } from "@viwoapp/sdk";

async function verifyAirdropEligibility(wallet: PublicKey): Promise<boolean> {
  const score = await client.fivea.getScore(wallet);
  
  // Require 50%+ composite score
  if (score.composite < 5000) {
    return false;
  }
  
  // Check individual dimensions
  if (score.authenticity < 4000) {
    return false; // Low identity verification
  }
  
  if (score.activity < 3000) {
    return false; // Inactive account
  }
  
  return true;
}

// Allocate based on score tier
function getAllocation(score: number): number {
  if (score >= 8000) return 1000;  // High trust: 1000 tokens
  if (score >= 6000) return 500;   // Medium trust: 500 tokens
  if (score >= 5000) return 100;   // Low trust: 100 tokens
  return 0;                         // Below threshold: none
}
```

### Reputation-Weighted Rewards

Distribute rewards proportionally to user reputation:

```typescript
interface UserClaim {
  wallet: PublicKey;
  baseAmount: number;
}

async function calculateWeightedRewards(
  users: UserClaim[]
): Promise<Map<string, number>> {
  const rewards = new Map<string, number>();
  
  for (const user of users) {
    const score = await client.fivea.getScore(user.wallet);
    const multiplier = client.fivea.getRewardMultiplier(score.composite);
    
    // Multiplier ranges from 1.0x to 2.0x based on score
    const finalAmount = user.baseAmount * multiplier;
    rewards.set(user.wallet.toBase58(), finalAmount);
  }
  
  return rewards;
}
```

### Governance Integration

Use veVCoin staking and 5A scores for governance:

```typescript
// Check if user can create proposals (needs 1000+ veVCoin)
const stake = await client.staking.getUserStake(wallet);
const canPropose = stake.veVcoinAmount >= 1000_000_000_000; // 1000 with 9 decimals

// Get voting power (quadratic + 5A boost)
const votingPower = await client.governance.getVotingPower(wallet);
console.log("Voting Power:", votingPower);

// Cast vote on proposal
const voteTx = await client.governance.buildVoteTransaction(proposalId, true);
await client.sendTransaction(voteTx);
```

### Content Quality Gating

Use 5A Accuracy scores for content platforms:

```typescript
// Only show content from users with high accuracy scores
async function filterQualityContent(contentIds: string[]): Promise<string[]> {
  const qualityContent = [];
  
  for (const id of contentIds) {
    const content = await getContent(id);
    const score = await client.fivea.getScore(content.author);
    
    // Require 70%+ accuracy for featured content
    if (score.accuracy >= 7000) {
      qualityContent.push(id);
    }
  }
  
  return qualityContent;
}
```

## SDK Integration

### Installation

```bash
# npm
npm install @viwoapp/sdk

# yarn
yarn add @viwoapp/sdk

# pnpm
pnpm add @viwoapp/sdk
```

### Peer Dependencies

```json
{
  "@coral-xyz/anchor": ">=0.30.0",
  "@solana/web3.js": ">=1.90.0"
}
```

### Client Initialization

```typescript
import { ViWoClient } from "@viwoapp/sdk";
import { Connection, clusterApiUrl } from "@solana/web3.js";

// Read-only client (for queries)
const connection = new Connection(clusterApiUrl("devnet"));
const readClient = new ViWoClient({ connection });

// Full client with wallet (for transactions)
const fullClient = new ViWoClient({
  connection: { endpoint: "https://api.devnet.solana.com" },
  wallet: walletAdapter,
  programIds: {
    // Optional: Override program IDs for different deployments
    vcoinMint: new PublicKey("YOUR_VCOIN_MINT"),
  },
});
```

### Available Modules

| Module | Purpose | Key Methods |
|--------|---------|-------------|
| `client.staking` | VCoin staking | `getPool()`, `getUserStake()`, `buildStakeTransaction()` |
| `client.governance` | Proposals/voting | `getProposals()`, `getVotingPower()`, `buildVoteTransaction()` |
| `client.rewards` | SSCRE rewards | `getStats()`, `getUserClaim()`, `buildClaimTransaction()` |
| `client.fivea` | 5A reputation | `getScore()`, `getRewardMultiplier()`, `canVouchFor()` |
| `client.identity` | DID/verification | `getIdentity()`, `getVerificationLevel()` |
| `client.vilink` | Action links | `createAction()`, `executeAction()`, `generateUri()` |
| `client.gasless` | Session keys | `createSession()`, `isSessionValid()`, `revokeSession()` |
| `client.content` | Content registry | `getContent()`, `getEnergy()`, `buildCreateTransaction()` |

### Common Patterns

**Error Handling:**

```typescript
try {
  const score = await client.fivea.getScore(wallet);
} catch (error) {
  if (error.message.includes("Account not found")) {
    // User has no 5A score yet
    console.log("User not registered");
  } else {
    throw error;
  }
}
```

**Transaction Building:**

```typescript
// Build transaction
const tx = await client.staking.buildStakeTransaction({
  amount: parseVCoin("1000"),
  lockDuration: LOCK_DURATIONS.threeMonths,
});

// Sign and send
const signature = await client.sendTransaction(tx);
console.log("Transaction:", signature);
```

**PDA Derivation:**

```typescript
// Get PDAs for accounts
const stakingPool = client.pdas.getStakingPool();
const userStake = client.pdas.getUserStake(walletPubkey);
const userScore = client.pdas.getFiveAScore(walletPubkey);
```

## CPI Integration

### Cross-Program Invocation from Rust

For Solana programs that want to call ViWo protocols directly:

**Add Dependency:**

```toml
# Cargo.toml
[dependencies]
five-a-protocol = { path = "../five-a-protocol" }
```

**Query 5A Score:**

```rust
use five_a_protocol::state::UserScore;
use anchor_lang::prelude::*;

pub fn verify_user_score(
    ctx: Context<VerifyScore>,
    min_score: u16,
) -> Result<()> {
    // Get 5A score account (passed as AccountInfo)
    let score_account = &ctx.accounts.user_score;
    
    // Verify PDA derivation
    let (expected_pda, _bump) = Pubkey::find_program_address(
        &[b"user_score", ctx.accounts.user.key().as_ref()],
        &five_a_protocol::ID,
    );
    require!(
        score_account.key() == expected_pda,
        CustomError::InvalidScorePDA
    );
    
    // Deserialize and check score
    let score_data = UserScore::try_deserialize(
        &mut &score_account.data.borrow()[..]
    )?;
    
    require!(
        score_data.composite_score >= min_score,
        CustomError::InsufficientScore
    );
    
    Ok(())
}

#[derive(Accounts)]
pub struct VerifyScore<'info> {
    pub user: Signer<'info>,
    /// CHECK: Verified via PDA derivation
    pub user_score: AccountInfo<'info>,
}
```

**Call Staking Protocol:**

```rust
use staking_protocol::cpi::{accounts::Stake, stake};
use staking_protocol::program::StakingProtocol;

pub fn stake_and_verify(
    ctx: Context<StakeAndVerify>,
    amount: u64,
    lock_duration: i64,
) -> Result<()> {
    // Build CPI context
    let cpi_accounts = Stake {
        user: ctx.accounts.user.to_account_info(),
        pool: ctx.accounts.staking_pool.to_account_info(),
        user_stake: ctx.accounts.user_stake.to_account_info(),
        vcoin_mint: ctx.accounts.vcoin_mint.to_account_info(),
        // ... other accounts
    };
    
    let cpi_program = ctx.accounts.staking_program.to_account_info();
    let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts);
    
    // Execute stake
    stake(cpi_ctx, amount, lock_duration)?;
    
    Ok(())
}
```

### PDA Seeds Reference

| Account | Seeds | Program |
|---------|-------|---------|
| UserScore | `["user_score", user_pubkey]` | five-a-protocol |
| UserStake | `["user_stake", user_pubkey]` | staking-protocol |
| UserVeVCoin | `["user_vevcoin", user_pubkey]` | vevcoin-token |
| Identity | `["identity", user_pubkey]` | identity-protocol |
| VCoinConfig | `["vcoin_config"]` | vcoin-token |
| StakingPool | `["staking_pool"]` | staking-protocol |

## Testing Your Integration

### Devnet Testing

All 11 programs are deployed on Solana Devnet:

| Program | Devnet Address |
|---------|----------------|
| vcoin-token | `Gg1dtrjAfGYi6NLC31WaJjZNBoucvD98rK2h1u9qrUjn` |
| vevcoin-token | `FB39ae9x53FxVL3pER9LqCPEx2TRnEnQP55i838Upnjx` |
| staking-protocol | `6EFcistyr2E81adLUcuBJRr8W2xzpt3D3dFYEcMewpWu` |
| five-a-protocol | `783PbtJw5cc7yatnr9fsvTGSnkKaV6iJe6E8VUPTYrT8` |
| governance-protocol | `3R256kBN9iXozjypQFRAmegBhd6HJqXWqdNG7Th78HYe` |
| sscre-protocol | `6AJNcQSfoiE2UAeUDyJUBumS9SBwhAdSznoAeYpXrxXZ` |
| identity-protocol | `3egAds3pFR5oog6iQCN42KPvgih8HQz2FGybNjiVWixG` |
| content-registry | `MJn1A4MPCBPJGWWuZrtq7bHSo2G289sUwW3ej2wcmLV` |
| vilink-protocol | `CFGXTS2MueQwTYTMMTBQbRWzJtSTC2p4ZRuKPpLDmrv7` |
| gasless-protocol | `FcXJAjzJs8eVY2WTRFXynQBpC7WZUqKZppyp9xS6PaB3` |
| transfer-hook | `9K14FcDRrBeHKD9FPNYeVJaEqJQTac2xspJyb1mM6m48` |

### Unit Testing Example

```typescript
import { describe, it } from "node:test";
import assert from "node:assert";
import { ViWoClient } from "@viwoapp/sdk";
import { Connection, Keypair } from "@solana/web3.js";

describe("5A Integration", () => {
  const connection = new Connection("https://api.devnet.solana.com");
  const client = new ViWoClient({ connection });

  it("should fetch 5A score", async () => {
    const wallet = Keypair.generate().publicKey;
    
    try {
      const score = await client.fivea.getScore(wallet);
      assert(score.composite >= 0);
      assert(score.composite <= 10000);
    } catch (error) {
      // New wallets won't have scores
      assert(error.message.includes("not found"));
    }
  });

  it("should calculate reward multiplier", () => {
    // 80% score = 1.8x multiplier
    const multiplier = client.fivea.getRewardMultiplier(8000);
    assert(multiplier >= 1.0);
    assert(multiplier <= 2.0);
  });
});
```

### BankRun Testing

For faster local testing without RPC:

```typescript
import { startAnchor } from "solana-bankrun";

describe("Local Integration", () => {
  it("should stake VCoin", async () => {
    const context = await startAnchor("./", [], []);
    const provider = context.provider;
    
    // Your test code
  });
});
```

## Support

- **Discord:** [discord.gg/viwoapp](https://discord.gg/viwoapp)
- **GitHub Issues:** For bugs and feature requests
- **Email:** dev@viwoapp.com

---

*See also:*
- [Architecture Documentation](ARCHITECTURE.md)
- [API Reference](API.md)
- [Program Documentation](programs/)

