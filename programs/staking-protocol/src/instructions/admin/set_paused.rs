use anchor_lang::prelude::*;

use crate::contexts::AdminAction;
use crate::errors::StakingError;
use crate::events::{PoolPaused, PoolUnpaused};

/// Pause/unpause the staking pool
pub fn handler(ctx: Context<AdminAction>, paused: bool) -> Result<()> {
    let pool = &mut ctx.accounts.pool;
    
    require!(
        ctx.accounts.authority.key() == pool.authority,
        StakingError::Unauthorized
    );
    
    pool.paused = paused;
    
    let clock = Clock::get()?;
    
    // L-01: Emit pause/unpause event
    if paused {
        emit!(PoolPaused {
            authority: ctx.accounts.authority.key(),
            timestamp: clock.unix_timestamp,
        });
    } else {
        emit!(PoolUnpaused {
            authority: ctx.accounts.authority.key(),
            timestamp: clock.unix_timestamp,
        });
    }
    
    msg!("Pool paused status: {}", paused);
    
    Ok(())
}

