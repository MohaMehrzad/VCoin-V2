//! Voter deposits: create, deposit, extend, withdraw, close.

use anchor_lang::prelude::*;
use anchor_spl::token_interface::{transfer_checked, TransferChecked};

use crate::constants::*;
use crate::contexts::*;
use crate::errors::StakingError;
use crate::events::*;
use crate::governance::assert_can_withdraw;
use crate::state::DepositEntry;

use super::token_ops::{
    burn_receipt, mint_receipt, vault_transfer, ReceiptAccounts, RegistrarSigner,
};

pub fn create_voter(ctx: Context<CreateVoter>) -> Result<()> {
    let voter = &mut ctx.accounts.voter;
    voter.owner = ctx.accounts.owner.key();
    voter.bump = ctx.bumps.voter;
    emit!(VoterCreated {
        voter: voter.key(),
        owner: voter.owner
    });
    Ok(())
}

fn check_index(index: u8) -> Result<usize> {
    let i = index as usize;
    require!(i < MAX_DEPOSITS, StakingError::InvalidDepositIndex);
    Ok(i)
}

fn check_lock_secs(lock_secs: i64) -> Result<()> {
    require!(
        (MIN_LOCK_SECS..=MAX_LOCK_SECS).contains(&lock_secs),
        StakingError::InvalidLockDuration
    );
    Ok(())
}

/// Locks `amount` VCoin in deposit slot `index` until at least `now + lock_secs` and mints
/// a 1:1 non-transferable veVCoin receipt. Adding to an existing (unexpired) slot keeps
/// the later of the two lock ends.
pub fn deposit(ctx: Context<MoveStake>, index: u8, amount: u64, lock_secs: i64) -> Result<()> {
    let accs = ctx.accounts;
    require!(!accs.registrar.paused, StakingError::Paused);
    require!(amount > 0, StakingError::ZeroAmount);
    let i = check_index(index)?;
    check_lock_secs(lock_secs)?;
    let now = Clock::get()?.unix_timestamp;
    let requested_end = now.checked_add(lock_secs).ok_or(StakingError::Overflow)?;

    // Credit exactly what the vault receives.
    let before = accs.vault.amount;
    transfer_checked(
        CpiContext::new(
            accs.token_program.to_account_info(),
            TransferChecked {
                from: accs.owner_vcoin.to_account_info(),
                mint: accs.vcoin_mint.to_account_info(),
                to: accs.vault.to_account_info(),
                authority: accs.owner.to_account_info(),
            },
        ),
        amount,
        accs.vcoin_mint.decimals,
    )?;
    accs.vault.reload()?;
    let credited = accs
        .vault
        .amount
        .checked_sub(before)
        .ok_or(StakingError::Overflow)?;
    require!(credited > 0, StakingError::NothingReceived);

    let entry = &mut accs.voter.deposits[i];
    let mut activated = credited;
    if entry.is_used {
        require!(entry.lock_end > now, StakingError::DepositExpired);
        if !entry.is_active {
            activated = activated
                .checked_add(entry.amount)
                .ok_or(StakingError::Overflow)?;
            entry.is_active = true;
        }
        entry.amount = entry
            .amount
            .checked_add(credited)
            .ok_or(StakingError::Overflow)?;
        entry.lock_end = entry.lock_end.max(requested_end);
    } else {
        *entry = DepositEntry {
            amount: credited,
            lock_start: now,
            lock_end: requested_end,
            is_used: true,
            is_active: true,
        };
    }
    let lock_end = entry.lock_end;
    accs.registrar.active_locked = accs
        .registrar
        .active_locked
        .checked_add(activated)
        .ok_or(StakingError::Overflow)?;
    accs.registrar.total_locked = accs
        .registrar
        .total_locked
        .checked_add(credited)
        .ok_or(StakingError::Overflow)?;

    mint_receipt(receipt_accounts(accs), &registrar_signer(accs), credited)?;

    emit!(Deposited {
        owner: accs.owner.key(),
        index,
        amount: credited,
        lock_end,
        total_locked: accs.registrar.total_locked,
    });
    Ok(())
}

/// Moves a deposit's lock end later (never earlier). Expired deposits may be relocked.
pub fn extend_lock(ctx: Context<ExtendLock>, index: u8, lock_secs: i64) -> Result<()> {
    require!(!ctx.accounts.registrar.paused, StakingError::Paused);
    let i = check_index(index)?;
    check_lock_secs(lock_secs)?;
    let now = Clock::get()?.unix_timestamp;
    let new_end = now.checked_add(lock_secs).ok_or(StakingError::Overflow)?;
    let entry = &mut ctx.accounts.voter.deposits[i];
    require!(entry.is_used, StakingError::DepositNotUsed);
    require!(new_end > entry.lock_end, StakingError::LockNotExtended);
    let old_end = entry.lock_end;
    entry.lock_end = new_end;
    if !entry.is_active {
        // Relocking a deactivated (expired) deposit gives it weight again.
        entry.is_active = true;
        let registrar = &mut ctx.accounts.registrar;
        registrar.active_locked = registrar
            .active_locked
            .checked_add(entry.amount)
            .ok_or(StakingError::Overflow)?;
    }
    emit!(LockExtended {
        owner: ctx.accounts.owner.key(),
        index,
        old_lock_end: old_end,
        new_lock_end: new_end
    });
    Ok(())
}

/// Returns unlocked VCoin and burns the matching receipt. Never paused (pausing must not
/// trap funds); blocked while the owner has unrelinquished Realms votes or open proposals
/// or a pending slash.
pub fn withdraw(ctx: Context<Withdraw>, index: u8, amount: u64) -> Result<()> {
    let accs = &mut ctx.accounts.stake;
    require!(amount > 0, StakingError::ZeroAmount);
    let i = check_index(index)?;
    let now = Clock::get()?.unix_timestamp;
    {
        let entry = &accs.voter.deposits[i];
        require!(entry.is_used, StakingError::DepositNotUsed);
        require!(now >= entry.lock_end, StakingError::StillLocked);
        require!(amount <= entry.amount, StakingError::InsufficientDeposit);
    }
    require!(accs.voter.pending_slashes == 0, StakingError::SlashPending);
    assert_can_withdraw(
        &accs.registrar,
        &accs.owner.key(),
        &ctx.accounts.token_owner_record.to_account_info(),
    )?;

    let signer = registrar_signer(accs);
    vault_transfer(
        accs.token_program.to_account_info(),
        accs.vault.to_account_info(),
        accs.vcoin_mint.to_account_info(),
        accs.owner_vcoin.to_account_info(),
        &signer,
        amount,
        accs.vcoin_mint.decimals,
    )?;
    burn_receipt(receipt_accounts(accs), &signer, amount)?;

    let entry = &mut accs.voter.deposits[i];
    if entry.is_active {
        // The lock has ended, so the whole deposit stops counting toward the max weight.
        entry.is_active = false;
        accs.registrar.active_locked = accs
            .registrar
            .active_locked
            .checked_sub(entry.amount)
            .ok_or(StakingError::Overflow)?;
    }
    entry.amount -= amount; // amount <= entry.amount checked above
    if entry.amount == 0 {
        *entry = DepositEntry::default();
    }
    accs.registrar.total_locked = accs
        .registrar
        .total_locked
        .checked_sub(amount)
        .ok_or(StakingError::Overflow)?;

    emit!(Withdrawn {
        owner: accs.owner.key(),
        index,
        amount,
        total_locked: accs.registrar.total_locked
    });
    Ok(())
}

/// Anyone may mark a deposit whose lock has ended as inactive: it has no voting weight, so it
/// must stop counting toward the max voter weight (re-review A2). A relock reactivates it.
pub fn deactivate_expired(ctx: Context<DeactivateExpired>, index: u8) -> Result<()> {
    let i = check_index(index)?;
    let now = Clock::get()?.unix_timestamp;
    let entry = &mut ctx.accounts.voter.deposits[i];
    require!(entry.is_used, StakingError::DepositNotUsed);
    require!(now >= entry.lock_end, StakingError::StillLocked);
    require!(entry.is_active, StakingError::AlreadyDeactivated);
    entry.is_active = false;
    let amount = entry.amount;
    let registrar = &mut ctx.accounts.registrar;
    registrar.active_locked = registrar
        .active_locked
        .checked_sub(amount)
        .ok_or(StakingError::Overflow)?;
    emit!(DepositDeactivated {
        owner: ctx.accounts.voter.owner,
        index,
        amount,
        active_locked: registrar.active_locked,
    });
    Ok(())
}

pub fn close_voter(ctx: Context<CloseVoter>) -> Result<()> {
    let voter = &ctx.accounts.voter;
    require!(
        voter.is_empty() && voter.pending_slashes == 0,
        StakingError::VoterNotEmpty
    );
    Ok(())
}

fn receipt_accounts<'info>(accs: &MoveStake<'info>) -> ReceiptAccounts<'info> {
    ReceiptAccounts {
        vevcoin_program: accs.vevcoin_program.to_account_info(),
        config: accs.vevcoin_config.to_account_info(),
        mint: accs.vevcoin_mint.to_account_info(),
        account: accs.owner_vevcoin.to_account_info(),
        token_program: accs.token_program.to_account_info(),
    }
}

fn registrar_signer<'info>(accs: &MoveStake<'info>) -> RegistrarSigner<'info> {
    RegistrarSigner {
        info: accs.registrar.to_account_info(),
        bump: accs.registrar.bump,
    }
}
