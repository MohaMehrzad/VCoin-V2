use anchor_lang::prelude::*;
use crate::contexts::DelegateVotes;
use crate::errors::GovernanceError;
use crate::events::DelegationCreated;

pub fn handler(
    ctx: Context<DelegateVotes>,
    delegation_type: u8,
    categories: u8,
    vevcoin_amount: u64,
    expires_at: i64,
    revocable: bool,
) -> Result<()> {
    let delegator_key = ctx.accounts.delegator.key();
    let delegate_key = ctx.accounts.delegate.key();
    
    require!(delegator_key != delegate_key, GovernanceError::CannotDelegateSelf);
    
    let clock = Clock::get()?;
    
    let delegation = &mut ctx.accounts.delegation;
    delegation.delegator = delegator_key;
    delegation.delegate = delegate_key;
    delegation.delegation_type = delegation_type;
    delegation.categories = categories;
    delegation.delegated_amount = vevcoin_amount;
    delegation.delegated_at = clock.unix_timestamp;
    delegation.expires_at = expires_at;
    delegation.revocable = revocable;
    delegation.bump = ctx.bumps.delegation;
    
    // Update delegate stats
    let delegate_stats = &mut ctx.accounts.delegate_stats;
    delegate_stats.delegate = delegate_key;
    delegate_stats.unique_delegators = delegate_stats.unique_delegators.saturating_add(1);
    delegate_stats.total_delegated_vevcoin = delegate_stats
        .total_delegated_vevcoin
        .saturating_add(vevcoin_amount);
    delegate_stats.bump = ctx.bumps.delegate_stats;
    
    emit!(DelegationCreated {
        delegator: delegator_key,
        delegate: delegate_key,
        amount: vevcoin_amount,
        delegation_type,
    });
    
    msg!("Delegation created: {} veVCoin", vevcoin_amount);
    Ok(())
}

