use anchor_lang::prelude::*;

use crate::contexts::UpdateAuthority;
use crate::errors::FiveAError;

/// Propose a new authority (step 1 of two-step transfer - H-02 security fix)
pub fn handler(ctx: Context<UpdateAuthority>, new_authority: Pubkey) -> Result<()> {
    let config = &mut ctx.accounts.five_a_config;
    
    require!(
        new_authority != config.authority,
        FiveAError::CannotProposeSelf
    );
    
    require!(
        new_authority != Pubkey::default(),
        FiveAError::InvalidAuthority
    );
    
    config.pending_authority = new_authority;
    
    msg!("Authority transfer proposed to: {}", new_authority);
    Ok(())
}

