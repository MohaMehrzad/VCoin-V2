# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [2.8.4] - 2025-12-26

### Fixed
- **SDK VCoin Mint Filter:** `getVCoinBalance()` now filters by mint address instead of summing all Token-2022 accounts
- **SDK Gasless Config:** Corrected all byte offsets (96 bytes off) to match on-chain struct
- **SDK Error Logging:** Silent failures replaced with `console.warn()` logging
- **SDK ViLink Batch:** Added `batch_nonce` for deterministic batch PDA derivation

### Changed
- On-chain `user_action_stats.rs`: Added `batch_nonce: u64` field
- ViLink `CreateBatch` context now uses `batch_nonce` instead of timestamp
- SDK version bump to 0.1.8

## [2.8.3] - 2025-12-24

### Fixed
- Slash request PDA validation (request_id must equal timestamp)
- Slash request target account binding constraint
- veVCoin CPI accounts added to `extend_lock`/`update_tier`
- ExtraAccountMetaList population in transfer-hook
- ViLink action PDA changed to deterministic nonce-based derivation

### Changed
- 4 programs upgraded on devnet (vcoin_token, staking_protocol, transfer_hook, vilink_protocol)
- SDK version bump to 0.1.7

## [2.8.2] - 2025-12-24

### Added
- `MERKLE_PROOF_MAX_SIZE` constant (32 levels)
- `MAX_EPOCH_BITMAP` constant (1023)
- `LEGACY_SLASH_DEPRECATED` constant
- `VoteChoice` enum for typed governance voting

### Changed
- SDK version 0.1.1 → 0.1.4
- Updated `SlashRequest` interface with `requestId`
- Updated `UserClaim` interface with bitmap storage

## [2.8.1] - 2025-12-24

### Changed
- All 11 programs upgraded on Solana Devnet with v2.8.0 security fixes
- Upgrade performed at existing addresses (no new program IDs)

## [2.8.0] - 2025-12-24

### Security - Phase 5 (9 issues fixed)

#### Critical
- **C-NEW-01:** Voting power parameters now verified on-chain (reads from accounts instead of caller params)
- **C-NEW-02:** Legacy `slash_tokens` function disabled (must use governance-approved flow)

#### High
- **H-NEW-01:** Authority transfer timelock now actually enforced (24 hours)
- **H-NEW-02:** Merkle proof vector limited to 32 levels (DoS prevention)
- **H-NEW-03:** Delegation amount validated against claimed veVCoin balance
- **H-NEW-04:** High epoch array replaced with bitmap (no overflow for epochs 512+)
- **H-NEW-05:** Proposal threshold now enforced on-chain

#### Medium
- **M-NEW-01:** veVCoin burn precision loss fixed (multiply before divide)
- **M-NEW-02:** Proposer lock documented as future enhancement

### Added
- `user_stake` and `user_score` accounts to `cast_vote` context
- `pending_authority_activated_at` field to config accounts
- 5 new error codes for voting validation

### Changed
- `cast_vote` signature changed (removed 3 parameters) - **BREAKING CHANGE**

## [2.7.2] - 2025-12-23

### Added
- SDK security types: `SlashRequest`, `DecryptionShare`, `PendingScoreUpdate`
- `SECURITY_CONSTANTS` with timelocks and limits
- `VALID_URI_PREFIXES` for proposal validation
- New PDA seeds for security features

### Changed
- All config types now extend `PendingAuthorityFields`
- SDK version 0.1.0 → 0.1.1

## [2.7.1] - 2025-12-23

### Changed
- All 11 programs upgraded on Solana Devnet with Phase 4 security fixes

## [2.7.0] - 2025-12-23

### Security - Phase 4 (8 Low severity issues fixed)

#### L-01: Events
- Added comprehensive events to staking-protocol, vcoin-token, vevcoin-token
- Created `events.rs` files with 10+ event types each

#### L-02: Constants Documentation
- Added governance path documentation to hardcoded constants

#### L-03: Slippage Protection
- Added `MAX_FEE_SLIPPAGE_BPS = 500` (5%) to gasless-protocol
- Added `max_slippage_bps` field to GaslessConfig

#### L-04: URI Validation
- Added `is_valid_uri()` function for proposal descriptions
- Validates ipfs://, https://, ar:// prefixes

#### L-05: dApp Authority Signature
- Changed `dapp_authority` from AccountInfo to Signer in vilink-protocol

#### L-06: Content Deletion Documentation
- Added soft-delete behavior documentation

#### L-07: Oracle Rate Limiting
- Added `MIN_SCORE_UPDATE_INTERVAL = 3600` (1 hour) to five-a-protocol
- Added `ScoreUpdateTooFrequent` error

#### L-08: Already Fixed
- Subscription tier validation already implemented in Phase 3

## [2.6.0] - 2025-12-23

### Security - Phase 3 (7 Medium severity issues fixed)

#### M-01: Reentrancy Guards
- Added `ReentrancyGuard` to `StakingPool`
- Lock/unlock pattern around CPI calls

#### M-02: Fee Bounds
- Added `MAX_PLATFORM_FEE_BPS = 1000` (10%)
- Added `MIN_PLATFORM_FEE_BPS = 10` (0.1%)

#### M-03: Merkle Leaf Domain Separation
- Added `SSCRE_LEAF_DOMAIN = b"SSCRE_CLAIM_V1"`

#### M-04: Wash Trading Prevention
- Added conditional block when `block_wash_trading` enabled

#### M-05: Circuit Breaker Cooldown
- Added `CIRCUIT_BREAKER_COOLDOWN = 21600` (6 hours)

#### M-06: Vault Seed Future-Proofing
- Updated vault PDA to include pool key

#### M-07: Delegation Expiry
- Added expiry validation for delegated votes

## [2.5.0] - 2025-12-23

### Security - Phase 2 (5 High severity issues fixed)

#### H-01: Governance Slashing
- Implemented propose → approve → execute flow
- 48-hour timelock for slash execution
- New `SlashRequest` account type

#### H-02: Two-Step Authority Transfer
- Implemented across all 11 protocols
- 24-hour timelock with explicit acceptance
- New instructions: `propose_authority`, `accept_authority`, `cancel_authority_transfer`

#### H-03: Session Key Verification
- Added `session_signer: Signer<'info>` to ExecuteSessionAction
- Cryptographic verification of session key holder

#### H-04: Epoch Claim Bitmap Extended
- Added `claimed_epochs_bitmap_ext` for epochs 256-511
- Added `high_epochs_claimed` for epochs 512+

#### H-05: Oracle Multi-Consensus
- Implemented 3-of-N oracle consensus for 5A scores
- New `PendingScoreUpdate` account type

### Added
- 34 new files for authority transfer across protocols
- New state types and contexts for security features

## [2.4.0] - 2025-12-23

### Security - Phase 1 (4 Critical issues fixed)

#### C-01: ZK Proof Verification
- Added `ZK_VOTING_ENABLED = false` feature flag
- Private voting blocked until proper ZK implemented

#### C-02: Decryption Share Storage
- Created `DecryptionShare` account type
- Shares now stored on-chain

#### C-03: Vote Aggregation
- Blocked until on-chain computation implemented

#### C-04: veVCoin CPI Integration
- Added full CPI to veVCoin program in staking
- Users now receive actual veVCoin tokens

### Added
- `DecryptionShare` account type (114 bytes)
- veVCoin CPI accounts to stake/unstake contexts

## [2.3.2] - 2025-12-23

### Added
- Published @viwoapp/sdk v0.1.0 to npm
- Comprehensive SDK documentation in README
- 9 SDK modules (Core, Staking, Governance, Rewards, Identity, 5A, Gasless, ViLink, Content)

## [2.3.1] - 2025-12-22

### Added
- Rust integration test infrastructure using solana-program-test
- 55 integration test files across all 11 programs
- Test coverage report generation

## [2.3.0] - 2025-12-22

### Added
- 279 Rust unit tests across all 11 programs
- 98 BankRun integration tests
- Code coverage analysis with Tarpaulin (10.22% source coverage)
- SDK test infrastructure with Jest

### Changed
- Total tests now 377+ (all passing)

## [2.2.0] - 2025-12-22

### Added
- 4-layer testing infrastructure
- Property-based testing with proptest
- Embedded Rust unit tests (not mocks)

## [2.1.0] - 2025-12-22

### Added
- Full Devnet deployment of all 11 programs
- Program IDs updated in all lib.rs files
- Anchor.toml updated with correct addresses

## [2.0.0] - 2025-12-22

### Changed
- **BREAKING:** Complete restructuring to modular architecture
- All 11 programs restructured for better maintainability
- Separated concerns: constants, errors, events, state, contexts, instructions

### Structure
- 8 programs: Full Modular structure
- 3 programs: Streamlined structure (governance, sscre, vilink)

## [1.0.0] - 2025-12-21

### Added
- Initial release with 11 Solana programs
- VCoin Token (Token-2022 with extensions)
- veVCoin Token (Soulbound governance token)
- Staking Protocol (Tier-based with veVCoin minting)
- Transfer Hook (Auto 5A updates, wash trading detection)
- Identity Protocol (DID with SAS integration)
- 5A Reputation Protocol (Oracle-based scoring)
- Content Registry (Energy system)
- Governance Protocol (veVCoin voting with ZK support)
- SSCRE Protocol (Merkle-based rewards)
- ViLink Protocol (Cross-dApp actions)
- Gasless Protocol (Paymaster + Session Keys)
- TypeScript SDK
- 8/11 programs deployed to Devnet

---

[unreleased]: https://github.com/MohaMehrzad/VCoin-V2/compare/v2.8.4...HEAD
[2.8.4]: https://github.com/MohaMehrzad/VCoin-V2/compare/v2.8.3...v2.8.4
[2.8.3]: https://github.com/MohaMehrzad/VCoin-V2/compare/v2.8.2...v2.8.3
[2.8.2]: https://github.com/MohaMehrzad/VCoin-V2/compare/v2.8.1...v2.8.2
[2.8.1]: https://github.com/MohaMehrzad/VCoin-V2/compare/v2.8.0...v2.8.1
[2.8.0]: https://github.com/MohaMehrzad/VCoin-V2/compare/v2.7.2...v2.8.0
[2.7.2]: https://github.com/MohaMehrzad/VCoin-V2/compare/v2.7.1...v2.7.2
[2.7.1]: https://github.com/MohaMehrzad/VCoin-V2/compare/v2.7.0...v2.7.1
[2.7.0]: https://github.com/MohaMehrzad/VCoin-V2/compare/v2.6.0...v2.7.0
[2.6.0]: https://github.com/MohaMehrzad/VCoin-V2/compare/v2.5.0...v2.6.0
[2.5.0]: https://github.com/MohaMehrzad/VCoin-V2/compare/v2.4.0...v2.5.0
[2.4.0]: https://github.com/MohaMehrzad/VCoin-V2/compare/v2.3.2...v2.4.0
[2.3.2]: https://github.com/MohaMehrzad/VCoin-V2/compare/v2.3.1...v2.3.2
[2.3.1]: https://github.com/MohaMehrzad/VCoin-V2/compare/v2.3.0...v2.3.1
[2.3.0]: https://github.com/MohaMehrzad/VCoin-V2/compare/v2.2.0...v2.3.0
[2.2.0]: https://github.com/MohaMehrzad/VCoin-V2/compare/v2.1.0...v2.2.0
[2.1.0]: https://github.com/MohaMehrzad/VCoin-V2/compare/v2.0.0...v2.1.0
[2.0.0]: https://github.com/MohaMehrzad/VCoin-V2/compare/v1.0.0...v2.0.0
[1.0.0]: https://github.com/MohaMehrzad/VCoin-V2/releases/tag/v1.0.0

