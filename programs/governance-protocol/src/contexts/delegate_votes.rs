use anchor_lang::prelude::*;
use crate::constants::*;
use crate::state::{Delegation, DelegateStats, GovernanceConfig};

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

    /// H-NEW-03: Governance config for staking program reference
    #[account(
        seeds = [GOV_CONFIG_SEED],
        bump
    )]
    pub config: Account<'info, GovernanceConfig>,

    /// H-NEW-03: Delegator's staking account to verify veVCoin balance
    /// CHECK: PDA validation performed in handler
    pub user_stake: AccountInfo<'info>,

    pub system_program: Program<'info, System>,
}

