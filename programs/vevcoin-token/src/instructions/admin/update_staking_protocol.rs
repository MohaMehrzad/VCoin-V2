use anchor_lang::prelude::*;

use crate::contexts::UpdateConfig;
use crate::errors::VeVCoinError;

/// Update the staking protocol address (only authority)
pub fn handler(ctx: Context<UpdateConfig>, new_staking_protocol: Pubkey) -> Result<()> {
    let config = &mut ctx.accounts.config;
    
    require!(
        ctx.accounts.authority.key() == config.authority,
        VeVCoinError::Unauthorized
    );
    
    config.staking_protocol = new_staking_protocol;
    
    msg!("Staking protocol updated to: {}", new_staking_protocol);
    
    Ok(())
}

