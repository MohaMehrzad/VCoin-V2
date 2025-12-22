use anchor_lang::prelude::*;

use crate::contexts::UpdateConfig;
use crate::errors::VeVCoinError;
use crate::events::StakingProtocolUpdated;

/// Update the staking protocol address (only authority)
pub fn handler(ctx: Context<UpdateConfig>, new_staking_protocol: Pubkey) -> Result<()> {
    let config = &mut ctx.accounts.config;
    
    require!(
        ctx.accounts.authority.key() == config.authority,
        VeVCoinError::Unauthorized
    );
    
    let old_staking_protocol = config.staking_protocol;
    config.staking_protocol = new_staking_protocol;
    
    let clock = Clock::get()?;
    
    // L-01: Emit staking protocol updated event
    emit!(StakingProtocolUpdated {
        old_staking_protocol,
        new_staking_protocol,
        timestamp: clock.unix_timestamp,
    });
    
    msg!("Staking protocol updated to: {}", new_staking_protocol);
    
    Ok(())
}

