use anchor_lang::prelude::*;
use anchor_spl::token_2022::Token2022;
use anchor_spl::token_interface::{Mint, TokenAccount};

use crate::constants::VCOIN_CONFIG_SEED;
use crate::errors::VCoinError;
use crate::state::VCoinConfig;

#[derive(Accounts)]
pub struct SlashTokens<'info> {
    /// The permanent delegate authority
    #[account(mut)]
    pub authority: Signer<'info>,
    
    #[account(
        seeds = [VCOIN_CONFIG_SEED],
        bump = config.bump
    )]
    pub config: Account<'info, VCoinConfig>,
    
    /// The VCoin mint
    #[account(
        mut,
        constraint = mint.key() == config.mint @ VCoinError::InvalidMint
    )]
    pub mint: InterfaceAccount<'info, Mint>,
    
    /// Target account to slash tokens from - MUST use VCoin mint
    #[account(
        mut,
        constraint = target_account.mint == config.mint @ VCoinError::InvalidMint
    )]
    pub target_account: InterfaceAccount<'info, TokenAccount>,
    
    pub token_program: Program<'info, Token2022>,
}

