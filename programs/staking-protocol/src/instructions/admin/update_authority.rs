use anchor_lang::prelude::*;

use crate::contexts::AdminAction;
use crate::errors::StakingError;

/// Update the pool authority
pub fn handler(ctx: Context<AdminAction>, new_authority: Pubkey) -> Result<()> {
    let pool = &mut ctx.accounts.pool;
    
    require!(
        ctx.accounts.authority.key() == pool.authority,
        StakingError::Unauthorized
    );
    
    pool.authority = new_authority;
    
    msg!("Authority updated to: {}", new_authority);
    
    Ok(())
}

