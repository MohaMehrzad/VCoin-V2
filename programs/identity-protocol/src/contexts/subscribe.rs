use anchor_lang::prelude::*;
use anchor_spl::token::{Mint, Token, TokenAccount};

use crate::constants::{IDENTITY_CONFIG_SEED, SUBSCRIPTION_SEED};
use crate::errors::IdentityError;
use crate::state::{IdentityConfig, Subscription};

#[derive(Accounts)]
pub struct Subscribe<'info> {
    #[account(
        seeds = [IDENTITY_CONFIG_SEED],
        bump = identity_config.bump
    )]
    pub identity_config: Account<'info, IdentityConfig>,

    #[account(
        init_if_needed,
        payer = user,
        space = Subscription::LEN,
        seeds = [SUBSCRIPTION_SEED, user.key().as_ref()],
        bump
    )]
    pub subscription: Account<'info, Subscription>,

    #[account(mut)]
    pub user: Signer<'info>,

    // C-AUDIT-22: USDC payment accounts
    #[account(
        constraint = usdc_mint.key() == identity_config.usdc_mint @ IdentityError::InvalidUsdcMint
    )]
    pub usdc_mint: Account<'info, Mint>,

    #[account(
        mut,
        constraint = user_token_account.owner == user.key() @ IdentityError::InvalidTokenAccountOwner,
        constraint = user_token_account.mint == identity_config.usdc_mint @ IdentityError::InvalidUsdcMint
    )]
    pub user_token_account: Account<'info, TokenAccount>,

    #[account(
        mut,
        constraint = treasury_token_account.owner == identity_config.treasury @ IdentityError::InvalidTreasury,
        constraint = treasury_token_account.mint == identity_config.usdc_mint @ IdentityError::InvalidUsdcMint
    )]
    pub treasury_token_account: Account<'info, TokenAccount>,

    pub token_program: Program<'info, Token>,

    pub system_program: Program<'info, System>,
}

