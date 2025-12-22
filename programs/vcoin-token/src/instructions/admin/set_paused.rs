use anchor_lang::prelude::*;

use crate::contexts::UpdateConfig;
use crate::errors::VCoinError;

/// Pause/unpause token operations
/// Only authority can pause
pub fn handler(ctx: Context<UpdateConfig>, paused: bool) -> Result<()> {
    let config = &mut ctx.accounts.config;
    
    require!(
        ctx.accounts.authority.key() == config.authority,
        VCoinError::Unauthorized
    );
    
    config.paused = paused;
    
    msg!("Token paused status: {}", paused);
    
    Ok(())
}

