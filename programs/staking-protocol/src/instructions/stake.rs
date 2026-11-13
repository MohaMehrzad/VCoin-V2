//! Voter deposits: create, deposit, extend, withdraw, close.

use anchor_lang::prelude::*;
use anchor_spl::token_interface::{transfer_checked, TransferChecked};

use crate::constants::*;
use crate::contexts::*;
use crate::errors::StakingError;
use crate::events::*;
use crate::state::DepositEntry;

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
    require!(!ctx.accounts.registrar.paused, StakingError::Paused);
    require!(amount > 0, StakingError::ZeroAmount);
    let i = check_index(index)?;
    check_lock_secs(lock_secs)?;
    let now = Clock::get()?.unix_timestamp;
    let requested_end = now.checked_add(lock_secs).ok_or(StakingError::Overflow)?;

    // Credit exactly what the vault receives.
    let before = ctx.accounts.vault.amount;
    transfer_checked(
        CpiContext::new(
            ctx.accounts.token_program.to_account_info(),
            TransferChecked {
                from: ctx.accounts.owner_vcoin.to_account_info(),
                mint: ctx.accounts.vcoin_mint.to_account_info(),
                to: ctx.accounts.vault.to_account_info(),
                authority: ctx.accounts.owner.to_account_info(),
            },
        ),
        amount,
        ctx.accounts.vcoin_mint.decimals,
    )?;
    ctx.accounts.vault.reload()?;
    let credited = ctx
        .accounts
        .vault
        .amount
        .checked_sub(before)
        .ok_or(StakingError::Overflow)?;
    require!(credited > 0, StakingError::NothingReceived);

    let entry = &mut ctx.accounts.voter.deposits[i];
    if entry.is_used {
        require!(entry.lock_end > now, StakingError::DepositExpired);
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
        };
    }
    let lock_end = entry.lock_end;

    let registrar = &mut ctx.accounts.registrar;
    registrar.total_locked = registrar
        .total_locked
        .checked_add(credited)
        .ok_or(StakingError::Overflow)?;

    mint_receipt(&ctx, credited)?;

    emit!(Deposited {
        owner: ctx.accounts.owner.key(),
        index,
        amount: credited,
        lock_end,
        total_locked: ctx.accounts.registrar.total_locked,
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
    emit!(LockExtended {
        owner: ctx.accounts.owner.key(),
        index,
        old_lock_end: old_end,
        new_lock_end: new_end
    });
    Ok(())
}

/// Returns unlocked VCoin and burns the matching receipt. Never paused: users can always
/// exit once their lock ends (pausing must not trap funds).
pub fn withdraw(ctx: Context<MoveStake>, index: u8, amount: u64) -> Result<()> {
    require!(amount > 0, StakingError::ZeroAmount);
    let i = check_index(index)?;
    let now = Clock::get()?.unix_timestamp;
    {
        let entry = &ctx.accounts.voter.deposits[i];
        require!(entry.is_used, StakingError::DepositNotUsed);
        require!(now >= entry.lock_end, StakingError::StillLocked);
        require!(amount <= entry.amount, StakingError::InsufficientDeposit);
    }

    let bump = [ctx.accounts.registrar.bump];
    let seeds: &[&[u8]] = &[REGISTRAR_SEED, &bump];
    transfer_checked(
        CpiContext::new_with_signer(
            ctx.accounts.token_program.to_account_info(),
            TransferChecked {
                from: ctx.accounts.vault.to_account_info(),
                mint: ctx.accounts.vcoin_mint.to_account_info(),
                to: ctx.accounts.owner_vcoin.to_account_info(),
                authority: ctx.accounts.registrar.to_account_info(),
            },
            &[seeds],
        ),
        amount,
        ctx.accounts.vcoin_mint.decimals,
    )?;

    burn_receipt(&ctx, amount)?;

    let entry = &mut ctx.accounts.voter.deposits[i];
    entry.amount -= amount; // amount <= entry.amount checked above
    if entry.amount == 0 {
        *entry = DepositEntry::default();
    }
    let registrar = &mut ctx.accounts.registrar;
    registrar.total_locked = registrar
        .total_locked
        .checked_sub(amount)
        .ok_or(StakingError::Overflow)?;

    emit!(Withdrawn {
        owner: ctx.accounts.owner.key(),
        index,
        amount,
        total_locked: ctx.accounts.registrar.total_locked,
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

fn registrar_signer_seeds(bump: &[u8; 1]) -> [&[u8]; 2] {
    [REGISTRAR_SEED, bump]
}

fn mint_receipt(ctx: &Context<MoveStake>, amount: u64) -> Result<()> {
    let bump = [ctx.accounts.registrar.bump];
    let seeds = registrar_signer_seeds(&bump);
    vevcoin_token::cpi::mint_vevcoin(
        CpiContext::new_with_signer(
            ctx.accounts.vevcoin_program.to_account_info(),
            vevcoin_token::cpi::accounts::MintVeVCoin {
                config: ctx.accounts.vevcoin_config.to_account_info(),
                mint: ctx.accounts.vevcoin_mint.to_account_info(),
                destination: ctx.accounts.owner_vevcoin.to_account_info(),
                staking_authority: ctx.accounts.registrar.to_account_info(),
                token_program: ctx.accounts.token_program.to_account_info(),
            },
            &[&seeds],
        ),
        amount,
    )
}

fn burn_receipt(ctx: &Context<MoveStake>, amount: u64) -> Result<()> {
    let bump = [ctx.accounts.registrar.bump];
    let seeds = registrar_signer_seeds(&bump);
    vevcoin_token::cpi::burn_vevcoin(
        CpiContext::new_with_signer(
            ctx.accounts.vevcoin_program.to_account_info(),
            vevcoin_token::cpi::accounts::BurnVeVCoin {
                config: ctx.accounts.vevcoin_config.to_account_info(),
                mint: ctx.accounts.vevcoin_mint.to_account_info(),
                source: ctx.accounts.owner_vevcoin.to_account_info(),
                staking_authority: ctx.accounts.registrar.to_account_info(),
                token_program: ctx.accounts.token_program.to_account_info(),
            },
            &[&seeds],
        ),
        amount,
    )
}
