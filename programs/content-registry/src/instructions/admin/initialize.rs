use anchor_lang::prelude::*;
use crate::contexts::Initialize;

pub fn handler(
    ctx: Context<Initialize>,
    identity_program: Pubkey,
    staking_program: Pubkey,
) -> Result<()> {
    let config = &mut ctx.accounts.registry_config;
    
    config.authority = ctx.accounts.authority.key();
    config.identity_program = identity_program;
    config.staking_program = staking_program;
    config.total_content_count = 0;
    config.active_content_count = 0;
    config.paused = false;
    config.bump = ctx.bumps.registry_config;
    
    msg!("Content registry initialized");
    Ok(())
}

