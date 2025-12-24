use anchor_lang::prelude::*;
use anchor_spl::token_2022::Token2022;
use anchor_spl::token_interface::{Mint, TokenAccount};

use crate::constants::{USER_STAKE_SEED, STAKING_POOL_SEED};
use crate::errors::StakingError;
use crate::state::{UserStake, StakingPool};

/// C-04 Security Fix: Added veVCoin CPI accounts for minting additional veVCoin
#[derive(Accounts)]
pub struct ExtendLock<'info> {
    #[account(mut)]
    pub user: Signer<'info>,
    
    #[account(
        mut,
        seeds = [USER_STAKE_SEED, user.key().as_ref()],
        bump,
        constraint = user_stake.owner == user.key()
    )]
    pub user_stake: Account<'info, UserStake>,
    
    /// Pool account for CPI signer seeds
    #[account(
        seeds = [STAKING_POOL_SEED],
        bump
    )]
    pub pool: Account<'info, StakingPool>,
    
    // === veVCoin accounts for CPI (C-04 Security Fix) ===
    
    /// veVCoin mint
    #[account(
        mut,
        constraint = vevcoin_mint.key() == pool.vevcoin_mint @ StakingError::InvalidMint
    )]
    pub vevcoin_mint: InterfaceAccount<'info, Mint>,
    
    /// User's veVCoin token account (Token-2022)
    #[account(
        mut,
        constraint = user_vevcoin_account.owner == user.key() @ StakingError::InvalidTokenAccount,
        constraint = user_vevcoin_account.mint == pool.vevcoin_mint @ StakingError::InvalidMint
    )]
    pub user_vevcoin_account: InterfaceAccount<'info, TokenAccount>,
    
    /// UserVeVCoin PDA tracking account
    /// CHECK: Validated in vevcoin program CPI
    #[account(mut)]
    pub user_vevcoin: UncheckedAccount<'info>,
    
    /// veVCoin config PDA
    /// CHECK: Validated in vevcoin program CPI
    #[account(mut)]
    pub vevcoin_config: UncheckedAccount<'info>,
    
    /// veVCoin program for CPI
    pub vevcoin_program: Program<'info, vevcoin_token::program::VevcoinToken>,
    
    pub token_program: Program<'info, Token2022>,
    pub system_program: Program<'info, System>,
}
