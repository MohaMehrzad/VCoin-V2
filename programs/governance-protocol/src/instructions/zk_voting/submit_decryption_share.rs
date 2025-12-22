use anchor_lang::prelude::*;
use crate::contexts::SubmitDecryptionShare;
use crate::errors::GovernanceError;

/// Submit a decryption share for ZK vote reveal
/// CRITICAL FIX C-02: Now actually stores the share on-chain
pub fn handler(
    ctx: Context<SubmitDecryptionShare>,
    decryption_share: [u8; 32],
    committee_index: u8,
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
    
    // === CRITICAL FIX C-02: Prevent double submission ===
    require!(
        !private_config.shares_submitted[committee_index as usize],
        GovernanceError::DecryptionShareAlreadySubmitted
    );
    // === END FIX ===
    
    // === CRITICAL FIX C-02: Store the share on-chain ===
    let share_account = &mut ctx.accounts.decryption_share;
    share_account.proposal = private_config.proposal;
    share_account.committee_member = committee_member;
    share_account.committee_index = committee_index;
    share_account.share = decryption_share;  // Actually store it!
    share_account.submitted_at = Clock::get()?.unix_timestamp;
    share_account.bump = ctx.bumps.decryption_share;
    // === END FIX ===
    
    // Update tracking in config
    private_config.shares_submitted[committee_index as usize] = true;
    private_config.shares_received = private_config.shares_received.saturating_add(1);
    
    msg!("Decryption share {} of {} stored on-chain", 
        private_config.shares_received, 
        private_config.decryption_threshold);
    Ok(())
}

