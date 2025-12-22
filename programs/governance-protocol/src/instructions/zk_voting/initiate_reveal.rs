use anchor_lang::prelude::*;
use crate::contexts::InitiateReveal;
use crate::errors::GovernanceError;

pub fn handler(ctx: Context<InitiateReveal>) -> Result<()> {
    let proposal = &ctx.accounts.proposal;
    let private_config = &mut ctx.accounts.private_voting_config;
    
    let clock = Clock::get()?;
    require!(
        clock.unix_timestamp > proposal.end_time,
        GovernanceError::VotingNotEnded
    );
    require!(!private_config.reveal_started, GovernanceError::RevealAlreadyComplete);
    
    private_config.reveal_started = true;
    
    msg!("ZK reveal initiated for proposal {}", proposal.id);
    Ok(())
}

