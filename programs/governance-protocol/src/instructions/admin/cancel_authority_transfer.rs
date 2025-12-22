use anchor_lang::prelude::*;
use crate::contexts::UpdateAuthority;
use crate::errors::GovernanceError;

/// Cancel a pending authority transfer (H-02 security fix)
pub fn handler(ctx: Context<UpdateAuthority>) -> Result<()> {
    let config = &mut ctx.accounts.governance_config;
    
    require!(
        config.pending_authority != Pubkey::default(),
        GovernanceError::NoPendingTransfer
    );
    
    let cancelled = config.pending_authority;
    config.pending_authority = Pubkey::default();
    
    msg!("Authority transfer to {} cancelled", cancelled);
    
    Ok(())
}

