use anchor_lang::prelude::*;

use crate::contexts::UpdateConfig;
use crate::errors::IdentityError;

/// Add trusted attester (admin only)
pub fn handler(ctx: Context<UpdateConfig>, attester: Pubkey) -> Result<()> {
    let config = &mut ctx.accounts.identity_config;
    
    require!(config.attester_count < 10, IdentityError::Overflow);
    
    let idx = config.attester_count as usize;
    config.trusted_attesters[idx] = attester;
    config.attester_count += 1;
    
    msg!("Trusted attester added: {}", attester);
    Ok(())
}

