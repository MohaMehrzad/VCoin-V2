use anchor_lang::prelude::*;

use crate::constants::{MAX_TRANSFERS_PER_HOUR, WASH_TRADING_COOLDOWN_SECONDS};
use crate::events::ActivityScoreUpdated;
use crate::state::{UserActivity, PairTracking};

/// Update user activity tracking
pub fn update_user_activity(
    activity: &mut UserActivity,
    user: Pubkey,
    amount: u64,
    is_sender: bool,
    current_time: i64,
) -> Result<()> {
    // Initialize if new
    if activity.user == Pubkey::default() {
        activity.user = user;
    }
    
    // Reset hourly counter if needed
    if current_time >= activity.hour_reset_time + 3600 {
        activity.transfers_this_hour = 0;
        activity.hour_reset_time = current_time;
    }
    
    // Update counters
    activity.transfers_this_hour = activity.transfers_this_hour.saturating_add(1);
    activity.last_transfer_time = current_time;
    
    if is_sender {
        activity.total_transfers_sent = activity.total_transfers_sent.saturating_add(1);
        activity.total_amount_sent = activity.total_amount_sent.saturating_add(amount);
    } else {
        activity.total_transfers_received = activity.total_transfers_received.saturating_add(1);
        activity.total_amount_received = activity.total_amount_received.saturating_add(amount);
    }
    
    // Calculate activity score contribution
    // Higher for consistent activity, diminishing for spam
    let base_contribution = if activity.transfers_this_hour <= MAX_TRANSFERS_PER_HOUR {
        100_u16
    } else {
        // Diminishing returns for excessive transfers
        50_u16.saturating_div(activity.transfers_this_hour as u16)
    };
    
    activity.activity_score_contribution = 
        activity.activity_score_contribution.saturating_add(base_contribution);
    
    emit!(ActivityScoreUpdated {
        user,
        new_contribution: activity.activity_score_contribution,
        transfers_this_hour: activity.transfers_this_hour,
    });
    
    Ok(())
}

/// Check for wash trading patterns
pub fn check_wash_trading(
    pair: &mut PairTracking,
    sender: Pubkey,
    receiver: Pubkey,
    amount: u64,
    current_time: i64,
) -> Result<bool> {
    // Initialize if new
    if pair.sender == Pubkey::default() {
        pair.sender = sender;
        pair.receiver = receiver;
        pair.trust_score = 5000; // Start neutral
    }
    
    // Reset daily counter if needed
    if current_time >= pair.day_reset_time + 86400 {
        pair.transfers_24h = 0;
        pair.amount_24h = 0;
        pair.day_reset_time = current_time;
    }
    
    // Check for wash trading pattern
    let time_since_last = current_time - pair.last_transfer_time;
    let is_rapid_transfer = time_since_last < WASH_TRADING_COOLDOWN_SECONDS && 
                            pair.last_transfer_time > 0;
    let is_high_frequency = pair.transfers_24h > 10;
    
    // Update pair tracking
    pair.last_transfer_time = current_time;
    pair.transfers_24h = pair.transfers_24h.saturating_add(1);
    pair.amount_24h = pair.amount_24h.saturating_add(amount);
    
    // Detect wash trading
    let is_wash_trading = is_rapid_transfer && is_high_frequency;
    
    if is_wash_trading {
        pair.wash_flags = pair.wash_flags.saturating_add(1);
        // Decrease trust score
        pair.trust_score = pair.trust_score.saturating_sub(500);
    } else if pair.trust_score < 10000 {
        // Slowly rebuild trust for legitimate activity
        pair.trust_score = pair.trust_score.saturating_add(10);
    }
    
    Ok(is_wash_trading)
}

