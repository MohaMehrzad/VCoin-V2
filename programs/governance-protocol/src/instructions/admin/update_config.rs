use anchor_lang::prelude::*;
use crate::contexts::UpdateConfig;

pub fn handler(
    ctx: Context<UpdateConfig>,
    proposal_threshold: Option<u64>,
    quorum: Option<u64>,
    voting_period: Option<i64>,
    timelock_delay: Option<i64>,
) -> Result<()> {
    let config = &mut ctx.accounts.governance_config;
    
    if let Some(threshold) = proposal_threshold {
        config.proposal_threshold = threshold;
    }
    if let Some(q) = quorum {
        config.quorum = q;
    }
    if let Some(period) = voting_period {
        config.voting_period = period;
    }
    if let Some(delay) = timelock_delay {
        config.timelock_delay = delay;
    }
    
    msg!("Governance config updated");
    Ok(())
}

