use anchor_lang::prelude::*;

use crate::contexts::InitializeMint;

/// Initialize the veVCoin mint with Token-2022 Non-Transferable extension
pub fn handler(ctx: Context<InitializeMint>, staking_protocol: Pubkey) -> Result<()> {
    let config = &mut ctx.accounts.config;
    
    config.authority = ctx.accounts.authority.key();
    config.mint = ctx.accounts.mint.key();
    config.staking_protocol = staking_protocol;
    config.total_supply = 0;
    config.total_holders = 0;
    config.bump = ctx.bumps.config;
    
    msg!("veVCoin mint initialized");
    msg!("Mint: {}", config.mint);
    msg!("Staking Protocol: {}", staking_protocol);
    
    Ok(())
}

