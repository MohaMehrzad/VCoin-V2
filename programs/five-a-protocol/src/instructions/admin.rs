//! Initialization, oracles, consensus, pause and authority.

use anchor_lang::prelude::*;
use vcoin_common::{require_upgrade_authority, AdminState, AUTHORITY_TRANSFER_DELAY};

use crate::constants::*;
use crate::contexts::*;
use crate::errors::FiveAError;
use crate::events::*;
use crate::state::OracleEntry;

pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
    let authority = ctx.accounts.authority.key();
    require_upgrade_authority(&ctx.accounts.program_data, &authority)?;
    let config = &mut ctx.accounts.config;
    config.admin = AdminState::new(authority);
    config.oracles = [OracleEntry::default(); MAX_ORACLES];
    config.required_consensus = DEFAULT_CONSENSUS;
    config.paused = false;
    config.bump = ctx.bumps.config;
    emit!(FiveAInitialized {
        authority,
        required_consensus: DEFAULT_CONSENSUS
    });
    Ok(())
}

/// Adds an oracle whose submissions count from 24h after registration.
pub fn register_oracle(ctx: Context<UpdateConfig>, oracle: Pubkey) -> Result<()> {
    let config = &mut ctx.accounts.config;
    config
        .admin
        .require_authority(&ctx.accounts.authority.key())?;
    require!(oracle != Pubkey::default(), FiveAError::InvalidOracle);
    require!(
        !config.oracles.iter().any(|o| o.key == oracle),
        FiveAError::OracleExists
    );
    let active_from = Clock::get()?
        .unix_timestamp
        .checked_add(ORACLE_ACTIVATION_DELAY)
        .ok_or(FiveAError::Overflow)?;
    let slot = config
        .oracles
        .iter_mut()
        .find(|o| o.key == Pubkey::default())
        .ok_or(FiveAError::TooManyOracles)?;
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

/// Removes an oracle immediately (its pending submissions stop counting). Consensus must
/// remain reachable, so lower it first if needed.
pub fn remove_oracle(ctx: Context<UpdateConfig>, oracle: Pubkey) -> Result<()> {
    let config = &mut ctx.accounts.config;
    config
        .admin
        .require_authority(&ctx.accounts.authority.key())?;
    let slot = config
        .oracles
        .iter_mut()
        .find(|o| o.key == oracle && oracle != Pubkey::default())
        .ok_or(FiveAError::OracleNotFound)?;
    *slot = OracleEntry::default();
    require!(
        config.registered_oracles() >= config.required_consensus as usize,
        FiveAError::InvalidConsensus
    );
    emit!(OracleRemoved { oracle });
    Ok(())
}

pub fn set_required_consensus(ctx: Context<UpdateConfig>, required_consensus: u8) -> Result<()> {
    let config = &mut ctx.accounts.config;
    config
        .admin
        .require_authority(&ctx.accounts.authority.key())?;
    require!(
        required_consensus >= MIN_CONSENSUS
            && required_consensus as usize
                <= config.registered_oracles().min(MAX_CONSENSUS as usize),
        FiveAError::InvalidConsensus
    );
    config.required_consensus = required_consensus;
    emit!(ConsensusSet { required_consensus });
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
