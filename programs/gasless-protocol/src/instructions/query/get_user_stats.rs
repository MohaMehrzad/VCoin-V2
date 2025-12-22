use anchor_lang::prelude::*;
use crate::contexts::GetUserStats;

pub fn handler(ctx: Context<GetUserStats>) -> Result<()> {
    let stats = &ctx.accounts.user_stats;
    msg!("User: {}", stats.user);
    msg!("Total gasless tx: {}", stats.total_gasless_tx);
    msg!("Total subsidized: {}", stats.total_subsidized);
    msg!("Total VCoin fees: {}", stats.total_vcoin_fees);
    msg!("Sessions created: {}", stats.sessions_created);
    Ok(())
}

