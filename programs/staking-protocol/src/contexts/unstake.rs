use anchor_lang::prelude::*;
use anchor_spl::token_2022::Token2022;
use anchor_spl::token_interface::{Mint, TokenAccount};

use crate::constants::{STAKING_POOL_SEED, USER_STAKE_SEED, POOL_VAULT_SEED};
use crate::errors::StakingError;
use crate::state::{StakingPool, UserStake};

#[derive(Accounts)]
pub struct Unstake<'info> {
    #[account(mut)]
    pub user: Signer<'info>,
    
    #[account(
        mut,
        seeds = [STAKING_POOL_SEED],
        bump
    )]
    pub pool: Account<'info, StakingPool>,
    
    #[account(
        mut,
        seeds = [USER_STAKE_SEED, user.key().as_ref()],
        bump,
        constraint = user_stake.owner == user.key()
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
    
    /// Pool vault
    #[account(
        mut,
        seeds = [POOL_VAULT_SEED],
        bump,
        constraint = pool_vault.mint == pool.vcoin_mint @ StakingError::InvalidMint
    )]
    pub pool_vault: InterfaceAccount<'info, TokenAccount>,
    
    pub token_program: Program<'info, Token2022>,
}

