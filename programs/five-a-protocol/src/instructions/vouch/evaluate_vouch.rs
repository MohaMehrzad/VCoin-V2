use anchor_lang::prelude::*;

use crate::constants::{VOUCH_EVALUATION_PERIOD, VOUCH_REWARD};
use crate::contexts::EvaluateVouch;
use crate::errors::FiveAError;
use crate::events::VouchEvaluated;
use crate::state::VouchStatus;

/// Evaluate vouch outcome after 90 days
pub fn handler(ctx: Context<EvaluateVouch>) -> Result<()> {
    let vouch = &mut ctx.accounts.vouch_record;
    
    require!(!vouch.outcome_evaluated, FiveAError::AlreadyEvaluated);
    
    let clock = Clock::get()?;
    let elapsed = clock.unix_timestamp - vouch.vouched_at;
    require!(elapsed >= VOUCH_EVALUATION_PERIOD, FiveAError::EvaluationNotComplete);
    
    // Check vouchee's current score
    let vouchee_score = &ctx.accounts.vouchee_score;
    let is_successful = vouchee_score.composite_score >= 5000; // 50%+
    
    vouch.outcome_evaluated = true;
    
    let voucher_stats = &mut ctx.accounts.voucher_stats;
    voucher_stats.vouches_active = voucher_stats.vouches_active.saturating_sub(1);
    
    if is_successful {
        vouch.status = VouchStatus::Rewarded as u8;
        voucher_stats.successful_vouches = voucher_stats.successful_vouches.saturating_add(1);
        voucher_stats.total_rewards_earned = voucher_stats.total_rewards_earned.saturating_add(VOUCH_REWARD);
        
        emit!(VouchEvaluated {
            voucher: vouch.voucher,
            vouchee: vouch.vouchee,
            success: true,
            reward_or_slash: VOUCH_REWARD,
        });
    } else {
        vouch.status = VouchStatus::Slashed as u8;
        voucher_stats.failed_vouches = voucher_stats.failed_vouches.saturating_add(1);
        voucher_stats.total_stake_lost = voucher_stats.total_stake_lost.saturating_add(vouch.vouch_stake);
        
        emit!(VouchEvaluated {
            voucher: vouch.voucher,
            vouchee: vouch.vouchee,
            success: false,
            reward_or_slash: vouch.vouch_stake,
        });
    }
    
    // Update accuracy
    let total = voucher_stats.successful_vouches + voucher_stats.failed_vouches;
    if total > 0 {
        voucher_stats.vouch_accuracy = 
            ((voucher_stats.successful_vouches as u32 * 10000) / total as u32) as u16;
    }
    
    msg!("Vouch evaluated: success={}", is_successful);
    Ok(())
}

