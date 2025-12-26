# API Reference

This document provides a high-level API reference for all ViWo Protocol programs.

## Table of Contents

- [Programs Overview](#programs-overview)
- [VCoin Token](#vcoin-token)
- [veVCoin Token](#vevcoin-token)
- [Staking Protocol](#staking-protocol)
- [5A Protocol](#5a-protocol)
- [Governance Protocol](#governance-protocol)
- [SSCRE Protocol](#sscre-protocol)
- [Identity Protocol](#identity-protocol)
- [Content Registry](#content-registry)
- [ViLink Protocol](#vilink-protocol)
- [Gasless Protocol](#gasless-protocol)
- [Transfer Hook](#transfer-hook)
- [Common Error Codes](#common-error-codes)

## Programs Overview

| Program | Devnet Address | Purpose |
|---------|----------------|---------|
| vcoin-token | `Gg1dtrjAfGYi6NLC31WaJjZNBoucvD98rK2h1u9qrUjn` | VCoin Token-2022 |
| vevcoin-token | `FB39ae9x53FxVL3pER9LqCPEx2TRnEnQP55i838Upnjx` | Soulbound veVCoin |
| staking-protocol | `6EFcistyr2E81adLUcuBJRr8W2xzpt3D3dFYEcMewpWu` | VCoin staking |
| five-a-protocol | `783PbtJw5cc7yatnr9fsvTGSnkKaV6iJe6E8VUPTYrT8` | 5A reputation |
| governance-protocol | `3R256kBN9iXozjypQFRAmegBhd6HJqXWqdNG7Th78HYe` | veVCoin governance |
| sscre-protocol | `6AJNcQSfoiE2UAeUDyJUBumS9SBwhAdSznoAeYpXrxXZ` | Sustainable rewards |
| identity-protocol | `3egAds3pFR5oog6iQCN42KPvgih8HQz2FGybNjiVWixG` | Portable DID |
| content-registry | `MJn1A4MPCBPJGWWuZrtq7bHSo2G289sUwW3ej2wcmLV` | Content tracking |
| vilink-protocol | `CFGXTS2MueQwTYTMMTBQbRWzJtSTC2p4ZRuKPpLDmrv7` | Cross-dApp actions |
| gasless-protocol | `FcXJAjzJs8eVY2WTRFXynQBpC7WZUqKZppyp9xS6PaB3` | Session keys |
| transfer-hook | `9K14FcDRrBeHKD9FPNYeVJaEqJQTac2xspJyb1mM6m48` | Transfer automation |

---

## VCoin Token

**Source:** [`programs/vcoin-token/`](../programs/vcoin-token/)

### Instructions

| Instruction | Description | Authority |
|-------------|-------------|-----------|
| `initialize_mint` | Create VCoin mint with Token-2022 extensions | Admin |
| `mint_tokens` | Mint VCoin to destination | Authority |
| `propose_slash` | Propose token slash (governance) | Authority |
| `approve_slash` | Approve slash request | Governance |
| `execute_slash` | Execute approved slash after timelock | Anyone |
| `set_paused` | Emergency pause/unpause | Authority |
| `update_authority` | Propose authority transfer | Authority |
| `accept_authority` | Accept proposed authority | Pending authority |

### Key Accounts

```rust
pub struct VCoinConfig {
    pub authority: Pubkey,
    pub mint: Pubkey,
    pub treasury: Pubkey,
    pub permanent_delegate: Pubkey,
    pub total_minted: u64,
    pub paused: bool,
    pub pending_authority: Option<Pubkey>,
    pub bump: u8,
}
```

---

## veVCoin Token

**Source:** [`programs/vevcoin-token/`](../programs/vevcoin-token/)

### Instructions

| Instruction | Description | Authority |
|-------------|-------------|-----------|
| `initialize_mint` | Create soulbound veVCoin mint | Admin |
| `mint_vevcoin` | Mint veVCoin (staking only) | Staking Protocol |
| `burn_vevcoin` | Burn veVCoin on unstake | Staking Protocol |
| `update_staking_protocol` | Update authorized staking program | Authority |
| `update_authority` | Propose authority transfer | Authority |
| `get_balance` | Query user balance | Anyone |

### Key Accounts

```rust
pub struct UserVeVCoin {
    pub owner: Pubkey,
    pub balance: u64,
    pub first_mint_at: i64,
    pub last_update_at: i64,
    pub bump: u8,
}
```

---

## Staking Protocol

**Source:** [`programs/staking-protocol/`](../programs/staking-protocol/)

### Instructions

| Instruction | Description | Authority |
|-------------|-------------|-----------|
| `initialize_pool` | Create staking pool | Admin |
| `stake` | Stake VCoin with lock duration | User |
| `extend_lock` | Extend lock to increase veVCoin | User |
| `unstake` | Withdraw after lock expires | User |
| `update_tier` | Recalculate user tier | Anyone |
| `set_paused` | Emergency pause | Authority |
| `get_stake_info` | Query stake details | Anyone |

### Key Accounts

```rust
pub struct UserStake {
    pub owner: Pubkey,
    pub staked_amount: u64,
    pub lock_duration: i64,
    pub lock_end: i64,
    pub stake_start: i64,
    pub tier: u8,            // 0=None, 1=Bronze, 2=Silver, 3=Gold, 4=Platinum
    pub ve_vcoin_amount: u64,
    pub bump: u8,
}
```

### Tier Thresholds

| Tier | Minimum Stake | Boost |
|------|---------------|-------|
| Bronze | 1,000 VCoin | 1.1x |
| Silver | 5,000 VCoin | 1.2x |
| Gold | 20,000 VCoin | 1.3x |
| Platinum | 100,000 VCoin | 1.4x |

---

## 5A Protocol

**Source:** [`programs/five-a-protocol/`](../programs/five-a-protocol/)

### Instructions

| Instruction | Description | Authority |
|-------------|-------------|-----------|
| `initialize` | Initialize 5A protocol | Admin |
| `register_oracle` | Register score oracle | Authority |
| `submit_score` | Submit user scores (3-of-N) | Oracle |
| `create_snapshot` | Create epoch snapshot | Oracle |
| `vouch_for_user` | Vouch for new user | User (60%+ score) |
| `evaluate_vouch` | Evaluate vouch after 90 days | Anyone |
| `enable_private_score` | Enable private mode | User |
| `get_score` | Query user score | Anyone |

### Key Accounts

```rust
pub struct UserScore {
    pub user: Pubkey,
    pub authenticity: u16,   // 0-10000 (0-100%)
    pub accuracy: u16,
    pub agility: u16,
    pub activity: u16,
    pub approved: u16,
    pub composite_score: u16,
    pub last_updated: i64,
    pub is_private: bool,
    pub bump: u8,
}
```

### Score Weights

| Dimension | Weight | Question |
|-----------|--------|----------|
| Authenticity | 25% | Are you a real person? |
| Accuracy | 20% | Is your content quality? |
| Agility | 15% | Are you responsive? |
| Activity | 25% | Do you show up daily? |
| Approved | 15% | Does the community trust you? |

---

## Governance Protocol

**Source:** [`programs/governance-protocol/`](../programs/governance-protocol/)

### Instructions

| Instruction | Description | Authority |
|-------------|-------------|-----------|
| `initialize` | Initialize governance | Admin |
| `create_proposal` | Create new proposal | veVCoin holder |
| `cast_vote` | Cast vote on proposal | veVCoin holder |
| `finalize_proposal` | Determine pass/fail | Anyone |
| `execute_proposal` | Execute after timelock | Anyone |
| `delegate_votes` | Delegate voting power | User |
| `revoke_delegation` | Revoke delegation | User |

### Key Accounts

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
    pub status: u8,          // 0=Active, 1=Passed, 2=Failed, 3=Executed
    pub voting_ends_at: i64,
    pub execution_eta: i64,
    pub executed: bool,
    pub bump: u8,
}
```

### Voting Power Formula

```
base_votes = sqrt(veVCoin_balance)
five_a_boost = 1.0 + (five_a_score / 100)  // 1.0x to 2.0x
tier_multiplier = [1.0, 1.0, 2.0, 5.0, 10.0]
effective_votes = base_votes × five_a_boost × tier_multiplier
```

---

## SSCRE Protocol

**Source:** [`programs/sscre-protocol/`](../programs/sscre-protocol/)

### Instructions

| Instruction | Description | Authority |
|-------------|-------------|-----------|
| `initialize_pool` | Create rewards pool | Admin |
| `start_epoch` | Begin monthly epoch | Authority |
| `update_merkle_root` | Push distribution | Oracle |
| `claim_rewards` | Claim with merkle proof | User |
| `trigger_circuit_breaker` | Emergency stop | Authority |
| `reset_circuit_breaker` | Resume (6h cooldown) | Authority |

### Key Accounts

```rust
pub struct UserClaim {
    pub user: Pubkey,
    pub total_claimed: u64,
    pub last_claim_epoch: u32,
    pub claimed_epochs_bitmap: [u64; 4],     // Epochs 0-255
    pub claimed_epochs_bitmap_ext: [u64; 4], // Epochs 256-511
    pub high_epochs_bitmap: [u64; 8],        // Epochs 512-1023
    pub bump: u8,
}
```

---

## Identity Protocol

**Source:** [`programs/identity-protocol/`](../programs/identity-protocol/)

### Instructions

| Instruction | Description | Authority |
|-------------|-------------|-----------|
| `initialize` | Initialize identity protocol | Admin |
| `create_identity` | Create DID anchor | User |
| `update_verification` | Update verification level | Authority |
| `link_sas_attestation` | Link SAS attestation | User |
| `subscribe` | Subscribe to tier | User |

### Verification Levels

| Level | Name | Requirements |
|-------|------|--------------|
| 0 | None | Wallet only |
| 1 | Basic | Email + phone |
| 2 | KYC | Identity documents |
| 3 | Full | KYC + biometric |
| 4 | Enhanced | Full + UniqueHuman |

---

## Content Registry

**Source:** [`programs/content-registry/`](../programs/content-registry/)

### Instructions

| Instruction | Description | Authority |
|-------------|-------------|-----------|
| `initialize` | Initialize registry | Admin |
| `create_content` | Register content hash | User |
| `edit_content` | Update content (1h window) | Author |
| `delete_content` | Soft delete | Author |
| `update_engagement` | Update engagement count | Backend |
| `claim_energy_refund` | Claim based on engagement | User |

### Energy System

| Tier | Max Energy | Regen/Hour |
|------|------------|------------|
| None | 200 | 20 |
| Bronze | 500 | 50 |
| Silver | 800 | 80 |
| Gold | 1,200 | 120 |
| Platinum | 2,000 | 200 |

---

## ViLink Protocol

**Source:** [`programs/vilink-protocol/`](../programs/vilink-protocol/)

### Instructions

| Instruction | Description | Authority |
|-------------|-------------|-----------|
| `initialize` | Setup ViLink | Admin |
| `create_action` | Create shareable action | User |
| `execute_tip_action` | Execute tip transfer | User |
| `execute_vouch_action` | Execute vouch | User |
| `register_dapp` | Authorize external dApp | Authority |
| `create_batch` | Create action batch | User |

### Action Types

| Type | Value | Description |
|------|-------|-------------|
| Tip | 0 | VCoin tip with 2.5% fee |
| Vouch | 1 | Reputation vouching |
| Follow | 2 | Social following |
| Challenge | 3 | User challenges |
| Stake | 4 | Staking actions |
| ContentReact | 5 | Content reactions |
| Delegate | 6 | Vote delegation |
| Vote | 7 | Governance voting |

---

## Gasless Protocol

**Source:** [`programs/gasless-protocol/`](../programs/gasless-protocol/)

### Instructions

| Instruction | Description | Authority |
|-------------|-------------|-----------|
| `initialize` | Setup gasless infrastructure | Admin |
| `create_session_key` | Create temp signing key | User |
| `execute_session_action` | Execute with session key | Session key |
| `deduct_vcoin_fee` | VCoin-based fee | Backend |
| `revoke_session_key` | Invalidate session | User |

### Session Key Scopes (Bitmap)

| Scope | Bit | Value |
|-------|-----|-------|
| Tip | 0 | 0x01 |
| Vouch | 1 | 0x02 |
| Content | 2 | 0x04 |
| Governance | 3 | 0x08 |
| Transfer | 4 | 0x10 |
| Stake | 5 | 0x20 |
| Claim | 6 | 0x40 |
| Follow | 7 | 0x80 |

---

## Transfer Hook

**Source:** [`programs/transfer-hook/`](../programs/transfer-hook/)

### Instructions

| Instruction | Description | Authority |
|-------------|-------------|-----------|
| `initialize` | Create hook config | Admin |
| `execute` | Called on every transfer | Token-2022 |
| `initialize_extra_accounts` | Setup extra accounts | Admin |
| `update_config` | Update hook parameters | Authority |

### Features

- Auto-update 5A Activity scores
- Detect wash trading patterns
- Record tip transactions
- Track engagement trust

---

## Common Error Codes

### Across All Protocols

| Error | Description |
|-------|-------------|
| `Unauthorized` | Signer is not the required authority |
| `Paused` | Protocol is paused |
| `InvalidPDA` | PDA derivation mismatch |
| `Overflow` | Arithmetic overflow |
| `InvalidAmount` | Amount is zero or exceeds limits |

### Authority Transfer

| Error | Description |
|-------|-------------|
| `NoAuthorityTransferPending` | No pending transfer to accept |
| `AuthorityTransferTimelock` | 24-hour timelock not elapsed |

### Staking

| Error | Description |
|-------|-------------|
| `LockNotExpired` | Cannot unstake before lock end |
| `InvalidLockDuration` | Duration outside allowed range |
| `InsufficientStake` | Not enough VCoin staked |

### Governance

| Error | Description |
|-------|-------------|
| `ProposalNotActive` | Voting has ended |
| `InsufficientVotingPower` | Below proposal threshold |
| `AlreadyVoted` | Cannot vote twice |
| `DelegationExpired` | Delegation has expired |

### 5A Protocol

| Error | Description |
|-------|-------------|
| `InsufficientOracleConsensus` | Less than 3 oracles agreed |
| `ScoreUpdateTooFrequent` | 1-hour cooldown not elapsed |
| `InsufficientScoreToVouch` | Need 60%+ score to vouch |

---

## SDK Reference

For TypeScript SDK documentation, see:
- [SDK README](../packages/viwoapp-sdk/README.md)
- [Integration Guide](INTEGRATION.md)

---

*For detailed implementation, see the source code in the respective `programs/` directories.*

