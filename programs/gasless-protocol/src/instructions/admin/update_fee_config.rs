use anchor_lang::prelude::*;
use crate::contexts::UpdateConfig;

pub fn handler(
    ctx: Context<UpdateConfig>,
    sol_fee_per_tx: u64,
    vcoin_fee_multiplier: u64,
    sscre_deduction_bps: u16,
) -> Result<()> {
    let config = &mut ctx.accounts.config;
    
    config.sol_fee_per_tx = sol_fee_per_tx;
    config.vcoin_fee_multiplier = vcoin_fee_multiplier;
    config.sscre_deduction_bps = sscre_deduction_bps;
    
    msg!("Fee config updated: SOL={}, mult={}, SSCRE={}bps",
        sol_fee_per_tx, vcoin_fee_multiplier, sscre_deduction_bps);
    Ok(())
}

