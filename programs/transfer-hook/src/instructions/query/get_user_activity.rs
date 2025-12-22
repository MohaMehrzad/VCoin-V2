use anchor_lang::prelude::*;

use crate::contexts::GetUserActivity;

/// Query user activity stats
pub fn handler(ctx: Context<GetUserActivity>) -> Result<()> {
    let activity = &ctx.accounts.user_activity;
    msg!("User: {}", activity.user);
    msg!("Total sent: {}", activity.total_transfers_sent);
    msg!("Total received: {}", activity.total_transfers_received);
    msg!("Activity contribution: {}", activity.activity_score_contribution);
    Ok(())
}

