use anchor_lang::prelude::*;

use crate::contexts::InitializeMint;
use crate::events::VCoinInitialized;

/// Initialize the VCoin mint with Token-2022 extensions
/// This creates the mint with:
/// - Metadata extension
/// - Permanent delegate extension (for slashing)
pub fn handler(ctx: Context<InitializeMint>, permanent_delegate: Pubkey) -> Result<()> {
    let config = &mut ctx.accounts.config;
    
    config.authority = ctx.accounts.authority.key();
    config.mint = ctx.accounts.mint.key();
    config.treasury = ctx.accounts.treasury.key();
    config.permanent_delegate = permanent_delegate;
    config.total_minted = 0;
    config.paused = false;
    config.bump = ctx.bumps.config;
    
    let clock = Clock::get()?;
    
    // L-01: Emit initialization event
    emit!(VCoinInitialized {
        authority: config.authority,
        mint: config.mint,
        permanent_delegate,
        timestamp: clock.unix_timestamp,
    });
    
    msg!("VCoin mint initialized");
    msg!("Mint: {}", config.mint);
    msg!("Authority: {}", config.authority);
    msg!("Permanent Delegate: {}", permanent_delegate);
    
    Ok(())
}

