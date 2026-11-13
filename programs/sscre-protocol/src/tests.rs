//! Unit tests: leaf format vector, merkle builder/verifier, parameter bounds, sizes.
//! Instruction behaviour: `integration-tests/tests/sscre.rs`.

use anchor_lang::prelude::*;
use sha3::{Digest, Keccak256};

use crate::constants::*;
use crate::merkle::builder::{Tree, TreeError};
use crate::merkle::*;
use crate::state::*;

/// Frozen test vector shared with the TypeScript SDK and `docs/offchain/merkle-leaf-format.md`.
pub const VECTOR_CLAIMANT: [u8; 32] = [7u8; 32];
pub const VECTOR_AMOUNT: u64 = 1_234_567_890;
pub const VECTOR_EPOCH: u64 = 3;
pub const VECTOR_LEAF_HEX: &str =
    "5867436a6f21c025885194461701203778b53c261bd3f449af326ca8b9cf0083";

fn hex(b: &[u8]) -> String {
    b.iter().map(|x| format!("{x:02x}")).collect()
}

#[test]
fn leaf_matches_an_independent_keccak_of_the_documented_preimage() {
    let claimant = Pubkey::new_from_array(VECTOR_CLAIMANT);
    let mut preimage = Vec::new();
    preimage.extend_from_slice(b"SSCRE_CLAIM_V1");
    preimage.extend_from_slice(&VECTOR_CLAIMANT);
    preimage.extend_from_slice(&VECTOR_AMOUNT.to_le_bytes());
    preimage.extend_from_slice(&VECTOR_EPOCH.to_le_bytes());
    assert_eq!(
        preimage.len(),
        62,
        "never 64 bytes, so a leaf cannot pose as a node"
    );
    let expected: [u8; 32] = Keccak256::digest(&preimage).into();
    let leaf = compute_leaf(&claimant, VECTOR_AMOUNT, VECTOR_EPOCH);
    assert_eq!(leaf, expected);
    assert_eq!(
        hex(&leaf),
        VECTOR_LEAF_HEX,
        "frozen vector changed: update the doc and the TS SDK"
    );
}

#[test]
fn every_leaf_of_a_built_tree_verifies_and_nothing_else_does() {
    for n in [1usize, 2, 3, 5, 8, 13, 64, 100] {
        let claims: Vec<(Pubkey, u64)> = (0..n)
            .map(|i| (Pubkey::new_unique(), (i as u64 + 1) * 1_000))
            .collect();
        let tree = Tree::new(9, &claims, u64::MAX).unwrap();
        let root = tree.root();
        for (i, (who, amount)) in claims.iter().enumerate() {
            let proof = tree.proof(i);
            assert!(
                verify_proof(&proof, &root, &compute_leaf(who, *amount, 9)),
                "n={n} i={i}"
            );
            assert!(!verify_proof(
                &proof,
                &root,
                &compute_leaf(who, amount + 1, 9)
            ));
            assert!(!verify_proof(
                &proof,
                &root,
                &compute_leaf(who, *amount, 10)
            ));
            assert!(!verify_proof(
                &proof,
                &root,
                &compute_leaf(&Pubkey::new_unique(), *amount, 9)
            ));
        }
    }
}

#[test]
fn builder_rejects_trees_the_program_could_not_honour() {
    let a = Pubkey::new_unique();
    assert_eq!(Tree::new(0, &[], 10).err(), Some(TreeError::Empty));
    assert_eq!(
        Tree::new(0, &[(a, 0)], 10).err(),
        Some(TreeError::ZeroAmount(a))
    );
    assert_eq!(
        Tree::new(0, &[(a, 1), (a, 2)], 10).err(),
        Some(TreeError::DuplicateClaimant(a))
    );
    assert_eq!(
        Tree::new(0, &[(a, 6), (Pubkey::new_unique(), 5)], 10).err(),
        Some(TreeError::OverBudget {
            total: 11,
            max_total_claim: 10
        })
    );
    assert_eq!(
        Tree::new(0, &[(a, 6), (Pubkey::new_unique(), 4)], 10)
            .unwrap()
            .total,
        10
    );
}

#[test]
fn params_bounds_and_sizes() {
    assert!(PoolParams::default().is_valid());
    for bad in [
        PoolParams {
            epoch_duration: DAY - 1,
            ..Default::default()
        },
        PoolParams {
            epoch_duration: MAX_EPOCH_DURATION + 1,
            ..Default::default()
        },
        PoolParams {
            claim_window: MIN_CLAIM_WINDOW - 1,
            ..Default::default()
        },
        PoolParams {
            dispute_delay: MIN_DISPUTE_DELAY - 1,
            ..Default::default()
        },
        PoolParams {
            decay_bps: MAX_DECAY_BPS + 1,
            ..Default::default()
        },
    ] {
        assert!(!bad.is_valid(), "{bad:?}");
    }
    let mut buf = Vec::new();
    ClaimStatus {
        epoch: Pubkey::default(),
        claimant: Pubkey::default(),
        amount: 0,
        claimed_at: 0,
        rent_payer: Pubkey::default(),
        bump: 0,
    }
    .serialize(&mut buf)
    .unwrap();
    assert_eq!(ClaimStatus::LEN, 8 + buf.len());
}

#[test]
fn emission_schedule_declines_two_percent_per_epoch() {
    let mut budget: u64 = 10_000_000 * 1_000_000_000;
    let mut total: u128 = 0;
    for _ in 0..120 {
        total += budget as u128;
        budget = (budget as u128 * (BPS - DEFAULT_DECAY_BPS as u64) as u128 / BPS as u128) as u64;
    }
    // 10M × (1 − 0.98^120) / 0.02 ≈ 456M over 10 years: the 350M reserve runs out first,
    // after which epochs pay min(budget, reserve).
    assert!(total > 350_000_000 * 1_000_000_000u128);
}
