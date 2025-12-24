use anchor_lang::prelude::*;
use crate::constants::*;
use crate::state::{GovernanceConfig, Proposal};

#[derive(Accounts)]
pub struct CreateProposal<'info> {
    #[account(
        mut,
        seeds = [GOV_CONFIG_SEED],
        bump = governance_config.bump
    )]
    pub governance_config: Account<'info, GovernanceConfig>,
    
    #[account(
        init,
        payer = proposer,
        space = Proposal::LEN,
        seeds = [PROPOSAL_SEED, (governance_config.proposal_count + 1).to_le_bytes().as_ref()],
        bump
    )]
    pub proposal: Account<'info, Proposal>,
    
    #[account(mut)]
    pub proposer: Signer<'info>,
    
    /// H-NEW-05: UserStake account from staking-protocol for proposal threshold verification
    /// CHECK: Verified in handler via PDA derivation from staking_program
    #[account()]
    pub proposer_stake: AccountInfo<'info>,
    
    pub system_program: Program<'info, System>,
}

