use anchor_lang::prelude::*;
use anchor_spl::token_2022;

use crate::constants::VEVCOIN_CONFIG_SEED;
use crate::contexts::MintVeVCoin;
use crate::errors::VeVCoinError;
use crate::events::VeVCoinMinted;

/// Mint veVCoin to a user (only callable by staking protocol)
/// Called when user stakes VCoin
pub fn handler(ctx: Context<MintVeVCoin>, amount: u64) -> Result<()> {
    // Get bump from context
    let config_bump = ctx.bumps.config;
    let user_account_bump = ctx.bumps.user_account;
    
    // Only staking protocol can mint
    require!(
        ctx.accounts.staking_protocol.key() == ctx.accounts.config.staking_protocol,
        VeVCoinError::Unauthorized
    );
    
    require!(amount > 0, VeVCoinError::ZeroAmount);
    
    let clock = Clock::get()?;
    let now = clock.unix_timestamp;
    
    // Check if first time user
    let is_new_user = ctx.accounts.user_account.balance == 0;
    let current_balance = ctx.accounts.user_account.balance;
    let current_total_supply = ctx.accounts.config.total_supply;
    let current_total_holders = ctx.accounts.config.total_holders;
    
    // Mint tokens using Token-2022 first
    let seeds = &[
        VEVCOIN_CONFIG_SEED,
        &[config_bump],
    ];
    let signer_seeds = &[&seeds[..]];
    
    token_2022::mint_to(
        CpiContext::new_with_signer(
            ctx.accounts.token_program.to_account_info(),
            token_2022::MintTo {
                mint: ctx.accounts.mint.to_account_info(),
                to: ctx.accounts.user_token_account.to_account_info(),
                authority: ctx.accounts.config.to_account_info(),
            },
            signer_seeds,
        ),
        amount,
    )?;
    
    // Now update state
    let user_account = &mut ctx.accounts.user_account;
    let config = &mut ctx.accounts.config;
    
    // Initialize user account if first time
    if is_new_user {
        user_account.owner = ctx.accounts.user.key();
        user_account.first_mint_at = now;
        user_account.bump = user_account_bump;
        config.total_holders = current_total_holders.checked_add(1).unwrap();
    }
    
    // Update balances
    user_account.balance = current_balance.checked_add(amount).unwrap();
    user_account.last_update_at = now;
    config.total_supply = current_total_supply.checked_add(amount).unwrap();
    
    // L-01: Emit veVCoin minted event
    emit!(VeVCoinMinted {
        user: ctx.accounts.user.key(),
        amount,
        total_supply: config.total_supply,
        timestamp: now,
    });
    
    msg!("Minted {} veVCoin to {}", amount, ctx.accounts.user.key());
    msg!("New balance: {}", user_account.balance);
    
    Ok(())
}

