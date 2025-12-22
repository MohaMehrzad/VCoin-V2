use anchor_lang::prelude::*;
use anchor_spl::token_2022::Token2022;
use anchor_spl::token_interface::{Mint, TokenAccount};

use crate::constants::{STAKING_POOL_SEED, POOL_VAULT_SEED};
use crate::state::StakingPool;

#[derive(Accounts)]
pub struct InitializePool<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,
    
    #[account(
        init,
        payer = authority,
        space = StakingPool::LEN,
        seeds = [STAKING_POOL_SEED],
        bump
    )]
    pub pool: Account<'info, StakingPool>,
    
    /// VCoin mint
    pub vcoin_mint: InterfaceAccount<'info, Mint>,
    
    /// veVCoin mint
    pub vevcoin_mint: InterfaceAccount<'info, Mint>,
    
    /// Pool vault for staked VCoin
    #[account(
        init,
        payer = authority,
        seeds = [POOL_VAULT_SEED],
        bump,
        token::mint = vcoin_mint,
        token::authority = pool,
        token::token_program = token_program,
    )]
    pub pool_vault: InterfaceAccount<'info, TokenAccount>,
    
    pub token_program: Program<'info, Token2022>,
    pub system_program: Program<'info, System>,
}

