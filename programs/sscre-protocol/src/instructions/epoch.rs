//! Epoch lifecycle: start (fund from reserve) -> oracle votes -> quorum -> dispute delay ->
//! activate -> claims -> clawback of the remainder to the reserve.

use anchor_lang::prelude::*;
use anchor_spl::token_interface::{close_account, transfer_checked, CloseAccount, TransferChecked};

use crate::constants::*;
use crate::contexts::*;
use crate::errors::SscreError;
use crate::events::*;
use crate::merkle::{compute_leaf, verify_proof};
use crate::state::{EpochState, OracleVote};

fn now() -> Result<i64> {
    Ok(Clock::get()?.unix_timestamp)
}

/// Funds epoch `epoch_count` with `min(next_epoch_budget, reserve)` and decays the schedule.
/// Epochs start at most once per `epoch_duration`.
pub fn start_epoch(ctx: Context<StartEpoch>) -> Result<()> {
    let config = &mut ctx.accounts.config;
    require!(!config.paused, SscreError::Paused);
    let now = now()?;
    if config.epoch_count > 0 {
        let next_start = config
            .last_epoch_start
            .checked_add(config.params.epoch_duration)
            .ok_or(SscreError::Overflow)?;
        require!(now >= next_start, SscreError::EpochTooSoon);
    }
    let budget = config
        .next_epoch_budget
        .min(ctx.accounts.reserve_vault.amount);
    require!(budget > 0, SscreError::ReserveEmpty);

    let bump = [config.bump];
    let seeds: &[&[u8]] = &[POOL_CONFIG_SEED, &bump];
    transfer_checked(
        CpiContext::new_with_signer(
            ctx.accounts.token_program.to_account_info(),
            TransferChecked {
                from: ctx.accounts.reserve_vault.to_account_info(),
                mint: ctx.accounts.vcoin_mint.to_account_info(),
                to: ctx.accounts.epoch_vault.to_account_info(),
                authority: config.to_account_info(),
            },
            &[seeds],
        ),
        budget,
        ctx.accounts.vcoin_mint.decimals,
    )?;

    let epoch = &mut ctx.accounts.epoch;
    epoch.index = config.epoch_count;
    epoch.budget = budget;
    epoch.vault = ctx.accounts.epoch_vault.key();
    epoch.start_ts = now;
    epoch.end_ts = now
        .checked_add(config.params.epoch_duration)
        .ok_or(SscreError::Overflow)?;
    epoch.pause_accum_at_start = config.pause_accum;
    epoch.votes = [OracleVote::default(); MAX_ORACLES];
    epoch.state = EpochState::Voting;
    epoch.rent_payer = ctx.accounts.payer.key();
    epoch.bump = ctx.bumps.epoch;
    epoch.vault_bump = ctx.bumps.epoch_vault;

    let decayed = (config.next_epoch_budget as u128
        * (BPS - config.params.decay_bps as u64) as u128
        / BPS as u128) as u64;
    config.next_epoch_budget = decayed;
    config.epoch_count = config
        .epoch_count
        .checked_add(1)
        .ok_or(SscreError::Overflow)?;
    config.last_epoch_start = now;

    emit!(EpochStarted {
        index: epoch.index,
        budget,
        start_ts: epoch.start_ts,
        end_ts: epoch.end_ts,
        next_epoch_budget: decayed,
    });
    Ok(())
}

/// One changeable vote per active oracle, after the epoch ends. When `root_quorum` votes
/// agree on `(root, max_total_claim)`, that candidate is proposed and the dispute delay
/// starts; if it later loses quorum the proposal is dropped.
pub fn vote_root(ctx: Context<VoteRoot>, root: [u8; 32], max_total_claim: u64) -> Result<()> {
    let config = &ctx.accounts.config;
    require!(!config.paused, SscreError::Paused);
    let now = now()?;
    let oracle = ctx.accounts.oracle.key();
    require!(
        config.is_active_oracle(&oracle, now),
        SscreError::NotActiveOracle
    );
    let epoch = &mut ctx.accounts.epoch;
    require!(epoch.state == EpochState::Voting, SscreError::NotVoting);
    require!(now >= epoch.end_ts, SscreError::EpochNotEnded);
    require!(
        now < epoch.root_deadline(config, now),
        SscreError::RootDeadlinePassed
    );
    require!(
        root != [0u8; 32] && max_total_claim > 0 && max_total_claim <= epoch.budget,
        SscreError::InvalidRoot
    );

    let vote = OracleVote {
        oracle,
        root,
        max_total_claim,
    };
    let slot = match epoch.votes.iter().position(|v| v.oracle == oracle) {
        Some(i) => i,
        None => epoch
            .votes
            .iter()
            .position(|v| v.oracle == Pubkey::default() || !config.is_active_oracle(&v.oracle, now))
            .ok_or(SscreError::TooManyOracles)?,
    };
    epoch.votes[slot] = vote;

    if epoch.proposed_at != 0 {
        let (proposed, max) = (epoch.proposed_root, epoch.proposed_max_total);
        if epoch.count_votes(config, &proposed, max, now) < config.root_quorum {
            epoch.proposed_at = 0;
        }
    }
    let support = epoch.count_votes(config, &root, max_total_claim, now);
    if support >= config.root_quorum && epoch.proposed_at == 0 {
        epoch.proposed_root = root;
        epoch.proposed_max_total = max_total_claim;
        epoch.proposed_at = now;
        emit!(RootProposed {
            index: epoch.index,
            root,
            max_total_claim,
            claimable_at: now.saturating_add(config.params.dispute_delay),
        });
    }
    emit!(RootVoted {
        index: epoch.index,
        oracle,
        root,
        max_total_claim,
        votes_for_candidate: support
    });
    Ok(())
}

/// The DAO discards the proposed root and every vote; oracles must vote again.
pub fn veto_root(ctx: Context<VetoRoot>) -> Result<()> {
    ctx.accounts
        .config
        .admin
        .require_authority(&ctx.accounts.authority.key())?;
    let epoch = &mut ctx.accounts.epoch;
    require!(epoch.state == EpochState::Voting, SscreError::NotVoting);
    let vetoed = epoch.proposed_root;
    epoch.votes = [OracleVote::default(); MAX_ORACLES];
    epoch.proposed_at = 0;
    emit!(RootVetoed {
        index: epoch.index,
        root: vetoed
    });
    Ok(())
}

/// Anyone activates a root that kept quorum through the dispute delay (audit SSC-02/04).
pub fn activate_root(ctx: Context<ActivateRoot>) -> Result<()> {
    let config = &ctx.accounts.config;
    require!(!config.paused, SscreError::Paused);
    let now = now()?;
    let epoch = &mut ctx.accounts.epoch;
    require!(epoch.state == EpochState::Voting, SscreError::NotVoting);
    require!(epoch.proposed_at != 0, SscreError::NoProposedRoot);
    let claimable_at = epoch
        .proposed_at
        .checked_add(config.params.dispute_delay)
        .ok_or(SscreError::Overflow)?;
    require!(now >= claimable_at, SscreError::DisputeDelayActive);
    require!(
        now < epoch.root_deadline(config, now),
        SscreError::RootDeadlinePassed
    );
    let (root, max_total) = (epoch.proposed_root, epoch.proposed_max_total);
    require!(
        epoch.count_votes(config, &root, max_total, now) >= config.root_quorum,
        SscreError::QuorumLost
    );

    epoch.root = root;
    epoch.max_total_claim = max_total;
    epoch.fee_bps = config.fee_bps;
    epoch.activated_at = now;
    epoch.claim_deadline = now
        .checked_add(config.params.claim_window)
        .ok_or(SscreError::Overflow)?;
    epoch.pause_accum_at_activation = config.paused_total(now);
    epoch.state = EpochState::Active;
    emit!(RootActivated {
        index: epoch.index,
        root,
        max_total_claim: max_total,
        claim_deadline: epoch.claim_deadline
    });
    Ok(())
}

/// Claims `amount` (gross) for the signer. The fee (rounded up) goes to the fee account.
pub fn claim(ctx: Context<Claim>, amount: u64, proof: Vec<[u8; 32]>) -> Result<()> {
    let config = &ctx.accounts.config;
    require!(!config.paused, SscreError::Paused);
    let now = now()?;
    let claimant = ctx.accounts.claimant.key();
    let epoch = &mut ctx.accounts.epoch;
    require!(epoch.state == EpochState::Active, SscreError::NotActive);
    require!(
        now <= epoch.effective_claim_deadline(config, now),
        SscreError::ClaimWindowClosed
    );
    require!(amount > 0, SscreError::ZeroAmount);
    require!(proof.len() <= MAX_PROOF_LEN, SscreError::ProofTooLong);
    let leaf = compute_leaf(&claimant, amount, epoch.index);
    require!(
        verify_proof(&proof, &epoch.root, &leaf),
        SscreError::InvalidProof
    );
    let total = epoch
        .total_claimed
        .checked_add(amount)
        .ok_or(SscreError::Overflow)?;
    require!(
        total <= epoch.max_total_claim,
        SscreError::ExceedsMaxTotalClaim
    );

    let fee = (amount as u128 * epoch.fee_bps as u128).div_ceil(BPS as u128) as u64;
    let net = amount - fee; // fee ≤ 1% of amount
    let bump = [config.bump];
    let seeds: &[&[u8]] = &[POOL_CONFIG_SEED, &bump];
    let decimals = ctx.accounts.vcoin_mint.decimals;
    for (to, value) in [
        (ctx.accounts.claimant_token.to_account_info(), net),
        (ctx.accounts.fee_destination.to_account_info(), fee),
    ] {
        if value == 0 {
            continue;
        }
        transfer_checked(
            CpiContext::new_with_signer(
                ctx.accounts.token_program.to_account_info(),
                TransferChecked {
                    from: ctx.accounts.vault.to_account_info(),
                    mint: ctx.accounts.vcoin_mint.to_account_info(),
                    to,
                    authority: config.to_account_info(),
                },
                &[seeds],
            ),
            value,
            decimals,
        )?;
    }

    epoch.total_claimed = total;
    epoch.num_claims = epoch
        .num_claims
        .checked_add(1)
        .ok_or(SscreError::Overflow)?;
    let status = &mut ctx.accounts.claim_status;
    status.epoch = epoch.key();
    status.claimant = claimant;
    status.amount = amount;
    status.claimed_at = now;
    status.rent_payer = ctx.accounts.payer.key();
    status.bump = ctx.bumps.claim_status;

    emit!(RewardClaimed {
        index: epoch.index,
        claimant,
        gross: amount,
        fee,
        total_claimed: total
    });
    Ok(())
}

/// After the claim deadline (or the root deadline, if no root activated), anyone returns
/// the remaining budget to the reserve and closes the epoch vault.
pub fn clawback(ctx: Context<Clawback>) -> Result<()> {
    let config = &ctx.accounts.config;
    require!(!config.paused, SscreError::Paused);
    let now = now()?;
    let epoch = &mut ctx.accounts.epoch;
    let closed = match epoch.state {
        EpochState::Active => now > epoch.effective_claim_deadline(config, now),
        EpochState::Voting => now >= epoch.root_deadline(config, now),
        EpochState::Closed => return err!(SscreError::NotActive),
    };
    require!(closed, SscreError::ClaimWindowOpen);

    let returned = ctx.accounts.vault.amount;
    let bump = [config.bump];
    let seeds: &[&[u8]] = &[POOL_CONFIG_SEED, &bump];
    if returned > 0 {
        transfer_checked(
            CpiContext::new_with_signer(
                ctx.accounts.token_program.to_account_info(),
                TransferChecked {
                    from: ctx.accounts.vault.to_account_info(),
                    mint: ctx.accounts.vcoin_mint.to_account_info(),
                    to: ctx.accounts.reserve_vault.to_account_info(),
                    authority: config.to_account_info(),
                },
                &[seeds],
            ),
            returned,
            ctx.accounts.vcoin_mint.decimals,
        )?;
    }
    close_account(CpiContext::new_with_signer(
        ctx.accounts.token_program.to_account_info(),
        CloseAccount {
            account: ctx.accounts.vault.to_account_info(),
            destination: ctx.accounts.rent_payer.to_account_info(),
            authority: config.to_account_info(),
        },
        &[seeds],
    ))?;
    epoch.state = EpochState::Closed;
    emit!(EpochClawedBack {
        index: epoch.index,
        returned,
        total_claimed: epoch.total_claimed
    });
    Ok(())
}

/// Returns a claim marker's rent once its epoch is closed (no double claim is possible then).
pub fn close_claim_status(ctx: Context<CloseClaimStatus>) -> Result<()> {
    require!(
        ctx.accounts.epoch.state == EpochState::Closed,
        SscreError::EpochNotClosed
    );
    Ok(())
}
