use anchor_lang::prelude::*;
use crate::constants::*;
use crate::contexts::Initialize;

pub fn handler(
    ctx: Context<Initialize>,
    staking_program: Pubkey,
    five_a_program: Pubkey,
) -> Result<()> {
    let config = &mut ctx.accounts.governance_config;
    
    config.authority = ctx.accounts.authority.key();
    config.staking_program = staking_program;
    config.five_a_program = five_a_program;
    config.proposal_threshold = DEFAULT_PROPOSAL_THRESHOLD;
    config.quorum = DEFAULT_QUORUM;
    config.voting_period = DEFAULT_VOTING_PERIOD;
    config.timelock_delay = DEFAULT_TIMELOCK_DELAY;
    config.proposal_count = 0;
    config.treasury_balance = 200_000_000 * 1_000_000_000; // 200M VCoin
    config.paused = false;
    config.bump = ctx.bumps.governance_config;
    
    msg!("Governance protocol initialized");
    Ok(())
}

