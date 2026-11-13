use anchor_lang::prelude::*;
use vcoin_common::{
    require_upgrade_authority, timelock_ready, validate_vcoin_mint, AdminState,
    AUTHORITY_TRANSFER_DELAY,
};

use crate::constants::*;
use crate::contexts::*;
use crate::errors::GaslessError;
use crate::events::*;

pub fn initialize(ctx: Context<Initialize>, vcoin_fee_per_tx: u64) -> Result<()> {
    require_upgrade_authority(&ctx.accounts.program_data, &ctx.accounts.authority.key())?;
    validate_vcoin_mint(&ctx.accounts.vcoin_mint.to_account_info())?;
    require!(
        vcoin_fee_per_tx <= MAX_VCOIN_FEE_PER_TX,
        GaslessError::FeeTooHigh
    );

    let config = &mut ctx.accounts.config;
    config.admin = AdminState::new(ctx.accounts.authority.key());
    config.vcoin_mint = ctx.accounts.vcoin_mint.key();
    config.fee_vault = ctx.accounts.fee_vault.key();
    config.treasury = ctx.accounts.treasury.key();
    config.vcoin_fee_per_tx = vcoin_fee_per_tx;
    config.bump = ctx.bumps.config;
    config.vault_bump = ctx.bumps.fee_vault;

    emit!(GaslessInitialized {
        config: config.key(),
        authority: ctx.accounts.authority.key(),
        fee_vault: config.fee_vault,
        treasury: config.treasury,
        vcoin_fee_per_tx,
    });
    Ok(())
}

pub fn schedule_fee(ctx: Context<UpdateConfig>, new_fee: u64) -> Result<()> {
    let config = &mut ctx.accounts.config;
    config
        .admin
        .require_authority(&ctx.accounts.authority.key())?;
    require!(new_fee <= MAX_VCOIN_FEE_PER_TX, GaslessError::FeeTooHigh);
    let effective_at = Clock::get()?
        .unix_timestamp
        .checked_add(PARAM_CHANGE_DELAY)
        .ok_or(GaslessError::Overflow)?;
    config.pending_fee = new_fee;
    config.fee_eta = effective_at;
    emit!(FeeChangeScheduled {
        new_fee,
        effective_at
    });
    Ok(())
}

pub fn apply_fee(ctx: Context<ApplyFee>) -> Result<()> {
    let config = &mut ctx.accounts.config;
    require!(config.fee_eta != 0, GaslessError::NothingScheduled);
    require!(
        timelock_ready(config.fee_eta, Clock::get()?.unix_timestamp),
        GaslessError::TimelockActive
    );
    let old_fee = config.vcoin_fee_per_tx;
    config.vcoin_fee_per_tx = config.pending_fee;
    config.pending_fee = 0;
    config.fee_eta = 0;
    emit!(FeeChanged {
        old_fee,
        new_fee: config.vcoin_fee_per_tx
    });
    Ok(())
}

pub fn schedule_treasury(ctx: Context<UpdateConfig>, new_treasury: Pubkey) -> Result<()> {
    let config = &mut ctx.accounts.config;
    config
        .admin
        .require_authority(&ctx.accounts.authority.key())?;
    let effective_at = Clock::get()?
        .unix_timestamp
        .checked_add(PARAM_CHANGE_DELAY)
        .ok_or(GaslessError::Overflow)?;
    config.pending_treasury = new_treasury;
    config.treasury_eta = effective_at;
    emit!(TreasuryChangeScheduled {
        new_treasury,
        effective_at
    });
    Ok(())
}

pub fn apply_treasury(ctx: Context<ApplyTreasury>) -> Result<()> {
    let config = &mut ctx.accounts.config;
    require!(config.treasury_eta != 0, GaslessError::NothingScheduled);
    require!(
        timelock_ready(config.treasury_eta, Clock::get()?.unix_timestamp),
        GaslessError::TimelockActive
    );
    let old_treasury = config.treasury;
    config.treasury = ctx.accounts.new_treasury.key();
    config.pending_treasury = Pubkey::default();
    config.treasury_eta = 0;
    emit!(TreasuryChanged {
        old_treasury,
        new_treasury: config.treasury
    });
    Ok(())
}

pub fn set_paused(ctx: Context<UpdateConfig>, paused: bool) -> Result<()> {
    let config = &mut ctx.accounts.config;
    config
        .admin
        .require_authority(&ctx.accounts.authority.key())?;
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
    let cancelled = admin.pending_authority;
    admin.cancel(&ctx.accounts.authority.key())?;
    emit!(AuthorityTransferCancelled {
        authority: admin.authority,
        cancelled
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
