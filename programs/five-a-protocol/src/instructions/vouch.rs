//! Vouching: reputation-only (no stake, no reward). An established user (score ≥ 60%)
//! vouches for a new or low-score user; after 90 days an oracle records whether the vouchee
//! reached 50%; after a year an unevaluated vouch expires. Every outcome frees the slot.

use anchor_lang::prelude::*;

use crate::constants::*;
use crate::contexts::*;
use crate::errors::FiveAError;
use crate::events::*;
use crate::state::{VouchOutcome, VoucherStats};

use super::read_score;

pub fn vouch_for_user(ctx: Context<VouchForUser>) -> Result<()> {
    require!(!ctx.accounts.config.paused, FiveAError::Paused);
    let now = Clock::get()?.unix_timestamp;
    let voucher = ctx.accounts.voucher.key();
    let vouchee = ctx.accounts.vouchee.key();
    require_keys_neq!(voucher, vouchee, FiveAError::CannotVouchSelf);
    require!(
        ctx.accounts.reverse_vouch.data_is_empty(),
        FiveAError::MutualVouch
    );

    let voucher_score = ctx.accounts.voucher_score.composite_score;
    require!(
        voucher_score >= MIN_VOUCHER_SCORE,
        FiveAError::VoucherScoreTooLow
    );
    let vouchee_score = read_score(&vouchee, &ctx.accounts.vouchee_score.to_account_info())?
        .map_or(0, |s| s.composite_score);
    require!(
        vouchee_score <= MAX_VOUCHEE_SCORE,
        FiveAError::VoucheeAlreadyEstablished
    );

    let stats = &mut ctx.accounts.voucher_stats;
    if stats.user == Pubkey::default() {
        stats.user = voucher;
        stats.bump = ctx.bumps.voucher_stats;
    }
    require!(
        stats.vouches_active < VoucherStats::max_vouches_for_score(voucher_score),
        FiveAError::MaxVouchesReached
    );
    let status = &mut ctx.accounts.vouchee_status;
    if status.user == Pubkey::default() {
        status.user = vouchee;
        status.bump = ctx.bumps.vouchee_status;
    }
    require!(
        status.vouches_received < MAX_VOUCHES_RECEIVED,
        FiveAError::VoucheeFullyVouched
    );

    stats.vouches_active += 1;
    stats.total_vouches_given = stats
        .total_vouches_given
        .checked_add(1)
        .ok_or(FiveAError::Overflow)?;
    status.vouches_received += 1;
    let record = &mut ctx.accounts.vouch_record;
    record.voucher = voucher;
    record.vouchee = vouchee;
    record.vouched_at = now;
    record.bump = ctx.bumps.vouch_record;
    emit!(VouchCreated {
        voucher,
        vouchee,
        vouched_at: now
    });
    Ok(())
}

fn settle(ctx: &mut CloseVouch, outcome: VouchOutcome, vouchee_score: u16) -> Result<()> {
    let stats = &mut ctx.voucher_stats;
    stats.vouches_active = stats.vouches_active.saturating_sub(1);
    let counter = match outcome {
        VouchOutcome::Succeeded => &mut stats.successful_vouches,
        VouchOutcome::Failed => &mut stats.failed_vouches,
        VouchOutcome::Expired => &mut stats.expired_vouches,
    };
    *counter = counter.checked_add(1).ok_or(FiveAError::Overflow)?;
    emit!(VouchClosed {
        voucher: ctx.vouch_record.voucher,
        vouchee: ctx.vouch_record.vouchee,
        outcome,
        vouchee_score,
    });
    Ok(())
}

/// Oracle-only, in `[vouched_at + 90d, vouched_at + 365d)`. A vouchee who was never scored
/// counts as failed, so evaluation can never get stuck (audit 5A-06).
pub fn evaluate_vouch(ctx: Context<EvaluateVouch>) -> Result<()> {
    let config = &ctx.accounts.close.config;
    require!(!config.paused, FiveAError::Paused);
    let now = Clock::get()?.unix_timestamp;
    require!(
        config.is_active_oracle(&ctx.accounts.oracle.key(), now),
        FiveAError::NotActiveOracle
    );
    let record = &ctx.accounts.close.vouch_record;
    let age = now.saturating_sub(record.vouched_at);
    require!(
        age >= VOUCH_EVALUATION_PERIOD,
        FiveAError::EvaluationTooEarly
    );
    require!(age < MAX_VOUCH_AGE, FiveAError::VouchExpired);
    let score = read_score(
        &record.vouchee,
        &ctx.accounts.vouchee_score.to_account_info(),
    )?
    .map_or(0, |s| s.composite_score);
    let outcome = if score >= VOUCH_SUCCESS_SCORE {
        VouchOutcome::Succeeded
    } else {
        VouchOutcome::Failed
    };
    settle(&mut ctx.accounts.close, outcome, score)
}

/// Anyone may expire a vouch nobody evaluated within a year, freeing the voucher's slot.
pub fn expire_vouch(ctx: Context<CloseVouch>) -> Result<()> {
    require!(!ctx.accounts.config.paused, FiveAError::Paused);
    let now = Clock::get()?.unix_timestamp;
    require!(
        now.saturating_sub(ctx.accounts.vouch_record.vouched_at) >= MAX_VOUCH_AGE,
        FiveAError::VouchNotExpired
    );
    settle(ctx.accounts, VouchOutcome::Expired, 0)
}
