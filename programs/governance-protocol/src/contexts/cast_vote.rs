use anchor_lang::prelude::*;
use crate::constants::*;
use crate::state::{Proposal, VoteRecord, Delegation, GovernanceConfig};

#[derive(Accounts)]
pub struct CastVote<'info> {
    /// C-NEW-01: GovernanceConfig to get staking_program and five_a_program addresses
    #[account(
        seeds = [GOV_CONFIG_SEED],
        bump = config.bump
    )]
    pub config: Account<'info, GovernanceConfig>,
    
    #[account(
        mut,
        seeds = [PROPOSAL_SEED, proposal.id.to_le_bytes().as_ref()],
        bump = proposal.bump
    )]
    pub proposal: Account<'info, Proposal>,
    
    #[account(
        init,
        payer = voter,
        space = VoteRecord::LEN,
        seeds = [VOTE_RECORD_SEED, proposal.key().as_ref(), voter.key().as_ref()],
        bump
    )]
    pub vote_record: Account<'info, VoteRecord>,
    
    #[account(mut)]
    pub voter: Signer<'info>,
    
    /// C-NEW-01: UserStake account from staking-protocol for on-chain tier and veVCoin verification
    /// CHECK: Verified in handler via PDA derivation from staking_program
    #[account()]
    pub user_stake: AccountInfo<'info>,
    
    /// C-NEW-01: UserScore account from five-a-protocol for on-chain 5A score verification
    /// CHECK: Verified in handler via PDA derivation from five_a_program
    #[account()]
    pub user_score: AccountInfo<'info>,
    
    /// M-07 Security Fix: Optional delegation account for voting on behalf of delegator
    /// If provided, expiry is validated in the handler
    pub delegation: Option<Account<'info, Delegation>>,
    
    pub system_program: Program<'info, System>,
}

