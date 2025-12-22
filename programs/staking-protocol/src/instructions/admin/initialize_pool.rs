use anchor_lang::prelude::*;

use crate::contexts::InitializePool;

/// Initialize the staking pool
pub fn handler(ctx: Context<InitializePool>, vevcoin_program: Pubkey) -> Result<()> {
    let pool = &mut ctx.accounts.pool;
    
    pool.authority = ctx.accounts.authority.key();
    pool.vcoin_mint = ctx.accounts.vcoin_mint.key();
    pool.vevcoin_mint = ctx.accounts.vevcoin_mint.key();
    pool.vevcoin_program = vevcoin_program;
    pool.pool_vault = ctx.accounts.pool_vault.key();
    pool.total_staked = 0;
    pool.total_stakers = 0;
    pool.paused = false;
    pool.bump = ctx.bumps.pool;
    pool.vault_bump = ctx.bumps.pool_vault;
    
    msg!("Staking pool initialized");
    msg!("VCoin Mint: {}", pool.vcoin_mint);
    msg!("veVCoin Mint: {}", pool.vevcoin_mint);
    
    Ok(())
}

