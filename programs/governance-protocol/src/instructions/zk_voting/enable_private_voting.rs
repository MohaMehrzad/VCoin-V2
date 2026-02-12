use anchor_lang::prelude::*;
use solana_curve25519::ristretto::{validate_ristretto, PodRistrettoPoint};

use crate::contexts::EnablePrivateVoting;
use crate::crypto::constants::RISTRETTO_IDENTITY;
use crate::errors::GovernanceError;

pub fn handler(
    ctx: Context<EnablePrivateVoting>,
    encryption_pubkey: [u8; 32],
    decryption_committee: [Pubkey; 5],
    committee_size: u8,
    decryption_threshold: u8,
    committee_elgamal_pubkeys: [[u8; 32]; 5],
) -> Result<()> {
    // Validate encryption pubkey is a valid Ristretto point
    require!(
        validate_ristretto(&PodRistrettoPoint(encryption_pubkey)),
        GovernanceError::InvalidRistrettoPoint
    );

    // Validate all active committee ElGamal pubkeys are valid Ristretto points
    for i in 0..(committee_size as usize) {
        require!(
            validate_ristretto(&PodRistrettoPoint(committee_elgamal_pubkeys[i])),
            GovernanceError::InvalidRistrettoPoint
        );
    }

    let private_config = &mut ctx.accounts.private_voting_config;

    private_config.proposal = ctx.accounts.proposal.key();
    private_config.is_enabled = true;
    private_config.encryption_pubkey = encryption_pubkey;
    private_config.decryption_committee = decryption_committee;
    private_config.committee_elgamal_pubkeys = committee_elgamal_pubkeys;
    private_config.committee_size = committee_size;
    private_config.decryption_threshold = decryption_threshold;
    private_config.shares_received = 0;
    private_config.reveal_started = false;
    private_config.reveal_completed = false;
    private_config.aggregated_for = 0;
    private_config.aggregated_against = 0;
    private_config.aggregated_abstain = 0;
    private_config.shares_submitted = [false; 5];
    private_config.verification_hash = [0u8; 32];

    // Initialize accumulated ciphertexts to identity (zero point)
    private_config.accumulated_ct_for_r = RISTRETTO_IDENTITY;
    private_config.accumulated_ct_for_c = RISTRETTO_IDENTITY;
    private_config.accumulated_ct_against_r = RISTRETTO_IDENTITY;
    private_config.accumulated_ct_against_c = RISTRETTO_IDENTITY;
    private_config.accumulated_ct_abstain_r = RISTRETTO_IDENTITY;
    private_config.accumulated_ct_abstain_c = RISTRETTO_IDENTITY;
    private_config.total_private_votes = 0;
    private_config.bump = ctx.bumps.private_voting_config;

    msg!("Private voting enabled with {}-of-{} threshold",
        decryption_threshold, committee_size);
    Ok(())
}
