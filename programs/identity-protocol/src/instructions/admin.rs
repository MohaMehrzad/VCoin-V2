//! Initialization, attesters, payment config, pause and authority.

use anchor_lang::prelude::*;
use vcoin_common::{require_upgrade_authority, AdminState, AUTHORITY_TRANSFER_DELAY};

use crate::constants::*;
use crate::contexts::*;
use crate::errors::IdentityError;
use crate::events::*;

pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
    let authority = ctx.accounts.authority.key();
    require_upgrade_authority(&ctx.accounts.program_data, &authority)?;
    let config = &mut ctx.accounts.config;
    config.admin = AdminState::new(authority);
    config.attesters = [Pubkey::default(); MAX_ATTESTERS];
    config.usdc_mint = ctx.accounts.usdc_mint.key();
    config.treasury_token = ctx.accounts.treasury_token.key();
    config.prices = DEFAULT_PRICES;
    config.pending_eta = 0;
    config.paused = false;
    config.bump = ctx.bumps.config;
    emit!(IdentityInitialized {
        authority,
        usdc_mint: config.usdc_mint,
        treasury_token: config.treasury_token
    });
    Ok(())
}

pub fn add_attester(ctx: Context<UpdateConfig>, attester: Pubkey) -> Result<()> {
    let config = &mut ctx.accounts.config;
    config
        .admin
        .require_authority(&ctx.accounts.authority.key())?;
    require!(
        attester != Pubkey::default(),
        IdentityError::InvalidAttester
    );
    require!(
        !config.attesters.contains(&attester),
        IdentityError::AttesterExists
    );
    let slot = config
        .attesters
        .iter_mut()
        .find(|a| **a == Pubkey::default())
        .ok_or(IdentityError::TooManyAttesters)?;
    *slot = attester;
    emit!(AttesterAdded { attester });
    Ok(())
}

/// Removing an attester voids every level it granted at once (`effective_level`).
pub fn remove_attester(ctx: Context<UpdateConfig>, attester: Pubkey) -> Result<()> {
    let config = &mut ctx.accounts.config;
    config
        .admin
        .require_authority(&ctx.accounts.authority.key())?;
    let slot = config
        .attesters
        .iter_mut()
        .find(|a| **a == attester && attester != Pubkey::default())
        .ok_or(IdentityError::AttesterNotFound)?;
    *slot = Pubkey::default();
    emit!(AttesterRemoved { attester });
    Ok(())
}

/// Schedules new prices and/or treasury account, applicable after 24h.
pub fn propose_payment(ctx: Context<ProposePayment>, prices: [u64; NUM_TIERS]) -> Result<()> {
    let config = &mut ctx.accounts.config;
    config
        .admin
        .require_authority(&ctx.accounts.authority.key())?;
    let eta = Clock::get()?
        .unix_timestamp
        .checked_add(PAYMENT_CONFIG_DELAY)
        .ok_or(IdentityError::Overflow)?;
    config.pending_prices = prices;
    config.pending_treasury_token = ctx.accounts.treasury_token.key();
    config.pending_eta = eta;
    emit!(PaymentChangeProposed {
        prices,
        treasury_token: config.pending_treasury_token,
        eta
    });
    Ok(())
}

pub fn apply_payment(ctx: Context<ApplyPayment>) -> Result<()> {
    let config = &mut ctx.accounts.config;
    require!(config.pending_eta != 0, IdentityError::NoPendingChange);
    require!(
        Clock::get()?.unix_timestamp >= config.pending_eta,
        IdentityError::TimelockActive
    );
    config.prices = config.pending_prices;
    config.treasury_token = config.pending_treasury_token;
    config.pending_eta = 0;
    emit!(PaymentChangeApplied {
        prices: config.prices,
        treasury_token: config.treasury_token
    });
    Ok(())
}

pub fn cancel_payment(ctx: Context<UpdateConfig>) -> Result<()> {
    let config = &mut ctx.accounts.config;
    config
        .admin
        .require_authority(&ctx.accounts.authority.key())?;
    require!(config.pending_eta != 0, IdentityError::NoPendingChange);
    config.pending_eta = 0;
    emit!(PaymentChangeCancelled {});
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
