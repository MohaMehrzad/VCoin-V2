use anchor_lang::prelude::*;
use crate::contexts::EnablePrivateVoting;

pub fn handler(
    ctx: Context<EnablePrivateVoting>,
    encryption_pubkey: Pubkey,
    decryption_committee: [Pubkey; 5],
    committee_size: u8,
    decryption_threshold: u8,
) -> Result<()> {
    let private_config = &mut ctx.accounts.private_voting_config;
    
    private_config.proposal = ctx.accounts.proposal.key();
    private_config.is_enabled = true;
    private_config.encryption_pubkey = encryption_pubkey;
    private_config.decryption_committee = decryption_committee;
    private_config.committee_size = committee_size;
    private_config.decryption_threshold = decryption_threshold;
    private_config.shares_received = 0;
    private_config.reveal_started = false;
    private_config.reveal_completed = false;
    private_config.aggregated_for = 0;
    private_config.aggregated_against = 0;
    private_config.aggregated_abstain = 0;
    private_config.bump = ctx.bumps.private_voting_config;
    
    msg!("Private voting enabled with {}-of-{} threshold", 
        decryption_threshold, committee_size);
    Ok(())
}

