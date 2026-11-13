use anchor_lang::prelude::*;
use anchor_spl::token_interface::{mint_to, MintTo};

use crate::constants::*;
use crate::contexts::MintTokens;
use crate::errors::VCoinError;
use crate::events::TokensMinted;

pub fn handler(ctx: Context<MintTokens>, amount: u64) -> Result<()> {
    let config = &ctx.accounts.config;
    config
        .admin
        .require_authority(&ctx.accounts.authority.key())?;
    require!(!config.paused, VCoinError::TokenPaused);
    require!(amount > 0, VCoinError::ZeroAmount);

    // Cap checked against the mint's real supply (audit MON-8): supply minted by any
    // path counts, and slashed/burned tokens can't be silently re-minted past the cap.
    let new_supply = ctx
        .accounts
        .mint
        .supply
        .checked_add(amount)
        .ok_or(VCoinError::ExceedsMaxSupply)?;
    require!(new_supply <= TOTAL_SUPPLY, VCoinError::ExceedsMaxSupply);

    let bump = [config.bump];
    let signer_seeds: &[&[u8]] = &[VCOIN_CONFIG_SEED, &bump];
    mint_to(
        CpiContext::new_with_signer(
            ctx.accounts.token_program.to_account_info(),
            MintTo {
                mint: ctx.accounts.mint.to_account_info(),
                to: ctx.accounts.destination.to_account_info(),
                authority: ctx.accounts.config.to_account_info(),
            },
            &[signer_seeds],
        ),
        amount,
    )?;

    emit!(TokensMinted {
        destination: ctx.accounts.destination.key(),
        amount,
        new_supply,
    });
    Ok(())
}
