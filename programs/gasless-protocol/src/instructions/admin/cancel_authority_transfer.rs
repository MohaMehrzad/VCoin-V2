use anchor_lang::prelude::*;
use crate::contexts::UpdateAuthority;
use crate::errors::GaslessError;

/// Cancel a pending authority transfer (H-02 security fix)
pub fn handler(ctx: Context<UpdateAuthority>) -> Result<()> {
    let config = &mut ctx.accounts.config;
    
    require!(
        config.pending_authority != Pubkey::default(),
        GaslessError::NoPendingTransfer
    );
    
    let cancelled = config.pending_authority;
    config.pending_authority = Pubkey::default();
    
    msg!("Authority transfer to {} cancelled", cancelled);
    
    Ok(())
}

