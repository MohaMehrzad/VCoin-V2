//! Creating, executing and closing actions.

use anchor_lang::prelude::*;
use anchor_spl::token_interface::{transfer_checked, TransferChecked};
use gasless_protocol::constants::{SCOPE_FOLLOW, SCOPE_VOUCH};

use crate::constants::*;
use crate::contexts::*;
use crate::errors::ViLinkError;
use crate::events::*;
use crate::state::*;

fn now() -> Result<i64> {
    Ok(Clock::get()?.unix_timestamp)
}

#[allow(clippy::too_many_arguments)]
pub fn create_action(
    ctx: Context<CreateAction>,
    action_type: u8,
    amount: u64,
    target: Pubkey,
    metadata_hash: [u8; 32],
    expiry_secs: i64,
    max_executions: u32,
    nonce: u64,
) -> Result<()> {
    let config = &ctx.accounts.config;
    require!(!config.paused, ViLinkError::Paused);
    require!(config.is_enabled(action_type), ViLinkError::ActionDisabled);
    let creator = ctx.accounts.creator.key();
    if action_type == ACTION_TIP {
        require!(
            (MIN_TIP_AMOUNT..=MAX_TIP_AMOUNT).contains(&amount),
            ViLinkError::InvalidAmount
        );
    } else {
        require!(amount == 0, ViLinkError::InvalidAmount);
    }
    require!(
        target != Pubkey::default() && target != creator,
        ViLinkError::InvalidTarget
    );
    require!(
        (MIN_ACTION_EXPIRY..=MAX_ACTION_EXPIRY).contains(&expiry_secs),
        ViLinkError::InvalidExpiry
    );
    require!(
        (1..=MAX_EXECUTIONS).contains(&max_executions),
        ViLinkError::InvalidMaxExecutions
    );

    let stats = &mut ctx.accounts.creator_stats;
    if stats.user == Pubkey::default() {
        stats.user = creator;
        stats.bump = ctx.bumps.creator_stats;
    }
    require!(nonce == stats.actions_created, ViLinkError::InvalidNonce);
    stats.actions_created = stats
        .actions_created
        .checked_add(1)
        .ok_or(ViLinkError::Overflow)?;

    let now = now()?;
    let action = &mut ctx.accounts.action;
    action.action_id = action_id(&creator, nonce, action_type, &target, amount, now);
    action.creator = creator;
    action.nonce = nonce;
    action.action_type = action_type;
    action.amount = amount;
    action.target = target;
    action.metadata_hash = metadata_hash;
    action.fee_bps = config.fee_bps;
    action.created_at = now;
    action.expires_at = now.checked_add(expiry_secs).ok_or(ViLinkError::Overflow)?;
    action.max_executions = max_executions;
    action.execution_count = 0;
    action.total_volume = 0;
    action.bump = ctx.bumps.action;

    emit!(ActionCreated {
        action: action.key(),
        action_id: action.action_id,
        creator,
        action_type,
        target,
        amount,
        fee_bps: action.fee_bps,
        expires_at: action.expires_at,
        max_executions,
    });
    Ok(())
}

/// Shared checks: not paused, type enabled now (VL-2), right type, still open, and the
/// executor is neither the creator nor the target (VL-9).
fn check_executable(
    config: &ViLinkConfig,
    action: &ViLinkAction,
    executor: &Pubkey,
    expected: u8,
    now: i64,
) -> Result<()> {
    require!(!config.paused, ViLinkError::Paused);
    require!(
        config.is_enabled(action.action_type),
        ViLinkError::ActionDisabled
    );
    require!(action.action_type == expected, ViLinkError::WrongActionType);
    require!(action.is_open(now), ViLinkError::ActionClosed);
    require!(
        *executor != action.creator && *executor != action.target,
        ViLinkError::InvalidExecutor
    );
    Ok(())
}

fn init_stats(stats: &mut UserActionStats, user: Pubkey, bump: u8) {
    if stats.user == Pubkey::default() {
        stats.user = user;
        stats.bump = bump;
    }
}

pub fn execute_tip(ctx: Context<ExecuteTip>) -> Result<()> {
    let now = now()?;
    let executor = ctx.accounts.executor.key();
    let action = &mut ctx.accounts.action;
    check_executable(&ctx.accounts.config, action, &executor, ACTION_TIP, now)?;

    let fee = tip_fee(action.amount, action.fee_bps);
    let net = action.amount - fee;
    let decimals = ctx.accounts.vcoin_mint.decimals;
    for (to, value) in [
        (ctx.accounts.target_token.to_account_info(), net),
        (ctx.accounts.treasury.to_account_info(), fee),
    ] {
        if value == 0 {
            continue;
        }
        transfer_checked(
            CpiContext::new(
                ctx.accounts.token_program.to_account_info(),
                TransferChecked {
                    from: ctx.accounts.executor_token.to_account_info(),
                    mint: ctx.accounts.vcoin_mint.to_account_info(),
                    to,
                    authority: ctx.accounts.executor.to_account_info(),
                },
            ),
            value,
            decimals,
        )?;
    }

    action.execution_count += 1;
    action.total_volume = action
        .total_volume
        .checked_add(action.amount)
        .ok_or(ViLinkError::Overflow)?;
    let receipt = &mut ctx.accounts.receipt;
    receipt.action = action.key();
    receipt.executor = executor;
    receipt.executed_at = now;
    receipt.rent_payer = executor;
    receipt.bump = ctx.bumps.receipt;
    let es = &mut ctx.accounts.executor_stats;
    init_stats(es, executor, ctx.bumps.executor_stats);
    es.tips_sent = es
        .tips_sent
        .checked_add(action.amount)
        .ok_or(ViLinkError::Overflow)?;
    let ts = &mut ctx.accounts.target_stats;
    init_stats(ts, action.target, ctx.bumps.target_stats);
    ts.tips_received = ts
        .tips_received
        .checked_add(net)
        .ok_or(ViLinkError::Overflow)?;

    emit!(ActionExecuted {
        action: action.key(),
        action_type: ACTION_TIP,
        executor,
        target: action.target,
        amount: action.amount,
        fee,
        execution_count: action.execution_count,
        signer: executor,
    });
    Ok(())
}

fn execute_social(ctx: Context<ExecuteSocial>, expected: u8, scope: u16) -> Result<()> {
    let now = now()?;
    let executor = ctx.accounts.executor.key();
    let signer = ctx.accounts.signer.key();
    if executor != signer {
        let session = ctx
            .accounts
            .session
            .as_ref()
            .ok_or(ViLinkError::Unauthorized)?;
        session.verify(&executor, &signer, &crate::ID, scope, now)?;
    }
    let action = &mut ctx.accounts.action;
    check_executable(&ctx.accounts.config, action, &executor, expected, now)?;

    action.execution_count += 1;
    let receipt = &mut ctx.accounts.receipt;
    receipt.action = action.key();
    receipt.executor = executor;
    receipt.executed_at = now;
    receipt.rent_payer = ctx.accounts.payer.key();
    receipt.bump = ctx.bumps.receipt;
    let es = &mut ctx.accounts.executor_stats;
    init_stats(es, executor, ctx.bumps.executor_stats);
    let ts = &mut ctx.accounts.target_stats;
    init_stats(ts, action.target, ctx.bumps.target_stats);
    if expected == ACTION_VOUCH {
        es.vouches_given = es.vouches_given.saturating_add(1);
        ts.vouches_received = ts.vouches_received.saturating_add(1);
    } else {
        es.follows_given = es.follows_given.saturating_add(1);
        ts.follows_received = ts.follows_received.saturating_add(1);
    }
    emit!(ActionExecuted {
        action: action.key(),
        action_type: expected,
        executor,
        target: action.target,
        amount: 0,
        fee: 0,
        execution_count: action.execution_count,
        signer,
    });
    Ok(())
}

pub fn execute_vouch(ctx: Context<ExecuteSocial>) -> Result<()> {
    execute_social(ctx, ACTION_VOUCH, SCOPE_VOUCH)
}

pub fn execute_follow(ctx: Context<ExecuteSocial>) -> Result<()> {
    execute_social(ctx, ACTION_FOLLOW, SCOPE_FOLLOW)
}

pub fn close_action(ctx: Context<CloseAction>) -> Result<()> {
    let action = &ctx.accounts.action;
    require!(!action.is_open(now()?), ViLinkError::ActionStillOpen);
    emit!(ActionClosed {
        action: action.key(),
        execution_count: action.execution_count
    });
    Ok(())
}

pub fn close_receipt(ctx: Context<CloseReceipt>) -> Result<()> {
    let info = ctx.accounts.action.to_account_info();
    if *info.owner == crate::ID && !info.data_is_empty() {
        let action = ViLinkAction::try_deserialize(&mut &info.try_borrow_data()?[..])?;
        require!(!action.is_open(now()?), ViLinkError::ActionStillOpen);
    }
    Ok(())
}
