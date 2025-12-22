use anchor_lang::prelude::*;
use crate::contexts::UpdateConfig;

pub fn handler(
    ctx: Context<UpdateConfig>,
    daily_budget: u64,
    max_per_user: u32,
) -> Result<()> {
    let config = &mut ctx.accounts.config;
    
    config.daily_subsidy_budget = daily_budget;
    config.max_subsidized_per_user = max_per_user;
    
    msg!("Daily budget updated: {} lamports, {} max per user",
        daily_budget, max_per_user);
    Ok(())
}

