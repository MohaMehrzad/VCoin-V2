use anchor_lang::prelude::*;
use anchor_spl::token_2022::Token2022;
use anchor_spl::token_interface::{Mint, TokenAccount};

use crate::constants::VEVCOIN_CONFIG_SEED;
use crate::errors::VeVCoinError;
use crate::state::{VeVCoinConfig, UserVeVCoin};

#[derive(Accounts)]
#[instruction(amount: u64)]
pub struct MintVeVCoin<'info> {
    /// The staking protocol (must match config)
    pub staking_protocol: Signer<'info>,
    
    /// The user receiving veVCoin
    /// CHECK: Just a pubkey for PDA derivation
    pub user: UncheckedAccount<'info>,
    
    #[account(
        mut,
        seeds = [VEVCOIN_CONFIG_SEED],
        bump
    )]
    pub config: Account<'info, VeVCoinConfig>,
    
    #[account(
        init_if_needed,
        payer = payer,
        space = UserVeVCoin::LEN,
        seeds = [UserVeVCoin::SEED, user.key().as_ref()],
        bump
    )]
    pub user_account: Account<'info, UserVeVCoin>,
    
    /// The veVCoin mint
    #[account(
        mut,
        constraint = mint.key() == config.mint @ VeVCoinError::InvalidMint
    )]
    pub mint: InterfaceAccount<'info, Mint>,
    
    /// User's token account for veVCoin - MUST be owned by user and use veVCoin mint
    #[account(
        mut,
        constraint = user_token_account.owner == user.key() @ VeVCoinError::InvalidTokenAccount,
        constraint = user_token_account.mint == config.mint @ VeVCoinError::InvalidMint
    )]
    pub user_token_account: InterfaceAccount<'info, TokenAccount>,
    
    #[account(mut)]
    pub payer: Signer<'info>,
    
    pub token_program: Program<'info, Token2022>,
    pub system_program: Program<'info, System>,
}

