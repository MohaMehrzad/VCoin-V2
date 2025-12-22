use anchor_lang::prelude::*;

use crate::contexts::UpdateConfig;
use crate::errors::IdentityError;

/// Remove trusted attester (admin only)
pub fn handler(ctx: Context<UpdateConfig>, attester: Pubkey) -> Result<()> {
    let config = &mut ctx.accounts.identity_config;
    
    // Find and remove the attester
    let mut found = false;
    let count = config.attester_count as usize;
    for i in 0..count {
        if config.trusted_attesters[i] == attester {
            // Shift remaining attesters
            let last_idx = (config.attester_count - 1) as usize;
            for j in i..last_idx {
                config.trusted_attesters[j] = config.trusted_attesters[j + 1];
            }
            config.trusted_attesters[last_idx] = Pubkey::default();
            config.attester_count -= 1;
            found = true;
            break;
        }
    }
    
    require!(found, IdentityError::UntrustedAttester);
    
    msg!("Trusted attester removed: {}", attester);
    Ok(())
}

