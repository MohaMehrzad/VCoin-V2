use anchor_lang::prelude::*;
use crate::contexts::SubmitDecryptionShare;
use crate::crypto::verify_dleq::verify_batched_dleq;
use crate::errors::GovernanceError;
use crate::events::DecryptionShareVerified;

/// Submit a decryption share for ZK vote reveal with DLEQ proof verification
pub fn handler(
    ctx: Context<SubmitDecryptionShare>,
    committee_index: u8,
    partial_for: [u8; 32],
    partial_against: [u8; 32],
    partial_abstain: [u8; 32],
    dleq_challenge: [u8; 32],
    dleq_response: [u8; 32],
) -> Result<()> {
    let private_config = &mut ctx.accounts.private_voting_config;

    // Validations
    require!(private_config.reveal_started, GovernanceError::RevealNotStarted);
    require!(!private_config.reveal_completed, GovernanceError::RevealAlreadyComplete);

    // Validate committee index is in range
    require!(
        committee_index < private_config.committee_size,
        GovernanceError::InvalidCommitteeIndex
    );

    // Verify committee member matches the registered member for this index
    let committee_member = ctx.accounts.committee_member.key();
    require!(
        private_config.decryption_committee[committee_index as usize] == committee_member,
        GovernanceError::Unauthorized
    );

    // Prevent double submission
    require!(
        !private_config.shares_submitted[committee_index as usize],
        GovernanceError::DecryptionShareAlreadySubmitted
    );

    // =========================================================================
    // Verify DLEQ proof that partial decryptions are correct
    // =========================================================================
    let elgamal_pk = &private_config.committee_elgamal_pubkeys[committee_index as usize];
    let bases = [
        private_config.accumulated_ct_for_r,
        private_config.accumulated_ct_against_r,
        private_config.accumulated_ct_abstain_r,
    ];
    let results = [partial_for, partial_against, partial_abstain];

    verify_batched_dleq(
        elgamal_pk,
        &bases,
        &results,
        &dleq_challenge,
        &dleq_response,
    )?;

    // =========================================================================
    // Store partial decryptions + DLEQ proof in DecryptionShare account
    // =========================================================================
    let share_account = &mut ctx.accounts.decryption_share;
    share_account.proposal = private_config.proposal;
    share_account.committee_member = committee_member;
    share_account.committee_index = committee_index;
    share_account.partial_for = partial_for;
    share_account.partial_against = partial_against;
    share_account.partial_abstain = partial_abstain;
    share_account.dleq_challenge = dleq_challenge;
    share_account.dleq_response = dleq_response;
    share_account.submitted_at = Clock::get()?.unix_timestamp;
    share_account.bump = ctx.bumps.decryption_share;

    // Update tracking in config
    private_config.shares_submitted[committee_index as usize] = true;
    private_config.shares_received = private_config.shares_received.saturating_add(1);

    emit!(DecryptionShareVerified {
        proposal_id: 0, // Proposal ID not stored on PrivateVotingConfig; use event indexing
        committee_member,
        committee_index,
        shares_received: private_config.shares_received,
    });

    msg!("Decryption share {} of {} verified and stored on-chain",
        private_config.shares_received,
        private_config.decryption_threshold);
    Ok(())
}
