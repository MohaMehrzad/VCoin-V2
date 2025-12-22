use crate::constants::*;

/// M-03 Security Fix: Domain separator for SSCRE merkle leaves
/// Prevents second preimage attacks by adding unique prefix to hash computation
pub const SSCRE_LEAF_DOMAIN: &[u8] = b"SSCRE_CLAIM_V1";

/// Get 5A score multiplier
pub fn get_five_a_multiplier(score: u16) -> u64 {
    if score >= 8000 {
        SCORE_MULT_80_100
    } else if score >= 6000 {
        SCORE_MULT_60_80
    } else if score >= 4000 {
        SCORE_MULT_40_60
    } else if score >= 2000 {
        SCORE_MULT_20_40
    } else {
        SCORE_MULT_0_20
    }
}

/// Compute merkle leaf from user, amount, and epoch
/// M-03 Security Fix: Added domain separation to prevent second preimage attacks
pub fn compute_leaf(user: &anchor_lang::prelude::Pubkey, amount: u64, epoch: u64) -> [u8; 32] {
    use solana_program::keccak;
    
    // M-03: Domain separator + user (32) + amount (8) + epoch (8) = 62 bytes
    let mut data = Vec::with_capacity(62);
    data.extend_from_slice(SSCRE_LEAF_DOMAIN);  // Domain separator for uniqueness
    data.extend_from_slice(user.as_ref());
    data.extend_from_slice(&amount.to_le_bytes());
    data.extend_from_slice(&epoch.to_le_bytes());
    
    keccak::hash(&data).to_bytes()
}

/// Verify merkle proof
pub fn verify_merkle_proof(proof: &[[u8; 32]], root: &[u8; 32], leaf: &[u8; 32]) -> bool {
    use solana_program::keccak;
    
    let mut computed_hash = *leaf;
    
    for proof_element in proof {
        let (left, right) = if computed_hash < *proof_element {
            (computed_hash, *proof_element)
        } else {
            (*proof_element, computed_hash)
        };
        
        let mut combined = [0u8; 64];
        combined[..32].copy_from_slice(&left);
        combined[32..].copy_from_slice(&right);
        
        computed_hash = keccak::hash(&combined).to_bytes();
    }
    
    computed_hash == *root
}

