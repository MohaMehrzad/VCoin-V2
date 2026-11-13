//! Realms (SPL Governance) voter-weight plugin instructions.

use anchor_lang::prelude::*;

use crate::constants::MAX_FIVE_A_SCORE;
use crate::contexts::*;
use crate::errors::StakingError;
use crate::events::{GateParamsUpdated, GovernanceConfigured};
use crate::governance::{five_a_score, validate_realm};
use crate::state::VoterWeightAction;
use crate::weight::{max_weight_for, voter_weight};

/// Wires the registrar to a realm (one-shot). The realm must be a RealmV2 of
/// `governance_program_id` with VCoin as its community mint, configured with this program
/// as voter-weight and max-voter-weight addin and the Dormant token type.
pub fn configure_governance(
    ctx: Context<ConfigureGovernance>,
    governance_program_id: Pubkey,
    min_vote_score: u16,
    min_propose_score: u16,
) -> Result<()> {
    let registrar = &mut ctx.accounts.registrar;
    registrar
        .admin
        .require_authority(&ctx.accounts.authority.key())?;
    require!(
        registrar.realm == Pubkey::default(),
        StakingError::GovernanceAlreadyConfigured
    );
    require!(
        min_vote_score <= MAX_FIVE_A_SCORE && min_propose_score <= MAX_FIVE_A_SCORE,
        StakingError::InvalidGateParams
    );
    validate_realm(
        &ctx.accounts.realm.to_account_info(),
        &governance_program_id,
        &registrar.vcoin_mint,
    )?;

    registrar.governance_program_id = governance_program_id;
    registrar.realm = ctx.accounts.realm.key();
    registrar.realm_governing_token_mint = registrar.vcoin_mint;
    registrar.slash_destination = ctx.accounts.slash_destination.key();
    registrar.min_vote_score = min_vote_score;
    registrar.min_propose_score = min_propose_score;

    let record = &mut ctx.accounts.max_voter_weight_record;
    record.realm = registrar.realm;
    record.governing_token_mint = registrar.realm_governing_token_mint;
    record.max_voter_weight =
        max_weight_for(registrar.total_locked, &registrar.weight).ok_or(StakingError::Overflow)?;
    record.max_voter_weight_expiry = Some(Clock::get()?.slot);

    emit!(GovernanceConfigured {
        realm: registrar.realm,
        governance_program_id,
        slash_destination: registrar.slash_destination,
    });
    Ok(())
}

pub fn create_voter_weight_record(ctx: Context<CreateVoterWeightRecord>) -> Result<()> {
    let registrar = &ctx.accounts.registrar;
    require!(
        registrar.realm != Pubkey::default(),
        StakingError::GovernanceNotConfigured
    );
    let record = &mut ctx.accounts.voter_weight_record;
    record.realm = registrar.realm;
    record.governing_token_mint = registrar.realm_governing_token_mint;
    record.governing_token_owner = ctx.accounts.owner.key();
    record.voter_weight = 0;
    record.voter_weight_expiry = Some(0);
    Ok(())
}

/// Writes the owner's weight for `action` (and its `target`), valid for the current slot
/// only. Participation is gated by the 5A reputation score; weight itself is linear in
/// locked VCoin and lock time, never multiplied by reputation (audit GOV-02).
pub fn update_voter_weight_record(
    ctx: Context<UpdateVoterWeightRecord>,
    action: VoterWeightAction,
    target: Option<Pubkey>,
) -> Result<()> {
    let registrar = &ctx.accounts.registrar;
    require!(
        registrar.realm != Pubkey::default(),
        StakingError::GovernanceNotConfigured
    );
    let owner = ctx.accounts.voter.owner;

    let gate = match action {
        VoterWeightAction::CastVote | VoterWeightAction::CommentProposal => {
            registrar.min_vote_score
        }
        _ => registrar.min_propose_score,
    };
    if gate > 0 {
        let score = five_a_score(&owner, &ctx.accounts.user_score.to_account_info())?;
        require!(score >= gate, StakingError::ScoreBelowGate);
    }

    let clock = Clock::get()?;
    let weight = voter_weight(&ctx.accounts.voter, &registrar.weight, clock.unix_timestamp)
        .ok_or(StakingError::Overflow)?;
    let record = &mut ctx.accounts.voter_weight_record;
    record.realm = registrar.realm;
    record.governing_token_mint = registrar.realm_governing_token_mint;
    record.governing_token_owner = owner;
    record.voter_weight = weight;
    record.voter_weight_expiry = Some(clock.slot);
    record.weight_action = Some(action);
    record.weight_action_target = target;
    Ok(())
}

/// Refreshes the realm-wide maximum (upper bound: every locked token at the maximum lock
/// bonus), valid for the current slot. Bundle it before CastVote / FinalizeVote.
pub fn update_max_voter_weight_record(ctx: Context<UpdateMaxVoterWeightRecord>) -> Result<()> {
    let registrar = &ctx.accounts.registrar;
    require!(
        registrar.realm != Pubkey::default(),
        StakingError::GovernanceNotConfigured
    );
    let record = &mut ctx.accounts.max_voter_weight_record;
    record.max_voter_weight =
        max_weight_for(registrar.total_locked, &registrar.weight).ok_or(StakingError::Overflow)?;
    record.max_voter_weight_expiry = Some(Clock::get()?.slot);
    Ok(())
}

pub fn set_gate_params(
    ctx: Context<UpdateRegistrar>,
    min_vote_score: u16,
    min_propose_score: u16,
) -> Result<()> {
    let registrar = &mut ctx.accounts.registrar;
    registrar
        .admin
        .require_authority(&ctx.accounts.authority.key())?;
    require!(
        min_vote_score <= MAX_FIVE_A_SCORE && min_propose_score <= MAX_FIVE_A_SCORE,
        StakingError::InvalidGateParams
    );
    registrar.min_vote_score = min_vote_score;
    registrar.min_propose_score = min_propose_score;
    emit!(GateParamsUpdated {
        min_vote_score,
        min_propose_score
    });
    Ok(())
}
