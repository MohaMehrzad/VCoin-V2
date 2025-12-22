use anchor_lang::prelude::*;

use crate::contexts::UpdateAuthority;

/// Update authority
pub fn handler(ctx: Context<UpdateAuthority>, new_authority: Pubkey) -> Result<()> {
    ctx.accounts.five_a_config.authority = new_authority;
    msg!("Authority updated to: {}", new_authority);
    Ok(())
}

