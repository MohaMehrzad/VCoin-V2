use anchor_lang::prelude::*;
use crate::contexts::UpdateConfig;

pub fn handler(ctx: Context<UpdateConfig>, new_fee_payer: Pubkey) -> Result<()> {
    ctx.accounts.config.fee_payer = new_fee_payer;
    msg!("Fee payer updated to: {}", new_fee_payer);
    Ok(())
}

