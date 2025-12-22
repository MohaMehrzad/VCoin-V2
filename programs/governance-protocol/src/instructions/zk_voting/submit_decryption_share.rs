use anchor_lang::prelude::*;
use crate::contexts::SubmitDecryptionShare;
use crate::errors::GovernanceError;

pub fn handler(
    ctx: Context<SubmitDecryptionShare>,
    _decryption_share: [u8; 32],
    committee_index: u8,
) -> Result<()> {
    let private_config = &mut ctx.accounts.private_voting_config;
    
    require!(private_config.reveal_started, GovernanceError::RevealNotStarted);
    require!(!private_config.reveal_completed, GovernanceError::RevealAlreadyComplete);
    
    // Verify committee member
    let committee_member = ctx.accounts.committee_member.key();
    require!(
        private_config.decryption_committee[committee_index as usize] == committee_member,
        GovernanceError::Unauthorized
    );
    
    private_config.shares_received = private_config.shares_received.saturating_add(1);
    
    msg!("Decryption share {} of {} received", 
        private_config.shares_received, 
        private_config.decryption_threshold);
    Ok(())
}

