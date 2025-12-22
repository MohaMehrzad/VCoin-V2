use anchor_lang::prelude::*;

use crate::contexts::UpdateConfig;
use crate::errors::VCoinError;

/// Update the authority
pub fn handler(ctx: Context<UpdateConfig>, new_authority: Pubkey) -> Result<()> {
    let config = &mut ctx.accounts.config;
    
    require!(
        ctx.accounts.authority.key() == config.authority,
        VCoinError::Unauthorized
    );
    
    config.authority = new_authority;
    
    msg!("Authority updated to: {}", new_authority);
    
    Ok(())
}

