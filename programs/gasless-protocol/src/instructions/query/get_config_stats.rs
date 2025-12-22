use anchor_lang::prelude::*;
use crate::contexts::GetConfigStats;

pub fn handler(ctx: Context<GetConfigStats>) -> Result<()> {
    let config = &ctx.accounts.config;
    msg!("Total subsidized tx: {}", config.total_subsidized_tx);
    msg!("Total SOL spent: {}", config.total_sol_spent);
    msg!("Total VCoin collected: {}", config.total_vcoin_collected);
    msg!("Daily budget: {}", config.daily_subsidy_budget);
    msg!("Today spent: {}", config.day_spent);
    msg!("Paused: {}", config.paused);
    Ok(())
}

