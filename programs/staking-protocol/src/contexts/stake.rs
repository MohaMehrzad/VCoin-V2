use anchor_lang::prelude::*;
use anchor_spl::token_2022::Token2022;
use anchor_spl::token_interface::{Mint, TokenAccount};

use crate::constants::{STAKING_POOL_SEED, USER_STAKE_SEED, POOL_VAULT_SEED};
use crate::errors::StakingError;
use crate::state::{StakingPool, UserStake};

#[derive(Accounts)]
pub struct Stake<'info> {
    #[account(mut)]
    pub user: Signer<'info>,
    
    #[account(
        mut,
        seeds = [STAKING_POOL_SEED],
        bump
    )]
    pub pool: Account<'info, StakingPool>,
    
    #[account(
        init_if_needed,
        payer = user,
        space = UserStake::LEN,
        seeds = [USER_STAKE_SEED, user.key().as_ref()],
        bump
    )]
    pub user_stake: Account<'info, UserStake>,
    
    /// VCoin mint
    #[account(constraint = vcoin_mint.key() == pool.vcoin_mint @ StakingError::InvalidMint)]
    pub vcoin_mint: InterfaceAccount<'info, Mint>,
    
    /// User's VCoin token account - MUST be owned by user and use VCoin mint
    #[account(
        mut,
        constraint = user_vcoin_account.owner == user.key() @ StakingError::InvalidTokenAccount,
        constraint = user_vcoin_account.mint == pool.vcoin_mint @ StakingError::InvalidMint
    )]
    pub user_vcoin_account: InterfaceAccount<'info, TokenAccount>,
    
    /// Pool vault for staked VCoin
    /// 
    /// M-06 Security Note: Current seed derivation uses only POOL_VAULT_SEED.
    /// This works for single-pool design but would cause collisions in multi-pool.
    /// 
    /// For future multi-pool support, update seed to include pool identifier:
    /// seeds = [POOL_VAULT_SEED, pool.key().as_ref()]
    /// 
    /// This would require migration strategy for existing vault accounts.
    #[account(
        mut,
        seeds = [POOL_VAULT_SEED],
        bump,
        constraint = pool_vault.mint == pool.vcoin_mint @ StakingError::InvalidMint
    )]
    pub pool_vault: InterfaceAccount<'info, TokenAccount>,
    
    // === veVCoin accounts for CPI ===
    
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

