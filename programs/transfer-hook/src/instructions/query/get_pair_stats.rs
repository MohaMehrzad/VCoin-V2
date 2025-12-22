use anchor_lang::prelude::*;

use crate::contexts::GetPairStats;

/// Query pair tracking stats
pub fn handler(ctx: Context<GetPairStats>) -> Result<()> {
    let pair = &ctx.accounts.pair_tracking;
    msg!("Sender: {}", pair.sender);
    msg!("Receiver: {}", pair.receiver);
    msg!("Transfers 24h: {}", pair.transfers_24h);
    msg!("Wash flags: {}", pair.wash_flags);
    msg!("Trust score: {}", pair.trust_score);
    Ok(())
}

