use anchor_lang::prelude::*;

use crate::contexts::AdminAction;
use crate::errors::StakingError;

/// Pause/unpause the staking pool
pub fn handler(ctx: Context<AdminAction>, paused: bool) -> Result<()> {
    let pool = &mut ctx.accounts.pool;
    
    require!(
        ctx.accounts.authority.key() == pool.authority,
        StakingError::Unauthorized
    );
    
    pool.paused = paused;
    
    msg!("Pool paused status: {}", paused);
    
    Ok(())
}

