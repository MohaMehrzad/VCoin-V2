use anchor_lang::prelude::*;
use anchor_spl::token_2022;

use crate::constants::{TOTAL_SUPPLY, VCOIN_CONFIG_SEED};
use crate::contexts::MintTokens;
use crate::errors::VCoinError;
use crate::events::TokensMinted;

/// Mint VCoin tokens to a specified account
/// Only the authority can mint tokens
pub fn handler(ctx: Context<MintTokens>, amount: u64) -> Result<()> {
    // Get config values for validation first
    let authority = ctx.accounts.config.authority;
    let paused = ctx.accounts.config.paused;
    let total_minted = ctx.accounts.config.total_minted;
    let bump = ctx.accounts.config.bump;
    
    // Check authorization
    require!(
        ctx.accounts.authority.key() == authority,
        VCoinError::Unauthorized
    );
    
    // Check not paused
    require!(!paused, VCoinError::TokenPaused);
    
    // Check supply limit
    require!(
        total_minted.checked_add(amount).unwrap() <= TOTAL_SUPPLY,
        VCoinError::ExceedsMaxSupply
    );
    
    // Mint tokens using Token-2022
    let seeds = &[
        VCOIN_CONFIG_SEED,
        &[bump],
    ];
    let signer_seeds = &[&seeds[..]];
    
    token_2022::mint_to(
        CpiContext::new_with_signer(
            ctx.accounts.token_program.to_account_info(),
            token_2022::MintTo {
                mint: ctx.accounts.mint.to_account_info(),
                to: ctx.accounts.destination.to_account_info(),
                authority: ctx.accounts.config.to_account_info(),
            },
            signer_seeds,
        ),
        amount,
    )?;
    
    // Update total minted
    ctx.accounts.config.total_minted = total_minted.checked_add(amount).unwrap();
    
    let clock = Clock::get()?;
    
    // L-01: Emit tokens minted event
    emit!(TokensMinted {
        recipient: ctx.accounts.destination.key(),
        amount,
        total_minted: ctx.accounts.config.total_minted,
        timestamp: clock.unix_timestamp,
    });
    
    msg!("Minted {} VCoin tokens", amount);
    msg!("Total minted: {}", ctx.accounts.config.total_minted);
    
    Ok(())
}

