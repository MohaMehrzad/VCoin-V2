use anchor_lang::prelude::*;
use crate::constants::*;
use crate::state::{Delegation, DelegateStats};

#[derive(Accounts)]
pub struct RevokeDelegation<'info> {
    #[account(
        mut,
        close = delegator,
        seeds = [DELEGATION_SEED, delegator.key().as_ref()],
        bump = delegation.bump,
        has_one = delegator
    )]
    pub delegation: Account<'info, Delegation>,
    
    #[account(
        mut,
        seeds = [DELEGATE_STATS_SEED, delegation.delegate.as_ref()],
        bump = delegate_stats.bump
    )]
    pub delegate_stats: Account<'info, DelegateStats>,
    
    #[account(mut)]
    pub delegator: Signer<'info>,
}

