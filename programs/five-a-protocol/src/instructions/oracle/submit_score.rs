use anchor_lang::prelude::*;

use crate::contexts::SubmitScore;
use crate::constants::{SCORE_UPDATE_EXPIRY, MIN_SCORE_UPDATE_INTERVAL};
use crate::errors::FiveAError;
use crate::events::ScoreUpdated;

/// Submit score update with multi-oracle consensus (H-05 Security Fix)
/// 
/// Flow:
/// 1. First oracle submits -> creates PendingScoreUpdate
/// 2. Subsequent oracles confirm (must match exact scores)
/// 3. When confirmation_count >= required_consensus -> apply to UserScore
/// 4. Reject mismatched scores, expire after 1 hour
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
    
    // Validate scores (max 100.00%)
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
    
    // L-07: Rate limiting check - prevent too frequent updates for same user
    let user_score = &ctx.accounts.user_score;
    if user_score.last_updated > 0 && user_score.user != Pubkey::default() {
        require!(
            clock.unix_timestamp >= user_score.last_updated + MIN_SCORE_UPDATE_INTERVAL,
            FiveAError::ScoreUpdateTooFrequent
        );
    }
    
    let pending_score = &mut ctx.accounts.pending_score;
    let required_consensus = config.required_consensus;
    
    // Check if there's an existing pending update
    let is_new_or_expired = pending_score.user == Pubkey::default() || 
                            clock.unix_timestamp > pending_score.expires_at ||
                            pending_score.is_applied;
    
    if is_new_or_expired {
        // Initialize new pending score update
        pending_score.user = ctx.accounts.user.key();
        pending_score.authenticity = authenticity;
        pending_score.accuracy = accuracy;
        pending_score.agility = agility;
        pending_score.activity = activity;
        pending_score.approved = approved;
        pending_score.confirming_oracles = [Pubkey::default(); 5];
        pending_score.confirmation_count = 1;
        pending_score.confirming_oracles[0] = oracle_key;
        pending_score.initiated_at = clock.unix_timestamp;
        pending_score.expires_at = clock.unix_timestamp + SCORE_UPDATE_EXPIRY;
        pending_score.is_applied = false;
        pending_score.bump = ctx.bumps.pending_score;
        
        msg!("New pending score update initiated by oracle: {}", oracle_key);
    } else {
        // Existing pending update - validate and add confirmation
        require!(!pending_score.is_applied, FiveAError::Overflow);
        require!(clock.unix_timestamp <= pending_score.expires_at, FiveAError::ScoreUpdateExpired);
        require!(!pending_score.has_oracle_submitted(&oracle_key), FiveAError::OracleAlreadySubmitted);
        
        // Scores must match exactly
        require!(
            pending_score.scores_match(authenticity, accuracy, agility, activity, approved),
            FiveAError::ScoreMismatch
        );
        
        // Add this oracle's confirmation
        pending_score.add_confirming_oracle(oracle_key);
        
        msg!("Oracle {} confirmed score update ({}/{})", 
            oracle_key, pending_score.confirmation_count, required_consensus);
    }
    
    // Update oracle stats
    let oracle_account = &mut ctx.accounts.oracle_account;
    oracle_account.total_submissions = oracle_account.total_submissions.saturating_add(1);
    oracle_account.last_submission = clock.unix_timestamp;
    
    // Check if consensus reached - apply score
    if pending_score.confirmation_count >= required_consensus {
        let user_score = &mut ctx.accounts.user_score;
        
        // Initialize user score if new
        if user_score.user == Pubkey::default() {
            user_score.user = ctx.accounts.user.key();
            let config = &mut ctx.accounts.five_a_config;
            config.total_users = config.total_users.saturating_add(1);
        }
        
        // Apply the consensus score
        user_score.authenticity = pending_score.authenticity;
        user_score.accuracy = pending_score.accuracy;
        user_score.agility = pending_score.agility;
        user_score.activity = pending_score.activity;
        user_score.approved = pending_score.approved;
        user_score.composite_score = user_score.calculate_composite();
        user_score.last_updated = clock.unix_timestamp;
        user_score.update_count = user_score.update_count.saturating_add(1);
        user_score.bump = ctx.bumps.user_score;
        
        // Mark pending as applied
        pending_score.is_applied = true;
        
        emit!(ScoreUpdated {
            user: user_score.user,
            authenticity: pending_score.authenticity,
            accuracy: pending_score.accuracy,
            agility: pending_score.agility,
            activity: pending_score.activity,
            approved: pending_score.approved,
            composite: user_score.composite_score,
            timestamp: clock.unix_timestamp,
        });
        
        msg!("Consensus reached! Score applied for: {}, composite: {}", 
            user_score.user, user_score.composite_score);
    }
    
    Ok(())
}

