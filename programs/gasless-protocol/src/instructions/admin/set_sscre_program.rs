use anchor_lang::prelude::*;
use crate::contexts::UpdateConfig;

pub fn handler(ctx: Context<UpdateConfig>, sscre_program: Pubkey) -> Result<()> {
    ctx.accounts.config.sscre_program = sscre_program;
    msg!("SSCRE program set to: {}", sscre_program);
    Ok(())
}

