use anchor_lang::prelude::*;
use anchor_spl::token_2022::Token2022;
use anchor_spl::token_interface::{Mint, TokenAccount};
use crate::constants::*;
use crate::errors::GaslessError;
use crate::state::{GaslessConfig, UserGaslessStats};

#[derive(Accounts)]
pub struct DeductVCoinFee<'info> {
    #[account(
        mut,
        seeds = [GASLESS_CONFIG_SEED],
        bump = config.bump
    )]
    pub config: Account<'info, GaslessConfig>,
    
    #[account(
        init_if_needed,
        payer = user,
        space = UserGaslessStats::LEN,
        seeds = [USER_GASLESS_SEED, user.key().as_ref()],
        bump
    )]
    pub user_stats: Account<'info, UserGaslessStats>,
    
    #[account(constraint = vcoin_mint.key() == config.vcoin_mint @ GaslessError::InvalidMint)]
    pub vcoin_mint: InterfaceAccount<'info, Mint>,
    
    /// User's VCoin account - MUST be owned by user and use VCoin mint
    #[account(
        mut,
        constraint = user_token_account.owner == user.key() @ GaslessError::InvalidTokenAccount,
        constraint = user_token_account.mint == config.vcoin_mint @ GaslessError::InvalidMint
    )]
    pub user_token_account: InterfaceAccount<'info, TokenAccount>,
    
    /// Fee vault - PDA owned vault with VCoin mint
    #[account(
        mut,
        seeds = [FEE_VAULT_SEED],
        bump,
        constraint = fee_vault.mint == config.vcoin_mint @ GaslessError::InvalidMint
    )]
    pub fee_vault: InterfaceAccount<'info, TokenAccount>,
    
    #[account(mut)]
    pub user: Signer<'info>,
    
    pub token_program: Program<'info, Token2022>,
    pub system_program: Program<'info, System>,
}

