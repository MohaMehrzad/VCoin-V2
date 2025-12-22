use anchor_lang::prelude::*;

use crate::contexts::AcceptAuthority;

/// Accept authority transfer (step 2 of two-step transfer - H-02 security fix)
pub fn handler(ctx: Context<AcceptAuthority>) -> Result<()> {
    let config = &mut ctx.accounts.hook_config;
    
    let old_authority = config.authority;
    let new_authority = ctx.accounts.new_authority.key();
    
    config.authority = new_authority;
    config.pending_authority = Pubkey::default();
    
    msg!("Authority transferred from {} to {}", old_authority, new_authority);
    
    Ok(())
}

