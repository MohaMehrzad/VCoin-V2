use anchor_lang::prelude::*;
use crate::errors::ContentError;
use crate::state::{UserEnergy, UserRateLimit};

/// Regenerate user energy based on elapsed time
pub fn regenerate_energy(energy: &mut UserEnergy, current_time: i64) -> Result<()> {
    let elapsed_seconds = current_time - energy.last_regen_time;
    if elapsed_seconds > 0 {
        let hours_elapsed = elapsed_seconds as f64 / 3600.0;
        let regen_amount = (hours_elapsed * energy.regen_rate as f64) as u16;
        
        energy.current_energy = energy.current_energy
            .saturating_add(regen_amount)
            .min(energy.max_energy);
        energy.last_regen_time = current_time;
    }
    
    // Reset daily counters if new day
    if current_time >= energy.last_reset + 86400 {
        energy.energy_spent_today = 0;
        energy.energy_refunded_today = 0;
        energy.last_reset = current_time;
    }
    
    Ok(())
}

/// Check and update rate limit for user
pub fn check_and_update_rate_limit(
    rate_limit: &mut UserRateLimit,
    tier: u8,
    current_time: i64,
) -> Result<()> {
    // Reset daily counter if new day
    if current_time >= rate_limit.day_reset_time + 86400 {
        rate_limit.posts_today = 0;
        rate_limit.day_reset_time = current_time;
    }
    
    // Reset hourly counter if new hour
    if current_time >= rate_limit.hour_reset_time + 3600 {
        rate_limit.edits_this_hour = 0;
        rate_limit.hour_reset_time = current_time;
    }
    
    // Check daily cap
    let daily_cap = UserRateLimit::daily_cap_for_tier(tier);
    require!(
        rate_limit.posts_today < daily_cap,
        ContentError::DailyCapExceeded
    );
    
    rate_limit.posts_today += 1;
    rate_limit.last_post_time = current_time;
    
    Ok(())
}

