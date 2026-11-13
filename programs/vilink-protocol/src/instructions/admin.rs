//! Initialization, fee/treasury timelock, action switches, pause and authority.

use anchor_lang::prelude::*;
use vcoin_common::{
    require_upgrade_authority, validate_vcoin_mint, AdminState, AUTHORITY_TRANSFER_DELAY,
};

use crate::constants::*;
use crate::contexts::*;
use crate::errors::ViLinkError;
use crate::events::*;

fn vcoin_mint_address() -> Pubkey {
    Pubkey::find_program_address(&[vcoin_token::constants::VCOIN_MINT_SEED], &vcoin_token::ID).0
}

fn check_fee(fee_bps: u16) -> Result<()> {
    require!(
        (MIN_FEE_BPS..=MAX_FEE_BPS).contains(&fee_bps),
        ViLinkError::InvalidFee
    );
    Ok(())
}

pub fn initialize(ctx: Context<Initialize>, fee_bps: u16) -> Result<()> {
    let authority = ctx.accounts.authority.key();
    require_upgrade_authority(&ctx.accounts.program_data, &authority)?;
    require_keys_eq!(
        ctx.accounts.vcoin_mint.key(),
        vcoin_mint_address(),
        ViLinkError::InvalidVCoinMint
    );
    validate_vcoin_mint(&ctx.accounts.vcoin_mint.to_account_info())?;
    check_fee(fee_bps)?;
    let config = &mut ctx.accounts.config;
    config.admin = AdminState::new(authority);
    config.vcoin_mint = ctx.accounts.vcoin_mint.key();
    config.treasury = ctx.accounts.treasury.key();
    config.fee_bps = fee_bps;
    config.pending_eta = 0;
    config.enabled_actions = ALL_ACTIONS_MASK;
    config.paused = false;
    config.bump = ctx.bumps.config;
    emit!(ViLinkInitialized {
        authority,
        treasury: config.treasury,
        fee_bps
    });
    Ok(())
}

/// Schedules a new fee and/or treasury, applicable after 24h.
pub fn propose_config(ctx: Context<ProposeConfig>, fee_bps: u16) -> Result<()> {
    let config = &mut ctx.accounts.config;
    config
        .admin
        .require_authority(&ctx.accounts.authority.key())?;
    check_fee(fee_bps)?;
    let eta = Clock::get()?
        .unix_timestamp
        .checked_add(CONFIG_CHANGE_DELAY)
        .ok_or(ViLinkError::Overflow)?;
    config.pending_fee_bps = fee_bps;
    config.pending_treasury = ctx.accounts.treasury.key();
    config.pending_eta = eta;
    emit!(ConfigChangeProposed {
        fee_bps,
        treasury: config.pending_treasury,
        eta
    });
    Ok(())
}

pub fn apply_config(ctx: Context<ApplyConfig>) -> Result<()> {
    let config = &mut ctx.accounts.config;
    require!(config.pending_eta != 0, ViLinkError::NoPendingChange);
    require!(
        Clock::get()?.unix_timestamp >= config.pending_eta,
        ViLinkError::TimelockActive
    );
    config.fee_bps = config.pending_fee_bps;
    config.treasury = config.pending_treasury;
    config.pending_eta = 0;
    emit!(ConfigChangeApplied {
        fee_bps: config.fee_bps,
        treasury: config.treasury
    });
    Ok(())
}

pub fn cancel_config(ctx: Context<UpdateConfig>) -> Result<()> {
    let config = &mut ctx.accounts.config;
    config
        .admin
        .require_authority(&ctx.accounts.authority.key())?;
    require!(config.pending_eta != 0, ViLinkError::NoPendingChange);
    config.pending_eta = 0;
    emit!(ConfigChangeCancelled {});
    Ok(())
}

/// Per-type kill switch, effective immediately for creation and execution.
pub fn set_enabled_actions(ctx: Context<UpdateConfig>, enabled_actions: u8) -> Result<()> {
    let config = &mut ctx.accounts.config;
    config
        .admin
        .require_authority(&ctx.accounts.authority.key())?;
    config.enabled_actions = enabled_actions & ALL_ACTIONS_MASK;
    emit!(EnabledActionsSet {
        enabled_actions: config.enabled_actions
    });
    Ok(())
}

pub fn set_paused(ctx: Context<UpdateConfig>, paused: bool) -> Result<()> {
    let config = &mut ctx.accounts.config;
    config
        .admin
        .require_pause_rights(&ctx.accounts.authority.key(), paused)?;
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

/// Sets or clears the emergency guardian, which may pause but never unpause.
pub fn set_guardian(ctx: Context<UpdateConfig>, guardian: Pubkey) -> Result<()> {
    let admin = &mut ctx.accounts.config.admin;
    admin.set_guardian(&ctx.accounts.authority.key(), guardian)?;
    emit!(GuardianSet { guardian });
    Ok(())
}
