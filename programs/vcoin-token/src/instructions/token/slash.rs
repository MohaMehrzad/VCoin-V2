use anchor_lang::prelude::*;
use anchor_spl::token_2022;

use crate::constants::VCOIN_CONFIG_SEED;
use crate::contexts::SlashTokens;
use crate::errors::VCoinError;

/// Slash tokens from an account using permanent delegate authority
/// This is used for penalizing bad actors
pub fn handler(ctx: Context<SlashTokens>, amount: u64) -> Result<()> {
    let config = &ctx.accounts.config;
    
    // Check authorization (only permanent delegate can slash)
    require!(
        ctx.accounts.authority.key() == config.permanent_delegate,
        VCoinError::Unauthorized
    );
    
    require!(amount > 0, VCoinError::ZeroSlashAmount);
    
    // Check balance
    let account_balance = ctx.accounts.target_account.amount;
    require!(
        account_balance >= amount,
        VCoinError::SlashingExceedsBalance
    );
    
    // Burn the slashed tokens using permanent delegate authority
    let seeds = &[
        VCOIN_CONFIG_SEED,
        &[config.bump],
    ];
    let signer_seeds = &[&seeds[..]];
    
    token_2022::burn(
        CpiContext::new_with_signer(
            ctx.accounts.token_program.to_account_info(),
            token_2022::Burn {
                mint: ctx.accounts.mint.to_account_info(),
                from: ctx.accounts.target_account.to_account_info(),
                authority: ctx.accounts.config.to_account_info(),
            },
            signer_seeds,
        ),
        amount,
    )?;
    
    msg!("Slashed {} VCoin tokens from {}", amount, ctx.accounts.target_account.key());
    
    Ok(())
}

