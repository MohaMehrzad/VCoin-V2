use anchor_lang::prelude::*;

use crate::contexts::GetScore;

/// Get user score
pub fn handler(ctx: Context<GetScore>) -> Result<()> {
    let score = &ctx.accounts.user_score;
    msg!("User: {}", score.user);
    msg!("Composite: {}", score.composite_score);
    msg!("A1 Authenticity: {}", score.authenticity);
    msg!("A2 Accuracy: {}", score.accuracy);
    msg!("A3 Agility: {}", score.agility);
    msg!("A4 Activity: {}", score.activity);
    msg!("A5 Approved: {}", score.approved);
    Ok(())
}

