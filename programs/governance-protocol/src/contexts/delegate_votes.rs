use anchor_lang::prelude::*;
use crate::constants::*;
use crate::state::{Delegation, DelegateStats};

#[derive(Accounts)]
pub struct DelegateVotes<'info> {
    #[account(
        init,
        payer = delegator,
        space = Delegation::LEN,
        seeds = [DELEGATION_SEED, delegator.key().as_ref()],
        bump
    )]
    pub delegation: Account<'info, Delegation>,
    
    #[account(
        init_if_needed,
        payer = delegator,
        space = DelegateStats::LEN,
        seeds = [DELEGATE_STATS_SEED, delegate.key().as_ref()],
        bump
    )]
    pub delegate_stats: Account<'info, DelegateStats>,
    
    #[account(mut)]
    pub delegator: Signer<'info>,
    
    /// CHECK: Delegate receiving voting power
    pub delegate: UncheckedAccount<'info>,
    
    pub system_program: Program<'info, System>,
}

