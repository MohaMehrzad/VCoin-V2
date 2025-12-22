//! # Trident Fuzz Tests for ViWoApp
//!
//! This module provides fuzz testing capabilities using the Trident framework.
//! Fuzz testing helps discover edge cases and vulnerabilities by automatically
//! generating random but valid inputs.
//!
//! ## Running Fuzz Tests
//!
//! ```bash
//! # Install Trident CLI
//! cargo install trident-cli
//!
//! # Initialize Trident in the workspace
//! trident init
//!
//! # Run fuzz tests
//! trident fuzz run fuzz_staking
//! trident fuzz run fuzz_governance
//! trident fuzz run fuzz_5a
//!
//! # Run with custom iterations
//! trident fuzz run fuzz_staking --iterations 10000
//!
//! # Run with timeout
//! trident fuzz run fuzz_staking --timeout 3600
//! ```
//!
//! ## Fuzz Targets
//!
//! - `fuzz_staking`: Tests staking protocol edge cases
//! - `fuzz_governance`: Tests governance voting and proposals
//! - `fuzz_5a`: Tests 5A score calculations
//!
//! ## Invariants Checked
//!
//! - No arithmetic overflows
//! - State consistency after operations
//! - Access control enforcement
//! - Balance conservation

pub mod fuzz_instructions;
pub mod accounts_snapshots;

