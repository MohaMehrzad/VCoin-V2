# Security Policy

## Supported Versions

The following versions of the ViWo Protocol Stack are currently supported with security updates:

| Version | Supported | Status |
|---------|-----------|--------|
| 2.8.x   | Yes       | Current stable release |
| 2.7.x   | Yes       | Maintenance mode |
| < 2.7   | No        | Unsupported |

## Reporting a Vulnerability

We take the security of the ViWo Protocol Stack seriously. If you believe you have found a security vulnerability, please report it to us responsibly.

### How to Report

**Email:** security@viwoapp.com

**Please include:**
- Description of the vulnerability
- Steps to reproduce
- Potential impact assessment
- Any suggested fixes (optional)

### Response Timeline

| Stage | Timeframe |
|-------|-----------|
| Initial acknowledgment | Within 24 hours |
| Preliminary assessment | Within 72 hours |
| Detailed response | Within 7 days |
| Fix deployment (critical) | Within 14 days |
| Fix deployment (high) | Within 30 days |

### Disclosure Policy

- We request 90 days to address the issue before public disclosure
- We will credit researchers in our security advisories (unless anonymity is requested)
- We do not pursue legal action against good-faith security researchers

## Security Audit Status

### Internal Review Complete (v2.8.4)

All security findings from our comprehensive internal review have been addressed:

| Severity | Issues Found | Fixed | Status |
|----------|--------------|-------|--------|
| **CRITICAL** | 6 | 6 | 100% Complete |
| **HIGH** | 10 | 10 | 100% Complete |
| **MEDIUM** | 9 | 9 | 100% Complete |
| **LOW** | 8 | 8 | 100% Complete |
| **On-Chain Total** | **33** | **33** | **100%** |
| **SDK Issues** | 4 | 4 | 100% Complete |
| **Grand Total** | **37** | **37** | **100%** |

### Critical Issues Fixed

1. **C-01: ZK Proof Verification** - Private voting blocked until proper ZK infrastructure implemented (`ZK_VOTING_ENABLED = false`)
2. **C-02: Decryption Share Storage** - Shares now properly stored on-chain
3. **C-03: Vote Aggregation** - Now blocked until on-chain computation implemented
4. **C-04: veVCoin CPI Integration** - Staking now mints/burns real veVCoin tokens
5. **C-NEW-01: Voting Power Verification** - Now reads from on-chain state
6. **C-NEW-02: Legacy Slash Disabled** - Must use governance-approved flow

### High Severity Issues Fixed

1. **H-01: Governance Slashing** - 48-hour timelock with proposal approval
2. **H-02: Two-Step Authority Transfer** - 24-hour timelock across all 11 protocols
3. **H-03: Session Key Verification** - Cryptographic signature verification added
4. **H-04: Epoch Claim Bitmap** - Extended to support epochs 0-1023 (85+ years)
5. **H-05: Oracle Multi-Consensus** - 3-of-N agreement required for 5A scores
6. **H-NEW-01: Authority Transfer Timelock** - Actual 24-hour enforcement
7. **H-NEW-02: Merkle Proof Size Limit** - Maximum 32 levels (DoS prevention)
8. **H-NEW-03: Delegation Amount Validation** - Prevents excess voting power
9. **H-NEW-04: High Epoch Bitmap** - Replaced array with bitmap (no overflow)
10. **H-NEW-05: Proposal Threshold** - On-chain verification of proposer holdings

### External Audit

**Status:** Recommended before mainnet launch

We recommend engaging a professional security auditor (e.g., Neodyme, OtterSec, Kudelski) for an independent review before mainnet deployment.

## Smart Contract Security Features

### Access Control

| Feature | Implementation |
|---------|----------------|
| Authority Checks | All admin functions require signer verification |
| Two-Step Authority Transfer | 24-hour timelock with explicit acceptance |
| PDA Seeds | Unique seeds prevent account collisions |
| Pausable | Emergency pause available for all protocols |

### Financial Security

| Feature | Implementation |
|---------|----------------|
| Checked Arithmetic | Using `checked_add`, `checked_sub`, etc. |
| Lock Duration Validation | Min 1 week, max 4 years enforced |
| Slippage Protection | 5% maximum on gasless fee calculations |
| Reentrancy Guards | Lock/unlock pattern for CPI calls |
| Circuit Breaker | 6-hour cooldown for SSCRE protocol |

### Governance Security

| Feature | Implementation |
|---------|----------------|
| Governance-Controlled Slashing | 48-hour timelock with approval |
| Proposal Threshold | On-chain verification of veVCoin holdings |
| Delegation Expiry | Time-bounded voting power |
| Quadratic Voting | Anti-whale with 5A boost |
| ZK Voting | Feature-flagged, disabled until ready |

### Oracle Security

| Feature | Implementation |
|---------|----------------|
| Multi-Oracle Consensus | 3-of-N agreement for 5A scores |
| Rate Limiting | 1-hour cooldown between score updates |
| Pending Score State | Consensus tracking before application |

## Security Configuration

### Required Flag States (Production)

```rust
// governance-protocol/src/constants.rs
pub const ZK_VOTING_ENABLED: bool = false;  // Must remain false until ZK ready
```

### Authority Addresses

All protocol authorities should be:
- Multisig wallets (e.g., Squads)
- Hardware wallet backed
- With appropriate threshold (3-of-5 or higher)

### Operator Procedures

1. **Authority transfers** require 24-hour waiting period after proposal
2. **Slashing** requires governance vote + 48-hour timelock
3. **Circuit breaker** requires 6-hour cooldown before reset
4. **Oracle updates** require 3-of-N consensus

## Bug Bounty Program

**Status:** Coming Soon

We are preparing a bug bounty program with the following anticipated structure:

| Severity | Reward Range |
|----------|--------------|
| Critical | $10,000 - $50,000 |
| High | $5,000 - $10,000 |
| Medium | $1,000 - $5,000 |
| Low | $100 - $1,000 |

Details will be announced on our official channels.

## Security Best Practices for Integrators

### SDK Usage

```typescript
// Always specify the VCoin mint for accurate balance queries
const client = new ViWoClient({
  connection: { endpoint: "https://api.mainnet.solana.com" },
  wallet: walletAdapter,
  programIds: {
    vcoinMint: new PublicKey("YOUR_VCOIN_MINT_ADDRESS"),
  },
});

// Handle errors appropriately - SDK now logs warnings
try {
  const balance = await client.getVCoinBalance();
} catch (error) {
  console.error("Balance query failed:", error);
}
```

### Program Integration (CPI)

```rust
// Always verify PDA derivation matches expected seeds
let (expected_pda, bump) = Pubkey::find_program_address(
    &[SEED, user.key().as_ref()],
    program_id
);
require!(account.key() == expected_pda, CustomError::InvalidPDA);

// Validate account ownership
require!(
    account.owner == &expected_program_id,
    CustomError::InvalidOwner
);
```

## Contact

- **Security Issues:** security@viwoapp.com
- **General Questions:** [Discord](https://discord.gg/viwoapp)
- **GitHub:** [github.com/MohaMehrzad/VCoin-V2](https://github.com/MohaMehrzad/VCoin-V2)

---

*Last Updated: December 2025*
*Version: 2.8.4*

