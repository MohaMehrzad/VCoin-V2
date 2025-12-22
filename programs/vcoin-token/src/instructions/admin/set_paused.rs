use anchor_lang::prelude::*;

use crate::contexts::UpdateConfig;
use crate::errors::VCoinError;
use crate::events::{ProtocolPaused, ProtocolUnpaused};

/// Pause/unpause token operations
/// Only authority can pause
pub fn handler(ctx: Context<UpdateConfig>, paused: bool) -> Result<()> {
    let config = &mut ctx.accounts.config;
    
    require!(
        ctx.accounts.authority.key() == config.authority,
        VCoinError::Unauthorized
    );
    
    config.paused = paused;
    
    let clock = Clock::get()?;
    
    // L-01: Emit pause/unpause event
    if paused {
        emit!(ProtocolPaused {
            authority: ctx.accounts.authority.key(),
            timestamp: clock.unix_timestamp,
        });
    } else {
        emit!(ProtocolUnpaused {
            authority: ctx.accounts.authority.key(),
            timestamp: clock.unix_timestamp,
        });
    }
    
    msg!("Token paused status: {}", paused);
    
    Ok(())
}

