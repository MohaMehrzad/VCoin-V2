//! Slashing of staked deposits (audit TOK-02/03, MON-6/7).
//!
//! Lifecycle: the registrar authority (the Realms governance PDA after handover, so every
//! slash is a DAO vote) proposes a slash with an evidence hash. The owner's withdrawals
//! freeze immediately. After a 48h appeal window anyone may execute it, taking
//! `min(amount, deposits)` from the vault to the slash destination and burning the matching
//! receipts; the authority may cancel it at any time before. A request not executed within
//! 7 days expires and anyone may close it, which unfreezes the owner.

use anchor_lang::prelude::*;

use crate::constants::*;
use crate::contexts::*;
use crate::errors::StakingError;
use crate::events::*;
use crate::state::DepositEntry;

use super::token_ops::{burn_receipt, vault_transfer, ReceiptAccounts, RegistrarSigner};

pub fn propose_slash(ctx: Context<ProposeSlash>, amount: u64, reason_hash: [u8; 32]) -> Result<()> {
    let registrar = &mut ctx.accounts.registrar;
    registrar
        .admin
        .require_authority(&ctx.accounts.authority.key())?;
    require!(
        registrar.slash_destination != Pubkey::default(),
        StakingError::GovernanceNotConfigured
    );
    require!(amount > 0, StakingError::ZeroAmount);

    let now = Clock::get()?.unix_timestamp;
    let executable_at = now.checked_add(SLASH_DELAY).ok_or(StakingError::Overflow)?;
    let expires_at = executable_at
        .checked_add(SLASH_EXECUTION_WINDOW)
        .ok_or(StakingError::Overflow)?;
    let voter = &mut ctx.accounts.voter;
    voter.pending_slashes = voter
        .pending_slashes
        .checked_add(1)
        .ok_or(StakingError::Overflow)?;

    let request = &mut ctx.accounts.slash_request;
    request.voter = voter.key();
    request.owner = voter.owner;
    request.amount = amount;
    request.reason_hash = reason_hash;
    request.proposed_at = now;
    request.executable_at = executable_at;
    request.expires_at = expires_at;
    request.nonce = registrar.slash_nonce;
    request.rent_payer = ctx.accounts.payer.key();
    request.bump = ctx.bumps.slash_request;
    registrar.slash_nonce = registrar
        .slash_nonce
        .checked_add(1)
        .ok_or(StakingError::Overflow)?;

    emit!(SlashProposed {
        request: request.key(),
        owner: request.owner,
        amount,
        reason_hash,
        executable_at,
        expires_at,
    });
    Ok(())
}

fn release(voter: &mut crate::state::Voter) -> Result<()> {
    voter.pending_slashes = voter
        .pending_slashes
        .checked_sub(1)
        .ok_or(StakingError::Overflow)?;
    Ok(())
}

/// Authority withdraws a pending request (e.g. after a successful appeal).
pub fn cancel_slash(ctx: Context<CancelSlash>) -> Result<()> {
    let close = &mut ctx.accounts.close;
    close
        .registrar
        .admin
        .require_authority(&ctx.accounts.authority.key())?;
    release(&mut close.voter)?;
    emit!(SlashCancelled {
        request: close.slash_request.key(),
        owner: close.slash_request.owner
    });
    Ok(())
}

/// Anyone may close a request whose execution window has passed.
pub fn expire_slash(ctx: Context<CloseSlash>) -> Result<()> {
    let now = Clock::get()?.unix_timestamp;
    require!(
        now >= ctx.accounts.slash_request.expires_at,
        StakingError::SlashNotExpired
    );
    release(&mut ctx.accounts.voter)?;
    emit!(SlashExpired {
        request: ctx.accounts.slash_request.key(),
        owner: ctx.accounts.slash_request.owner
    });
    Ok(())
}

/// Anyone may execute a request inside `[executable_at, expires_at)`.
pub fn execute_slash(ctx: Context<ExecuteSlash>) -> Result<()> {
    let now = Clock::get()?.unix_timestamp;
    let (requested, executable_at, expires_at) = {
        let r = &ctx.accounts.slash_request;
        (r.amount, r.executable_at, r.expires_at)
    };
    require!(now >= executable_at, StakingError::SlashNotReady);
    require!(now < expires_at, StakingError::SlashWindowClosed);

    // Take from deposits in slot order, locked or not (withdrawals were frozen since proposal).
    let voter = &mut ctx.accounts.voter;
    let mut remaining = requested;
    let mut active_taken = 0u64;
    for entry in voter.deposits.iter_mut().filter(|d| d.is_used) {
        if remaining == 0 {
            break;
        }
        let take = remaining.min(entry.amount);
        if entry.is_active {
            active_taken += take; // ≤ requested
        }
        entry.amount -= take;
        remaining -= take;
        if entry.amount == 0 {
            *entry = DepositEntry::default();
        }
    }
    let slashed = requested - remaining;
    release(voter)?;

    let accs = &ctx.accounts;
    let signer = RegistrarSigner {
        info: accs.registrar.to_account_info(),
        bump: accs.registrar.bump,
    };
    let receipts_burned = slashed.min(accs.owner_vevcoin.amount);
    if slashed > 0 {
        vault_transfer(
            accs.token_program.to_account_info(),
            accs.vault.to_account_info(),
            accs.vcoin_mint.to_account_info(),
            accs.slash_destination.to_account_info(),
            &signer,
            slashed,
            accs.vcoin_mint.decimals,
        )?;
    }
    if receipts_burned > 0 {
        let receipt = ReceiptAccounts {
            vevcoin_program: accs.vevcoin_program.to_account_info(),
            config: accs.vevcoin_config.to_account_info(),
            mint: accs.vevcoin_mint.to_account_info(),
            account: accs.owner_vevcoin.to_account_info(),
            token_program: accs.token_program.to_account_info(),
        };
        burn_receipt(receipt, &signer, receipts_burned)?;
    }

    let registrar = &mut ctx.accounts.registrar;
    registrar.active_locked = registrar
        .active_locked
        .checked_sub(active_taken)
        .ok_or(StakingError::Overflow)?;
    registrar.total_locked = registrar
        .total_locked
        .checked_sub(slashed)
        .ok_or(StakingError::Overflow)?;
    emit!(SlashExecuted {
        request: ctx.accounts.slash_request.key(),
        owner: ctx.accounts.slash_request.owner,
        requested,
        slashed,
        receipts_burned,
        total_locked: registrar.total_locked,
    });
    Ok(())
}

/// Changes where slashed VCoin goes. Requires governance to be wired first.
pub fn set_slash_destination(ctx: Context<SetSlashDestination>) -> Result<()> {
    let registrar = &mut ctx.accounts.registrar;
    registrar
        .admin
        .require_authority(&ctx.accounts.authority.key())?;
    require!(
        registrar.realm != Pubkey::default(),
        StakingError::GovernanceNotConfigured
    );
    let previous = registrar.slash_destination;
    registrar.slash_destination = ctx.accounts.slash_destination.key();
    emit!(SlashDestinationChanged {
        previous,
        new_destination: registrar.slash_destination
    });
    Ok(())
}
