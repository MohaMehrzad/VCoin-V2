use anchor_lang::prelude::*;

use crate::constants::{MIN_VOUCHER_SCORE, VOUCH_STAKE_AMOUNT, VOUCHES_REQUIRED};
use crate::contexts::VouchForUser;
use crate::errors::FiveAError;
use crate::events::VouchCreated;
use crate::state::{VouchStatus, VoucherStats};

/// Vouch for a new user
pub fn handler(ctx: Context<VouchForUser>) -> Result<()> {
    let config = &ctx.accounts.five_a_config;
    require!(!config.paused, FiveAError::ProtocolPaused);
    
    let voucher_key = ctx.accounts.voucher.key();
    let vouchee_key = ctx.accounts.vouchee.key();
    
    // Cannot vouch for self
    require!(voucher_key != vouchee_key, FiveAError::CannotVouchSelf);
    
    // Check voucher's 5A score
    let voucher_score = &ctx.accounts.voucher_score;
    require!(
        voucher_score.composite_score >= MIN_VOUCHER_SCORE,
        FiveAError::VoucherScoreTooLow
    );
    
    // Check voucher stats
    let voucher_stats = &ctx.accounts.voucher_stats;
    let max_vouches = VoucherStats::max_vouches_for_score(voucher_score.composite_score);
    require!(
        voucher_stats.vouches_active < max_vouches,
        FiveAError::MaxVouchesReached
    );
    
    // Check vouchee status
    let vouchee_status = &ctx.accounts.vouchee_status;
    require!(
        vouchee_status.vouches_received < VOUCHES_REQUIRED,
        FiveAError::AlreadyFullyVouched
    );
    
    // Check not already vouched by this voucher
    for i in 0..vouchee_status.vouches_received as usize {
        require!(
            vouchee_status.vouchers[i] != voucher_key,
            FiveAError::AlreadyVouched
        );
    }
    
    let clock = Clock::get()?;
    
    // Create vouch record
    let vouch = &mut ctx.accounts.vouch_record;
    vouch.voucher = voucher_key;
    vouch.vouchee = vouchee_key;
    vouch.vouched_at = clock.unix_timestamp;
    vouch.vouch_stake = VOUCH_STAKE_AMOUNT;
    vouch.status = VouchStatus::Active as u8;
    vouch.outcome_evaluated = false;
    vouch.bump = ctx.bumps.vouch_record;
    
    // Update vouchee status
    let vouchee_status = &mut ctx.accounts.vouchee_status;
    vouchee_status.user = vouchee_key;
    let vouch_idx = vouchee_status.vouches_received as usize;
    vouchee_status.vouchers[vouch_idx] = voucher_key;
    vouchee_status.vouches_received += 1;
    let multiplier = vouchee_status.get_multiplier();
    vouchee_status.reward_multiplier = multiplier;
    
    if vouchee_status.vouches_received >= VOUCHES_REQUIRED {
        vouchee_status.is_fully_vouched = true;
        vouchee_status.vouch_completed_at = clock.unix_timestamp;
    }
    vouchee_status.bump = ctx.bumps.vouchee_status;
    
    // Update voucher stats
    let voucher_stats = &mut ctx.accounts.voucher_stats;
    voucher_stats.user = voucher_key;
    voucher_stats.total_vouches_given = voucher_stats.total_vouches_given.saturating_add(1);
    voucher_stats.vouches_active = voucher_stats.vouches_active.saturating_add(1);
    voucher_stats.max_concurrent_vouches = max_vouches;
    voucher_stats.bump = ctx.bumps.voucher_stats;
    
    emit!(VouchCreated {
        voucher: voucher_key,
        vouchee: vouchee_key,
        stake: VOUCH_STAKE_AMOUNT,
        timestamp: clock.unix_timestamp,
    });
    
    msg!("Vouch created: {} -> {}", voucher_key, vouchee_key);
    Ok(())
}

