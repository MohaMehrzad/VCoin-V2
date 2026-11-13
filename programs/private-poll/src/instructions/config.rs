//! Config, committee registration and authority.

use anchor_lang::prelude::*;
use vcoin_common::{require_upgrade_authority, AdminState, AUTHORITY_TRANSFER_DELAY};
use zk_poll_core::verify_committee;

use crate::constants::*;
use crate::contexts::*;
use crate::errors::{zk, PollError};
use crate::events::*;
use crate::state::{Committee, PollParams};

pub fn initialize_config(ctx: Context<InitializeConfig>, params: PollParams) -> Result<()> {
    let authority = ctx.accounts.authority.key();
    require_upgrade_authority(&ctx.accounts.program_data, &authority)?;
    require!(params.is_valid(), PollError::InvalidParams);
    let config = &mut ctx.accounts.config;
    config.admin = AdminState::new(authority);
    config.registrar = ctx.accounts.registrar.key();
    config.committee = Committee::default();
    config.params = params;
    config.poll_count = 0;
    config.paused = false;
    config.paused_since = 0;
    config.pause_accum = 0;
    config.bump = ctx.bumps.config;
    emit!(ConfigInitialized {
        authority,
        registrar: config.registrar
    });
    Ok(())
}

/// Registers the decryption committee from a Pedersen DKG (see `zk-voting-sdk::dkg`).
/// Verifies shape, the election key, distinct members and every member key against the
/// Feldman commitments (audit ZK-03). Running polls keep their snapshot.
pub fn set_committee(
    ctx: Context<UpdateConfig>,
    threshold: u8,
    commitments: Vec<[u8; 32]>,
    member_pks: Vec<[u8; 32]>,
    member_authorities: Vec<Pubkey>,
) -> Result<()> {
    let config = &mut ctx.accounts.config;
    config
        .admin
        .require_authority(&ctx.accounts.authority.key())?;
    require!(
        member_authorities.len() == member_pks.len()
            && member_authorities.len() <= MAX_COMMITTEE
            && member_authorities.iter().all(|a| *a != Pubkey::default())
            && member_authorities
                .iter()
                .enumerate()
                .all(|(i, a)| !member_authorities[..i].contains(a)),
        PollError::InvalidCommittee
    );
    let election_pk = *commitments.first().ok_or(PollError::InvalidCommittee)?;
    zk(verify_committee(
        threshold,
        &election_pk,
        &commitments,
        &member_pks,
    ))?;

    let mut committee = Committee {
        epoch: config
            .committee
            .epoch
            .checked_add(1)
            .ok_or(PollError::Overflow)?,
        threshold,
        size: member_pks.len() as u8,
        election_pk,
        ..Committee::default()
    };
    committee.commitments[..commitments.len()].copy_from_slice(&commitments);
    committee.member_pks[..member_pks.len()].copy_from_slice(&member_pks);
    committee.member_authorities[..member_authorities.len()].copy_from_slice(&member_authorities);
    config.committee = committee;

    emit!(CommitteeSet {
        epoch: committee.epoch,
        threshold,
        size: committee.size,
        election_pk
    });
    Ok(())
}

pub fn set_poll_params(ctx: Context<UpdateConfig>, params: PollParams) -> Result<()> {
    let config = &mut ctx.accounts.config;
    config
        .admin
        .require_authority(&ctx.accounts.authority.key())?;
    require!(params.is_valid(), PollError::InvalidParams);
    config.params = params;
    emit!(PollParamsUpdated { params });
    Ok(())
}

/// Pausing stops creating polls, voting, revealing and settling. Reveal deadlines are
/// extended by the time spent paused and polls cannot expire while paused.
pub fn set_paused(ctx: Context<UpdateConfig>, paused: bool) -> Result<()> {
    let config = &mut ctx.accounts.config;
    config
        .admin
        .require_pause_rights(&ctx.accounts.authority.key(), paused)?;
    let now = Clock::get()?.unix_timestamp;
    if paused && !config.paused {
        config.paused_since = now;
    } else if !paused && config.paused {
        config.pause_accum = config
            .pause_accum
            .checked_add(now.saturating_sub(config.paused_since))
            .ok_or(PollError::Overflow)?;
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

/// Sets or clears the emergency guardian, which may pause but never unpause.
pub fn set_guardian(ctx: Context<UpdateConfig>, guardian: Pubkey) -> Result<()> {
    let admin = &mut ctx.accounts.config.admin;
    admin.set_guardian(&ctx.accounts.authority.key(), guardian)?;
    emit!(GuardianSet { guardian });
    Ok(())
}
