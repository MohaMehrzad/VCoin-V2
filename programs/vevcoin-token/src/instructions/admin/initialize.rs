use anchor_lang::prelude::*;

use crate::contexts::InitializeMint;
use crate::events::VeVCoinInitialized;

/// Initialize the veVCoin mint with Token-2022 Non-Transferable extension
pub fn handler(ctx: Context<InitializeMint>, staking_protocol: Pubkey) -> Result<()> {
    let config = &mut ctx.accounts.config;
    
    config.authority = ctx.accounts.authority.key();
    config.mint = ctx.accounts.mint.key();
    config.staking_protocol = staking_protocol;
    config.total_supply = 0;
    config.total_holders = 0;
    config.bump = ctx.bumps.config;
    
    let clock = Clock::get()?;
    
    // L-01: Emit initialization event
    emit!(VeVCoinInitialized {
        authority: config.authority,
        mint: config.mint,
        staking_protocol,
        timestamp: clock.unix_timestamp,
    });
    
    msg!("veVCoin mint initialized");
    msg!("Mint: {}", config.mint);
    msg!("Staking Protocol: {}", staking_protocol);
    
    Ok(())
}

