use anchor_lang::prelude::*;
use crate::contexts::UpdateAuthority;

pub fn handler(ctx: Context<UpdateAuthority>, new_authority: Pubkey) -> Result<()> {
    ctx.accounts.config.authority = new_authority;
    msg!("Authority updated to: {}", new_authority);
    Ok(())
}

