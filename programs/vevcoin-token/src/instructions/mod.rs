//! veVCoin handlers.

use anchor_lang::prelude::*;
use anchor_lang::system_program::{
    allocate, assign, create_account, transfer, Allocate, Assign, CreateAccount, Transfer,
};
use anchor_spl::token_2022::spl_token_2022::{extension::ExtensionType, state::Mint as MintState};
use anchor_spl::token_interface::{
    burn, initialize_mint2, mint_to, non_transferable_mint_initialize,
    permanent_delegate_initialize, Burn, InitializeMint2, MintTo, NonTransferableMintInitialize,
    PermanentDelegateInitialize,
};
use vcoin_common::{require_upgrade_authority, AdminState, AUTHORITY_TRANSFER_DELAY};

use crate::constants::*;
use crate::contexts::*;
use crate::errors::VeVCoinError;
use crate::events::*;

pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
    require_upgrade_authority(&ctx.accounts.program_data, &ctx.accounts.authority.key())?;

    let config_key = ctx.accounts.config.key();
    let mint_info = ctx.accounts.mint.to_account_info();
    let token_program = ctx.accounts.token_program.to_account_info();

    // Create the mint account sized for NonTransferable + PermanentDelegate.
    let space = ExtensionType::try_calculate_account_len::<MintState>(&[
        ExtensionType::NonTransferable,
        ExtensionType::PermanentDelegate,
    ])?;
    let mint_bump = [ctx.bumps.mint];
    let mint_seeds: &[&[u8]] = &[VEVCOIN_MINT_SEED, &mint_bump];
    let rent = Rent::get()?.minimum_balance(space);
    let system_program = ctx.accounts.system_program.to_account_info();
    if mint_info.lamports() == 0 {
        create_account(
            CpiContext::new_with_signer(
                system_program,
                CreateAccount {
                    from: ctx.accounts.authority.to_account_info(),
                    to: mint_info.clone(),
                },
                &[mint_seeds],
            ),
            rent,
            space as u64,
            token_program.key,
        )?;
    } else {
        // The PDA already holds lamports (anyone can send some to a known address), and
        // `create_account` refuses such accounts. Top up, allocate and assign instead, as
        // Anchor's `init` does, so a donation cannot block initialization.
        let top_up = rent.saturating_sub(mint_info.lamports());
        if top_up > 0 {
            transfer(
                CpiContext::new(
                    system_program.clone(),
                    Transfer {
                        from: ctx.accounts.authority.to_account_info(),
                        to: mint_info.clone(),
                    },
                ),
                top_up,
            )?;
        }
        allocate(
            CpiContext::new_with_signer(
                system_program.clone(),
                Allocate {
                    account_to_allocate: mint_info.clone(),
                },
                &[mint_seeds],
            ),
            space as u64,
        )?;
        assign(
            CpiContext::new_with_signer(
                system_program,
                Assign {
                    account_to_assign: mint_info.clone(),
                },
                &[mint_seeds],
            ),
            token_program.key,
        )?;
    }

    // Extensions must be initialized before the base mint.
    non_transferable_mint_initialize(CpiContext::new(
        token_program.clone(),
        NonTransferableMintInitialize {
            token_program_id: token_program.clone(),
            mint: mint_info.clone(),
        },
    ))?;
    permanent_delegate_initialize(
        CpiContext::new(
            token_program.clone(),
            PermanentDelegateInitialize {
                token_program_id: token_program.clone(),
                mint: mint_info.clone(),
            },
        ),
        &config_key,
    )?;
    initialize_mint2(
        CpiContext::new(token_program, InitializeMint2 { mint: mint_info }),
        VEVCOIN_DECIMALS,
        &config_key,
        None,
    )?;

    let staking_authority =
        Pubkey::find_program_address(&[STAKING_REGISTRAR_SEED], &STAKING_PROGRAM_ID).0;
    let config = &mut ctx.accounts.config;
    config.admin = AdminState::new(ctx.accounts.authority.key());
    config.mint = ctx.accounts.mint.key();
    config.staking_authority = staking_authority;
    config.bump = ctx.bumps.config;
    config.mint_bump = ctx.bumps.mint;

    emit!(VeVCoinInitialized {
        config: config_key,
        mint: config.mint,
        authority: ctx.accounts.authority.key(),
        staking_authority,
    });
    Ok(())
}

pub fn mint_vevcoin(ctx: Context<MintVeVCoin>, amount: u64) -> Result<()> {
    require!(amount > 0, VeVCoinError::ZeroAmount);
    let bump = [ctx.accounts.config.bump];
    let seeds: &[&[u8]] = &[VEVCOIN_CONFIG_SEED, &bump];
    mint_to(
        CpiContext::new_with_signer(
            ctx.accounts.token_program.to_account_info(),
            MintTo {
                mint: ctx.accounts.mint.to_account_info(),
                to: ctx.accounts.destination.to_account_info(),
                authority: ctx.accounts.config.to_account_info(),
            },
            &[seeds],
        ),
        amount,
    )?;
    emit!(VeVCoinMinted {
        destination: ctx.accounts.destination.key(),
        amount
    });
    Ok(())
}

/// Burns `min(amount, balance)` using the config PDA as permanent delegate, so a holder
/// who already burned their own receipt can never be blocked from unstaking (audit VEV-01).
pub fn burn_vevcoin(ctx: Context<BurnVeVCoin>, amount: u64) -> Result<()> {
    let burned = amount.min(ctx.accounts.source.amount);
    if burned > 0 {
        let bump = [ctx.accounts.config.bump];
        let seeds: &[&[u8]] = &[VEVCOIN_CONFIG_SEED, &bump];
        burn(
            CpiContext::new_with_signer(
                ctx.accounts.token_program.to_account_info(),
                Burn {
                    mint: ctx.accounts.mint.to_account_info(),
                    from: ctx.accounts.source.to_account_info(),
                    authority: ctx.accounts.config.to_account_info(),
                },
                &[seeds],
            ),
            burned,
        )?;
    }
    emit!(VeVCoinBurned {
        source: ctx.accounts.source.key(),
        requested: amount,
        burned
    });
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
    let admin = &mut ctx.accounts.config.admin;
    let previous = admin.accept(&ctx.accounts.new_authority.key(), now)?;
    emit!(AuthorityTransferred {
        previous,
        new_authority: admin.authority
    });
    Ok(())
}
