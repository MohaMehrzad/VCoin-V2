//! Pool initialization, oracles, parameters, fees, pause and authority.

use anchor_lang::prelude::*;
use vcoin_common::{
    require_upgrade_authority, validate_vcoin_mint, AdminState, AUTHORITY_TRANSFER_DELAY,
};

use crate::constants::*;
use crate::contexts::*;
use crate::errors::SscreError;
use crate::events::*;
use crate::state::{OracleEntry, PoolParams};

fn vcoin_mint_address() -> Pubkey {
    Pubkey::find_program_address(&[vcoin_token::constants::VCOIN_MINT_SEED], &vcoin_token::ID).0
}

pub fn initialize_pool(
    ctx: Context<InitializePool>,
    params: PoolParams,
    first_epoch_budget: u64,
    fee_bps: u16,
) -> Result<()> {
    let authority = ctx.accounts.authority.key();
    require_upgrade_authority(&ctx.accounts.program_data, &authority)?;
    require_keys_eq!(
        ctx.accounts.vcoin_mint.key(),
        vcoin_mint_address(),
        SscreError::InvalidVCoinMint
    );
    validate_vcoin_mint(&ctx.accounts.vcoin_mint.to_account_info())?;
    require!(
        params.is_valid() && first_epoch_budget > 0,
        SscreError::InvalidParams
    );
    require!(fee_bps <= MAX_FEE_BPS, SscreError::FeeTooHigh);

    let config = &mut ctx.accounts.config;
    config.admin = AdminState::new(authority);
    config.vcoin_mint = ctx.accounts.vcoin_mint.key();
    config.reserve_vault = ctx.accounts.reserve_vault.key();
    config.fee_bps = fee_bps;
    config.fee_destination = ctx.accounts.fee_destination.key();
    config.pending_fee_bps = 0;
    config.pending_fee_destination = Pubkey::default();
    config.pending_fee_eta = 0;
    config.params = params;
    config.max_epoch_budget = first_epoch_budget;
    config.next_epoch_budget = first_epoch_budget;
    config.epoch_count = 0;
    config.last_epoch_start = 0;
    config.oracles = [OracleEntry::default(); MAX_ORACLES];
    config.root_quorum = DEFAULT_ROOT_QUORUM;
    config.paused = false;
    config.paused_since = 0;
    config.pause_accum = 0;
    config.bump = ctx.bumps.config;
    config.reserve_bump = ctx.bumps.reserve_vault;

    emit!(PoolInitialized {
        authority,
        reserve_vault: config.reserve_vault,
        first_epoch_budget,
        fee_bps
    });
    Ok(())
}

/// Adds an oracle that may vote 24h from now.
pub fn register_oracle(ctx: Context<UpdateConfig>, oracle: Pubkey) -> Result<()> {
    let config = &mut ctx.accounts.config;
    config
        .admin
        .require_authority(&ctx.accounts.authority.key())?;
    require!(oracle != Pubkey::default(), SscreError::InvalidOracle);
    require!(
        !config.oracles.iter().any(|o| o.key == oracle),
        SscreError::OracleExists
    );
    let active_from = Clock::get()?
        .unix_timestamp
        .checked_add(ORACLE_ACTIVATION_DELAY)
        .ok_or(SscreError::Overflow)?;
    let slot = config
        .oracles
        .iter_mut()
        .find(|o| o.key == Pubkey::default())
        .ok_or(SscreError::TooManyOracles)?;
    *slot = OracleEntry {
        key: oracle,
        active_from,
    };
    emit!(OracleRegistered {
        oracle,
        active_from
    });
    Ok(())
}

/// Removes an oracle immediately; its votes stop counting. The quorum must stay reachable.
pub fn remove_oracle(ctx: Context<UpdateConfig>, oracle: Pubkey) -> Result<()> {
    let config = &mut ctx.accounts.config;
    config
        .admin
        .require_authority(&ctx.accounts.authority.key())?;
    let slot = config
        .oracles
        .iter_mut()
        .find(|o| o.key == oracle && oracle != Pubkey::default())
        .ok_or(SscreError::OracleNotFound)?;
    *slot = OracleEntry::default();
    require!(
        config.registered_oracles() >= config.root_quorum as usize,
        SscreError::InvalidQuorum
    );
    emit!(OracleRemoved { oracle });
    Ok(())
}

pub fn set_root_quorum(ctx: Context<UpdateConfig>, quorum: u8) -> Result<()> {
    let config = &mut ctx.accounts.config;
    config
        .admin
        .require_authority(&ctx.accounts.authority.key())?;
    require!(
        quorum >= MIN_ROOT_QUORUM
            && quorum as usize <= config.registered_oracles().min(MAX_ORACLES),
        SscreError::InvalidQuorum
    );
    config.root_quorum = quorum;
    emit!(RootQuorumSet { quorum });
    Ok(())
}

pub fn set_params(ctx: Context<UpdateConfig>, params: PoolParams) -> Result<()> {
    let config = &mut ctx.accounts.config;
    config
        .admin
        .require_authority(&ctx.accounts.authority.key())?;
    require!(params.is_valid(), SscreError::InvalidParams);
    config.params = params;
    emit!(ParamsUpdated { params });
    Ok(())
}

/// Resets the schedule's next budget, never above the ceiling fixed at initialization.
pub fn set_next_epoch_budget(ctx: Context<UpdateConfig>, budget: u64) -> Result<()> {
    let config = &mut ctx.accounts.config;
    config
        .admin
        .require_authority(&ctx.accounts.authority.key())?;
    require!(budget <= config.max_epoch_budget, SscreError::BudgetTooHigh);
    config.next_epoch_budget = budget;
    emit!(NextEpochBudgetSet { budget });
    Ok(())
}

/// Schedules a fee change (≤ 1%) and/or new fee account, applicable after 24h.
pub fn propose_fee(ctx: Context<ProposeFee>, fee_bps: u16) -> Result<()> {
    let config = &mut ctx.accounts.config;
    config
        .admin
        .require_authority(&ctx.accounts.authority.key())?;
    require!(fee_bps <= MAX_FEE_BPS, SscreError::FeeTooHigh);
    let eta = Clock::get()?
        .unix_timestamp
        .checked_add(FEE_CHANGE_DELAY)
        .ok_or(SscreError::Overflow)?;
    config.pending_fee_bps = fee_bps;
    config.pending_fee_destination = ctx.accounts.fee_destination.key();
    config.pending_fee_eta = eta;
    emit!(FeeChangeProposed {
        fee_bps,
        fee_destination: config.pending_fee_destination,
        eta
    });
    Ok(())
}

pub fn apply_fee(ctx: Context<ApplyFee>) -> Result<()> {
    let config = &mut ctx.accounts.config;
    require!(config.pending_fee_eta != 0, SscreError::NoPendingFee);
    require!(
        Clock::get()?.unix_timestamp >= config.pending_fee_eta,
        SscreError::FeeTimelockActive
    );
    config.fee_bps = config.pending_fee_bps;
    config.fee_destination = config.pending_fee_destination;
    config.pending_fee_eta = 0;
    emit!(FeeChangeApplied {
        fee_bps: config.fee_bps,
        fee_destination: config.fee_destination
    });
    Ok(())
}

pub fn cancel_fee(ctx: Context<UpdateConfig>) -> Result<()> {
    let config = &mut ctx.accounts.config;
    config
        .admin
        .require_authority(&ctx.accounts.authority.key())?;
    require!(config.pending_fee_eta != 0, SscreError::NoPendingFee);
    config.pending_fee_eta = 0;
    emit!(FeeChangeCancelled {});
    Ok(())
}

/// Pausing stops starting, voting, activating and claiming; every deadline is extended by
/// the time spent paused, so a pause never forfeits rewards (SSC-05).
pub fn set_paused(ctx: Context<UpdateConfig>, paused: bool) -> Result<()> {
    let config = &mut ctx.accounts.config;
    config
        .admin
        .require_authority(&ctx.accounts.authority.key())?;
    let now = Clock::get()?.unix_timestamp;
    if paused && !config.paused {
        config.paused_since = now;
    } else if !paused && config.paused {
        config.pause_accum = config
            .pause_accum
            .checked_add(now.saturating_sub(config.paused_since))
            .ok_or(SscreError::Overflow)?;
    }
    config.paused = paused;
    emit!(PauseChanged { paused });
    Ok(())
}

pub fn propose_authority(ctx: Context<UpdateConfig>, new_authority: Pubkey) -> Result<()> {
    let now = Clock::get()?.unix_timestamp;
    let admin = &mut ctx.accounts.config.admin;
    admin.propose(&ctx.accounts.authority.key(), new_authority, now)?;
    emit!(AuthorityProposed {
        current: admin.authority,
        proposed: new_authority,
        ready_at: now.saturating_add(AUTHORITY_TRANSFER_DELAY),
    });
    Ok(())
}

pub fn cancel_authority_transfer(ctx: Context<UpdateConfig>) -> Result<()> {
    let admin = &mut ctx.accounts.config.admin;
    admin.cancel(&ctx.accounts.authority.key())?;
    emit!(AuthorityTransferCancelled {
        authority: admin.authority
    });
    Ok(())
}

pub fn accept_authority(ctx: Context<AcceptAuthority>) -> Result<()> {
    let now = Clock::get()?.unix_timestamp;
    let admin = &mut ctx.accounts.config.admin;
    let previous = admin.accept(&ctx.accounts.new_authority.key(), now)?;
    emit!(AuthorityTransferred {
        previous,
        new_authority: admin.authority
    });
    Ok(())
}
