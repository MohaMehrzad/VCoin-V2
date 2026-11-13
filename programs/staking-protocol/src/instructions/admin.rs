//! Registrar creation and administration.

use anchor_lang::prelude::*;
use vcoin_common::{
    require_upgrade_authority, validate_vcoin_mint, AdminState, AUTHORITY_TRANSFER_DELAY,
};

use crate::contexts::*;
use crate::errors::StakingError;
use crate::events::*;
use crate::state::{Registrar, WeightParams};

/// The VCoin mint PDA of the vcoin-token program.
pub fn vcoin_mint_address() -> Pubkey {
    Pubkey::find_program_address(&[vcoin_token::constants::VCOIN_MINT_SEED], &vcoin_token::ID).0
}

/// The veVCoin mint PDA of the vevcoin-token program.
pub fn vevcoin_mint_address() -> Pubkey {
    Pubkey::find_program_address(
        &[vevcoin_token::constants::VEVCOIN_MINT_SEED],
        &vevcoin_token::ID,
    )
    .0
}

pub fn create_registrar(ctx: Context<CreateRegistrar>) -> Result<()> {
    require_upgrade_authority(&ctx.accounts.program_data, &ctx.accounts.authority.key())?;
    require_keys_eq!(
        ctx.accounts.vcoin_mint.key(),
        vcoin_mint_address(),
        StakingError::InvalidVCoinMint
    );
    require_keys_eq!(
        ctx.accounts.vevcoin_mint.key(),
        vevcoin_mint_address(),
        StakingError::InvalidVeVCoinMint
    );
    // Reject any mint carrying a transfer hook / fee / permanent delegate / freeze authority:
    // vault accounting credits exactly what arrives and must never be frozen (audit STK-01).
    validate_vcoin_mint(&ctx.accounts.vcoin_mint.to_account_info())?;

    let registrar = &mut ctx.accounts.registrar;
    ***registrar = Registrar {
        admin: AdminState::new(ctx.accounts.authority.key()),
        vcoin_mint: ctx.accounts.vcoin_mint.key(),
        vault: ctx.accounts.vault.key(),
        vevcoin_mint: ctx.accounts.vevcoin_mint.key(),
        weight: WeightParams::default(),
        bump: ctx.bumps.registrar,
        vault_bump: ctx.bumps.vault,
        ..Registrar::default()
    };

    emit!(RegistrarCreated {
        registrar: registrar.key(),
        authority: ctx.accounts.authority.key(),
        vcoin_mint: registrar.vcoin_mint,
        vevcoin_mint: registrar.vevcoin_mint,
        vault: registrar.vault,
    });
    Ok(())
}

pub fn set_paused(ctx: Context<UpdateRegistrar>, paused: bool) -> Result<()> {
    let registrar = &mut ctx.accounts.registrar;
    registrar
        .admin
        .require_pause_rights(&ctx.accounts.authority.key(), paused)?;
    registrar.paused = paused;
    emit!(PauseChanged { paused });
    Ok(())
}

pub fn set_weight_params(ctx: Context<UpdateRegistrar>, params: WeightParams) -> Result<()> {
    let registrar = &mut ctx.accounts.registrar;
    registrar
        .admin
        .require_authority(&ctx.accounts.authority.key())?;
    require!(params.is_valid(), StakingError::InvalidWeightParams);
    registrar.weight = params;
    emit!(WeightParamsUpdated {
        baseline_factor: params.baseline_factor,
        max_extra_factor: params.max_extra_factor,
        saturation_secs: params.saturation_secs,
        min_lock_remaining_to_vote: params.min_lock_remaining_to_vote,
    });
    Ok(())
}

pub fn propose_authority(ctx: Context<UpdateRegistrar>, new_authority: Pubkey) -> Result<()> {
    let now = Clock::get()?.unix_timestamp;
    let admin = &mut ctx.accounts.registrar.admin;
    admin.propose(&ctx.accounts.authority.key(), new_authority, now)?;
    emit!(AuthorityProposed {
        current: admin.authority,
        proposed: new_authority,
        ready_at: now.saturating_add(AUTHORITY_TRANSFER_DELAY),
    });
    Ok(())
}

pub fn cancel_authority_transfer(ctx: Context<UpdateRegistrar>) -> Result<()> {
    let admin = &mut ctx.accounts.registrar.admin;
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
    let admin = &mut ctx.accounts.registrar.admin;
    let previous = admin.accept(&ctx.accounts.new_authority.key(), now)?;
    emit!(AuthorityTransferred {
        previous,
        new_authority: admin.authority
    });
    Ok(())
}

/// Sets or clears the emergency guardian, which may pause but never unpause.
pub fn set_guardian(ctx: Context<UpdateRegistrar>, guardian: Pubkey) -> Result<()> {
    let admin = &mut ctx.accounts.registrar.admin;
    admin.set_guardian(&ctx.accounts.authority.key(), guardian)?;
    emit!(GuardianSet { guardian });
    Ok(())
}
