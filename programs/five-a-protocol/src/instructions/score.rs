//! Oracle score consensus.

use anchor_lang::prelude::*;

use crate::constants::*;
use crate::contexts::*;
use crate::errors::FiveAError;
use crate::events::*;
use crate::state::{OracleSubmission, ScoreValues};

/// An active oracle submits (or changes) its values for `user`. When `required_consensus`
/// active oracles agree on identical values, they are applied and the round closes.
pub fn submit_score(ctx: Context<SubmitScore>, values: ScoreValues) -> Result<()> {
    let config = &ctx.accounts.config;
    require!(!config.paused, FiveAError::Paused);
    let now = Clock::get()?.unix_timestamp;
    let oracle = ctx.accounts.oracle.key();
    let user = ctx.accounts.user.key();
    require!(
        config.is_active_oracle(&oracle, now),
        FiveAError::NotActiveOracle
    );
    require!(values.is_valid(), FiveAError::InvalidScore);

    let score = &mut ctx.accounts.user_score;
    if score.user == Pubkey::default() {
        score.user = user;
        score.bump = ctx.bumps.user_score;
    }
    if score.update_count > 0 {
        let next = score
            .last_updated
            .checked_add(MIN_SCORE_UPDATE_INTERVAL)
            .ok_or(FiveAError::Overflow)?;
        require!(now >= next, FiveAError::ScoreUpdateTooFrequent);
    }

    let pending = &mut ctx.accounts.pending;
    if pending.user == Pubkey::default() {
        // Fresh round: the signing oracle opens it and is refunded when it closes.
        pending.user = user;
        pending.rent_payer = oracle;
        pending.bump = ctx.bumps.pending;
    }
    if pending.user != user || pending.opened_at == 0 || now > pending.expires_at {
        pending.submissions = [OracleSubmission::default(); MAX_ORACLES];
        pending.opened_at = now;
        pending.expires_at = now
            .checked_add(SCORE_UPDATE_EXPIRY)
            .ok_or(FiveAError::Overflow)?;
    }
    require_keys_eq!(
        ctx.accounts.rent_payer.key(),
        pending.rent_payer,
        FiveAError::WrongRentPayer
    );

    let slot = match pending.submissions.iter().position(|s| s.oracle == oracle) {
        Some(i) => i,
        None => pending
            .submissions
            .iter()
            .position(|s| s.oracle == Pubkey::default() || !config.is_active_oracle(&s.oracle, now))
            .ok_or(FiveAError::TooManyOracles)?,
    };
    pending.submissions[slot] = OracleSubmission { oracle, values };
    let support = pending.support(config, &values, now);
    emit!(ScoreSubmitted {
        user,
        oracle,
        values,
        support
    });

    if support >= config.required_consensus {
        score.authenticity = values.authenticity;
        score.accuracy = values.accuracy;
        score.agility = values.agility;
        score.activity = values.activity;
        score.approved = values.approved;
        score.composite_score = values.composite();
        score.last_updated = now;
        score.update_count = score
            .update_count
            .checked_add(1)
            .ok_or(FiveAError::Overflow)?;
        emit!(ScoreApplied {
            user,
            values,
            composite_score: score.composite_score,
            update_count: score.update_count,
        });
        ctx.accounts
            .pending
            .close(ctx.accounts.rent_payer.to_account_info())?;
    }
    Ok(())
}

pub fn close_expired_round(ctx: Context<CloseExpiredRound>) -> Result<()> {
    require!(
        Clock::get()?.unix_timestamp > ctx.accounts.pending.expires_at,
        FiveAError::RoundNotExpired
    );
    emit!(ScoreRoundExpired {
        user: ctx.accounts.pending.user
    });
    Ok(())
}
