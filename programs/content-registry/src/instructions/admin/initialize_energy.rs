use anchor_lang::prelude::*;
use crate::constants::*;
use crate::contexts::InitializeEnergy;

pub fn handler(ctx: Context<InitializeEnergy>) -> Result<()> {
    let energy_config = &mut ctx.accounts.energy_config;
    
    energy_config.authority = ctx.accounts.authority.key();
    energy_config.base_regen_rate = REGEN_RATE_NONE;
    energy_config.engagement_check_delay = ENGAGEMENT_CHECK_DELAY;
    energy_config.viral_threshold = REFUND_THRESHOLD_1000;
    energy_config.paused = false;
    energy_config.bump = ctx.bumps.energy_config;
    
    msg!("Energy system initialized");
    Ok(())
}

