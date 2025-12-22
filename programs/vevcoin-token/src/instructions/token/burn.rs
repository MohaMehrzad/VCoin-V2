use anchor_lang::prelude::*;
use anchor_spl::token_2022;

use crate::constants::VEVCOIN_CONFIG_SEED;
use crate::contexts::BurnVeVCoin;
use crate::errors::VeVCoinError;

/// Burn veVCoin from a user (only callable by staking protocol)
/// Called when user unstakes VCoin
pub fn handler(ctx: Context<BurnVeVCoin>, amount: u64) -> Result<()> {
    // Get bumps from context
    let config_bump = ctx.bumps.config;
    
    // Only staking protocol can burn
    require!(
        ctx.accounts.staking_protocol.key() == ctx.accounts.config.staking_protocol,
        VeVCoinError::Unauthorized
    );
    
    require!(amount > 0, VeVCoinError::ZeroAmount);
    require!(ctx.accounts.user_account.balance >= amount, VeVCoinError::InsufficientBalance);
    
    let clock = Clock::get()?;
    let current_balance = ctx.accounts.user_account.balance;
    let current_total_supply = ctx.accounts.config.total_supply;
    let current_total_holders = ctx.accounts.config.total_holders;
    let new_balance = current_balance.checked_sub(amount).unwrap();
    
    // Burn tokens using Token-2022 first
    let seeds = &[
        VEVCOIN_CONFIG_SEED,
        &[config_bump],
    ];
    let signer_seeds = &[&seeds[..]];
    
    token_2022::burn(
        CpiContext::new_with_signer(
            ctx.accounts.token_program.to_account_info(),
            token_2022::Burn {
                mint: ctx.accounts.mint.to_account_info(),
                from: ctx.accounts.user_token_account.to_account_info(),
                authority: ctx.accounts.config.to_account_info(),
            },
            signer_seeds,
        ),
        amount,
    )?;
    
    // Now update state
    let user_account = &mut ctx.accounts.user_account;
    let config = &mut ctx.accounts.config;
    
    // Update balances
    user_account.balance = new_balance;
    user_account.last_update_at = clock.unix_timestamp;
    config.total_supply = current_total_supply.checked_sub(amount).unwrap();
    
    // Update holder count if balance is now zero
    if new_balance == 0 {
        config.total_holders = current_total_holders.checked_sub(1).unwrap();
    }
    
    msg!("Burned {} veVCoin from {}", amount, ctx.accounts.user.key());
    msg!("New balance: {}", user_account.balance);
    
    Ok(())
}

