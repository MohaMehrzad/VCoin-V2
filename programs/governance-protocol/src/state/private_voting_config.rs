use anchor_lang::prelude::*;

/// ZK Private voting configuration (per proposal)
#[account]
#[derive(Default)]
pub struct PrivateVotingConfig {
    /// Proposal
    pub proposal: Pubkey,
    /// Whether private voting is enabled
    pub is_enabled: bool,
    /// Threshold ElGamal encryption public key (Ristretto255 compressed point)
    pub encryption_pubkey: [u8; 32],
    /// Decryption threshold (e.g., 3-of-5)
    pub decryption_threshold: u8,
    /// Decryption committee Solana addresses (max 5)
    pub decryption_committee: [Pubkey; 5],
    /// ElGamal public keys for each committee member (Ristretto points)
    pub committee_elgamal_pubkeys: [[u8; 32]; 5],
    /// Committee size
    pub committee_size: u8,
    /// Decryption shares received
    pub shares_received: u8,
    /// Whether reveal has started
    pub reveal_started: bool,
    /// Whether reveal is complete
    pub reveal_completed: bool,
    /// Aggregated votes for (revealed plaintext)
    pub aggregated_for: u128,
    /// Aggregated votes against
    pub aggregated_against: u128,
    /// Aggregated abstain
    pub aggregated_abstain: u128,
    /// Track which committee members have submitted shares (C-02 fix)
    pub shares_submitted: [bool; 5],
    /// Verification hash: H(for || against || abstain || share_0 || ... || share_n)
    /// Binds claimed vote totals to the actual decryption shares for off-chain audit
    pub verification_hash: [u8; 32],
    /// Accumulated ciphertext sums for homomorphic aggregation (6 Ristretto points)
    pub accumulated_ct_for_r: [u8; 32],
    pub accumulated_ct_for_c: [u8; 32],
    pub accumulated_ct_against_r: [u8; 32],
    pub accumulated_ct_against_c: [u8; 32],
    pub accumulated_ct_abstain_r: [u8; 32],
    pub accumulated_ct_abstain_c: [u8; 32],
    /// Number of private votes cast (bounds BSGS search in tally recovery)
    pub total_private_votes: u32,
    /// PDA bump
    pub bump: u8,
}

impl PrivateVotingConfig {
    pub const LEN: usize = 8 + // discriminator
        32 + // proposal
        1 +  // is_enabled
        32 + // encryption_pubkey ([u8; 32])
        1 +  // decryption_threshold
        (32 * 5) + // decryption_committee
        (32 * 5) + // committee_elgamal_pubkeys
        1 +  // committee_size
        1 +  // shares_received
        1 +  // reveal_started
        1 +  // reveal_completed
        16 + // aggregated_for
        16 + // aggregated_against
        16 + // aggregated_abstain
        5 +  // shares_submitted (C-02 fix)
        32 + // verification_hash
        32 + // accumulated_ct_for_r
        32 + // accumulated_ct_for_c
        32 + // accumulated_ct_against_r
        32 + // accumulated_ct_against_c
        32 + // accumulated_ct_abstain_r
        32 + // accumulated_ct_abstain_c
        4 +  // total_private_votes
        1;   // bump
    // Total: 680 bytes
}
