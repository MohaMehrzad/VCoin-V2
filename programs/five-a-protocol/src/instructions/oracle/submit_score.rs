use anchor_lang::prelude::*;

use crate::contexts::SubmitScore;
use crate::errors::FiveAError;
use crate::events::ScoreUpdated;

/// Submit score update (oracle only)
pub fn handler(
    ctx: Context<SubmitScore>,
    authenticity: u16,
    accuracy: u16,
    agility: u16,
    activity: u16,
    approved: u16,
) -> Result<()> {
    let config = &ctx.accounts.five_a_config;
    require!(!config.paused, FiveAError::ProtocolPaused);
    
    // Validate scores
    require!(authenticity <= 10000, FiveAError::InvalidScore);
    require!(accuracy <= 10000, FiveAError::InvalidScore);
    require!(agility <= 10000, FiveAError::InvalidScore);
    require!(activity <= 10000, FiveAError::InvalidScore);
    require!(approved <= 10000, FiveAError::InvalidScore);
    
    // Verify oracle is registered
    let oracle_key = ctx.accounts.oracle.key();
    let is_oracle = config.oracles[..config.oracle_count as usize]
        .contains(&oracle_key);
    require!(is_oracle, FiveAError::NotOracle);
    
    let clock = Clock::get()?;
    let user_score = &mut ctx.accounts.user_score;
    
    // Initialize if new
    if user_score.user == Pubkey::default() {
        user_score.user = ctx.accounts.user.key();
        let config = &mut ctx.accounts.five_a_config;
        config.total_users = config.total_users.saturating_add(1);
    }
    
    // Update scores
    user_score.authenticity = authenticity;
    user_score.accuracy = accuracy;
    user_score.agility = agility;
    user_score.activity = activity;
    user_score.approved = approved;
    user_score.composite_score = user_score.calculate_composite();
    user_score.last_updated = clock.unix_timestamp;
    user_score.update_count = user_score.update_count.saturating_add(1);
    user_score.bump = ctx.bumps.user_score;
    
    // Update oracle stats
    let oracle_account = &mut ctx.accounts.oracle_account;
    oracle_account.total_submissions = oracle_account.total_submissions.saturating_add(1);
    oracle_account.last_submission = clock.unix_timestamp;
    
    emit!(ScoreUpdated {
        user: user_score.user,
        authenticity,
        accuracy,
        agility,
        activity,
        approved,
        composite: user_score.composite_score,
        timestamp: clock.unix_timestamp,
    });
    
    msg!("Score updated for: {}, composite: {}", user_score.user, user_score.composite_score);
    Ok(())
}

