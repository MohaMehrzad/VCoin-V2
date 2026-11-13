//! Initialization, engagement oracle, pause and authority.

use anchor_lang::prelude::*;
use vcoin_common::{require_upgrade_authority, AdminState, AUTHORITY_TRANSFER_DELAY};

use crate::constants::*;
use crate::contexts::*;
use crate::errors::ContentError;
use crate::events::*;

pub fn initialize(ctx: Context<Initialize>, engagement_oracle: Pubkey) -> Result<()> {
    let authority = ctx.accounts.authority.key();
    require_upgrade_authority(&ctx.accounts.program_data, &authority)?;
    require!(
        engagement_oracle != Pubkey::default(),
        ContentError::InvalidKey
    );
    let config = &mut ctx.accounts.config;
    config.admin = AdminState::new(authority);
    config.engagement_oracle = engagement_oracle;
    config.pending_oracle = Pubkey::default();
    config.pending_oracle_eta = 0;
    config.paused = false;
    config.bump = ctx.bumps.config;
    emit!(RegistryInitialized {
        authority,
        engagement_oracle
    });
    Ok(())
}

/// Schedules a new engagement oracle, effective after 24h.
pub fn propose_oracle(ctx: Context<UpdateConfig>, oracle: Pubkey) -> Result<()> {
    let config = &mut ctx.accounts.config;
    config
        .admin
        .require_authority(&ctx.accounts.authority.key())?;
    require!(oracle != Pubkey::default(), ContentError::InvalidKey);
    let eta = Clock::get()?
        .unix_timestamp
        .checked_add(ORACLE_CHANGE_DELAY)
        .ok_or(ContentError::Overflow)?;
    config.pending_oracle = oracle;
    config.pending_oracle_eta = eta;
    emit!(OracleChangeProposed { oracle, eta });
    Ok(())
}

pub fn apply_oracle(ctx: Context<ApplyOracle>) -> Result<()> {
    let config = &mut ctx.accounts.config;
    require!(
        config.pending_oracle_eta != 0,
        ContentError::NoPendingChange
    );
    require!(
        Clock::get()?.unix_timestamp >= config.pending_oracle_eta,
        ContentError::TimelockActive
    );
    config.engagement_oracle = config.pending_oracle;
    config.pending_oracle_eta = 0;
    emit!(OracleChanged {
        oracle: config.engagement_oracle
    });
    Ok(())
}

pub fn cancel_oracle(ctx: Context<UpdateConfig>) -> Result<()> {
    let config = &mut ctx.accounts.config;
    config
        .admin
        .require_authority(&ctx.accounts.authority.key())?;
    require!(
        config.pending_oracle_eta != 0,
        ContentError::NoPendingChange
    );
    config.pending_oracle_eta = 0;
    emit!(OracleChangeCancelled {});
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
