use anchor_lang::prelude::*;
use anchor_spl::token_2022::Token2022;
use anchor_spl::token_interface::{Mint, TokenAccount};

use crate::constants::VCOIN_CONFIG_SEED;
use crate::errors::VCoinError;
use crate::state::VCoinConfig;

#[derive(Accounts)]
pub struct MintTokens<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,
    
    #[account(
        mut,
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
    
    /// Destination token account - MUST use VCoin mint
    #[account(
        mut,
        constraint = destination.mint == config.mint @ VCoinError::InvalidMint
    )]
    pub destination: InterfaceAccount<'info, TokenAccount>,
    
    pub token_program: Program<'info, Token2022>,
}

