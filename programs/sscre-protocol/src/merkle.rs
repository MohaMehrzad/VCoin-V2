//! Claim leaves and proofs. Format v1 (unchanged; see `docs/offchain/merkle-leaf-format.md`):
//!
//! * leaf = keccak256("SSCRE_CLAIM_V1" ‖ claimant (32) ‖ amount u64 LE ‖ epoch u64 LE)
//!   — a 62-byte preimage, so it can never collide with a 64-byte internal node;
//! * node = keccak256(min(a, b) ‖ max(a, b)) (sorted pairs, so proofs carry no directions).

use anchor_lang::prelude::Pubkey;
use solana_keccak_hasher::hashv;

pub const LEAF_DOMAIN: &[u8] = b"SSCRE_CLAIM_V1";

pub fn compute_leaf(claimant: &Pubkey, amount: u64, epoch: u64) -> [u8; 32] {
    hashv(&[
        LEAF_DOMAIN,
        claimant.as_ref(),
        &amount.to_le_bytes(),
        &epoch.to_le_bytes(),
    ])
    .to_bytes()
}

pub fn hash_pair(a: &[u8; 32], b: &[u8; 32]) -> [u8; 32] {
    if a <= b {
        hashv(&[a, b]).to_bytes()
    } else {
        hashv(&[b, a]).to_bytes()
    }
}

pub fn verify_proof(proof: &[[u8; 32]], root: &[u8; 32], leaf: &[u8; 32]) -> bool {
    proof
        .iter()
        .fold(*leaf, |node, sibling| hash_pair(&node, sibling))
        == *root
}

/// Reference tree builder for oracles and tests (off-chain only).
#[cfg(not(target_os = "solana"))]
pub mod builder {
    use super::*;

    #[derive(Debug, PartialEq, Eq)]
    pub enum TreeError {
        Empty,
        ZeroAmount(Pubkey),
        DuplicateClaimant(Pubkey),
        /// The leaves sum to more than the epoch's `max_total_claim` (audit SSC-12).
        OverBudget {
            total: u128,
            max_total_claim: u64,
        },
    }

    /// A tree over `(claimant, amount)` for `epoch`. Leaves are kept in the given order.
    pub struct Tree {
        layers: Vec<Vec<[u8; 32]>>,
        pub total: u64,
    }

    impl Tree {
        pub fn new(
            epoch: u64,
            claims: &[(Pubkey, u64)],
            max_total_claim: u64,
        ) -> Result<Self, TreeError> {
            if claims.is_empty() {
                return Err(TreeError::Empty);
            }
            let mut total = 0u128;
            for (i, (who, amount)) in claims.iter().enumerate() {
                if *amount == 0 {
                    return Err(TreeError::ZeroAmount(*who));
                }
                if claims[..i].iter().any(|(w, _)| w == who) {
                    return Err(TreeError::DuplicateClaimant(*who));
                }
                total += *amount as u128;
            }
            if total > max_total_claim as u128 {
                return Err(TreeError::OverBudget {
                    total,
                    max_total_claim,
                });
            }
            let mut layers = vec![claims
                .iter()
                .map(|(w, a)| compute_leaf(w, *a, epoch))
                .collect::<Vec<_>>()];
            while layers.last().unwrap().len() > 1 {
                let next = layers
                    .last()
                    .unwrap()
                    .chunks(2)
                    .map(|c| {
                        if c.len() == 2 {
                            hash_pair(&c[0], &c[1])
                        } else {
                            c[0]
                        }
                    })
                    .collect();
                layers.push(next);
            }
            Ok(Self {
                layers,
                total: total as u64,
            })
        }

        pub fn root(&self) -> [u8; 32] {
            self.layers.last().unwrap()[0]
        }

        /// Proof for the leaf at `index` (an odd node out is promoted without a sibling).
        pub fn proof(&self, mut index: usize) -> Vec<[u8; 32]> {
            let mut proof = Vec::new();
            for layer in &self.layers[..self.layers.len() - 1] {
                let sibling = index ^ 1;
                if sibling < layer.len() {
                    proof.push(layer[sibling]);
                }
                index /= 2;
            }
            proof
        }
    }
}
