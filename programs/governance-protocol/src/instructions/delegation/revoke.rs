use anchor_lang::prelude::*;
use crate::contexts::RevokeDelegation;

pub fn handler(ctx: Context<RevokeDelegation>) -> Result<()> {
    let delegation = &ctx.accounts.delegation;
    let delegate_stats = &mut ctx.accounts.delegate_stats;
    
    // Update delegate stats
    delegate_stats.unique_delegators = delegate_stats.unique_delegators.saturating_sub(1);
    delegate_stats.total_delegated_vevcoin = delegate_stats
        .total_delegated_vevcoin
        .saturating_sub(delegation.delegated_amount);
    
    msg!("Delegation revoked");
    Ok(())
}

