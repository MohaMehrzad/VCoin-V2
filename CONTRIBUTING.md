# Contributing to ViWo Protocol Stack

Thank you for your interest in contributing to the ViWo Protocol Stack! This document provides guidelines for contributing to this open-source project.

## Code of Conduct

By participating in this project, you agree to maintain a respectful and inclusive environment. Please:

- Be respectful and constructive in discussions
- Welcome newcomers and help them get started
- Focus on the technical merits of contributions
- Report unacceptable behavior to the maintainers

## Getting Started

### Prerequisites

Ensure you have the following installed:

```bash
# Rust (1.79.0+)
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
rustup default 1.79.0

# Solana CLI (2.0+)
curl --proto '=https' --tlsv1.2 -sSfL https://solana-install.solana.workers.dev | bash

# Anchor CLI (0.32.0)
cargo install --git https://github.com/coral-xyz/anchor avm --locked
avm install 0.32.0
avm use 0.32.0

# Node.js (20+)
# Use nvm or download from nodejs.org

# Yarn
npm install -g yarn
```

### Fork and Clone

1. Fork the repository on GitHub
2. Clone your fork:

```bash
git clone https://github.com/YOUR_USERNAME/VCoin-V2.git
cd VCoin-V2/VCoinContract/vcoin_workspace
```

### Setup Development Environment

```bash
# Install dependencies
yarn install

# Build all programs
anchor build

# Run tests to verify setup
cargo test --workspace
```

## Development Workflow

### Branch Naming

Use descriptive branch names with the following prefixes:

| Prefix | Purpose | Example |
|--------|---------|---------|
| `feat/` | New features | `feat/add-batch-claiming` |
| `fix/` | Bug fixes | `fix/slash-pda-derivation` |
| `docs/` | Documentation | `docs/update-api-reference` |
| `test/` | Test additions | `test/governance-edge-cases` |
| `refactor/` | Code refactoring | `refactor/staking-cpi` |
| `security/` | Security fixes | `security/authority-timelock` |

### Commit Messages

Follow [Conventional Commits](https://www.conventionalcommits.org/):

```
<type>(<scope>): <description>

[optional body]

[optional footer]
```

**Types:**
- `feat`: New feature
- `fix`: Bug fix
- `docs`: Documentation only
- `test`: Adding tests
- `refactor`: Code refactoring
- `security`: Security fix
- `chore`: Maintenance tasks

**Examples:**
```
feat(staking): add extend_lock instruction

fix(governance): correct voting power calculation
- Read veVCoin balance from on-chain state
- Add PDA verification for user_stake account
- Fixes #123

security(vcoin-token): implement two-step authority transfer
BREAKING CHANGE: update_authority now proposes transfer instead of immediate change
```

### Making Changes

1. Create a feature branch from `main`:
   ```bash
   git checkout -b feat/your-feature
   ```

2. Make your changes following the code style guidelines

3. Run tests:
   ```bash
   cargo test --workspace
   ```

4. Format and lint:
   ```bash
   cargo fmt --all
   cargo clippy --all-targets --all-features
   ```

5. Commit your changes with a descriptive message

6. Push to your fork:
   ```bash
   git push origin feat/your-feature
   ```

7. Open a Pull Request on GitHub

## Pull Request Process

### Before Submitting

- [ ] All tests pass (`cargo test --workspace`)
- [ ] Code is formatted (`cargo fmt --all -- --check`)
- [ ] No clippy warnings (`cargo clippy --all-targets`)
- [ ] Documentation updated if needed
- [ ] CHANGELOG.md updated for notable changes

### PR Template

When opening a PR, include:

```markdown
## Description
Brief description of changes

## Type of Change
- [ ] Bug fix
- [ ] New feature
- [ ] Breaking change
- [ ] Documentation update

## Testing
Describe how you tested these changes

## Checklist
- [ ] Tests pass
- [ ] Code formatted
- [ ] Documentation updated
- [ ] CHANGELOG updated (if applicable)
```

### Review Process

1. Maintainers will review within 3-5 business days
2. Address any requested changes
3. Once approved, a maintainer will merge

## Testing Requirements

### Maintain Test Coverage

The project maintains **377+ tests**. New code should include appropriate tests:

| Layer | Location | Command |
|-------|----------|---------|
| Rust Unit Tests | `programs/*/src/tests.rs` | `cargo test --workspace` |
| Rust Integration | `programs/*/tests/` | `cargo test --workspace` |
| BankRun Tests | `tests-bankrun/` | `cd tests-bankrun && npm test` |
| TypeScript E2E | `tests/` | `anchor test` |

### Adding Tests

**Rust Unit Tests:**
```rust
// programs/your-protocol/src/tests.rs
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_your_function() {
        // Arrange
        let input = ...;
        
        // Act
        let result = your_function(input);
        
        // Assert
        assert_eq!(result, expected);
    }
}
```

**BankRun Tests:**
```typescript
// tests-bankrun/tests/your-protocol.test.ts
import { describe, it } from "node:test";
import assert from "node:assert";

describe("YourProtocol", () => {
  it("should do something", async () => {
    // Setup
    // Execute
    // Assert
  });
});
```

## Code Style Guidelines

### Rust Style

Follow the [Rust Style Guide](https://rust-lang.github.io/api-guidelines/):

```rust
// Use descriptive names
pub fn calculate_vevcoin_amount(
    staked_amount: u64,
    lock_duration: i64,
    tier_boost: u64,
) -> Result<u64> {
    // Use checked arithmetic
    let base_amount = staked_amount
        .checked_mul(lock_duration as u64)
        .ok_or(ProtocolError::Overflow)?;
    
    Ok(base_amount)
}

// Document public functions
/// Calculates veVCoin amount based on stake parameters.
/// 
/// # Arguments
/// * `staked_amount` - Amount of VCoin staked
/// * `lock_duration` - Lock duration in seconds
/// * `tier_boost` - Tier multiplier (1000 = 1.0x)
/// 
/// # Returns
/// The calculated veVCoin amount
pub fn calculate_vevcoin_amount(...) -> Result<u64> {
    // ...
}
```

### Modular Structure

Follow the established modular architecture:

```
programs/your-protocol/src/
├── lib.rs              # Entry point
├── constants.rs        # Protocol constants
├── errors.rs           # Error definitions
├── events.rs           # Event definitions
├── tests.rs            # Unit tests
├── state/              # Account structures
│   ├── mod.rs
│   └── *.rs
├── contexts/           # Anchor contexts
│   ├── mod.rs
│   └── *.rs
└── instructions/       # Instruction handlers
    ├── mod.rs
    ├── admin/
    └── user/
```

### TypeScript/SDK Style

```typescript
// Use TypeScript strict mode
// Export types explicitly
export interface StakeParams {
  amount: BN;
  lockDuration: number;
}

// Use async/await
async function getStakingPool(): Promise<StakingPool> {
  const accountInfo = await connection.getAccountInfo(poolPda);
  if (!accountInfo) {
    throw new Error("Pool account not found");
  }
  return parseStakingPool(accountInfo.data);
}

// Document public functions
/**
 * Builds a stake transaction
 * @param params - Stake parameters
 * @returns Transaction ready for signing
 */
async buildStakeTransaction(params: StakeParams): Promise<Transaction> {
  // ...
}
```

## Reporting Issues

### Bug Reports

Include:
- Clear description of the bug
- Steps to reproduce
- Expected vs actual behavior
- Environment (OS, Rust version, Solana CLI version)
- Relevant logs or error messages

### Security Issues

**Do NOT open public issues for security vulnerabilities.**

Email security@viwoapp.com instead. See [SECURITY.md](SECURITY.md) for details.

### Feature Requests

- Describe the feature and its use case
- Explain why it would benefit the project
- Consider implementation complexity

## Documentation

### When to Update Docs

- New features require documentation
- API changes require docs updates
- Bug fixes may need docs clarification

### Documentation Locations

| Type | Location |
|------|----------|
| Main README | `README.md` |
| Architecture | `docs/ARCHITECTURE.md` |
| API Reference | `docs/API.md` |
| Integration Guide | `docs/INTEGRATION.md` |
| Per-Program Docs | `docs/programs/*.md` |
| SDK Docs | `packages/viwoapp-sdk/README.md` |

## Release Process

Releases are managed by maintainers following semantic versioning:

- **Major (X.0.0):** Breaking changes
- **Minor (X.Y.0):** New features, backward compatible
- **Patch (X.Y.Z):** Bug fixes, backward compatible

## Getting Help

- **Discord:** [discord.gg/viwoapp](https://discord.gg/viwoapp)
- **GitHub Discussions:** For general questions
- **GitHub Issues:** For bugs and feature requests

## Recognition

Contributors are recognized in:
- Release notes
- CHANGELOG.md
- README.md contributors section (for significant contributions)

---

Thank you for contributing to the ViWo Protocol Stack! Your contributions help build open-source infrastructure for the Solana ecosystem.

