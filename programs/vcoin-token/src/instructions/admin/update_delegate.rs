use anchor_lang::prelude::*;

use crate::contexts::UpdateConfig;
use crate::errors::VCoinError;

/// Update the permanent delegate (for slashing)
pub fn handler(ctx: Context<UpdateConfig>, new_delegate: Pubkey) -> Result<()> {
    let config = &mut ctx.accounts.config;
    
    require!(
        ctx.accounts.authority.key() == config.authority,
        VCoinError::Unauthorized
    );
    
    // M-13: Validate delegate address is not zero
    require!(
        new_delegate != Pubkey::default(),
        VCoinError::InvalidAuthority
    );

    config.permanent_delegate = new_delegate;

    msg!("Permanent delegate updated to: {}", new_delegate);
    
    Ok(())
}

