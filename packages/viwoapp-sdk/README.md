# @viwoapp/sdk

[![npm version](https://img.shields.io/npm/v/@viwoapp/sdk.svg)](https://www.npmjs.com/package/@viwoapp/sdk)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

TypeScript SDK for VCoin Protocol Integration on Solana.

**Version:** 2.0.1 (Delegation Balance Validation & 100% Audit Completion)

## What's New in v2.0.1

This release completes the security audit with **100% of all 68 findings resolved** and adds delegation balance validation.

### H-NEW-03: Delegation Balance Validation

The `delegateVotes` instruction now validates the delegation amount against the delegator's actual veVCoin balance from the staking protocol:

```typescript
// Build delegation transaction (validates balance on-chain)
const delegateTx = await client.governance.buildDelegateVotesTransaction({
  delegate: delegatePubkey,
  delegationType: 0,  // Full delegation
  categories: 0xFF,   // All categories
  amount: new BN(5000),
  expiresAt: new BN(0),
  revocable: true,
});

// Revoke delegation
const revokeTx = await client.governance.buildRevokeDelegationTransaction();

// Check existing delegation
const delegation = await client.governance.getDelegation();
```

**On-chain changes:**
- `DelegateVotes` context now requires `config` and `userStake` accounts
- Handler validates `vevcoin_amount <= delegator's actual veVCoin balance`
- Cross-program PDA verification from staking protocol
- Empty staking accounts correctly return balance of 0

### New SDK Features (v2.0.1)

- **`buildDelegateVotesTransaction()`**: Build delegation with on-chain balance validation
- **`buildRevokeDelegationTransaction()`**: Revoke existing delegation
- **`getDelegation()`**: Query delegation state for a user
- **`getDelegation()` PDA helper**: Derives delegation PDA from delegator pubkey
- **`getDelegateStats()` PDA helper**: Derives delegate stats PDA from delegate pubkey
- **`DelegateVotesParams`**: New type for delegation parameters

### Audit Completion

All 68 security findings now resolved:

| Severity | Count | Status |
|----------|-------|--------|
| Critical | 12 | ✅ 100% Fixed |
| High | 8 | ✅ 100% Fixed |
| Medium | 40 | ✅ 100% Fixed |
| Low | 8 | ✅ 100% Fixed |

Key fixes in this release:
- **H-NEW-03**: Delegation amount validated against veVCoin balance
- **M-01**: Voting power precision loss fixed (scale before divide)
- **M-04**: Tier update spam prevention (TierUnchanged check)

### Migration from v2.0.0

```typescript
// Old: delegateVotes only needed 5 accounts
await program.methods.delegateVotes(0, 0, amount, expiresAt, true)
  .accounts({
    delegation, delegateStats, delegator, delegate, systemProgram,
  }).rpc();

// New: delegateVotes requires config and userStake for balance validation
await program.methods.delegateVotes(0, 0, amount, expiresAt, true)
  .accounts({
    delegation, delegateStats, delegator, delegate,
    config: govConfigPda,      // NEW: governance config
    userStake: userStakePda,   // NEW: delegator's staking account
    systemProgram,
  }).rpc();
```

---

## What's New in v2.0.0 (Major Release)

This is a **major release** with production-ready ZK private voting, comprehensive security audit remediation, and breaking changes across all protocol modules.

### ZK Private Voting (Production Ready)

The governance module now supports fully encrypted, verifiable private voting:

- **Twisted ElGamal Encryption** on Ristretto255 with compressed sigma proofs
- **On-chain Vote Verification**: Validates vote proofs without revealing vote content
- **Homomorphic Tallying**: Encrypted votes accumulate without decryption
- **Threshold Decryption**: Committee-based trustless vote reveal with DLEQ proofs
- **Verifiable Results**: Anyone can verify tally correctness using cryptographic proofs

**New ZK Voting API:**

```typescript
import { VoteChoice } from "@viwoapp/sdk";

// Create private vote with encryption
const voteTx = await client.governance.buildCastPrivateVoteTransaction({
  proposalId: new BN(1),
  voteChoice: VoteChoice.For,
  votingPower: new BN(1000),
});

// Submit decryption share (committee members)
const decryptTx = await client.governance.buildSubmitDecryptionShareTransaction({
  proposalId: new BN(1),
  share: decryptionShareData,
  dleqProof: dleqProofData,
});

// Aggregate revealed votes
const aggregateTx = await client.governance.buildAggregateRevealedVotesTransaction({
  proposalId: new BN(1),
});
```

**Off-chain ZK SDK:**
For vote encryption/decryption operations, use the companion Rust crate:
```bash
cd packages/zk-voting-sdk
cargo add zk-voting-sdk
```

### Security Audit Remediation (14 Fixes)

All critical, high, and medium severity findings remediated across 11 programs:

| Finding | Severity | Protocol | Fix |
|---------|----------|----------|-----|
| H-02 | High | All protocols | Two-step authority transfer with 24h timelock |
| H-AUDIT-12 | High | Gasless | Per-user session limit (MAX=5) |
| H-AUDIT-13 | High | Identity | Require verification before DID changes |
| H-NEW-02 | High | SSCRE | Merkle proof size limited to 32 levels |
| C-03 | Critical | Governance | Exclude abstains from quorum calculation |
| C-05/C-06 | Critical | Multiple | Ceiling division for fee calculations |
| C-08 | Critical | 5A | Mutual vouch prevention |
| C-AUDIT-10 | Critical | Content | Monotonic engagement enforcement |
| C-AUDIT-17 | Critical | Identity | SAS attestation required |
| C-AUDIT-22 | Critical | Identity | USDC payment enforcement |
| M-02 | Medium | ViLink | Platform fee bounds (0.1%-10%) |
| M-04 | Medium | ViLink | Nonce-based deterministic PDAs |
| M-05 | Medium | Staking | TierUnchanged error for no-ops |
| M-18 | Medium | 5A | Vouch expiry (MAX_AGE=1yr) |

### Breaking Changes

#### 1. Updated State Account Structures
All protocol state accounts now include security-related fields. Existing accounts must be migrated:

- **All Configs**: Added `pending_authority` and `transfer_initiated_at`
- **Gasless**: Added `max_sessions_per_user` to config
- **Identity**: Added `verification_required_for_did_change` flag
- **SSCRE**: Added `merkle_proof_max_size` constant
- **5A**: Added `max_vouch_age` to config

#### 2. New Governance Instructions
```typescript
// Old: Only public voting
await client.governance.buildVoteTransaction(proposalId, true);

// New: Choose public or private voting
await client.governance.buildVoteTransaction(proposalId, VoteChoice.For); // Public
await client.governance.buildCastPrivateVoteTransaction({ ... });         // Private
```

#### 3. Updated Types & Constants

```typescript
// New VoteChoice enum replaces boolean
export enum VoteChoice {
  Against = 0,
  For = 1,
  Abstain = 2,
}

// New security constants
import { SECURITY_CONSTANTS } from "@viwoapp/sdk";

SECURITY_CONSTANTS.authorityTransferTimelock;    // 86400 (24h)
SECURITY_CONSTANTS.slashApprovalTimelock;        // 172800 (48h)
SECURITY_CONSTANTS.merkleProofMaxSize;           // 32
SECURITY_CONSTANTS.maxSessionsPerUser;           // 5
SECURITY_CONSTANTS.maxVouchAge;                  // 31536000 (1yr)
```

#### 4. Updated Instruction Parameters

Many instructions now require additional parameters for security validation:

```typescript
// Gasless: Session creation now validates session limits
await client.gasless.buildCreateSessionTransaction({
  sessionPubkey,
  scope,
  durationSeconds,
  maxActions,
  feeMethod,
  // SDK now enforces max 5 sessions per user
});

// ViLink: Platform fee must be within bounds
await client.vilink.buildUpdateConfigTransaction({
  platformFeeBps, // Must be between 10 (0.1%) and 1000 (10%)
});
```

### Migration Guide

**Step 1: Update Program Deployments**
All 11 programs have been redeployed with audit fixes. Update your program IDs if using custom deployments.

**Step 2: Migrate State Accounts**
Run migration scripts to add new fields to existing state accounts (contact team for migration tools).

**Step 3: Update SDK**
```bash
npm install @viwoapp/sdk@2.0.1
```

**Step 4: Update Code**
- Replace boolean votes with `VoteChoice` enum
- Handle new error codes (`TierUnchanged`, `MutualVouchPrevented`, etc.)
- Update type imports for new state account fields

### New Features

- **ZK Private Voting**: Full production implementation with threshold decryption
- **Enhanced Security**: 14 audit findings remediated
- **Improved Error Handling**: New error codes for edge cases
- **Better Type Safety**: Expanded type definitions with security fields

## What's New in v0.1.8 (Bug Fixes)

This release addresses 4 SDK issues identified during code review:

| Finding | Severity | Fix |
|---------|----------|-----|
| #2 | Medium | VCoin balance now correctly filters by mint address |
| #5 | Medium | ViLink batch now uses deterministic nonce PDA |
| #8 | High | Gasless config byte offsets corrected |
| #9 | Low | Error handling improved with console warnings |

### Changes

- **VCoin Mint Filter (Finding #2):** `getVCoinBalance()` now filters by VCoin mint address instead of summing all Token-2022 accounts. Set `programIds.vcoinMint` in your config.
- **Gasless Config Fix (Finding #8):** All byte offsets in `gasless.getConfig()` corrected to match on-chain struct after H-02 security fix added `pending_authority`.
- **ViLink Config Fix:** ViLink config byte offsets also corrected for H-02 compatibility.
- **Error Logging (Finding #9):** Silent `catch { return null }` blocks replaced with `console.warn('[ViWoSDK] ...')` for easier debugging.
- **Batch Nonce (Finding #5):** Added `batchNonce` to `UserActionStatsExtended` for deterministic batch PDA derivation.
- **New Fields:** `GaslessConfig` now includes `feeVault`, `sscreProgram`, `sscreDeductionBps`, `maxSubsidizedPerUser`, `totalSolSpent`, `currentDay`, `daySpent`.

### Configuration Update Required

```typescript
const client = new ViWoClient({
  connection: { endpoint: "https://api.devnet.solana.com" },
  wallet: walletAdapter,
  programIds: {
    // IMPORTANT: Set your VCoin mint address for accurate balance queries
    vcoinMint: new PublicKey("YOUR_VCOIN_MINT_ADDRESS"),
  },
});
```

## What's New in v0.1.7

- **ViLink Nonce-Based PDA:** Action PDAs now use deterministic `nonce` instead of `timestamp`
- **New Field:** `actionNonce` added to `ViLinkAction` interface
- **New Type:** `UserActionStatsExtended` with `actionNonce` counter
- **New Method:** `getViLinkActionByNonce()` PDA helper for new derivation
- **New Method:** `getNextNonce()` utility to get next nonce for action creation
- **Updated Methods:** `getAction()`, `isActionValid()`, `buildExecuteTipAction()` now use nonce
- **Deprecated:** `getActionByTimestamp()` - use `getAction()` with nonce instead

### Breaking Change (v0.1.7)

ViLink action PDA derivation changed from timestamp to nonce:
```typescript
// Old (deprecated)
const action = await client.vilink.getAction(creator, timestamp);

// New (v0.1.5+)
const nonce = await client.vilink.getNextNonce(creator);
const action = await client.vilink.getAction(creator, nonce);
```

## What's New in v0.1.4

- **New Constants:** `MERKLE_PROOF_MAX_SIZE`, `MAX_EPOCH_BITMAP`, `LEGACY_SLASH_DEPRECATED`
- **New Enum:** `VoteChoice` for typed governance voting (Against, For, Abstain)
- **Updated Types:** `SlashRequest` (added `requestId`), `UserClaim` (bitmap storage)
- **Updated Docs:** `buildVoteTransaction` - voting power now verified on-chain
- **SECURITY_CONSTANTS:** Added `merkleProofMaxSize`, `maxEpochBitmap`, `votingPowerVerifiedOnChain`

## What's New in v0.1.1

- Added security types for Phase 1-4 fixes
- New types: `SlashRequest`, `DecryptionShare`, `PendingScoreUpdate`
- All config types now support two-step authority transfer
- Added `SECURITY_CONSTANTS` for timelocks and limits
- Added `VALID_URI_PREFIXES` for proposal URI validation
- New PDA seeds: `slashRequest`, `decryptionShare`, `pendingScore`

## Installation

```bash
npm install @viwoapp/sdk
# or
yarn add @viwoapp/sdk
```

## Quick Start

```typescript
import { ViWoClient, parseVCoin, formatVCoin, LOCK_DURATIONS } from "@viwoapp/sdk";

// Initialize client
const client = new ViWoClient({
  connection: { endpoint: "https://api.devnet.solana.com" },
  wallet: walletAdapter, // Your wallet adapter
});

// Get VCoin balance
const balance = await client.getVCoinBalance();
console.log("Balance:", formatVCoin(balance));

// Stake VCoin
const stakeTx = await client.staking.buildStakeTransaction({
  amount: parseVCoin("1000"),
  lockDuration: LOCK_DURATIONS.threeMonths,
});
await client.sendTransaction(stakeTx);
```

## Modules

### Core (`@viwoapp/sdk`)

Connection management, utilities, and PDA derivation.

```typescript
import { ViWoClient, PDAs, formatVCoin, parseVCoin } from "@viwoapp/sdk";

const client = new ViWoClient({ connection, wallet });

// Check connection health
const health = await client.healthCheck();

// Get PDAs
const stakingPool = client.pdas.getStakingPool();
const userStake = client.pdas.getUserStake(walletPubkey);
```

### Staking (`client.staking`)

VCoin staking operations for veVCoin.

```typescript
// Get staking pool info
const pool = await client.staking.getPool();

// Get user stake
const stake = await client.staking.getUserStake();
console.log("Staked:", formatVCoin(stake.stakedAmount));
console.log("Tier:", client.staking.getTierName(stake.tier));

// Calculate veVCoin for stake
const vevcoin = client.staking.calculateVeVCoin(amount, lockDuration);

// Build transactions
const stakeTx = await client.staking.buildStakeTransaction({ amount, lockDuration });
const unstakeTx = await client.staking.buildUnstakeTransaction();
```

### Governance (`client.governance`)

Proposal creation and voting.

```typescript
// Get active proposals
const proposals = await client.governance.getActiveProposals();

// Get proposal details
const proposal = await client.governance.getProposal(proposalId);
const progress = await client.governance.getProposalProgress(proposalId);

// Check voting power
const votingPower = await client.governance.getVotingPower();

// Build transactions
const voteTx = await client.governance.buildVoteTransaction(proposalId, true);

// Delegation (v2.0.1)
const delegateTx = await client.governance.buildDelegateVotesTransaction({
  delegate: delegatePubkey, delegationType: 0, categories: 0xFF,
  amount: new BN(1000), expiresAt: new BN(0), revocable: true,
});
const revokeTx = await client.governance.buildRevokeDelegationTransaction();
```

### Rewards (`client.rewards`)

SSCRE rewards claiming.

```typescript
// Get pool stats
const stats = await client.rewards.getStats();

// Get user claim history
const claims = await client.rewards.getUserClaim();

// Get unclaimed epochs
const unclaimed = await client.rewards.getUnclaimedEpochs();

// Build claim transaction
const claimTx = await client.rewards.buildClaimTransaction({
  epoch,
  amount,
  merkleProof,
});
```

### ViLink (`client.vilink`)

Cross-dApp action deep links.

```typescript
// Create tip action
const tipTx = await client.vilink.buildCreateTipAction({
  target: recipientPubkey,
  amount: parseVCoin("10"),
  expirySeconds: 86400, // 1 day
});

// Generate shareable URI
const uri = client.vilink.generateUri(actionId);
// => viwo://action/abc123...

// Generate QR code data
const qrData = client.vilink.generateQRData(actionId);

// Get next nonce for action creation (v0.1.5+)
const nonce = await client.vilink.getNextNonce();

// Get action by creator + nonce (v0.1.5+)
const action = await client.vilink.getAction(creator, nonce);

// Check action validity (uses nonce, not timestamp)
const { valid, reason } = await client.vilink.isActionValid(creator, nonce);
```

### Gasless (`client.gasless`)

Session keys and gasless transactions.

```typescript
import { ACTION_SCOPES, FeeMethod } from "@viwoapp/sdk";

// Create session key
const sessionKeypair = Keypair.generate();
const scope = ACTION_SCOPES.tip | ACTION_SCOPES.vouch;

const sessionTx = await client.gasless.buildCreateSessionTransaction({
  sessionPubkey: sessionKeypair.publicKey,
  scope,
  durationSeconds: 24 * 3600,
  maxActions: 100,
  feeMethod: FeeMethod.VCoinDeduction,
});

// Check session validity
const { valid } = await client.gasless.isSessionValid(user, sessionPubkey);

// Revoke session
const revokeTx = await client.gasless.buildRevokeSessionTransaction(sessionPubkey);
```

### Identity (`client.identity`)

User identity management.

```typescript
// Get identity
const identity = await client.identity.getIdentity();
console.log("Level:", client.identity.getVerificationLevelName(identity.verificationLevel));

// Get verification requirements
const reqs = client.identity.getVerificationRequirements(level);
```

### 5A Protocol (`client.fivea`)

Reputation scoring.

```typescript
// Get 5A score
const score = await client.fivea.getScore();
console.log("Composite:", client.fivea.formatScore(score.composite));
console.log("Tier:", client.fivea.getScoreTier(score.composite));

// Get score breakdown
const breakdown = client.fivea.getScoreBreakdown(score);

// Get reward multiplier
const multiplier = client.fivea.getRewardMultiplier(score.composite);

// Check vouch capability
const { canVouch, reason } = await client.fivea.canVouchFor(target);
```

### Content (`client.content`)

Content registry operations.

```typescript
// Get user energy
const energy = await client.content.getEnergy();
const currentEnergy = client.content.calculateRegenEnergy(energy);

// Check create capability
const { canCreate } = await client.content.canCreateContent();

// Build transactions
const createTx = await client.content.buildCreateContentTransaction(contentHash);
const editTx = await client.content.buildEditContentTransaction(contentId, newHash);
```

## Constants

```typescript
import {
  PROGRAM_IDS,
  SEEDS,
  VCOIN_DECIMALS,
  STAKING_TIERS,
  LOCK_DURATIONS,
  SSCRE_CONSTANTS,
  VILINK_CONSTANTS,
  GASLESS_CONSTANTS,
  ACTION_SCOPES,
  FIVE_A_CONSTANTS,
  GOVERNANCE_CONSTANTS,
  CONTENT_CONSTANTS,
  // Security constants
  SECURITY_CONSTANTS,
  VALID_URI_PREFIXES,
  MERKLE_CONSTANTS,
  // v0.1.4 additions
  MERKLE_PROOF_MAX_SIZE,    // 32 - prevents DoS
  MAX_EPOCH_BITMAP,         // 1023 - max epoch with bitmap
  LEGACY_SLASH_DEPRECATED,  // true - use propose_slash flow
} from "@viwoapp/sdk";

// Security constants
SECURITY_CONSTANTS.authorityTransferTimelock;    // 24 hours
SECURITY_CONSTANTS.slashApprovalTimelock;        // 48 hours
SECURITY_CONSTANTS.maxFeeSlippageBps;            // 500 (5%)
SECURITY_CONSTANTS.oracleConsensusRequired;      // 3-of-N
SECURITY_CONSTANTS.circuitBreakerCooldown;       // 6 hours
SECURITY_CONSTANTS.merkleProofMaxSize;           // 32 (v0.1.4)
SECURITY_CONSTANTS.maxEpochBitmap;               // 1023 (v0.1.4)
SECURITY_CONSTANTS.votingPowerVerifiedOnChain;   // true (v0.1.4)
```

## Types

```typescript
import type {
  // Staking
  StakingPool,
  UserStake,
  StakingTier,
  StakeParams,
  
  // Governance
  Proposal,
  VoteRecord,
  ProposalStatus,
  VoteChoice,            // v0.1.4: Against, For, Abstain
  GovernanceConfig,
  Delegation,
  DelegateVotesParams,  // v2.0.1: Delegation parameters
  PrivateVotingConfig,
  DecryptionShare,       // v0.1.1: ZK voting
  
  // Rewards
  RewardsPoolConfig,
  EpochDistribution,
  UserClaim,
  ClaimRewardsParams,
  
  // ViLink
  ViLinkConfig,
  ViLinkAction,              // v0.1.5: includes actionNonce
  ActionType,
  CreateActionParams,
  UserActionStatsExtended,   // v0.1.5: includes actionNonce counter
  
  // Gasless
  GaslessConfig,
  SessionKey,
  FeeMethod,
  CreateSessionParams,
  
  // Identity
  Identity,
  IdentityConfig,
  VerificationLevel,
  
  // 5A
  FiveAScore,
  FiveAConfig,
  VouchRecord,
  PendingScoreUpdate,    // v0.1.1: Oracle consensus
  
  // Content
  ContentRecord,
  RegistryConfig,
  UserEnergy,
  ContentState,
  
  // Security (v0.1.1)
  PendingAuthorityFields,  // Two-step authority transfer
  SlashRequest,            // Governance slashing
  SlashStatus,
  HookConfig,              // Transfer hook config
} from "@viwoapp/sdk";
```

## Utilities

```typescript
import {
  formatVCoin,
  parseVCoin,
  getCurrentTimestamp,
  timestampToDate,
  dateToTimestamp,
  TransactionBuilder,
} from "@viwoapp/sdk";

// Format VCoin amount
formatVCoin(new BN(1000000000)); // "1.000000000"

// Parse VCoin string to BN
parseVCoin("100.5"); // BN

// Transaction builder
const builder = new TransactionBuilder();
builder.add(instruction1).add(instruction2);
const tx = builder.build();
```

## License

MIT

